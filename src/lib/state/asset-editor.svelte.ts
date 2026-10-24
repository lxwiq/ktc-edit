import { invoke } from "@tauri-apps/api/core";
import type { JSONValue } from "$lib/types";
import { saveEditor } from "./save-editor.svelte";

export interface AssetSettings {
  griffinRunSpeed: number;
  griffinForestMultiplier: number;
  griffinRunStaminaRate: number;
  griffinSkillStaminaCost: number;
  griffinPushWidth: number;
  griffinPushHeight: number;
  lizardRunSpeed: number;
  lizardRunStaminaRate: number;
  lizardSkillStaminaCost: number;
  horseRunSpeed: number;
  horseRunStaminaRate: number;
  warhorseRunSpeed: number;
  warhorseRunStaminaRate: number;
  warhorsePlagueRunSpeed: number;
  warhorsePlagueRunStaminaRate: number;
  warhorseSkillStaminaCost: number;
  warhorseCooldown: number;
  warhorsePlagueCooldown: number;
  warhorseBuffDuration: number;
  warhorseBuffRange: number;
  archerWalkSpeed: number;
  archerRunSpeed: number;
  archerShootPrepTime: number;
  archerShootCooldownTime: number;
  archerShootCooldownWithKnightTime: number;
  archerIntervalMin: number;
  archerIntervalMax: number;
  archerFormationIntervalMin: number;
  archerFormationIntervalMax: number;
  builderWalkSpeed: number;
  builderRunSpeed: number;
  builderWorkTime: number;
  wall5StandardHitPoints: number;
  wall5NorselandsHitPoints: number;
  // null when this game build's layout is not recognized for these fields.
  knightWalkSpeed: number | null;
  peasantWalkSpeed: number | null;
  peasantRunSpeed: number | null;
  knightRunSpeed: number | null;
  knightRetreatSpeed: number | null;
  knightWalletCapacity: number;
  playerWalletCapacity: number;
  knightWalletPayTaxesAbove: number;
  knightCoinDropProbability: number;
  archerInvulnerable: boolean | null;
  builderInvulnerable: boolean | null;
  farmerInvulnerable: boolean | null;
  bagScale: number;
}

export type AssetNumberKey = {
  [K in keyof AssetSettings]: AssetSettings[K] extends boolean | null ? never : K;
}[keyof AssetSettings];

export type AssetBooleanKey = "archerInvulnerable" | "builderInvulnerable" | "farmerInvulnerable";

interface AssetResponse {
  dataDirectory: string;
  settings: AssetSettings;
}

interface AssetApplyResponse {
  resourcesBackup: string;
  sharedAssetsBackup: string;
  settings: AssetSettings;
  saveClearedWalls?: number | null;
}

interface ColliderLimitResponse {
  limit: number;
  backup: string | null;
}

const DIRECTORY_KEY = "ktcedit.gameDataDirectory";

/** Last game folder that loaded, so a folder picked by hand is found again on the next launch. */
function storedDirectory(): string | null {
  try {
    return localStorage.getItem(DIRECTORY_KEY);
  } catch {
    return null;
  }
}

function storeDirectory(directory: string | null) {
  try {
    if (directory) localStorage.setItem(DIRECTORY_KEY, directory);
    else localStorage.removeItem(DIRECTORY_KEY);
  } catch {
    // Storage unavailable: the game is still detected in the Steam libraries.
  }
}

async function loadAssets(directory: string | null): Promise<AssetResponse> {
  if (directory !== null) {
    return invoke<AssetResponse>("load_game_assets", { dataDirectory: directory });
  }
  const remembered = storedDirectory();
  if (remembered !== null) {
    try {
      return await invoke<AssetResponse>("load_game_assets", { dataDirectory: remembered });
    } catch {
      // The remembered folder moved or changed: fall back to detection below.
      storeDirectory(null);
    }
  }
  // Without a directory, the backend searches every Steam library.
  return invoke<AssetResponse>("load_game_assets", { dataDirectory: null });
}

class AssetEditor {
  active = $state(false);
  directory = $state<string | null>(null);
  settings = $state<AssetSettings | null>(null);
  busy = $state(false);
  status = $state<string | null>(null);
  error = $state<string | null>(null);
  backups = $state<string[]>([]);
  warhorseColliderLimit = $state<number | null>(null);
  coinVisualLimit = $state<number | null>(30);
  coinVisualCurrent = $state<number | null>(null);
  coinVisualAvailable = $state(false);
  coinVisualStatus = $state<string | null>(null);
  coinVisualError = $state<string | null>(null);
  campEnabled = $state(false);
  bloodMoonEnabled = $state(false);
  bloodMoonAvailable = $state(false);
  bloodMoonStatus = $state<string | null>(null);
  bloodMoonError = $state<string | null>(null);
  clockMultiplier = $state<number | null>(3);
  clockCurrent = $state<number | null>(null);
  clockAvailable = $state(false);
  clockStatus = $state<string | null>(null);
  clockError = $state<string | null>(null);
  campNeedsUpgrade = $state(false);
  campAvailable = $state(false);
  campStatus = $state<string | null>(null);
  campError = $state<string | null>(null);
  warhorseColliderBackup = $state<string | null>(null);
  warhorseColliderError = $state<string | null>(null);
  warhorseColliderStatus = $state<string | null>(null);
  resetSaveWalls = $state(true);
  unitResetStatus = $state<string | null>(null);
  unitResetError = $state<string | null>(null);

  close() {
    this.active = false;
    this.status = null;
    this.error = null;
  }

  async open(selectFolder = false) {
    this.active = true;
    this.busy = true;
    this.status = null;
    this.error = null;
    try {
      const response = selectFolder
        ? await invoke<AssetResponse>("select_game_data_directory")
        : await loadAssets(this.directory);
      storeDirectory(response.dataDirectory);
      if (this.directory !== response.dataDirectory) this.warhorseColliderBackup = null;
      this.directory = response.dataDirectory;
      this.settings = response.settings;
      this.status = "Loaded the current values from the game.";
      this.coinVisualAvailable = false;
      this.coinVisualStatus = null;
      this.coinVisualError = null;
      try {
        const coins = await invoke<{limit: number | null}>("load_coin_visual_limit", {dataDirectory: response.dataDirectory});
        this.coinVisualCurrent = coins.limit;
        this.coinVisualLimit = coins.limit ?? 30;
        this.coinVisualAvailable = true;
      } catch (error) { this.coinVisualError = String(error); }
      this.campAvailable = false;
      this.bloodMoonAvailable = false;
      this.bloodMoonStatus = null;
      this.bloodMoonError = null;
      try {
        const result = await invoke<{enabled: boolean}>("load_blood_moon", {dataDirectory: response.dataDirectory});
        this.bloodMoonEnabled = result.enabled;
        this.bloodMoonAvailable = true;
      } catch (error) { this.bloodMoonError = String(error); }
      this.clockAvailable = false;
      this.clockCurrent = null;
      this.clockStatus = null;
      this.clockError = null;
      try {
        this.clockCurrent = await invoke<number>("load_clock_speed", {dataDirectory: response.dataDirectory});
        this.clockMultiplier = this.clockCurrent;
        this.clockAvailable = true;
      } catch (error) { this.clockError = String(error); }
      this.campStatus = null;
      this.campError = null;
      try {
        const camp = await invoke<{enabled: boolean; needsUpgrade: boolean}>("load_camp_preservation", {dataDirectory: response.dataDirectory});
        this.campEnabled = camp.enabled;
        this.campNeedsUpgrade = camp.needsUpgrade;
        this.campAvailable = true;
      } catch (error) { this.campError = String(error); }
      this.warhorseColliderLimit = null;
      this.warhorseColliderError = null;
      this.warhorseColliderStatus = null;
      try {
        const collider = await invoke<ColliderLimitResponse>("load_warhorse_collider_limit", {
          dataDirectory: response.dataDirectory
        });
        this.warhorseColliderLimit = collider.limit;
      } catch (error) {
        this.warhorseColliderError = String(error);
      }
    } catch (error) {
      this.error = String(error);
    } finally {
      this.busy = false;
    }
  }

  async applyCoinVisualLimit(disable = false) {
    if (!this.directory || !this.coinVisualAvailable || this.busy) return;
    this.coinVisualError = null;
    this.coinVisualStatus = null;
    const limit = this.coinVisualLimit;
    if (!disable && (limit === null || !Number.isInteger(limit) || limit < 1 || limit > 65535)) {
      this.coinVisualError = "Enter a positive whole number. ARM64 immediate maximum: 65535.";
      return;
    }
    this.busy = true;
    try {
      const response = await invoke<{limit: number | null; backup: string | null}>("apply_coin_visual_limit", {
        dataDirectory: this.directory, limit: disable ? null : this.coinVisualLimit
      });
      this.coinVisualCurrent = response.limit;
      this.coinVisualStatus = (response.limit === null ? "Original coin display restored." : `Coin display limited to ${response.limit}.`) +
        (response.backup ? ` Backup: ${response.backup}` : " No changes needed.");
    } catch (error) { this.coinVisualError = String(error); }
    finally { this.busy = false; }
  }

  async applyCampPreservation(enabled: boolean) {
    if (!this.directory || !this.campAvailable || this.busy) return;
    this.campError = null;
    this.campStatus = null;
    this.busy = true;
    try {
      const response = await invoke<{enabled: boolean; backup: string | null}>("apply_camp_preservation", {
        dataDirectory: this.directory, enabled
      });
      this.campEnabled = response.enabled;
      this.campNeedsUpgrade = false;
      this.campStatus = (response.enabled ? "Vagrant camps are preserved when the forest disappears." : "Original camp removal restored.") +
        (response.backup ? ` Backup: ${response.backup}` : " No changes needed.");
    } catch (error) { this.campError = String(error); }
    finally { this.busy = false; }
  }

  async applyBloodMoon(enabled: boolean) {
    if (!this.directory || !this.bloodMoonAvailable || this.busy) return;
    this.busy = true;
    this.bloodMoonError = null;
    this.bloodMoonStatus = null;
    try {
      const response = await invoke<{enabled: boolean; backup: string | null; save_backup: string | null; converted_events: number; added_events: number; editor_data: JSONValue | null}>("apply_blood_moon", {
        dataDirectory: this.directory, enabled, savePath: saveEditor.filePath || null,
        editorData: saveEditor.data
      });
      this.bloodMoonEnabled = response.enabled;
      if (response.editor_data) saveEditor.data = response.editor_data;
      this.bloodMoonStatus = (response.enabled ? "Every-night Blood Moon patch enabled." : "Original day classification restored.") +
        (response.enabled ? ` Converted ${response.converted_events} pending Regular events; added ${response.added_events} Boss events toward ten per side/day.` : " Converted and added save events are not reverted; to undo them, restore a global-v35_<timestamp>.bak save backup made before enabling.") +
        (response.backup ? ` Native backup: ${response.backup}` : "") +
        (response.save_backup ? ` Save written and verified. Backup: ${response.save_backup}` : " Save unchanged.");
    } catch (error) { this.bloodMoonError = String(error); }
    finally { this.busy = false; }
  }

  async applyClockSpeed(multiplier: number | null) {
    if (!this.directory || !this.clockAvailable || this.busy) return;
    this.busy = true; this.clockStatus = null; this.clockError = null;
    try {
      const response = await invoke<{multiplier: number; native_backup: string | null; save_backup: string | null; updated_directors: number; editor_data: JSONValue | null}>("apply_clock_speed", {
        dataDirectory: this.directory, multiplier, savePath: saveEditor.filePath || null,
        editorData: saveEditor.data
      });
      this.clockMultiplier = response.multiplier;
      this.clockCurrent = response.multiplier;
      if (response.editor_data) saveEditor.data = response.editor_data;
      this.clockStatus = `ResumeTime now uses ×${response.multiplier}; updated ${response.updated_directors} saved island clocks.` +
        (response.native_backup ? ` Dylib backup: ${response.native_backup}` : "") +
        (response.save_backup ? ` Save backup: ${response.save_backup}` : "");
    } catch (error) { this.clockError = String(error); }
    finally { this.busy = false; }
  }

  async applyWarhorseColliderLimit() {
    if (!this.directory || this.warhorseColliderLimit === null) return;
    this.warhorseColliderError = null;
    this.warhorseColliderStatus = null;
    if (!Number.isInteger(this.warhorseColliderLimit) || this.warhorseColliderLimit < 1 || this.warhorseColliderLimit > 65535) {
      this.warhorseColliderError = "Enter a positive whole number. ARM64 immediate maximum: 65535.";
      return;
    }
    this.busy = true;
    try {
      const response = await invoke<ColliderLimitResponse>("apply_warhorse_collider_limit", {
        dataDirectory: this.directory,
        limit: this.warhorseColliderLimit
      });
      this.warhorseColliderLimit = response.limit;
      this.warhorseColliderBackup = response.backup;
      this.warhorseColliderStatus = response.backup
        ? `GameAssembly.dylib updated, signed, and verified. Backup: ${response.backup}`
        : "Collider limit is already set to this value.";
    } catch (error) {
      this.warhorseColliderError = String(error);
    } finally {
      this.busy = false;
    }
  }

  async restoreWarhorseColliderLimit() {
    if (!this.directory || !this.warhorseColliderBackup) return;
    this.warhorseColliderError = null;
    this.warhorseColliderStatus = null;
    this.busy = true;
    try {
      const response = await invoke<ColliderLimitResponse>("restore_warhorse_collider_limit", {
        dataDirectory: this.directory,
        backup: this.warhorseColliderBackup
      });
      this.warhorseColliderLimit = response.limit;
      this.warhorseColliderBackup = response.backup;
      this.warhorseColliderStatus = `Previous collider limit restored (${response.limit}).`;
    } catch (error) {
      this.warhorseColliderError = String(error);
    } finally {
      this.busy = false;
    }
  }

  async apply() {
    if (!this.directory || !this.settings) return;
    this.busy = true;
    this.status = null;
    this.error = null;
    try {
      const response = await invoke<AssetApplyResponse>("apply_game_assets", {
        dataDirectory: this.directory,
        settings: this.settings,
        resetSaveWalls: this.resetSaveWalls
      });
      this.settings = response.settings;
      this.backups = [response.resourcesBackup, response.sharedAssetsBackup];
      if (typeof response.saveClearedWalls === 'number' && response.saveClearedWalls > 0) {
        this.status = `Changes applied to assets. Also cleaned Damageable overrides on ${response.saveClearedWalls} walls in the save file.`;
      } else {
        this.status = "Changes applied and verified in both resources.assets and sharedassets0.assets.";
      }
    } catch (error) {
      this.error = String(error);
    } finally {
      this.busy = false;
    }
  }

  async restore() {
    if (!this.directory || this.backups.length !== 2) return;
    this.busy = true;
    this.status = null;
    this.error = null;
    try {
      const response = await invoke<AssetResponse>("restore_game_assets", {
        dataDirectory: this.directory,
        resourcesBackup: this.backups[0],
        sharedAssetsBackup: this.backups[1]
      });
      this.settings = response.settings;
      this.backups = [];
      this.status = "Backups restored. A safety backup of the previous state was also created.";
    } catch (error) {
      this.error = String(error);
    } finally {
      this.busy = false;
    }
  }

  async resetUnitDamageable() {
    if (this.busy || saveEditor.isLoading) return;
    this.busy = true;
    this.unitResetStatus = null;
    this.unitResetError = null;
    try {
      const filePath = saveEditor.filePath;
      const response = await invoke<{path: string; backup: string | null; clearedUnits: number; editorData: JSONValue | null}>("reset_unit_damageable_save", {
        path: filePath, editorData: filePath ? saveEditor.data : null
      });
      if (saveEditor.filePath === response.path && response.editorData !== null) saveEditor.data = response.editorData;
      this.unitResetStatus = `Saved directly to ${response.path}: removed Damageable overrides from ${response.clearedUnits} units. No extra Save step needed.` +
        (response.backup ? ` Backup: ${response.backup}.` : " No disk changes were needed.");
    } catch (error) { this.unitResetError = String(error); }
    finally { this.busy = false; }
  }

  setNumber(key: AssetNumberKey, value: number) {
    if (!this.settings) return;
    this.settings = { ...this.settings, [key]: value };
  }

  setBoolean(key: AssetBooleanKey, value: boolean) {
    if (!this.settings) return;
    this.settings = { ...this.settings, [key]: value };
  }
}

export const assetEditor = new AssetEditor();
