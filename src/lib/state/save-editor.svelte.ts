import { invoke } from "@tauri-apps/api/core";
import { toast } from "svelte-sonner";
import { detectLanguage, getTranslations, type Language } from "$lib/i18n";
import { getIslandOverview } from "$lib/saveEditActions";
import type { JSONValue } from "$lib/types";

export type TargetMode = "campaign" | "challenge";
export type SaveTab = "inspector" | "resources" | "navigation" | "combat" | "construction" | "recruitment";

interface LoadResponse {
  path: string;
  data: JSONValue;
}

export interface CampaignSummary {
  label: string;
  islands: number;
}

export interface ChallengeSummary {
  index: number;
  challengeId: number;
  label: string;
}

export const ARCHER_PREFABS = [
  "Prefabs/Characters/Archer",
  "Prefabs/Characters/norselands/Archer_norselands"
];
export const WORKER_PREFABS = [
  "Prefabs/Characters/Worker",
  "Prefabs/Characters/norselands/Worker_norselands"
];
export const FARMER_PREFABS = [
  "Prefabs/Characters/Farmer",
  "Prefabs/Characters/norselands/Farmer_norselands"
];
export const PIKEMAN_PREFABS = [
  "Prefabs/Characters/Pikeman",
  "Prefabs/Characters/norselands/Knight_norselands",
  "Prefabs/Characters/norselands/Pikeman_norselands"
];
export const KNIGHT_PREFABS = [
  "Prefabs/Characters/Knight",
  "Prefabs/Characters/Squire",
  "Prefabs/Characters/norselands/Knight_norselands"
];

export function errorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

function getContextObjects(
  value: JSONValue | null,
  mode: TargetMode,
  campaignIndex: number,
  islandIndex: number,
  challengeIndex: number
): JSONValue[] | null {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    return null;
  }

  const root = value as Record<string, JSONValue>;
  if (mode === "challenge") {
    const challenges = root.challenges;
    if (!Array.isArray(challenges)) return null;
    const challenge = challenges[challengeIndex];
    if (!challenge || typeof challenge !== "object" || Array.isArray(challenge)) return null;
    const islands = (challenge as Record<string, JSONValue>)._islands;
    if (!Array.isArray(islands) || islands.length === 0) return null;
    const island = islands[0];
    if (!island || typeof island !== "object" || Array.isArray(island)) return null;
    const objects = (island as Record<string, JSONValue>).objects;
    return Array.isArray(objects) ? objects : null;
  }

  const campaigns = root.campaigns;
  if (!Array.isArray(campaigns)) return null;
  const campaign = campaigns[campaignIndex];
  if (!campaign || typeof campaign !== "object" || Array.isArray(campaign)) return null;
  const islands = (campaign as Record<string, JSONValue>)._islands;
  if (!Array.isArray(islands)) return null;
  const island = islands[islandIndex];
  if (!island || typeof island !== "object" || Array.isArray(island)) return null;
  const objects = (island as Record<string, JSONValue>).objects;
  return Array.isArray(objects) ? objects : null;
}

export function countUnits(
  value: JSONValue | null,
  campaignIndex: number,
  islandIndex: number,
  prefabs: string[],
  mode: TargetMode = "campaign",
  challengeIndex: number = 0
): number | null {
  const objects = getContextObjects(value, mode, campaignIndex, islandIndex, challengeIndex);
  if (!objects) return null;

  return objects.reduce<number>((count, entry) => {
    if (entry && typeof entry === "object" && !Array.isArray(entry)) {
      const record = entry as Record<string, JSONValue>;
      const prefabPath = record.prefabPath;
      if (typeof prefabPath === "string" && prefabs.some((prefab) => prefabPath.includes(prefab))) {
        return count + 1;
      }
    }
    return count;
  }, 0);
}

export function countKnightsBySide(
  value: JSONValue | null,
  campaignIndex: number,
  islandIndex: number,
  mode: TargetMode = "campaign",
  challengeIndex: number = 0
): { total: number; left: number; right: number } {
  const objects = getContextObjects(value, mode, campaignIndex, islandIndex, challengeIndex);
  if (!objects) {
    return { total: 0, left: 0, right: 0 };
  }

  let left = 0;
  let right = 0;

  for (const entry of objects) {
    if (entry && typeof entry === "object" && !Array.isArray(entry)) {
      const record = entry as Record<string, JSONValue>;
      const prefabPath = record.prefabPath;
      if (typeof prefabPath === "string" && KNIGHT_PREFABS.some((prefab) => prefabPath.includes(prefab))) {
        let side = 1;
        const comps = record.componentData2 as any;
        if (Array.isArray(comps)) {
          const kComp = comps.find((c) => c && typeof c === "object" && c.name === "Knight") as any;
          if (kComp && typeof kComp.data === "string") {
            try {
              const parsed = JSON.parse(kComp.data);
              if (typeof parsed.side === "number") {
                side = parsed.side;
              }
            } catch {
              // ignore
            }
          }
        }
        if (side < 0) {
          left++;
        } else {
          right++;
        }
      }
    }
  }

  return { total: left + right, left, right };
}

function computeCampaignSummaries(value: JSONValue | null): CampaignSummary[] {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    return [];
  }

  const campaigns = (value as Record<string, JSONValue>).campaigns;
  if (!Array.isArray(campaigns)) {
    return [];
  }

  return campaigns.map((entry, index) => {
    const record = entry && typeof entry === "object" && !Array.isArray(entry)
      ? (entry as Record<string, JSONValue>)
      : {};
    const biome = typeof record.biomeIndex === "number" ? record.biomeIndex : null;
    const islandCount = Array.isArray(record._islands) ? record._islands.length : 0;
    return {
      label: biome === null ? `Campaign ${index + 1}` : `Campaign ${index + 1} (Biome ${biome})`,
      islands: islandCount,
    } satisfies CampaignSummary;
  });
}

function computeChallengeSummaries(value: JSONValue | null): ChallengeSummary[] {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    return [];
  }

  const challenges = (value as Record<string, JSONValue>).challenges;
  if (!Array.isArray(challenges)) {
    return [];
  }

  return challenges.map((entry, index) => {
    const record = entry && typeof entry === "object" && !Array.isArray(entry)
      ? (entry as Record<string, JSONValue>)
      : {};
    const cid = typeof record.challengeId === "number" ? record.challengeId : index;
    let name = `Challenge ${index + 1}`;
    if (cid === 2) name = "Plague Island";
    else if (cid === 0) name = "Skull Island";
    else if (cid === 1) name = "Plague Island";
    else if (cid === 3) name = "Dire Island";
    else if (cid === 4) name = "Trade Routes";
    else if (cid === 9) name = "Lost Islands / Daily";
    return {
      index,
      challengeId: cid,
      label: name,
    } satisfies ChallengeSummary;
  });
}

function computeIslandCount(value: JSONValue | null, campaignIndex: number): number {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    return 0;
  }

  const campaigns = (value as Record<string, JSONValue>).campaigns;
  if (!Array.isArray(campaigns)) {
    return 0;
  }

  const campaign = campaigns[campaignIndex];
  if (!campaign || typeof campaign !== "object" || Array.isArray(campaign)) {
    return 0;
  }

  const islands = (campaign as Record<string, JSONValue>)._islands;
  return Array.isArray(islands) ? islands.length : 0;
}

class SaveEditor {
  lang = $state<Language>(detectLanguage());
  t = $derived(getTranslations(this.lang));

  filePath = $state<string | null>(null);
  // Every edit in saveEditActions returns a fresh copy of the document, so the save is
  // never mutated in place and does not need a deep reactive proxy.
  data = $state.raw<JSONValue | null>(null);
  isLoading = $state(false);
  backupPath = $state<string | null>(null);
  dirty = $state(false);

  targetMode = $state<TargetMode>("campaign");
  selectedChallenge = $state(1); // Default to Plague Island if available
  selectedCampaign = $state(0);
  selectedIsland = $state(0);
  activeTab = $state<SaveTab>("inspector");

  campaigns = $derived(computeCampaignSummaries(this.data));
  challenges = $derived(computeChallengeSummaries(this.data));
  islands = $derived(computeIslandCount(this.data, this.selectedCampaign));

  islandOverview = $derived(
    this.data ? getIslandOverview(this.data, {
      campaignIndex: this.selectedCampaign,
      islandIndex: this.selectedIsland,
      targetMode: this.targetMode,
      challengeIndex: this.selectedChallenge
    }) : null
  );

  canSave = $derived(Boolean(this.filePath && this.data && !this.isLoading));

  /** Selection passed to the resource and recruitment actions. */
  get selection() {
    return {
      mode: this.targetMode,
      challengeIndex: this.selectedChallenge,
      campaignIndex: this.selectedCampaign,
      islandIndex: this.selectedIsland,
    };
  }

  /** Selection passed to the inspector actions, which work in both modes. */
  get islandTarget() {
    return {
      targetMode: this.targetMode,
      challengeIndex: this.selectedChallenge,
      campaignIndex: this.selectedCampaign,
      islandIndex: this.selectedIsland,
    };
  }

  /** Selection passed to the campaign-only actions. */
  get campaignIsland() {
    return {
      campaignIndex: this.selectedCampaign,
      islandIndex: this.selectedIsland,
    };
  }

  setTargetMode(mode: TargetMode) {
    this.targetMode = mode;
    if (mode === "challenge" && this.activeTab !== "resources" && this.activeTab !== "recruitment" && this.activeTab !== "inspector") {
      this.activeTab = "resources";
    }
  }

  /** Keeps the selection inside the bounds of the loaded save. */
  clampSelection() {
    if (this.selectedCampaign >= this.campaigns.length) {
      this.selectedCampaign = 0;
    }

    if (this.selectedChallenge >= this.challenges.length) {
      this.selectedChallenge = 0;
    }

    if (this.selectedIsland >= this.islands) {
      this.selectedIsland = 0;
    }
  }

  requireData(): JSONValue {
    if (!this.data) {
      throw new Error(this.t.status.noFileLoaded);
    }

    return this.data;
  }

  /** Applies an in-memory edit to the open save and reports the outcome. */
  edit(update: (doc: JSONValue) => JSONValue, successMessage: string): boolean {
    try {
      this.data = update(this.requireData());
      this.backupPath = null;
      this.dirty = true;
      toast.success(successMessage);
      return true;
    } catch (error) {
      toast.error(errorMessage(error));
      return false;
    }
  }

  /** global-v35 found at the game's default location, if any. */
  detectedSavePath = $state<string | null>(null);

  async detectSave() {
    try {
      this.detectedSavePath = await invoke<string | null>("find_default_save_file");
    } catch (error) {
      console.warn("Save detection failed", error);
    }
  }

  /** Opens the detected save directly, or asks for the file when none was found. */
  async openSave() {
    if (this.detectedSavePath) {
      await this.loadFrom(() => invoke<LoadResponse>("load_save_file", { path: this.detectedSavePath }));
    } else {
      await this.selectFile();
    }
  }

  async selectFile() {
    console.info("Selecting a global-v35 file");
    await this.loadFrom(() => invoke<LoadResponse>("select_save_file"));
  }

  private async loadFrom(load: () => Promise<LoadResponse>) {
    this.backupPath = null;

    try {
      this.isLoading = true;
      const response = await load();
      console.info("global-v35 file loaded", {
        path: response.path,
        topLevelKeys: response.data && typeof response.data === "object" && !Array.isArray(response.data)
          ? Object.keys(response.data as Record<string, JSONValue>)
          : []
      });
      this.data = response.data;
      this.filePath = response.path;
      this.selectedCampaign = 0;
      this.selectedIsland = 0;
      this.dirty = false;
      toast.success("global-v35 loaded");
    } catch (error) {
      const message = errorMessage(error);
      console.error("Failed to load global-v35", error);
      if (message !== "Selection cancelled") {
        toast.error(message);
      }
    } finally {
      this.isLoading = false;
    }
  }

  async reloadFile() {
    if (!this.filePath) {
      toast.error(this.t.status.noFileLoaded);
      return;
    }

    try {
      this.isLoading = true;
      console.info("Reloading file", { path: this.filePath });
      const response = await invoke<LoadResponse>("load_save_file", { path: this.filePath });
      this.data = response.data;
      this.dirty = false;
      console.info("File reloaded successfully");
      toast.success("Save file reloaded");
    } catch (error) {
      console.error("Failed to reload file", error);
      toast.error(errorMessage(error));
    } finally {
      this.isLoading = false;
    }
  }

  async save() {
    if (!this.filePath || !this.data) {
      toast.error(this.t.status.noFileLoaded);
      return;
    }

    try {
      this.isLoading = true;
      console.info("Saving", { path: this.filePath });
      const savedBackupPath = await invoke<string>("save_save_file", { path: this.filePath, data: this.data });
      console.info("Save completed", { backupPath: savedBackupPath });
      this.backupPath = savedBackupPath;
      this.dirty = false;
      try {
        console.info("Reloading file to verify changes");
        const refreshed = await invoke<LoadResponse>("load_save_file", { path: this.filePath });
        this.data = refreshed.data;
      } catch (error) {
        console.error("Unable to reload file after saving", error);
      }
      toast.success(this.t.status.saveApplied, {
        description: `${this.t.header.backupCreated} ${savedBackupPath}`
      });
    } catch (error) {
      console.error("Save failed", error);
      toast.error(errorMessage(error));
    } finally {
      this.isLoading = false;
    }
  }
}

export const saveEditor = new SaveEditor();
