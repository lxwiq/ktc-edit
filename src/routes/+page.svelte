<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { JSONValue } from "$lib/components/FieldEditor.svelte";
  import { detectLanguage, getTranslations, type Language } from "$lib/i18n";
  import {
    getPlayerCoins,
    getPlayerGems,
    setPlayerCoins,
    setPlayerGems,
    spawnArchers,
    spawnFarmers,
    spawnPikemen,
    spawnKnights,
    spawnWorkers,
    destroyPortals,
    exterminateEnemies,
    gotoIsland,
    markTreesForRemoval,
    pimpIsland,
    spawnUnits,
    startFormationAssault,
    takeOverIsland,
    getIslandOverview,
    upgradeSpecificWall,
    batchUpgradeWalls,
    upgradeIslandCastle,
    upgradeSpecificTower,
    batchUpgradeTowers,
    setTreeMark,
    setDeityStatus,
    unlockAllDeities,
    setCatapultOilBarrels,
    teleportPlayer,
    type IslandOverview,
    type WallInfo,
    type TowerInfo,
    type TowerType,
    type CatapultInfo,
    type TreeInfo,
    type DeityInfo
  } from "$lib/saveEditActions";

  interface LoadResponse {
    path: string;
    data: JSONValue;
  }

  interface AssetSettings {
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
    knightWalletCapacity: number;
    playerWalletCapacity: number;
    knightWalletPayTaxesAbove: number;
    knightCoinDropProbability: number;
    bagScale: number;
  }

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

  let filePath = $state<string | null>(null);
  let data = $state<JSONValue | null>(null);
  let isLoading = $state(false);
  let statusType = $state<"error" | "success" | null>(null);
  let statusMessage = $state<string | null>(null);
  let backupPath = $state<string | null>(null);
  let assetMode = $state(false);
  let assetDirectory = $state<string | null>(null);
  let assetSettings = $state<AssetSettings | null>(null);
  let assetBusy = $state(false);
  let assetStatus = $state<string | null>(null);
  let assetError = $state<string | null>(null);
  let assetBackups = $state<string[]>([]);
  let warhorseColliderLimit = $state<number | null>(null);
  let coinVisualLimit = $state(30);
  let coinVisualCurrent = $state<number | null>(null);
  let coinVisualAvailable = $state(false);
  let coinVisualStatus = $state<string | null>(null);
  let coinVisualError = $state<string | null>(null);
  let campEnabled = $state(false);
  let campNeedsUpgrade = $state(false);
  let campAvailable = $state(false);
  let campStatus = $state<string | null>(null);
  let campError = $state<string | null>(null);
  let warhorseColliderBackup = $state<string | null>(null);
  let warhorseColliderError = $state<string | null>(null);
  let warhorseColliderStatus = $state<string | null>(null);
  let resetSaveWalls = $state(true);
  let targetMode = $state<"campaign" | "challenge">("campaign");
  let selectedChallenge = $state(1); // Default to Plague Island if available
  let selectedCampaign = $state(0);
  let selectedIsland = $state(0);
  let coinsPlayerIndex = $state(0);
  let coinsAmount = $state(0);
  let gemsPlayerIndex = $state(0);
  let gemsAmount = $state(0);
  let archerCount = $state(1);
  let workerCount = $state(1);
  let farmerCount = $state(1);
  let pikemanCount = $state(1);
  let knightCount = $state(1);
  let knightSide = $state<1 | -1>(1);
  let knightWithArchers = $state(true);
  let coinsSelectionSignature = $state<string | null>(null);
  let lastCoinsData = $state<JSONValue | null>(null);
  let gemsSelectionSignature = $state<string | null>(null);
  let lastGemsData = $state<JSONValue | null>(null);
  let archerPlayerIndex = $state(0);
  let gotoCoins = $state(69);
  let gotoGems = $state(3);
  let gotoPikemen = $state(4);
  let gotoFarmers = $state(4);
  let gotoBoats = $state(4);
  let takeOverCoins = $state(69);
  let takeOverArchers = $state(1);
  let takeOverWorkers = $state(1);
  let takeOverOffset = $state(3);
  let formationPlayerIndex = $state(0);
  let formationPosition = $state(0);
  let formationArchers = $state(5);
  let formationPikemen = $state(5);
  let teleportPlayerIndex = $state(0);
  let teleportTargetX = $state(0);
  let pimpCoins = $state(69);
  let pimpSpawnCount = $state(10);
  let treesPlayerIndex = $state(0);
  let treesCoins = $state(69);
  let comboArchers = $state(0);
  let comboWorkers = $state(0);
  let comboPikemen = $state(0);
  let activeTab = $state<'inspector' | 'resources' | 'navigation' | 'combat' | 'construction' | 'recruitment'>('inspector');
  let currentLang = $state<Language>(detectLanguage());
  let t = $derived(getTranslations(currentLang));

  let islandOverview = $derived(
    data ? getIslandOverview(data, {
      campaignIndex: selectedCampaign,
      islandIndex: selectedIsland,
      targetMode: targetMode,
      challengeIndex: selectedChallenge
    }) : null
  );

  let leftCatapult = $derived(
    islandOverview?.catapults.find(c => c.side === -1) ?? null
  );
  let rightCatapult = $derived(
    islandOverview?.catapults.find(c => c.side === 1) ?? null
  );

  let customLeftBarrels = $state<number | null>(null);
  let customRightBarrels = $state<number | null>(null);

  let leftCatapultInput = $derived(
    customLeftBarrels !== null ? customLeftBarrels : (leftCatapult?.oilBarrels ?? 30)
  );
  let rightCatapultInput = $derived(
    customRightBarrels !== null ? customRightBarrels : (rightCatapult?.oilBarrels ?? 30)
  );

  let knightStats = $derived(
    countKnightsBySide(data, selectedCampaign, selectedIsland, targetMode, selectedChallenge)
  );

  let treeFilterSide = $state<'all' | 'left' | 'right'>('all');
  let treeFilterStatus = $state<'all' | 'standing' | 'marked' | 'danger'>('all');
  let treeRangeMin = $state(-50);
  let treeRangeMax = $state(50);
  let wallFilterSide = $state<'all' | 'left' | 'right'>('all');
  let towerFilterSide = $state<'all' | 'left' | 'right'>('all');

  interface CampaignSummary {
    label: string;
    islands: number;
  }

  function getDefaultSavePath(): string {
    const platform = navigator.platform.toLowerCase();
    const userAgent = navigator.userAgent.toLowerCase();

    if (platform.includes('mac') || userAgent.includes('mac')) {
      const username = Intl.DateTimeFormat().resolvedOptions().timeZone.split('/')[0] || 'USERNAME';
      return `~/Library/Application Support/nl.noio.kingdom-two-crowns/Release/global-v35`;
    } else if (platform.includes('win') || userAgent.includes('windows')) {
      return `C:\\Users\\USERNAME\\AppData\\LocalLow\\noio\\KingdomTwoCrowns\\Release\\global-v35`;
    } else {
      return `~/.config/unity3d/noio/KingdomTwoCrowns/Release/global-v35`;
    }
  }

  function getOSName(): string {
    const platform = navigator.platform.toLowerCase();
    const userAgent = navigator.userAgent.toLowerCase();

    if (platform.includes('mac') || userAgent.includes('mac')) {
      return 'macOS';
    } else if (platform.includes('win') || userAgent.includes('windows')) {
      return 'Windows';
    } else if (platform.includes('linux') || userAgent.includes('linux')) {
      return 'Linux';
    } else {
      return 'Unknown system';
    }
  }

  let detectedOS = $state(getOSName());
  let defaultPath = $state(getDefaultSavePath());



  const ARCHER_PREFABS = [
    "Prefabs/Characters/Archer",
    "Prefabs/Characters/norselands/Archer_norselands"
  ];
  const WORKER_PREFABS = [
    "Prefabs/Characters/Worker",
    "Prefabs/Characters/norselands/Worker_norselands"
  ];
  const FARMER_PREFABS = [
    "Prefabs/Characters/Farmer",
    "Prefabs/Characters/norselands/Farmer_norselands"
  ];
  const PIKEMAN_PREFABS = [
    "Prefabs/Characters/Pikeman",
    "Prefabs/Characters/norselands/Knight_norselands",
    "Prefabs/Characters/norselands/Pikeman_norselands"
  ];
  const KNIGHT_PREFABS = [
    "Prefabs/Characters/Knight",
    "Prefabs/Characters/Squire",
    "Prefabs/Characters/norselands/Knight_norselands"
  ];

  function getContextObjects(
    value: JSONValue | null,
    mode: "campaign" | "challenge",
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

  function countUnits(
    value: JSONValue | null,
    campaignIndex: number,
    islandIndex: number,
    prefabs: string[],
    mode: "campaign" | "challenge" = "campaign",
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

  function countKnightsBySide(
    value: JSONValue | null,
    campaignIndex: number,
    islandIndex: number,
    mode: "campaign" | "challenge" = "campaign",
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

  interface ChallengeSummary {
    index: number;
    challengeId: number;
    label: string;
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

  function resetStatus() {
    statusType = null;
    statusMessage = null;
  }

  function showError(message: string) {
    statusType = "error";
    statusMessage = message;
  }

  function showSuccess(message: string) {
    statusType = "success";
    statusMessage = message;
  }

  async function selectFile() {
    resetStatus();
    backupPath = null;

    try {
      isLoading = true;
      console.info("Selecting a global-v35 file");
      const response = await invoke<LoadResponse>("select_save_file");
      console.info("global-v35 file loaded", {
        path: response.path,
        topLevelKeys: response.data && typeof response.data === "object" && !Array.isArray(response.data)
          ? Object.keys(response.data as Record<string, JSONValue>)
          : []
      });
      data = response.data;
      filePath = response.path;
      selectedCampaign = 0;
      selectedIsland = 0;
      showSuccess("global-v35 loaded");
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      console.error("Failed to load global-v35", error);
      if (message !== "Selection cancelled") {
        showError(message);
      }
    } finally {
      isLoading = false;
    }
  }

  async function reloadFile() {
    resetStatus();
    
    if (!filePath) {
      showError(t.status.noFileLoaded);
      return;
    }

    try {
      isLoading = true;
      console.info("Reloading file", { path: filePath });
      const response = await invoke<LoadResponse>("load_save_file", { path: filePath });
      data = response.data;
      console.info("File reloaded successfully");
      showSuccess("Save file reloaded");
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      console.error("Failed to reload file", error);
      showError(message);
    } finally {
      isLoading = false;
    }
  }



  async function save() {
    resetStatus();

    if (!filePath || !data) {
      showError(t.status.noFileLoaded);
      return;
    }

    try {
      isLoading = true;
      console.info("Saving", {
        path: filePath,
        hasData: Boolean(data)
      });
      const serialized = JSON.parse(JSON.stringify(data));
      const savedBackupPath = await invoke<string>("save_save_file", { path: filePath, data: serialized });
      console.info("Save completed", { backupPath: savedBackupPath });
      backupPath = savedBackupPath;
      try {
        console.info("Reloading file to verify changes");
        const refreshed = await invoke<LoadResponse>("load_save_file", { path: filePath });
        data = refreshed.data;
        const workerTotal = countUnits(refreshed.data, selectedCampaign, selectedIsland, WORKER_PREFABS);
        const farmerTotal = countUnits(refreshed.data, selectedCampaign, selectedIsland, FARMER_PREFABS);
        const archerTotal = countUnits(refreshed.data, selectedCampaign, selectedIsland, ARCHER_PREFABS);
        const pikemanTotal = countUnits(refreshed.data, selectedCampaign, selectedIsland, PIKEMAN_PREFABS);
        console.info("Reload complete", {
          workers: workerTotal,
          farmers: farmerTotal,
          archers: archerTotal,
          pikemen: pikemanTotal
        });
      } catch (error) {
        console.error("Unable to reload file after saving", error);
      }
      showSuccess(t.status.saveApplied);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      console.error("Save failed", error);
      showError(message);
    } finally {
      isLoading = false;
    }
  }

  function canSave() {
    return Boolean(filePath && data && !isLoading);
  }

  let campaigns = $derived(computeCampaignSummaries(data));
  let challenges = $derived(computeChallengeSummaries(data));
  let islands = $derived(computeIslandCount(data, selectedCampaign));

  $effect(() => {
    if (selectedCampaign >= campaigns.length) {
      selectedCampaign = 0;
    }

    if (selectedChallenge >= challenges.length) {
      selectedChallenge = 0;
    }

    if (selectedIsland >= islands) {
      selectedIsland = 0;
    }
  });

  $effect(() => {
    if (!data) {
      coinsSelectionSignature = null;
      lastCoinsData = null;
      coinsAmount = 0;
      gemsSelectionSignature = null;
      lastGemsData = null;
      gemsAmount = 0;
      return;
    }

    const signature = `${targetMode}|${selectedCampaign}|${selectedChallenge}|${selectedIsland}|${coinsPlayerIndex}`;
    const selectionChanged = coinsSelectionSignature !== signature;
    const dataChanged = lastCoinsData !== data;
    if (!selectionChanged && !dataChanged) {
      return;
    }

    coinsSelectionSignature = signature;
    lastCoinsData = data;

    const coinsValue = getPlayerCoins(data, {
      mode: targetMode,
      challengeIndex: selectedChallenge,
      campaignIndex: selectedCampaign,
      islandIndex: selectedIsland,
      playerIndex: coinsPlayerIndex,
    });

    if (typeof coinsValue === "number") {
      coinsAmount = coinsValue;
    } else {
      coinsAmount = 0;
    }
  });

  $effect(() => {
    if (!data) {
      return;
    }

    const signature = `${targetMode}|${selectedCampaign}|${selectedChallenge}|${selectedIsland}|${gemsPlayerIndex}`;
    const selectionChanged = gemsSelectionSignature !== signature;
    const dataChanged = lastGemsData !== data;
    if (!selectionChanged && !dataChanged) {
      return;
    }

    gemsSelectionSignature = signature;
    lastGemsData = data;

    const gemsValue = getPlayerGems(data, {
      mode: targetMode,
      challengeIndex: selectedChallenge,
      campaignIndex: selectedCampaign,
      islandIndex: selectedIsland,
      playerIndex: gemsPlayerIndex,
    });

    if (typeof gemsValue === "number") {
      gemsAmount = gemsValue;
    } else {
      gemsAmount = 0;
    }
  });

  function requireData(): JSONValue {
    if (!data) {
      throw new Error(t.status.noFileLoaded);
    }

    return data;
  }

  function handleCoinsUpdate() {
    resetStatus();

    try {
      const current = requireData();
      const beforeCoins = getPlayerCoins(current, {
        mode: targetMode,
        challengeIndex: selectedChallenge,
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        playerIndex: coinsPlayerIndex,
      });
      console.info("Coin update requested", {
        mode: targetMode,
        challenge: selectedChallenge,
        campaign: selectedCampaign,
        island: selectedIsland,
        player: coinsPlayerIndex,
        before: beforeCoins,
        target: coinsAmount,
      });
      const updated = setPlayerCoins(current, {
        mode: targetMode,
        challengeIndex: selectedChallenge,
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        playerIndex: coinsPlayerIndex,
        coins: coinsAmount,
      });
      data = updated;
      backupPath = null;
      const afterCoins = getPlayerCoins(updated, {
        mode: targetMode,
        challengeIndex: selectedChallenge,
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        playerIndex: coinsPlayerIndex,
      });
      if (typeof afterCoins === "number") {
        coinsAmount = afterCoins;
      }
      console.info("Coins updated", {
        mode: targetMode,
        challenge: selectedChallenge,
        campaign: selectedCampaign,
        island: selectedIsland,
        player: coinsPlayerIndex,
        after: afterCoins,
      });
      showSuccess(t.status.coinsUpdated);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleGemsUpdate() {
    resetStatus();

    try {
      const current = requireData();
      const beforeGems = getPlayerGems(current, {
        mode: targetMode,
        challengeIndex: selectedChallenge,
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        playerIndex: gemsPlayerIndex,
      });
      console.info("Gem update requested", {
        mode: targetMode,
        challenge: selectedChallenge,
        campaign: selectedCampaign,
        island: selectedIsland,
        player: gemsPlayerIndex,
        before: beforeGems,
        target: gemsAmount,
      });
      const updated = setPlayerGems(current, {
        mode: targetMode,
        challengeIndex: selectedChallenge,
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        playerIndex: gemsPlayerIndex,
        gems: gemsAmount,
      });
      data = updated;
      backupPath = null;
      const afterGems = getPlayerGems(updated, {
        mode: targetMode,
        challengeIndex: selectedChallenge,
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        playerIndex: gemsPlayerIndex,
      });
      if (typeof afterGems === "number") {
        gemsAmount = afterGems;
      }

      console.info("Gems updated", {
        mode: targetMode,
        challenge: selectedChallenge,
        campaign: selectedCampaign,
        island: selectedIsland,
        player: gemsPlayerIndex,
        after: afterGems,
      });
      showSuccess(t.status.gemsUpdated);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleSpawnArchers() {
    resetStatus();

    try {
      const current = requireData();
      const beforeArchers = countUnits(current, selectedCampaign, selectedIsland, ARCHER_PREFABS, targetMode, selectedChallenge);
      console.info("Archer addition requested", {
        mode: targetMode,
        challenge: selectedChallenge,
        campaign: selectedCampaign,
        island: selectedIsland,
        player: archerPlayerIndex,
        before: beforeArchers,
        added: archerCount,
      });
      const updated = spawnArchers(current, {
        mode: targetMode,
        challengeIndex: selectedChallenge,
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        playerIndex: archerPlayerIndex,
        count: archerCount,
      });
      data = updated;
      backupPath = null;
      const afterArchers = countUnits(updated, selectedCampaign, selectedIsland, ARCHER_PREFABS, targetMode, selectedChallenge);
      console.info("Archers added", {
        mode: targetMode,
        challenge: selectedChallenge,
        campaign: selectedCampaign,
        island: selectedIsland,
        player: archerPlayerIndex,
        after: afterArchers,
      });
      showSuccess(t.status.archersAdded);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleSpawnWorkers() {
    resetStatus();

    try {
      const current = requireData();
      const beforeWorkers = countUnits(current, selectedCampaign, selectedIsland, WORKER_PREFABS, targetMode, selectedChallenge);
      console.info("Worker addition requested", {
        mode: targetMode,
        challenge: selectedChallenge,
        campaign: selectedCampaign,
        island: selectedIsland,
        player: archerPlayerIndex,
        before: beforeWorkers,
        added: workerCount,
      });
      const updated = spawnWorkers(current, {
        mode: targetMode,
        challengeIndex: selectedChallenge,
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        playerIndex: archerPlayerIndex,
        count: workerCount,
      });
      data = updated;
      backupPath = null;
      const afterWorkers = countUnits(updated, selectedCampaign, selectedIsland, WORKER_PREFABS, targetMode, selectedChallenge);
      console.info("Workers added", {
        mode: targetMode,
        challenge: selectedChallenge,
        campaign: selectedCampaign,
        island: selectedIsland,
        after: afterWorkers,
      });
      showSuccess(t.status.workersAdded);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleSpawnFarmers() {
    resetStatus();

    try {
      const current = requireData();
      const beforeFarmers = countUnits(current, selectedCampaign, selectedIsland, FARMER_PREFABS, targetMode, selectedChallenge);
      console.info("Farmer addition requested", {
        mode: targetMode,
        challenge: selectedChallenge,
        campaign: selectedCampaign,
        island: selectedIsland,
        player: archerPlayerIndex,
        before: beforeFarmers,
        added: farmerCount,
      });
      const updated = spawnFarmers(current, {
        mode: targetMode,
        challengeIndex: selectedChallenge,
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        playerIndex: archerPlayerIndex,
        count: farmerCount,
      });
      data = updated;
      backupPath = null;
      const afterFarmers = countUnits(updated, selectedCampaign, selectedIsland, FARMER_PREFABS, targetMode, selectedChallenge);
      console.info("Farmers added", {
        mode: targetMode,
        challenge: selectedChallenge,
        campaign: selectedCampaign,
        island: selectedIsland,
        after: afterFarmers,
      });
      showSuccess(t.status.farmersAdded);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleSpawnPikemen() {
    resetStatus();

    try {
      const current = requireData();
      const beforePikemen = countUnits(current, selectedCampaign, selectedIsland, PIKEMAN_PREFABS, targetMode, selectedChallenge);
      console.info("Pikeman addition requested", {
        mode: targetMode,
        challenge: selectedChallenge,
        campaign: selectedCampaign,
        island: selectedIsland,
        player: archerPlayerIndex,
        before: beforePikemen,
        added: pikemanCount,
      });
      const updated = spawnPikemen(current, {
        mode: targetMode,
        challengeIndex: selectedChallenge,
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        playerIndex: archerPlayerIndex,
        count: pikemanCount,
      });
      data = updated;
      backupPath = null;
      const afterPikemen = countUnits(updated, selectedCampaign, selectedIsland, PIKEMAN_PREFABS, targetMode, selectedChallenge);
      console.info("Pikemen added", {
        mode: targetMode,
        challenge: selectedChallenge,
        campaign: selectedCampaign,
        island: selectedIsland,
        after: afterPikemen,
      });
      showSuccess(t.status.pikemenAdded);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleSpawnKnights() {
    resetStatus();

    try {
      const current = requireData();
      const beforeKnights = countUnits(current, selectedCampaign, selectedIsland, KNIGHT_PREFABS, targetMode, selectedChallenge);
      console.info("Knight addition requested", {
        mode: targetMode,
        challenge: selectedChallenge,
        campaign: selectedCampaign,
        island: selectedIsland,
        before: beforeKnights,
        added: knightCount,
        side: knightSide,
        withArchers: knightWithArchers
      });
      const updated = spawnKnights(current, {
        mode: targetMode,
        challengeIndex: selectedChallenge,
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        playerIndex: archerPlayerIndex,
        count: knightCount,
        side: knightSide,
        withArchers: knightWithArchers
      });
      data = updated;
      backupPath = null;
      const afterKnights = countUnits(updated, selectedCampaign, selectedIsland, KNIGHT_PREFABS, targetMode, selectedChallenge);
      console.info("Knights added", {
        mode: targetMode,
        challenge: selectedChallenge,
        campaign: selectedCampaign,
        island: selectedIsland,
        after: afterKnights
      });
      showSuccess(t.status.knightsAdded);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleGoto() {
    resetStatus();

    try {
      const current = requireData();
      console.info("Island change requested", {
        campaign: selectedCampaign,
        island: selectedIsland,
        coins: gotoCoins,
        gems: gotoGems,
        pikemen: gotoPikemen,
        farmers: gotoFarmers,
        boats: gotoBoats,
      });
      const updated = gotoIsland(current, {
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        coins: gotoCoins,
        gems: gotoGems,
        pikemen: gotoPikemen,
        farmers: gotoFarmers,
        boats: gotoBoats,
      });
      data = updated;
      backupPath = null;
      showSuccess(t.status.destinationUpdated);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleTakeOver() {
    resetStatus();

    try {
      const current = requireData();
      console.info("Takeover requested", {
        campaign: selectedCampaign,
        island: selectedIsland,
        coins: takeOverCoins,
        archers: takeOverArchers,
        workers: takeOverWorkers,
        offset: takeOverOffset,
      });
      const updated = takeOverIsland(current, {
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        coins: takeOverCoins,
        archerCount: takeOverArchers,
        workerCount: takeOverWorkers,
        formationOffset: takeOverOffset,
      });
      data = updated;
      backupPath = null;
      showSuccess(t.status.islandSecured);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleDestroyPortals() {
    resetStatus();

    try {
      const current = requireData();
      console.info("Portal destruction requested", {
        campaign: selectedCampaign,
        island: selectedIsland,
      });
      const updated = destroyPortals(current, {
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
      });
      data = updated;
      backupPath = null;
      showSuccess(t.status.portalsDestroyed);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleExterminate() {
    resetStatus();

    try {
      const current = requireData();
      console.info("Extermination requested", {
        campaign: selectedCampaign,
        island: selectedIsland,
      });
      const updated = exterminateEnemies(current, {
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
      });
      data = updated;
      backupPath = null;
      showSuccess(t.status.enemiesRemoved);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleTeleportPlayer(targetX?: number) {
    resetStatus();

    try {
      const current = requireData();
      const x = targetX !== undefined ? targetX : teleportTargetX;
      console.info("Player teleport requested", {
        campaign: selectedCampaign,
        island: selectedIsland,
        player: teleportPlayerIndex,
        x,
      });
      const updated = teleportPlayer(current, {
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        playerIndex: teleportPlayerIndex,
        x,
      });
      data = updated;
      backupPath = null;
      teleportTargetX = x;
      showSuccess(t.status.playerTeleported);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleFormation() {
    resetStatus();

    try {
      const current = requireData();
      console.info("Battle formation requested", {
        campaign: selectedCampaign,
        island: selectedIsland,
        player: formationPlayerIndex,
        position: formationPosition,
        archers: formationArchers,
        pikemen: formationPikemen,
      });
      const updated = startFormationAssault(current, {
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        playerIndex: formationPlayerIndex,
        position: formationPosition,
        archers: formationArchers,
        pikemen: formationPikemen,
      });
      data = updated;
      backupPath = null;
      showSuccess(t.status.formationDeployed);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleSetCatapultBarrels(side: -1 | 1) {
    resetStatus();

    try {
      const current = requireData();
      const barrels = side === -1 ? leftCatapultInput : rightCatapultInput;
      console.info("Catapult barrels update requested", {
        campaign: selectedCampaign,
        island: selectedIsland,
        side,
        barrels,
      });
      const updated = setCatapultOilBarrels(current, {
        targetMode,
        challengeIndex: selectedChallenge,
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        side,
        oilBarrels: barrels,
      });
      data = updated;
      backupPath = null;
      if (side === -1) {
        customLeftBarrels = null;
      } else {
        customRightBarrels = null;
      }
      showSuccess(t.status.catapultOilUpdated);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handlePimp() {
    resetStatus();

    try {
      const current = requireData();
      console.info("Pimp my island", {
        campaign: selectedCampaign,
        island: selectedIsland,
        coins: pimpCoins,
        spawn: pimpSpawnCount,
      });
      const updated = pimpIsland(current, {
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        coins: pimpCoins,
        spawnCount: pimpSpawnCount,
      });
      data = updated;
      backupPath = null;
      showSuccess(t.status.islandUpgraded);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleMarkTrees() {
    resetStatus();

    try {
      const current = requireData();
      console.info("Tree marking", {
        campaign: selectedCampaign,
        island: selectedIsland,
        player: treesPlayerIndex,
        coins: treesCoins,
      });
      const updated = markTreesForRemoval(current, {
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        playerIndex: treesPlayerIndex,
        coins: treesCoins,
      });
      data = updated;
      backupPath = null;
      showSuccess(t.status.treesMarked);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleUpgradeSingleWall(wallId: string, targetLevel: number, addHorn: boolean = false) {
    resetStatus();
    try {
      const current = requireData();
      const updated = upgradeSpecificWall(current, {
        targetMode,
        challengeIndex: selectedChallenge,
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        wallId,
        targetLevel,
        addHorn,
      });
      data = updated;
      showSuccess(t.status.wallUpdated);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleBatchUpgradeWalls(mode: 'all' | 'built_only' | 'mounds_only', targetLevel: number = 5, addHorn: boolean = false) {
    resetStatus();
    try {
      const current = requireData();
      const updated = batchUpgradeWalls(current, {
        targetMode,
        challengeIndex: selectedChallenge,
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        mode,
        targetLevel,
        addHorn,
      });
      data = updated;
      showSuccess(t.status.wallsBatchUpdated);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleUpgradeCastle(targetLevel: number = 7) {
    resetStatus();
    try {
      const current = requireData();
      const updated = upgradeIslandCastle(current, {
        targetMode,
        challengeIndex: selectedChallenge,
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        targetLevel,
      });
      data = updated;
      showSuccess(t.status.castleUpdated);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleUpgradeSingleTower(towerId: string, targetType: TowerType) {
    resetStatus();
    try {
      const current = requireData();
      const updated = upgradeSpecificTower(current, {
        targetMode,
        challengeIndex: selectedChallenge,
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        towerId,
        targetType,
      });
      data = updated;
      showSuccess(t.status.towerUpdated);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleBatchUpgradeTowers(mode: 'all' | 'built_only' | 'mounds_only', targetType: TowerType) {
    resetStatus();
    try {
      const current = requireData();
      const updated = batchUpgradeTowers(current, {
        targetMode,
        challengeIndex: selectedChallenge,
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        mode,
        targetType,
      });
      data = updated;
      showSuccess(t.status.towersBatchUpdated);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleToggleTreeMark(treeId: string, currentMarked: boolean) {
    resetStatus();
    try {
      const current = requireData();
      const updated = setTreeMark(current, {
        targetMode,
        challengeIndex: selectedChallenge,
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        treeId,
        marked: !currentMarked,
      });
      data = updated;
      showSuccess(t.status.treeMarkUpdated);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleBatchTreeMark(marked: boolean) {
    resetStatus();
    try {
      const current = requireData();
      const minVal = Math.min(treeRangeMin, treeRangeMax);
      const maxVal = Math.max(treeRangeMin, treeRangeMax);
      const updated = setTreeMark(current, {
        targetMode,
        challengeIndex: selectedChallenge,
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        xMin: minVal,
        xMax: maxVal,
        marked,
      });
      data = updated;
      showSuccess(t.status.treeMarkUpdated);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleSetDeityStatus(deityIndex: number, status: number) {
    resetStatus();
    try {
      const current = requireData();
      const updated = setDeityStatus(current, {
        targetMode,
        challengeIndex: selectedChallenge,
        campaignIndex: selectedCampaign,
        deityIndex,
        status,
      });
      data = updated;
      showSuccess(t.status.deityUpdated);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleUnlockAllDeities(status: number = 2) {
    resetStatus();
    try {
      const current = requireData();
      const updated = unlockAllDeities(current, {
        targetMode,
        challengeIndex: selectedChallenge,
        campaignIndex: selectedCampaign,
        status,
      });
      data = updated;
      showSuccess(t.status.deitiesAllUpdated);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  function handleSpawnCombo() {
    resetStatus();

    const archers = Math.max(0, comboArchers);
    const workers = Math.max(0, comboWorkers);
    const pikemen = Math.max(0, comboPikemen);

    if (archers === 0 && workers === 0 && pikemen === 0) {
      showError(t.status.addAtLeastOneUnit);
      return;
    }

    try {
      const current = requireData();
      console.info("Combined recruitment", {
        campaign: selectedCampaign,
        island: selectedIsland,
        player: archerPlayerIndex,
        archers,
        workers,
        pikemen,
      });
      const updated = spawnUnits(current, {
        mode: targetMode,
        challengeIndex: selectedChallenge,
        campaignIndex: selectedCampaign,
        islandIndex: selectedIsland,
        playerIndex: archerPlayerIndex,
        archers,
        workers,
        pikemen,
      });
      data = updated;
      backupPath = null;
      showSuccess(t.status.unitsAdded);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      showError(message);
    }
  }

  async function openAssetEditor(selectFolder = false) {
    assetMode = true;
    assetBusy = true;
    assetStatus = null;
    assetError = null;
    try {
      const response = selectFolder
        ? await invoke<AssetResponse>("select_game_data_directory")
        : await invoke<AssetResponse>("load_game_assets", { dataDirectory: assetDirectory });
      if (assetDirectory !== response.dataDirectory) warhorseColliderBackup = null;
      assetDirectory = response.dataDirectory;
      assetSettings = response.settings;
      assetStatus = "Loaded the current values from the game.";
      coinVisualAvailable = false;
      coinVisualStatus = null;
      coinVisualError = null;
      try {
        const coins = await invoke<{limit: number | null}>("load_coin_visual_limit", {dataDirectory: response.dataDirectory});
        coinVisualCurrent = coins.limit;
        coinVisualLimit = coins.limit ?? 30;
        coinVisualAvailable = true;
      } catch (error) { coinVisualError = String(error); }
      campAvailable = false;
      campStatus = null;
      campError = null;
      try {
        const camp = await invoke<{enabled: boolean; needsUpgrade: boolean}>("load_camp_preservation", {dataDirectory: response.dataDirectory});
        campEnabled = camp.enabled;
        campNeedsUpgrade = camp.needsUpgrade;
        campAvailable = true;
      } catch (error) { campError = String(error); }
      warhorseColliderLimit = null;
      warhorseColliderError = null;
      warhorseColliderStatus = null;
      try {
        const collider = await invoke<ColliderLimitResponse>("load_warhorse_collider_limit", {
          dataDirectory: response.dataDirectory
        });
        warhorseColliderLimit = collider.limit;
      } catch (error) {
        warhorseColliderError = String(error);
      }
    } catch (error) {
      assetError = String(error);
    } finally {
      assetBusy = false;
    }
  }

  async function applyCoinVisualLimit(disable = false) {
    if (!assetDirectory || !coinVisualAvailable || assetBusy) return;
    coinVisualError = null;
    coinVisualStatus = null;
    if (!disable && (!Number.isInteger(coinVisualLimit) || coinVisualLimit < 1 || coinVisualLimit > 65535)) {
      coinVisualError = "Enter a positive whole number. ARM64 immediate maximum: 65535.";
      return;
    }
    assetBusy = true;
    try {
      const response = await invoke<{limit: number | null; backup: string | null}>("apply_coin_visual_limit", {
        dataDirectory: assetDirectory, limit: disable ? null : coinVisualLimit
      });
      coinVisualCurrent = response.limit;
      coinVisualStatus = (response.limit === null ? "Original coin display restored." : `Coin display limited to ${response.limit}.`) +
        (response.backup ? ` Backup: ${response.backup}` : " No changes needed.");
    } catch (error) { coinVisualError = String(error); }
    finally { assetBusy = false; }
  }

  async function applyCampPreservation(enabled: boolean) {
    if (!assetDirectory || !campAvailable || assetBusy) return;
    campError = null;
    campStatus = null;
    assetBusy = true;
    try {
      const response = await invoke<{enabled: boolean; backup: string | null}>("apply_camp_preservation", {
        dataDirectory: assetDirectory, enabled
      });
      campEnabled = response.enabled;
      campNeedsUpgrade = false;
      campStatus = (response.enabled ? "Vagrant camps are preserved when the forest disappears." : "Original camp removal restored.") +
        (response.backup ? ` Backup: ${response.backup}` : " No changes needed.");
    } catch (error) { campError = String(error); }
    finally { assetBusy = false; }
  }

  async function applyWarhorseColliderLimit() {
    if (!assetDirectory || warhorseColliderLimit === null) return;
    warhorseColliderError = null;
    warhorseColliderStatus = null;
    if (!Number.isInteger(warhorseColliderLimit) || warhorseColliderLimit < 1 || warhorseColliderLimit > 65535) {
      warhorseColliderError = "Enter a positive whole number. ARM64 immediate maximum: 65535.";
      return;
    }
    assetBusy = true;
    try {
      const response = await invoke<ColliderLimitResponse>("apply_warhorse_collider_limit", {
        dataDirectory: assetDirectory,
        limit: warhorseColliderLimit
      });
      warhorseColliderLimit = response.limit;
      warhorseColliderBackup = response.backup;
      warhorseColliderStatus = response.backup
        ? `GameAssembly.dylib updated, signed, and verified. Backup: ${response.backup}`
        : "Collider limit is already set to this value.";
    } catch (error) {
      warhorseColliderError = String(error);
    } finally {
      assetBusy = false;
    }
  }

  async function restoreWarhorseColliderLimit() {
    if (!assetDirectory || !warhorseColliderBackup) return;
    warhorseColliderError = null;
    warhorseColliderStatus = null;
    assetBusy = true;
    try {
      const response = await invoke<ColliderLimitResponse>("restore_warhorse_collider_limit", {
        dataDirectory: assetDirectory,
        backup: warhorseColliderBackup
      });
      warhorseColliderLimit = response.limit;
      warhorseColliderBackup = response.backup;
      warhorseColliderStatus = `Previous collider limit restored (${response.limit}).`;
    } catch (error) {
      warhorseColliderError = String(error);
    } finally {
      assetBusy = false;
    }
  }

  async function applyAssetChanges() {
    if (!assetDirectory || !assetSettings) return;
    assetBusy = true;
    assetStatus = null;
    assetError = null;
    try {
      const response = await invoke<AssetApplyResponse>("apply_game_assets", {
        dataDirectory: assetDirectory,
        settings: assetSettings,
        resetSaveWalls: resetSaveWalls
      });
      assetSettings = response.settings;
      assetBackups = [response.resourcesBackup, response.sharedAssetsBackup];
      if (typeof response.saveClearedWalls === 'number' && response.saveClearedWalls > 0) {
        assetStatus = `Changes applied to assets. Also cleaned Damageable overrides on ${response.saveClearedWalls} walls in the save file.`;
      } else {
        assetStatus = "Changes applied and verified in both resources.assets and sharedassets0.assets.";
      }
    } catch (error) {
      assetError = String(error);
    } finally {
      assetBusy = false;
    }
  }

  async function restoreAssetChanges() {
    if (!assetDirectory || assetBackups.length !== 2) return;
    assetBusy = true;
    assetStatus = null;
    assetError = null;
    try {
      const response = await invoke<AssetResponse>("restore_game_assets", {
        dataDirectory: assetDirectory,
        resourcesBackup: assetBackups[0],
        sharedAssetsBackup: assetBackups[1]
      });
      assetSettings = response.settings;
      assetBackups = [];
      assetStatus = "Backups restored. A safety backup of the previous state was also created.";
    } catch (error) {
      assetError = String(error);
    } finally {
      assetBusy = false;
    }
  }

  function updateAssetNumber(key: keyof AssetSettings, event: Event) {
    if (!assetSettings) return;
    const value = Number((event.currentTarget as HTMLInputElement).value);
    assetSettings = { ...assetSettings, [key]: value };
  }
</script>

<main>
  {#if assetMode}
    <div class="asset-editor-shell">
      <header class="asset-header">
        <div>
          <h1>Game Assets</h1>
          <p class="muted">Edit mounts, units, and bag items directly in Kingdom Two Crowns.</p>
        </div>
        <div class="header-actions">
          <button type="button" onclick={() => { assetMode = false; assetStatus = null; assetError = null; }}>Back</button>
          <button type="button" onclick={() => openAssetEditor(true)} disabled={assetBusy}>Select Data Folder</button>
          <button type="button" onclick={() => openAssetEditor(false)} disabled={assetBusy}>Reload</button>
          <button type="button" onclick={restoreAssetChanges} disabled={assetBusy || assetBackups.length !== 2}>Restore</button>
          <button class="primary" type="button" onclick={applyAssetChanges} disabled={assetBusy || !assetSettings}>Apply</button>
        </div>
      </header>

      <section class="asset-content">
        {#if assetDirectory}<p class="file-path">Data: {assetDirectory}</p>{/if}
        {#if assetStatus}<p class="status success">{assetStatus}</p>{/if}
        {#if assetError}<p class="status error">{assetError}</p>{/if}
        {#if assetBusy}<p class="status info">Processing…</p>{/if}

        {#if assetSettings}
          <div class="asset-grid">
            <section class="card asset-card">
              <h2>Griffin</h2>
              <label>Run speed<input type="number" step="0.1" value={assetSettings.griffinRunSpeed} oninput={(e) => updateAssetNumber('griffinRunSpeed', e)} /></label>
              <label>Forest multiplier<input type="number" step="0.1" value={assetSettings.griffinForestMultiplier} oninput={(e) => updateAssetNumber('griffinForestMultiplier', e)} /></label>
              <label>Run stamina rate<input type="number" step="0.01" value={assetSettings.griffinRunStaminaRate} oninput={(e) => updateAssetNumber('griffinRunStaminaRate', e)} /></label>
              <label>Skill stamina cost<input type="number" step="0.01" value={assetSettings.griffinSkillStaminaCost} oninput={(e) => updateAssetNumber('griffinSkillStaminaCost', e)} /></label>
              <label>Push range width<input type="number" min="0" step="0.1" value={assetSettings.griffinPushWidth} oninput={(e) => updateAssetNumber('griffinPushWidth', e)} /></label>
              <label>Push range height<input type="number" min="0" step="0.1" value={assetSettings.griffinPushHeight} oninput={(e) => updateAssetNumber('griffinPushHeight', e)} /></label>
            </section>

            <section class="card asset-card">
              <h2>Regular Horse</h2>
              <label>Run speed<input type="number" step="0.1" value={assetSettings.horseRunSpeed} oninput={(e) => updateAssetNumber('horseRunSpeed', e)} /></label>
              <label>Run stamina rate<input type="number" step="0.01" value={assetSettings.horseRunStaminaRate} oninput={(e) => updateAssetNumber('horseRunStaminaRate', e)} /></label>
            </section>

            <section class="card asset-card">
              <h2>Lizard</h2>
              <label>Run speed<input type="number" step="0.1" value={assetSettings.lizardRunSpeed} oninput={(e) => updateAssetNumber('lizardRunSpeed', e)} /></label>
              <label>Run stamina rate<input type="number" step="0.01" value={assetSettings.lizardRunStaminaRate} oninput={(e) => updateAssetNumber('lizardRunStaminaRate', e)} /></label>
              <label>Skill stamina cost<input type="number" min="0" step="0.01" value={assetSettings.lizardSkillStaminaCost} oninput={(e) => updateAssetNumber('lizardSkillStaminaCost', e)} /></label>
            </section>

            <section class="card asset-card">
              <h2>Warhorse</h2>
              <label>Run speed<input type="number" step="0.1" value={assetSettings.warhorseRunSpeed} oninput={(e) => updateAssetNumber('warhorseRunSpeed', e)} /></label>
              <label>Run stamina rate<input type="number" step="0.01" value={assetSettings.warhorseRunStaminaRate} oninput={(e) => updateAssetNumber('warhorseRunStaminaRate', e)} /></label>
              <label>Skill stamina cost<input type="number" step="0.01" value={assetSettings.warhorseSkillStaminaCost} oninput={(e) => updateAssetNumber('warhorseSkillStaminaCost', e)} /></label>
              <label>Skill cooldown (seconds)<input type="number" step="0.1" value={assetSettings.warhorseCooldown} oninput={(e) => updateAssetNumber('warhorseCooldown', e)} /></label>
              <label>Buff duration (seconds)<input type="number" min="0" max="1000000" step="0.1" value={assetSettings.warhorseBuffDuration} oninput={(e) => updateAssetNumber('warhorseBuffDuration', e)} /></label>
              <label>Buff range<input type="number" step="0.1" value={assetSettings.warhorseBuffRange} oninput={(e) => updateAssetNumber('warhorseBuffRange', e)} /></label>
              <p class="muted">Skill stamina cost and buff range also apply to Warhorse Plague.</p>
              {#if warhorseColliderLimit !== null}
                <label>Collider scan limit<input type="number" min="1" max="65535" step="1" value={warhorseColliderLimit} oninput={(e) => warhorseColliderLimit = Number((e.currentTarget as HTMLInputElement).value)} /></label>
                <button type="button" onclick={applyWarhorseColliderLimit} disabled={assetBusy}>Apply collider limit</button>
                <button type="button" onclick={restoreWarhorseColliderLimit} disabled={assetBusy || !warhorseColliderBackup}>Restore collider limit</button>
                <p class="muted">Separate from Apply above. Patches and re-signs GameAssembly.dylib on Apple Silicon Macs; close the game first. This limits colliders scanned, not the exact number of soldiers buffed.</p>
                {#if warhorseColliderStatus}<p class="status success">{warhorseColliderStatus}</p>{/if}
              {:else}
                <p class="muted">Collider scan limit is unavailable for this game build.</p>
              {/if}
              {#if warhorseColliderError}<p class="status error">{warhorseColliderError}</p>{/if}
            </section>

            <section class="card asset-card">
              <h2>Coin bag display</h2>
              <p class="muted">Current: {coinVisualCurrent === null ? 'Original display (no visual cap)' : `${coinVisualCurrent} coin objects maximum`}</p>
              <label>Maximum visible coins<input type="number" min="1" max="65535" step="1" bind:value={coinVisualLimit} /></label>
              <button type="button" onclick={() => applyCoinVisualLimit()} disabled={assetBusy || !coinVisualAvailable}>Apply coin display limit</button>
              <button type="button" onclick={() => applyCoinVisualLimit(true)} disabled={assetBusy || !coinVisualAvailable || coinVisualCurrent === null}>Restore original coin display</button>
              <p class="muted">Separate from asset Apply. Patches and re-signs the selected game's GameAssembly.dylib on supported Apple Silicon builds. Close the game first. Wallet balance and gems are unchanged. Ground coins are not capped. Backups are created automatically.</p>
              {#if coinVisualStatus}<p class="status success">{coinVisualStatus}</p>{/if}
              {#if coinVisualError}<p class="status error">{coinVisualError}</p>{/if}
            </section>

            <section class="card asset-card">
              <h2>Vagrant camps</h2>
              <p class="muted">Current: {!campAvailable ? 'Unavailable for this game build' : campNeedsUpgrade ? 'Old patch — upgrade required to detach camps from the forest' : campEnabled ? 'Preserved and detached after deforestation' : 'Original camp removal'}</p>
              <button type="button" onclick={() => applyCampPreservation(true)} disabled={assetBusy || !campAvailable || (campEnabled && !campNeedsUpgrade)}>{campNeedsUpgrade ? 'Upgrade camp preservation' : 'Preserve vagrant camps'}</button>
              <button type="button" onclick={() => applyCampPreservation(false)} disabled={assetBusy || !campAvailable || !campEnabled}>Restore camp removal</button>
              <p class="muted">Separate from asset Apply. Patches and re-signs GameAssembly.dylib on supported Apple Silicon builds; close the game first. Forest boundaries still shrink normally. Restore keeps other patches. Backups are automatic. Multiplayer is unverified.</p>
              {#if campStatus}<p class="status success">{campStatus}</p>{/if}
              {#if campError}<p class="status error">{campError}</p>{/if}
            </section>

            <section class="card asset-card">
              <h2>Warhorse Plague</h2>
              <label>Run speed<input type="number" min="0" step="0.1" value={assetSettings.warhorsePlagueRunSpeed} oninput={(e) => updateAssetNumber('warhorsePlagueRunSpeed', e)} /></label>
              <label>Run stamina rate<input type="number" step="0.01" value={assetSettings.warhorsePlagueRunStaminaRate} oninput={(e) => updateAssetNumber('warhorsePlagueRunStaminaRate', e)} /></label>
              <label>Skill cooldown (seconds)<input type="number" step="0.1" value={assetSettings.warhorsePlagueCooldown} oninput={(e) => updateAssetNumber('warhorsePlagueCooldown', e)} /></label>
              <p class="muted">Applied to Warhorse Plague P1 and P2 only.</p>
            </section>

            <section class="card asset-card">
              <h2>Archer</h2>
              <label>Walk speed<input type="number" min="0" step="0.05" value={assetSettings.archerWalkSpeed} oninput={(e) => updateAssetNumber('archerWalkSpeed', e)} /></label>
              <label>Run speed<input type="number" min="0" step="0.05" value={assetSettings.archerRunSpeed} oninput={(e) => updateAssetNumber('archerRunSpeed', e)} /></label>
              <label>Shoot preparation (seconds)<input type="number" min="0" step="0.05" value={assetSettings.archerShootPrepTime} oninput={(e) => updateAssetNumber('archerShootPrepTime', e)} /></label>
              <label>Normal cooldown (seconds)<input type="number" min="0" step="0.05" value={assetSettings.archerShootCooldownTime} oninput={(e) => updateAssetNumber('archerShootCooldownTime', e)} /></label>
              <label>Knight squad cooldown (seconds)<input type="number" min="0" step="0.05" value={assetSettings.archerShootCooldownWithKnightTime} oninput={(e) => updateAssetNumber('archerShootCooldownWithKnightTime', e)} /></label>
              <label>Normal interval minimum (seconds)<input type="number" min="0" step="0.05" value={assetSettings.archerIntervalMin} oninput={(e) => updateAssetNumber('archerIntervalMin', e)} /></label>
              <label>Normal interval maximum (seconds)<input type="number" min="0" step="0.05" value={assetSettings.archerIntervalMax} oninput={(e) => updateAssetNumber('archerIntervalMax', e)} /></label>
              <label>Formation interval minimum (seconds)<input type="number" min="0" step="0.05" value={assetSettings.archerFormationIntervalMin} oninput={(e) => updateAssetNumber('archerFormationIntervalMin', e)} /></label>
              <label>Formation interval maximum (seconds)<input type="number" min="0" step="0.05" value={assetSettings.archerFormationIntervalMax} oninput={(e) => updateAssetNumber('archerFormationIntervalMax', e)} /></label>
              <p class="muted">Applied to regular, Norse Lands, and Knight squad archers.</p>
            </section>

            <section class="card asset-card">
              <h2>Builder</h2>
              <label>Walk speed<input type="number" min="0" step="0.05" value={assetSettings.builderWalkSpeed} oninput={(e) => updateAssetNumber('builderWalkSpeed', e)} /></label>
              <label>Run speed<input type="number" min="0" step="0.05" value={assetSettings.builderRunSpeed} oninput={(e) => updateAssetNumber('builderRunSpeed', e)} /></label>
              <label>Work time (seconds)<input type="number" min="0" step="0.05" value={assetSettings.builderWorkTime} oninput={(e) => updateAssetNumber('builderWorkTime', e)} /></label>
              <p class="muted">Applied to regular and Norse Lands builders. Lower work time means faster building and chopping.</p>
            </section>

            <section class="card asset-card">
              <h2>Player Wallet</h2>
              <label>Total capacity<input type="number" min="1" max="1000000" step="1" value={assetSettings.playerWalletCapacity} oninput={(e) => updateAssetNumber('playerWalletCapacity', e)} /></label>
              <p class="muted">Edits Wallet.TotalCapacity on the Player prefab in resources.assets. Current save and runtime state may affect when the change takes effect.</p>
            </section>

            <section class="card asset-card">
              <h2>Knight Wallet</h2>
              <label>Coin drop probability (0–1)<input type="number" min="0" max="1" step="0.05" value={assetSettings.knightCoinDropProbability} oninput={(e) => updateAssetNumber('knightCoinDropProbability', e)} /></label>
              <p class="muted">Character.coinDropProbability: 0 means no coin loss through this damage path; 0.4 means 40% per coin-drop check. This is not full invulnerability.</p>
              <label>Coin capacity<input type="number" min="1" max="1000000" step="1" value={assetSettings.knightWalletCapacity} oninput={(e) => updateAssetNumber('knightWalletCapacity', e)} /></label>
              <label>Pay taxes above<input type="number" min="0" max="1000000" step="1" value={assetSettings.knightWalletPayTaxesAbove} oninput={(e) => updateAssetNumber('knightWalletPayTaxesAbove', e)} /></label>
              <p class="muted">Applied to regular, Norse Lands, and Greece Knights. Squires are unchanged.</p>
            </section>

            <section class="card asset-card">
              <h2>Iron Wall (Wall5)</h2>
              <label>Base HP — regular, Greece, Bamboo<input type="number" min="1" max="1000000" step="1" value={assetSettings.wall5StandardHitPoints} oninput={(e) => updateAssetNumber('wall5StandardHitPoints', e)} /></label>
              <label>Base HP — Norse Lands<input type="number" min="1" max="1000000" step="1" value={assetSettings.wall5NorselandsHitPoints} oninput={(e) => updateAssetNumber('wall5NorselandsHitPoints', e)} /></label>
              <label class="asset-checkbox-label">
                <input type="checkbox" bind:checked={resetSaveWalls} />
                <span>Reset all existing walls in save (removes Damageable overrides so all walls use this new HP)</span>
              </label>
              <p class="muted">Applies to normal and horn variants. Wall Statue multipliers are unchanged.</p>
            </section>

            <section class="card asset-card">
              <h2>Bag</h2>
              <label>Coin and gem scale<input type="number" step="0.05" value={assetSettings.bagScale} oninput={(e) => updateAssetNumber('bagScale', e)} /></label>
              <p class="muted">This value is applied to coins, gems, and biome variants.</p>
            </section>
          </div>
          <section class="card safety-card">
            <h3>Safety</h3>
            <p>The app writes only when both files match the verified game profile. Two backups are created before every change.</p>
            {#if assetBackups.length}
              <ul>{#each assetBackups as path}<li><code>{path}</code></li>{/each}</ul>
            {/if}
          </section>
        {:else if !assetBusy}
          <section class="card empty-assets">
            <h2>Assets not loaded</h2>
            <p>Select <code>KingdomTwoCrowns.app/Contents/Resources/Data</code> if the game is not installed in the default Steam location.</p>
            <button class="primary" type="button" onclick={() => openAssetEditor(true)}>Select Data Folder</button>
          </section>
        {/if}
      </section>
    </div>
  {:else if !data}
    <div class="welcome">
      <div class="welcome-content">
        <h1>{t.welcome.title}</h1>
        <p class="subtitle">{t.welcome.subtitle}</p>
        
        <div class="os-detection">
          <div class="os-badge">
            <svg width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M10 2L3 6V10C3 14.55 6.84 18.74 10 19.5C13.16 18.74 17 14.55 17 10V6L10 2Z" fill="currentColor" opacity="0.2"/>
              <path d="M10 2L3 6V10C3 14.55 6.84 18.74 10 19.5C13.16 18.74 17 14.55 17 10V6L10 2Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            {t.welcome.osDetected} <strong>{detectedOS}</strong>
          </div>
        </div>

        <div class="default-path">
          <div class="path-label">{t.welcome.defaultPath}</div>
          <div class="path-display">
            <code>{defaultPath}</code>
          </div>
          <p class="path-hint">{t.welcome.pathHint}</p>
        </div>

        <div class="welcome-actions">
          <button class="primary large" onclick={selectFile} disabled={isLoading}>
            {#if isLoading}
              <svg class="spinner" width="20" height="20" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                <circle cx="10" cy="10" r="8" stroke="currentColor" stroke-width="3" fill="none" stroke-dasharray="50" stroke-dashoffset="25">
                  <animateTransform attributeName="transform" type="rotate" from="0 10 10" to="360 10 10" dur="1s" repeatCount="indefinite"/>
                </circle>
              </svg>
              {t.welcome.loading}
            {:else}
              <svg width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M13 2H5C4.46957 2 3.96086 2.21071 3.58579 2.58579C3.21071 2.96086 3 3.46957 3 4V16C3 16.5304 3.21071 17.0391 3.58579 17.4142C3.96086 17.7893 4.46957 18 5 18H15C15.5304 18 16.0391 17.7893 16.4142 17.4142C16.7893 17.0391 17 16.5304 17 16V6L13 2Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M13 2V6H17" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M10 10V14" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M8 12L10 14L12 12" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
              {t.welcome.openFile}
            {/if}
          </button>
          <button class="large" onclick={() => openAssetEditor(false)} disabled={assetBusy}>
            Edit Game Assets
          </button>
        </div>

        <div class="features">
          <div class="feature">
            <div class="feature-icon">💰</div>
            <h3>{t.welcome.features.resources.title}</h3>
            <p>{t.welcome.features.resources.desc}</p>
          </div>
          <div class="feature">
            <div class="feature-icon">🗺️</div>
            <h3>{t.welcome.features.navigation.title}</h3>
            <p>{t.welcome.features.navigation.desc}</p>
          </div>
          <div class="feature">
            <div class="feature-icon">⚔️</div>
            <h3>{t.welcome.features.combat.title}</h3>
            <p>{t.welcome.features.combat.desc}</p>
          </div>
          <div class="feature">
            <div class="feature-icon">🏰</div>
            <h3>{t.welcome.features.construction.title}</h3>
            <p>{t.welcome.features.construction.desc}</p>
          </div>
          <div class="feature">
            <div class="feature-icon">🎖️</div>
            <h3>{t.welcome.features.recruitment.title}</h3>
            <p>{t.welcome.features.recruitment.desc}</p>
          </div>
        </div>
      </div>
    </div>
  {:else}
    <header>
      <div class="header-top">
        <h1>{t.header.title}</h1>
        <div class="header-actions">
          <button onclick={reloadFile} disabled={isLoading} title="Reload current save file">
            <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M14 8C14 11.3137 11.3137 14 8 14C4.68629 14 2 11.3137 2 8C2 4.68629 4.68629 2 8 2C9.86384 2 11.5289 2.87401 12.6 4.2" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
              <path d="M12 1V4.5H8.5" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </button>
          <button onclick={selectFile} disabled={isLoading}>
            {isLoading ? t.welcome.loading : t.header.changeFile}
          </button>
          <button class="primary" onclick={save} disabled={!canSave()}>
            {t.header.save}
          </button>
        </div>
      </div>
      {#if statusMessage}
        <p class={`status ${statusType ?? ""}`}>{statusMessage}</p>
      {/if}
      {#if backupPath}
        <p class="status info">{t.header.backupCreated} <code>{backupPath}</code></p>
      {/if}
      {#if filePath}
        <p class="file-path">{t.header.filePath} {filePath}</p>
      {/if}
    </header>

    <section class="content">
      <div class="editor-area">
        <section class="context-card card">
          <h3>{t.context.title}</h3>
          <p class="muted">{t.context.description}</p>
          <div class="form-row wrap">
            <label>
              {t.context.mode}
              <select
                value={targetMode}
                onchange={(event) => {
                  targetMode = (event.currentTarget as HTMLSelectElement).value as "campaign" | "challenge";
                  if (targetMode === "challenge" && activeTab !== "resources" && activeTab !== "recruitment" && activeTab !== "inspector") {
                    activeTab = "resources";
                  }
                }}
              >
                <option value="campaign">{t.context.modeCampaign}</option>
                <option value="challenge">{t.context.modeChallenge}</option>
              </select>
            </label>

            {#if targetMode === "campaign"}
              <label>
                {t.context.campaign}
                <select
                  value={selectedCampaign}
                  onchange={(event) => (selectedCampaign = Number((event.currentTarget as HTMLSelectElement).value))}
                >
                  {#each campaigns as campaign, index}
                    <option value={index}>{campaign.label}</option>
                  {/each}
                </select>
              </label>
              <label>
                {t.context.island}
                <select
                  value={selectedIsland}
                  onchange={(event) => (selectedIsland = Number((event.currentTarget as HTMLSelectElement).value))}
                >
                  {#each Array.from({ length: Math.max(0, islands) }) as _, index}
                    <option value={index}>{t.context.island} {index + 1}</option>
                  {/each}
                </select>
              </label>
            {:else}
              <label>
                {t.context.challenge}
                <select
                  value={selectedChallenge}
                  onchange={(event) => (selectedChallenge = Number((event.currentTarget as HTMLSelectElement).value))}
                >
                  {#each challenges as challenge, index}
                    <option value={index}>{challenge.label}</option>
                  {/each}
                </select>
              </label>
            {/if}
          </div>
        </section>

        <div class="tabs">
          <button 
            type="button"
            class="tab"
            class:active={activeTab === 'inspector'}
            onclick={() => activeTab = 'inspector'}
          >
            🗺️ {t.tabs.inspector}
          </button>
          <button 
            type="button"
            class="tab"
            class:active={activeTab === 'resources'}
            onclick={() => activeTab = 'resources'}
          >
            {t.tabs.resources}
          </button>
          {#if targetMode === "campaign"}
            <button 
              type="button"
              class="tab"
              class:active={activeTab === 'navigation'}
              onclick={() => activeTab = 'navigation'}
            >
              {t.tabs.navigation}
            </button>
          {/if}
          <button 
            type="button"
            class="tab"
            class:active={activeTab === 'combat'}
            onclick={() => activeTab = 'combat'}
          >
            {t.tabs.combat}
          </button>
          {#if targetMode === "campaign"}
            <button 
              type="button"
              class="tab"
              class:active={activeTab === 'construction'}
              onclick={() => activeTab = 'construction'}
            >
              {t.tabs.construction}
            </button>
          {/if}
          <button 
            type="button"
            class="tab"
            class:active={activeTab === 'recruitment'}
            onclick={() => activeTab = 'recruitment'}
          >
            {t.tabs.recruitment}
          </button>
        </div>

        {#if activeTab === 'inspector'}
        <div class="tab-content">
          {#if islandOverview}
            <!-- Castle & Overview Summary Card -->
            <section class="card">
              <div class="card-header-flex">
                <h3>🏰 {t.inspector.castleInfo}</h3>
                <div class="batch-actions">
                  <button
                    type="button"
                    class="btn-sm"
                    disabled={islandOverview.castleLevel >= 4}
                    onclick={() => handleUpgradeCastle(4)}
                    title="Stone Castle (Level 4)"
                  >
                    🪨 {t.inspector.upgradeCastleStone}
                  </button>
                  <button
                    type="button"
                    class="btn-sm gold"
                    disabled={islandOverview.castleLevel >= 7}
                    onclick={() => handleUpgradeCastle(7)}
                    title="Iron Castle (Level 7)"
                  >
                    ⚡ {t.inspector.upgradeCastleIron}
                  </button>
                </div>
              </div>
              <div class="overview-grid">
                <div class="overview-item highlight">
                  <span class="overview-label">{t.inspector.castlePosition}</span>
                  <span class="overview-value text-gold">X = {islandOverview.castleX}</span>
                  <span class="overview-sub">{islandOverview.castleName} (Level {islandOverview.castleLevel})</span>
                </div>
                {#if islandOverview.players && islandOverview.players.length > 0}
                  {#each islandOverview.players as player}
                    <div class="overview-item">
                      <span class="overview-label">👑 {player.name} Position</span>
                      <span class="overview-value" style="color: #64b5f6;">X = {player.x}</span>
                      <span class="overview-sub">Y = {player.y} (Dist: {Math.round(Math.abs(player.x - islandOverview.castleX) * 100) / 100}m)</span>
                    </div>
                  {/each}
                {/if}
                <div class="overview-item">
                  <span class="overview-label">{t.inspector.totalWalls}</span>
                  <span class="overview-value">{islandOverview.walls.length}</span>
                  <span class="overview-sub">
                    {islandOverview.walls.filter(w => w.level > 0).length} built / {islandOverview.walls.filter(w => w.level === 0).length} mounds
                  </span>
                </div>
                <div class="overview-item">
                  <span class="overview-label">{t.inspector.totalTrees}</span>
                  <span class="overview-value">{islandOverview.trees.length}</span>
                  <span class="overview-sub">
                    {islandOverview.trees.filter(tr => tr.marked).length} {t.inspector.markedToCut}
                  </span>
                </div>
              </div>

              {#if islandOverview.specialPoints.length > 0}
                <div class="special-points-bar">
                  <span class="special-points-title">📍 {t.inspector.specialPoints}:</span>
                  <div class="special-points-list">
                    {#each islandOverview.specialPoints as pt}
                      <span class="special-badge {pt.type}">
                        {#if pt.type === 'beggar_camp'}🏕️{:else if pt.type === 'portal'}🌀{:else if pt.type === 'statue'}🗿{:else if pt.type === 'mine'}⛏️{:else if pt.type === 'wreck'}⚓{:else if pt.type === 'boat'}⛵{:else}📍{/if}
                        {pt.name} (X = {pt.x})
                      </span>
                    {/each}
                  </div>
                </div>
              {/if}
            </section>

            <!-- Deities & Shrines Card -->
            <section class="card">
              <div class="card-header-flex">
                <h3>⛩️ {t.inspector.deitiesTitle}</h3>
                <div class="batch-actions">
                  <button type="button" class="btn-sm gold" onclick={() => handleUnlockAllDeities(2)}>
                    {t.inspector.activateAllDeities}
                  </button>
                  <button type="button" class="btn-sm secondary" onclick={() => handleUnlockAllDeities(1)}>
                    {t.inspector.unlockAllDeities}
                  </button>
                  <button type="button" class="btn-sm danger" onclick={() => handleUnlockAllDeities(0)}>
                    {t.inspector.lockAllDeities}
                  </button>
                </div>
              </div>

              {#if islandOverview.deities && islandOverview.deities.length > 0}
                <div class="deities-grid">
                  {#each islandOverview.deities as deity}
                    <div class="deity-card" class:active={deity.status === 2} class:unlocked={deity.status === 1}>
                      <div class="deity-header">
                        <span class="deity-name">
                          {#if deity.index === 0}🏹{:else if deity.index === 1}🌾{:else if deity.index === 2}🔨{:else}⚔️{/if}
                          {deity.name}
                        </span>
                        {#if targetMode === 'campaign'}
                          <span class="deity-island-tag">Island {deity.island}</span>
                        {:else}
                          <span class="deity-island-tag">Global Buff</span>
                        {/if}
                      </div>

                      <div class="deity-status-bar">
                        <span class="deity-status-pill status-{deity.status}">
                          {#if deity.status === 2}
                            {t.inspector.deityStatusActive}
                          {:else if deity.status === 1}
                            {t.inspector.deityStatusUnlocked}
                          {:else}
                            {t.inspector.deityStatusLocked}
                          {/if}
                        </span>
                      </div>

                      <div class="deity-actions">
                        <button
                          type="button"
                          class="btn-xs gold"
                          disabled={deity.status === 2}
                          onclick={() => handleSetDeityStatus(deity.index, 2)}
                        >
                          ⚡ Active
                        </button>
                        <button
                          type="button"
                          class="btn-xs"
                          disabled={deity.status === 1}
                          onclick={() => handleSetDeityStatus(deity.index, 1)}
                        >
                          🔓 Unlock
                        </button>
                        <button
                          type="button"
                          class="btn-xs danger"
                          disabled={deity.status === 0}
                          onclick={() => handleSetDeityStatus(deity.index, 0)}
                        >
                          🔒 Lock
                        </button>
                      </div>
                    </div>
                  {/each}
                </div>
              {/if}
            </section>

            <!-- Walls Fortification Card -->
            <section class="card">
              <div class="card-header-flex">
                <h3>🧱 {t.inspector.walls}</h3>
                <div class="batch-actions">
                  <button type="button" class="btn-sm gold" onclick={() => handleBatchUpgradeWalls('built_only', 5, false)}>
                    ⚡ {t.inspector.batchUpgradeBuilt}
                  </button>
                  <button type="button" class="btn-sm" onclick={() => handleBatchUpgradeWalls('all', 5, false)}>
                    🏰 {t.inspector.batchUpgradeAll}
                  </button>
                </div>
              </div>

              <div class="filter-row">
                <span class="filter-label">Filter:</span>
                <button
                  type="button"
                  class="filter-btn"
                  class:active={wallFilterSide === 'all'}
                  onclick={() => wallFilterSide = 'all'}
                >All ({islandOverview.walls.length})</button>
                <button
                  type="button"
                  class="filter-btn"
                  class:active={wallFilterSide === 'left'}
                  onclick={() => wallFilterSide = 'left'}
                >⬅️ {t.inspector.leftSide} ({islandOverview.walls.filter(w => w.side === 'left').length})</button>
                <button
                  type="button"
                  class="filter-btn"
                  class:active={wallFilterSide === 'right'}
                  onclick={() => wallFilterSide = 'right'}
                >➡️ {t.inspector.rightSide} ({islandOverview.walls.filter(w => w.side === 'right').length})</button>
              </div>

              <div class="walls-columns">
                {#if wallFilterSide === 'all' || wallFilterSide === 'left'}
                  <div class="wall-column">
                    <h4>⬅️ {t.inspector.leftSide}</h4>
                    <div class="entities-list">
                      {#each islandOverview.walls.filter(w => w.side === 'left').reverse() as wall}
                        <div class="entity-card wall-card" class:level-max={wall.level === 5}>
                          <div class="entity-main">
                            <div class="entity-coords">
                              <span class="coord-badge">X = {wall.x}</span>
                              <span class="dist-badge">{t.inspector.distance} {wall.distanceToCastle}m</span>
                            </div>
                            <div class="entity-type">
                              <span class="wall-level-pill lvl-{wall.level}" class:horn={wall.hasHorn}>
                                {#if wall.level === 5 && wall.hasHorn}
                                  🛡️📯 Level 5 + Horn
                                {:else if wall.level === 5}
                                  🛡️ Level 5 (Iron)
                                {:else if wall.level > 0}
                                  🧱 Level {wall.level}
                                {:else}
                                  🪨 Dirt Mound (L0)
                                {/if}
                              </span>
                            </div>
                          </div>
                          <div class="entity-actions">
                            <button
                              type="button"
                              class="btn-xs"
                              disabled={wall.level === 5 && !wall.hasHorn}
                              onclick={() => handleUpgradeSingleWall(wall.id, 5, false)}
                            >
                              L5
                            </button>
                            <button
                              type="button"
                              class="btn-xs gold"
                              disabled={wall.level === 5 && wall.hasHorn}
                              onclick={() => handleUpgradeSingleWall(wall.id, 5, true)}
                            >
                              L5+Horn
                            </button>
                            {#if wall.level > 0}
                              <button
                                type="button"
                                class="btn-xs danger"
                                onclick={() => handleUpgradeSingleWall(wall.id, 0, false)}
                              >
                                L0
                              </button>
                            {/if}
                          </div>
                        </div>
                      {/each}
                    </div>
                  </div>
                {/if}

                {#if wallFilterSide === 'all' || wallFilterSide === 'right'}
                  <div class="wall-column">
                    <h4>➡️ {t.inspector.rightSide}</h4>
                    <div class="entities-list">
                      {#each islandOverview.walls.filter(w => w.side === 'right') as wall}
                        <div class="entity-card wall-card" class:level-max={wall.level === 5}>
                          <div class="entity-main">
                            <div class="entity-coords">
                              <span class="coord-badge">X = {wall.x}</span>
                              <span class="dist-badge">{t.inspector.distance} {wall.distanceToCastle}m</span>
                            </div>
                            <div class="entity-type">
                              <span class="wall-level-pill lvl-{wall.level}" class:horn={wall.hasHorn}>
                                {#if wall.level === 5 && wall.hasHorn}
                                  🛡️📯 Level 5 + Horn
                                {:else if wall.level === 5}
                                  🛡️ Level 5 (Iron)
                                {:else if wall.level > 0}
                                  🧱 Level {wall.level}
                                {:else}
                                  🪨 Dirt Mound (L0)
                                {/if}
                              </span>
                            </div>
                          </div>
                          <div class="entity-actions">
                            <button
                              type="button"
                              class="btn-xs"
                              disabled={wall.level === 5 && !wall.hasHorn}
                              onclick={() => handleUpgradeSingleWall(wall.id, 5, false)}
                            >
                              L5
                            </button>
                            <button
                              type="button"
                              class="btn-xs gold"
                              disabled={wall.level === 5 && wall.hasHorn}
                              onclick={() => handleUpgradeSingleWall(wall.id, 5, true)}
                            >
                              L5+Horn
                            </button>
                            {#if wall.level > 0}
                              <button
                                type="button"
                                class="btn-xs danger"
                                onclick={() => handleUpgradeSingleWall(wall.id, 0, false)}
                              >
                                L0
                              </button>
                            {/if}
                          </div>
                        </div>
                      {/each}
                    </div>
                  </div>
                {/if}
              </div>
            </section>

            <!-- Catapult Section inside Inspector -->
            <section class="card">
              <div class="card-header-flex">
                <h3>☄️ {t.combat.catapultAmmo}</h3>
              </div>
              <div class="form-row wrap" style="align-items: flex-end; gap: 1.5rem; margin-top: 0.5rem;">
                <!-- Left Catapult -->
                <div style="flex: 1; min-width: 250px; background: rgba(255,255,255,0.02); padding: 0.75rem 1rem; border-radius: 8px; border: 1px solid var(--color-border);">
                  <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.5rem;">
                    <span style="font-weight: 600; color: #64b5f6;">{t.combat.catapultLeft}</span>
                    {#if leftCatapult}
                      <span class="coord-badge" style="font-size: 0.8rem;">X = {leftCatapult.x}</span>
                    {:else}
                      <span style="font-size: 0.8rem; color: var(--color-text-secondary);">(Not built)</span>
                    {/if}
                  </div>
                  {#if leftCatapult}
                    <div style="display: flex; gap: 0.5rem; align-items: center;">
                      <label style="flex: 1; margin: 0;">
                        {t.combat.barrelsCount}
                        <input
                          type="number"
                          min="0"
                          max="999"
                          value={leftCatapultInput}
                          oninput={(event) => (customLeftBarrels = Math.max(0, Number((event.currentTarget as HTMLInputElement).value)))}
                        />
                      </label>
                      <button type="button" onclick={() => handleSetCatapultBarrels(-1)} style="margin-top: 1.25rem;">
                        {t.combat.catapultApply}
                      </button>
                    </div>
                  {:else}
                    <p class="muted" style="margin: 0; font-size: 0.85rem;">{t.combat.noCatapultOnIsland}</p>
                  {/if}
                </div>

                <!-- Right Catapult -->
                <div style="flex: 1; min-width: 250px; background: rgba(255,255,255,0.02); padding: 0.75rem 1rem; border-radius: 8px; border: 1px solid var(--color-border);">
                  <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.5rem;">
                    <span style="font-weight: 600; color: #ffb74d;">{t.combat.catapultRight}</span>
                    {#if rightCatapult}
                      <span class="coord-badge" style="font-size: 0.8rem;">X = {rightCatapult.x}</span>
                    {:else}
                      <span style="font-size: 0.8rem; color: var(--color-text-secondary);">(Not built)</span>
                    {/if}
                  </div>
                  {#if rightCatapult}
                    <div style="display: flex; gap: 0.5rem; align-items: center;">
                      <label style="flex: 1; margin: 0;">
                        {t.combat.barrelsCount}
                        <input
                          type="number"
                          min="0"
                          max="999"
                          value={rightCatapultInput}
                          oninput={(event) => (customRightBarrels = Math.max(0, Number((event.currentTarget as HTMLInputElement).value)))}
                        />
                      </label>
                      <button type="button" onclick={() => handleSetCatapultBarrels(1)} style="margin-top: 1.25rem;">
                        {t.combat.catapultApply}
                      </button>
                    </div>
                  {:else}
                    <p class="muted" style="margin: 0; font-size: 0.85rem;">{t.combat.noCatapultOnIsland}</p>
                  {/if}
                </div>
              </div>
            </section>

            <!-- Archer Towers Card -->
            <section class="card">
              <div class="card-header-flex">
                <h3>🏹 {t.inspector.towers}</h3>
                <div class="batch-actions">
                  <button type="button" class="btn-sm" onclick={() => handleBatchUpgradeTowers('built_only', 'tower5')}>
                    🛡️ {t.inspector.batchUpgradeTowersRoof}
                  </button>
                  <button type="button" class="btn-sm gold" onclick={() => handleBatchUpgradeTowers('built_only', 'tower6')}>
                    ⚡ {t.inspector.batchUpgradeTowersL6}
                  </button>
                </div>
              </div>

              <div class="filter-row">
                <span class="filter-label">Filter:</span>
                <button
                  type="button"
                  class="filter-btn"
                  class:active={towerFilterSide === 'all'}
                  onclick={() => towerFilterSide = 'all'}
                >All ({islandOverview.towers.length})</button>
                <button
                  type="button"
                  class="filter-btn"
                  class:active={towerFilterSide === 'left'}
                  onclick={() => towerFilterSide = 'left'}
                >⬅️ {t.inspector.leftSide} ({islandOverview.towers.filter(tw => tw.side === 'left').length})</button>
                <button
                  type="button"
                  class="filter-btn"
                  class:active={towerFilterSide === 'right'}
                  onclick={() => towerFilterSide = 'right'}
                >➡️ {t.inspector.rightSide} ({islandOverview.towers.filter(tw => tw.side === 'right').length})</button>
              </div>

              <div class="walls-columns">
                {#if towerFilterSide === 'all' || towerFilterSide === 'left'}
                  <div class="wall-column">
                    <h4>⬅️ {t.inspector.leftSide}</h4>
                    <div class="entities-list">
                      {#each islandOverview.towers.filter(tw => tw.side === 'left').reverse() as tower}
                        <div class="entity-card tower-card" class:level-max={tower.type === 'tower6'}>
                          <div class="entity-main">
                            <div class="entity-coords">
                              <span class="coord-badge">X = {tower.x}</span>
                              <span class="dist-badge">{t.inspector.distance} {tower.distanceToCastle}m</span>
                            </div>
                            <div class="entity-type">
                              <span class="tower-type-pill type-{tower.type}">
                                {#if tower.type === 'tower6'}
                                  🏹🏹🏹🏹 Level 6 (4 Archers)
                                {:else if tower.type === 'tower5'}
                                  🛡️ Level 5 (Roof)
                                {:else if tower.type === 'ballista'}
                                  🎯 Ballista
                                {:else if tower.type === 'baker'}
                                  🍞 Bakery
                                {:else if tower.type === 'knight'}
                                  ⚔️ Knight Tower
                                {:else if tower.type === 'tower0'}
                                  🪨 Dirt Mound (L0)
                                {:else}
                                  🏹 {tower.typeLabel}
                                {/if}
                              </span>
                            </div>
                          </div>
                          <div class="entity-actions tower-actions">
                            <button
                              type="button"
                              class="btn-xs gold"
                              disabled={tower.type === 'tower6'}
                              onclick={() => handleUpgradeSingleTower(tower.id, 'tower6')}
                              title="4 Archers"
                            >
                              4 Archers
                            </button>
                            <button
                              type="button"
                              class="btn-xs"
                              disabled={tower.type === 'tower5'}
                              onclick={() => handleUpgradeSingleTower(tower.id, 'tower5')}
                              title="Roof"
                            >
                              Roof (L5)
                            </button>
                            <button
                              type="button"
                              class="btn-xs"
                              disabled={tower.type === 'ballista'}
                              onclick={() => handleUpgradeSingleTower(tower.id, 'ballista')}
                              title="Ballista Tower"
                            >
                              Ballista
                            </button>
                            <button
                              type="button"
                              class="btn-xs"
                              disabled={tower.type === 'baker'}
                              onclick={() => handleUpgradeSingleTower(tower.id, 'baker')}
                              title="Bakery Tower"
                            >
                              Bakery
                            </button>
                            <button
                              type="button"
                              class="btn-xs"
                              disabled={tower.type === 'knight'}
                              onclick={() => handleUpgradeSingleTower(tower.id, 'knight')}
                              title="Knight Tower"
                            >
                              Knight
                            </button>
                            {#if tower.type !== 'tower0'}
                              <button
                                type="button"
                                class="btn-xs danger"
                                onclick={() => handleUpgradeSingleTower(tower.id, 'tower0')}
                                title="Reset to Dirt Mound"
                              >
                                L0
                              </button>
                            {/if}
                          </div>
                        </div>
                      {/each}
                    </div>
                  </div>
                {/if}

                {#if towerFilterSide === 'all' || towerFilterSide === 'right'}
                  <div class="wall-column">
                    <h4>➡️ {t.inspector.rightSide}</h4>
                    <div class="entities-list">
                      {#each islandOverview.towers.filter(tw => tw.side === 'right') as tower}
                        <div class="entity-card tower-card" class:level-max={tower.type === 'tower6'}>
                          <div class="entity-main">
                            <div class="entity-coords">
                              <span class="coord-badge">X = {tower.x}</span>
                              <span class="dist-badge">{t.inspector.distance} {tower.distanceToCastle}m</span>
                            </div>
                            <div class="entity-type">
                              <span class="tower-type-pill type-{tower.type}">
                                {#if tower.type === 'tower6'}
                                  🏹🏹🏹🏹 Level 6 (4 Archers)
                                {:else if tower.type === 'tower5'}
                                  🛡️ Level 5 (Roof)
                                {:else if tower.type === 'ballista'}
                                  🎯 Ballista
                                {:else if tower.type === 'baker'}
                                  🍞 Bakery
                                {:else if tower.type === 'knight'}
                                  ⚔️ Knight Tower
                                {:else if tower.type === 'tower0'}
                                  🪨 Dirt Mound (L0)
                                {:else}
                                  🏹 {tower.typeLabel}
                                {/if}
                              </span>
                            </div>
                          </div>
                          <div class="entity-actions tower-actions">
                            <button
                              type="button"
                              class="btn-xs gold"
                              disabled={tower.type === 'tower6'}
                              onclick={() => handleUpgradeSingleTower(tower.id, 'tower6')}
                              title="4 Archers"
                            >
                              4 Archers
                            </button>
                            <button
                              type="button"
                              class="btn-xs"
                              disabled={tower.type === 'tower5'}
                              onclick={() => handleUpgradeSingleTower(tower.id, 'tower5')}
                              title="Roof"
                            >
                              Roof (L5)
                            </button>
                            <button
                              type="button"
                              class="btn-xs"
                              disabled={tower.type === 'ballista'}
                              onclick={() => handleUpgradeSingleTower(tower.id, 'ballista')}
                              title="Ballista Tower"
                            >
                              Ballista
                            </button>
                            <button
                              type="button"
                              class="btn-xs"
                              disabled={tower.type === 'baker'}
                              onclick={() => handleUpgradeSingleTower(tower.id, 'baker')}
                              title="Bakery Tower"
                            >
                              Bakery
                            </button>
                            <button
                              type="button"
                              class="btn-xs"
                              disabled={tower.type === 'knight'}
                              onclick={() => handleUpgradeSingleTower(tower.id, 'knight')}
                              title="Knight Tower"
                            >
                              Knight
                            </button>
                            {#if tower.type !== 'tower0'}
                              <button
                                type="button"
                                class="btn-xs danger"
                                onclick={() => handleUpgradeSingleTower(tower.id, 'tower0')}
                                title="Reset to Dirt Mound"
                              >
                                L0
                              </button>
                            {/if}
                          </div>
                        </div>
                      {/each}
                    </div>
                  </div>
                {/if}
              </div>
            </section>

            <!-- Trees Card -->
            <section class="card">
              <div class="card-header-flex">
                <h3>🌲 {t.inspector.trees}</h3>
              </div>

              <!-- Safe Range Marking Tool -->
              <div class="range-tool-box">
                <div class="range-inputs">
                  <label class="range-label">
                    {t.inspector.rangeFrom}
                    <input
                      type="number"
                      bind:value={treeRangeMin}
                      class="coord-input"
                    />
                  </label>
                  <label class="range-label">
                    {t.inspector.rangeTo}
                    <input
                      type="number"
                      bind:value={treeRangeMax}
                      class="coord-input"
                    />
                  </label>
                  <button type="button" class="btn-sm" onclick={() => handleBatchTreeMark(true)}>
                    🪓 {t.inspector.markRange}
                  </button>
                  <button type="button" class="btn-sm secondary" onclick={() => handleBatchTreeMark(false)}>
                    🛡️ {t.inspector.unmarkRange}
                  </button>
                </div>
                <div class="range-hint">
                  ℹ️ {t.inspector.nearBeggarCampWarn}
                </div>
              </div>

              <!-- Filter bar for trees -->
              <div class="filter-row">
                <span class="filter-label">Filter:</span>
                <button
                  type="button"
                  class="filter-btn"
                  class:active={treeFilterStatus === 'all'}
                  onclick={() => treeFilterStatus = 'all'}
                >All ({islandOverview.trees.length})</button>
                <button
                  type="button"
                  class="filter-btn"
                  class:active={treeFilterStatus === 'standing'}
                  onclick={() => treeFilterStatus = 'standing'}
                >🟢 {t.inspector.standing} ({islandOverview.trees.filter(tr => !tr.marked).length})</button>
                <button
                  type="button"
                  class="filter-btn"
                  class:active={treeFilterStatus === 'marked'}
                  onclick={() => treeFilterStatus = 'marked'}
                >🪓 {t.inspector.markedToCut} ({islandOverview.trees.filter(tr => tr.marked).length})</button>
                <button
                  type="button"
                  class="filter-btn danger"
                  class:active={treeFilterStatus === 'danger'}
                  onclick={() => treeFilterStatus = 'danger'}
                >⚠️ Near Camps ({islandOverview.trees.filter(tr => tr.isNearBeggarCamp).length})</button>
              </div>

              <!-- Trees scrollable list -->
              <div class="trees-grid">
                {#each islandOverview.trees.filter(tr => {
                  if (treeFilterStatus === 'standing' && tr.marked) return false;
                  if (treeFilterStatus === 'marked' && !tr.marked) return false;
                  if (treeFilterStatus === 'danger' && !tr.isNearBeggarCamp) return false;
                  return true;
                }) as tree}
                  <div class="tree-item-card" class:near-camp={tree.isNearBeggarCamp} class:is-marked={tree.marked}>
                    <div class="tree-info">
                      <span class="coord-badge">X = {tree.x}</span>
                      {#if tree.isNearBeggarCamp}
                        <span class="camp-warning" title="Warning: Close to beggar camp!">
                          ⚠️ Camp {tree.nearCampDist}m
                        </span>
                      {:else}
                        <span class="dist-badge">{tree.distanceToCastle}m</span>
                      {/if}
                    </div>
                    <div class="tree-actions">
                      {#if tree.marked}
                        <button
                          type="button"
                          class="btn-xs marked-btn"
                          onclick={() => handleToggleTreeMark(tree.id, true)}
                        >
                          🪓 {t.inspector.markedToCut}
                        </button>
                      {:else}
                        <button
                          type="button"
                          class="btn-xs stand-btn"
                          onclick={() => handleToggleTreeMark(tree.id, false)}
                        >
                          🌲 {t.inspector.markTree}
                        </button>
                      {/if}
                    </div>
                  </div>
                {/each}
              </div>
            </section>
          {:else}
            <div class="empty-state">
              <p>{t.status.noFileLoaded}</p>
            </div>
          {/if}
        </div>
        {/if}

        {#if activeTab === 'resources'}
        <div class="tab-content">
          <section class="card">
            <h3>{t.resources.title}</h3>
            <div class="card-section">
              <div class="card-section-header">
                <h4>{t.resources.playerCoins}</h4>
                <span class="current-value">
                  {t.resources.current} {(() => {
                    const value = getPlayerCoins(data, {
                      mode: targetMode,
                      challengeIndex: selectedChallenge,
                      campaignIndex: selectedCampaign,
                      islandIndex: selectedIsland,
                      playerIndex: coinsPlayerIndex
                    });
                    return typeof value === "number" ? value : 0;
                  })()}
                </span>
              </div>
              <div class="form-row wrap">
                <label>
                  {t.resources.player}
                  <select
                    value={coinsPlayerIndex}
                    onchange={(event) => (coinsPlayerIndex = Number((event.currentTarget as HTMLSelectElement).value))}
                  >
                    <option value={0}>{t.resources.player} 1</option>
                    <option value={1}>{t.resources.player} 2</option>
                  </select>
                </label>
                <label>
                  {t.resources.coins}
                  <input
                    type="number"
                    min="0"
                    value={coinsAmount}
                    oninput={(event) => (coinsAmount = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <button type="button" onclick={handleCoinsUpdate}>{t.resources.apply}</button>
              </div>
            </div>

            <div class="card-section">
              <div class="card-section-header">
                <h4>{t.resources.playerGems}</h4>
                <span class="current-value">
                  {t.resources.current} {(() => {
                    const value = getPlayerGems(data, {
                      mode: targetMode,
                      challengeIndex: selectedChallenge,
                      campaignIndex: selectedCampaign,
                      islandIndex: selectedIsland,
                      playerIndex: gemsPlayerIndex
                    });
                    return typeof value === "number" ? value : 0;
                  })()}
                </span>
              </div>
              <div class="form-row wrap">
                <label>
                  {t.resources.player}
                  <select
                    value={gemsPlayerIndex}
                    onchange={(event) => (gemsPlayerIndex = Number((event.currentTarget as HTMLSelectElement).value))}
                  >
                    <option value={0}>{t.resources.player} 1</option>
                    <option value={1}>{t.resources.player} 2</option>
                  </select>
                </label>
                <label>
                  {t.resources.gems}
                  <input
                    type="number"
                    min="0"
                    value={gemsAmount}
                    oninput={(event) => (gemsAmount = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <button type="button" onclick={handleGemsUpdate}>{t.resources.apply}</button>
              </div>
            </div>
          </section>
        </div>
        {/if}

        {#if activeTab === 'navigation'}
        <div class="tab-content">
          <section class="card">
            <h3>{t.navigation.title}</h3>
            <div class="card-section">
              <div class="card-section-header">
                <h4>{t.navigation.fastTravel}</h4>
              </div>
              <div class="form-row wrap">
                <label>
                  {t.resources.coins}
                  <input
                    type="number"
                    min="0"
                    value={gotoCoins}
                    oninput={(event) => (gotoCoins = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <label>
                  {t.resources.gems}
                  <input
                    type="number"
                    min="0"
                    value={gotoGems}
                    oninput={(event) => (gotoGems = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <label>
                  {t.navigation.pikemen}
                  <input
                    type="number"
                    min="0"
                    value={gotoPikemen}
                    oninput={(event) => (gotoPikemen = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <label>
                  {t.navigation.farmers}
                  <input
                    type="number"
                    min="0"
                    value={gotoFarmers}
                    oninput={(event) => (gotoFarmers = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <label>
                  {t.navigation.boats}
                  <input
                    type="number"
                    min="0"
                    value={gotoBoats}
                    oninput={(event) => (gotoBoats = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <button type="button" onclick={handleGoto}>{t.navigation.updateDestination}</button>
              </div>
            </div>

            <div class="card-section">
              <div class="card-section-header">
                <h4>{t.navigation.takeOver}</h4>
              </div>
              <div class="form-row wrap">
                <label>
                  {t.navigation.playerCoins}
                  <input
                    type="number"
                    min="0"
                    value={takeOverCoins}
                    oninput={(event) => (takeOverCoins = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <label>
                  {t.navigation.archers}
                  <input
                    type="number"
                    min="0"
                    value={takeOverArchers}
                    oninput={(event) => (takeOverArchers = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <label>
                  {t.navigation.workers}
                  <input
                    type="number"
                    min="0"
                    value={takeOverWorkers}
                    oninput={(event) => (takeOverWorkers = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <label>
                  {t.navigation.formationOffset}
                  <input
                    type="number"
                    value={takeOverOffset}
                    oninput={(event) => (takeOverOffset = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <button type="button" onclick={handleTakeOver}>{t.navigation.cleanIsland}</button>
              </div>
            </div>

            <div class="card-section">
              <div class="button-group">
                <button type="button" onclick={handleDestroyPortals}>{t.navigation.destroy}</button>
                <button type="button" onclick={handleExterminate}>{t.navigation.exterminateEnemies}</button>
              </div>
            </div>

            <div class="card-section">
              <div class="card-section-header">
                <h4>{t.navigation.markTrees}</h4>
              </div>
              <div class="form-row wrap">
                <label>
                  {t.resources.player}
                  <select
                    value={treesPlayerIndex}
                    onchange={(event) => (treesPlayerIndex = Number((event.currentTarget as HTMLSelectElement).value))}
                  >
                    <option value={0}>{t.resources.player} 1</option>
                    <option value={1}>{t.resources.player} 2</option>
                  </select>
                </label>
                <label>
                  {t.resources.coins}
                  <input
                    type="number"
                    min="0"
                    value={treesCoins}
                    oninput={(event) => (treesCoins = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <button type="button" onclick={handleMarkTrees}>{t.navigation.mark}</button>
              </div>
            </div>

            <div class="card-section">
              <div class="card-section-header">
                <h4>{t.navigation.teleportTitle}</h4>
              </div>
              <div class="form-row wrap" style="align-items: flex-end; gap: 1rem;">
                <label>
                  {t.resources.player}
                  <select
                    value={teleportPlayerIndex}
                    onchange={(event) => (teleportPlayerIndex = Number((event.currentTarget as HTMLSelectElement).value))}
                  >
                    <option value={0}>{t.resources.player} 1</option>
                    <option value={1}>{t.resources.player} 2</option>
                  </select>
                </label>

                {#if islandOverview?.players}
                  {@const curPlayer = islandOverview.players.find(p => p.name === `Player ${teleportPlayerIndex + 1}`)}
                  {#if curPlayer}
                    <div style="display: flex; flex-direction: column; gap: 0.25rem; justify-content: center; padding-bottom: 0.25rem;">
                      <span style="font-size: 0.8rem; color: var(--color-text-secondary);">{t.resources.current}</span>
                      <span class="coord-badge" style="color: #64b5f6; font-size: 0.95rem;">X = {curPlayer.x}</span>
                    </div>
                  {/if}
                {/if}

                <label>
                  {t.navigation.teleportX}
                  <input
                    type="number"
                    step="0.5"
                    value={teleportTargetX}
                    oninput={(event) => (teleportTargetX = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>

                <button type="button" onclick={() => handleTeleportPlayer()}>
                  {t.navigation.teleportButton}
                </button>

                {#if islandOverview}
                  <button type="button" onclick={() => handleTeleportPlayer(islandOverview.castleX)} style="background: rgba(255,255,255,0.06);">
                    🏰 {t.navigation.teleportToCastle} ({islandOverview.castleX})
                  </button>
                {/if}
              </div>
            </div>
          </section>
        </div>
        {/if}

        {#if activeTab === 'combat'}
        <div class="tab-content">
          <section class="card">
            <h3>{t.combat.title}</h3>
            <div class="card-section">
              <div class="card-section-header">
                <h4>{t.combat.battleFormation}</h4>
              </div>
              <div class="form-row wrap">
                <label>
                  {t.resources.player}
                  <select
                    value={formationPlayerIndex}
                    onchange={(event) => (formationPlayerIndex = Number((event.currentTarget as HTMLSelectElement).value))}
                  >
                    <option value={0}>{t.resources.player} 1</option>
                    <option value={1}>{t.resources.player} 2</option>
                  </select>
                </label>
                <label>
                  {t.combat.positionX}
                  <input
                    type="number"
                    value={formationPosition}
                    oninput={(event) => (formationPosition = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <label>
                  {t.navigation.archers}
                  <input
                    type="number"
                    min="0"
                    value={formationArchers}
                    oninput={(event) => (formationArchers = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <label>
                  {t.navigation.pikemen}
                  <input
                    type="number"
                    min="0"
                    value={formationPikemen}
                    oninput={(event) => (formationPikemen = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <button type="button" onclick={handleFormation}>{t.combat.deploy}</button>
              </div>
            </div>

            <div class="card-section">
              <div class="card-section-header">
                <h4>{t.combat.catapultAmmo}</h4>
              </div>
              <div class="form-row wrap" style="align-items: flex-end; gap: 1.5rem;">
                <!-- Left Catapult -->
                <div style="flex: 1; min-width: 250px; background: rgba(255,255,255,0.02); padding: 0.75rem 1rem; border-radius: 8px; border: 1px solid var(--color-border);">
                  <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.5rem;">
                    <span style="font-weight: 600; color: #64b5f6;">{t.combat.catapultLeft}</span>
                    {#if leftCatapult}
                      <span class="coord-badge" style="font-size: 0.8rem;">X = {leftCatapult.x}</span>
                    {:else}
                      <span style="font-size: 0.8rem; color: var(--color-text-secondary);">(Not built)</span>
                    {/if}
                  </div>
                  {#if leftCatapult}
                    <div style="display: flex; gap: 0.5rem; align-items: center;">
                      <label style="flex: 1; margin: 0;">
                        {t.combat.barrelsCount}
                        <input
                          type="number"
                          min="0"
                          max="999"
                          value={leftCatapultInput}
                          oninput={(event) => (customLeftBarrels = Math.max(0, Number((event.currentTarget as HTMLInputElement).value)))}
                        />
                      </label>
                      <button type="button" onclick={() => handleSetCatapultBarrels(-1)} style="margin-top: 1.25rem;">
                        {t.combat.catapultApply}
                      </button>
                    </div>
                  {:else}
                    <p class="muted" style="margin: 0; font-size: 0.85rem;">{t.combat.noCatapultOnIsland}</p>
                  {/if}
                </div>

                <!-- Right Catapult -->
                <div style="flex: 1; min-width: 250px; background: rgba(255,255,255,0.02); padding: 0.75rem 1rem; border-radius: 8px; border: 1px solid var(--color-border);">
                  <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.5rem;">
                    <span style="font-weight: 600; color: #ffb74d;">{t.combat.catapultRight}</span>
                    {#if rightCatapult}
                      <span class="coord-badge" style="font-size: 0.8rem;">X = {rightCatapult.x}</span>
                    {:else}
                      <span style="font-size: 0.8rem; color: var(--color-text-secondary);">(Not built)</span>
                    {/if}
                  </div>
                  {#if rightCatapult}
                    <div style="display: flex; gap: 0.5rem; align-items: center;">
                      <label style="flex: 1; margin: 0;">
                        {t.combat.barrelsCount}
                        <input
                          type="number"
                          min="0"
                          max="999"
                          value={rightCatapultInput}
                          oninput={(event) => (customRightBarrels = Math.max(0, Number((event.currentTarget as HTMLInputElement).value)))}
                        />
                      </label>
                      <button type="button" onclick={() => handleSetCatapultBarrels(1)} style="margin-top: 1.25rem;">
                        {t.combat.catapultApply}
                      </button>
                    </div>
                  {:else}
                    <p class="muted" style="margin: 0; font-size: 0.85rem;">{t.combat.noCatapultOnIsland}</p>
                  {/if}
                </div>
              </div>
            </div>
          </section>
        </div>
        {/if}

        {#if activeTab === 'construction'}
        <div class="tab-content">
          <section class="card">
            <h3>{t.construction.title}</h3>
            <div class="card-section">
              <div class="card-section-header">
                <h4>{t.construction.pimpIsland}</h4>
              </div>
              <div class="form-row wrap">
                <label>
                  {t.navigation.playerCoins}
                  <input
                    type="number"
                    min="0"
                    value={pimpCoins}
                    oninput={(event) => (pimpCoins = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <label>
                  {t.construction.unitsToRecruit}
                  <input
                    type="number"
                    min="0"
                    value={pimpSpawnCount}
                    oninput={(event) => (pimpSpawnCount = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <button type="button" onclick={handlePimp}>{t.construction.upgrade}</button>
              </div>
            </div>
          </section>
        </div>
        {/if}

        {#if activeTab === 'recruitment'}
        <div class="tab-content">
          <section class="card">
            <h3>{t.recruitment.title}</h3>
            <div class="card-section">
              <div class="card-section-header">
                <h4>{t.recruitment.rallyPoint}</h4>
              </div>
              <div class="form-row wrap">
                <label>
                  {t.resources.player}
                  <select
                    value={archerPlayerIndex}
                    onchange={(event) => (archerPlayerIndex = Number((event.currentTarget as HTMLSelectElement).value))}
                  >
                    <option value={0}>{t.resources.player} 1</option>
                    <option value={1}>{t.resources.player} 2</option>
                  </select>
                </label>
                {#if islandOverview?.players}
                  {@const curP = islandOverview.players.find(p => p.name === `Player ${archerPlayerIndex + 1}`)}
                  {#if curP}
                    <div style="display: flex; flex-direction: column; gap: 0.25rem; justify-content: center; padding-bottom: 0.25rem;">
                      <span style="font-size: 0.8rem; color: var(--color-text-secondary); font-family: 'Cinzel', serif;">Player Position:</span>
                      <span class="coord-badge" style="color: #64b5f6; font-size: 0.95rem;">X = {curP.x} (Y = {curP.y})</span>
                    </div>
                  {/if}
                {/if}
              </div>
            </div>

            <div class="card-section">
              <div class="card-section-header">
                <h4>{t.recruitment.archers}</h4>
                <span class="current-value">{t.recruitment.current} {countUnits(data, selectedCampaign, selectedIsland, ARCHER_PREFABS, targetMode, selectedChallenge) ?? 0}</span>
              </div>
              <div class="form-row wrap">
                <label>
                  {t.recruitment.quantity}
                  <input
                    type="number"
                    min="1"
                    value={archerCount}
                    oninput={(event) => (archerCount = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <button type="button" onclick={handleSpawnArchers}>{t.recruitment.add}</button>
              </div>
            </div>

            <div class="card-section">
              <div class="card-section-header">
                <h4>{t.recruitment.workers}</h4>
                <span class="current-value">{t.recruitment.current} {countUnits(data, selectedCampaign, selectedIsland, WORKER_PREFABS, targetMode, selectedChallenge) ?? 0}</span>
              </div>
              <div class="form-row wrap">
                <label>
                  {t.recruitment.quantity}
                  <input
                    type="number"
                    min="1"
                    value={workerCount}
                    oninput={(event) => (workerCount = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <button type="button" onclick={handleSpawnWorkers}>{t.recruitment.add}</button>
              </div>
            </div>

            <div class="card-section">
              <div class="card-section-header">
                <h4>{t.recruitment.farmers}</h4>
                <span class="current-value">{t.recruitment.current} {countUnits(data, selectedCampaign, selectedIsland, FARMER_PREFABS, targetMode, selectedChallenge) ?? 0}</span>
              </div>
              <div class="form-row wrap">
                <label>
                  {t.recruitment.quantity}
                  <input
                    type="number"
                    min="1"
                    value={farmerCount}
                    oninput={(event) => (farmerCount = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <button type="button" onclick={handleSpawnFarmers}>{t.recruitment.add}</button>
              </div>
            </div>

            <div class="card-section">
              <div class="card-section-header">
                <h4>{t.recruitment.pikemen}</h4>
                <span class="current-value">{t.recruitment.current} {countUnits(data, selectedCampaign, selectedIsland, PIKEMAN_PREFABS, targetMode, selectedChallenge) ?? 0}</span>
              </div>
              <div class="form-row wrap">
                <label>
                  {t.recruitment.quantity}
                  <input
                    type="number"
                    min="1"
                    value={pikemanCount}
                    oninput={(event) => (pikemanCount = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <button type="button" onclick={handleSpawnPikemen}>{t.recruitment.add}</button>
              </div>
            </div>

            <div class="card-section">
              <div class="card-section-header">
                <h4>{t.recruitment.knights}</h4>
                <span class="current-value">
                  {t.recruitment.current} {knightStats.total} 
                  <span style="font-size: 0.8rem; color: var(--color-gold-light); margin-left: 0.35rem;">
                    (⬅️ {knightStats.left} | {knightStats.right} ➡️)
                  </span>
                </span>
              </div>
              <div class="form-row wrap">
                <label>
                  {t.recruitment.quantity}
                  <input
                    type="number"
                    min="1"
                    value={knightCount}
                    oninput={(event) => (knightCount = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <label>
                  {t.recruitment.side}
                  <select
                    value={knightSide}
                    onchange={(event) => (knightSide = Number((event.currentTarget as HTMLSelectElement).value) as 1 | -1)}
                  >
                    <option value={1}>{t.recruitment.sideRight}</option>
                    <option value={-1}>{t.recruitment.sideLeft}</option>
                  </select>
                </label>
                <label class="checkbox-label">
                  <input
                    type="checkbox"
                    bind:checked={knightWithArchers}
                    class="custom-checkbox"
                  />
                  <span>{t.recruitment.withArchers}</span>
                </label>
                <button type="button" onclick={handleSpawnKnights}>{t.recruitment.add}</button>
              </div>
            </div>

            <div class="card-section">
              <div class="card-section-header">
                <h4>{t.recruitment.combinedRecruitment}</h4>
              </div>
              <div class="form-row wrap">
                <label>
                  {t.recruitment.archers}
                  <input
                    type="number"
                    min="0"
                    value={comboArchers}
                    oninput={(event) => (comboArchers = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <label>
                  {t.recruitment.workers}
                  <input
                    type="number"
                    min="0"
                    value={comboWorkers}
                    oninput={(event) => (comboWorkers = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <label>
                  {t.recruitment.pikemen}
                  <input
                    type="number"
                    min="0"
                    value={comboPikemen}
                    oninput={(event) => (comboPikemen = Number((event.currentTarget as HTMLInputElement).value))}
                  />
                </label>
                <button type="button" onclick={handleSpawnCombo}>{t.recruitment.addUnits}</button>
              </div>
            </div>
          </section>
        </div>
        {/if}
      </div>
    </section>
  {/if}
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    background: #0a0a0a;
    color: #ffffff;
    font-family: "Inter", system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    line-height: 1.6;
  }

  main {
    margin: 0 auto;
    padding: 2rem;
    max-width: 1400px;
    min-height: 100vh;
  }

  .welcome {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2rem;
  }

  .welcome-content {
    max-width: 900px;
    width: 100%;
    text-align: center;
  }

  .welcome h1 {
    font-size: 3rem;
    margin-bottom: 1rem;
  }

  .subtitle {
    font-size: 1.25rem;
    color: #888888;
    margin-bottom: 3rem;
  }

  .os-detection {
    margin-bottom: 2.5rem;
  }

  .os-badge {
    display: inline-flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem 1.5rem;
    background: #141414;
    border: 1px solid #2a2a2a;
    border-radius: 12px;
    font-size: 1rem;
    color: #cccccc;
  }

  .os-badge svg {
    color: #3b82f6;
  }

  .os-badge strong {
    color: #ffffff;
  }

  .default-path {
    margin-bottom: 3rem;
    text-align: left;
    background: #141414;
    padding: 1.5rem;
    border-radius: 16px;
    border: 1px solid #2a2a2a;
  }

  .path-label {
    display: block;
    font-weight: 600;
    margin-bottom: 1rem;
    color: #cccccc;
  }

  .path-display {
    margin-bottom: 0.75rem;
  }

  .path-display code {
    display: block;
    padding: 1rem;
    background: #0a0a0a;
    border-radius: 8px;
    font-size: 0.95rem;
    word-break: break-all;
  }

  .path-hint {
    font-size: 0.85rem;
    color: #666666;
    margin: 0;
  }

  .welcome-actions {
    margin-bottom: 4rem;
  }

  button.large {
    padding: 1.25rem 3rem;
    font-size: 1.1rem;
    display: inline-flex;
    align-items: center;
    gap: 0.75rem;
  }

  button.large svg {
    flex-shrink: 0;
  }

  .spinner {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  .features {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: 1.5rem;
    text-align: center;
  }

  .feature {
    padding: 1.5rem;
    background: var(--color-bg-card);
    border: 2px solid var(--color-border);
    border-radius: 8px;
    transition: all 0.3s ease;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.4);
    position: relative;
  }

  .feature::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 2px;
    background: linear-gradient(90deg, transparent 0%, var(--color-gold) 50%, transparent 100%);
    opacity: 0.3;
  }

  .feature:hover {
    border-color: var(--color-gold);
    transform: translateY(-4px);
    box-shadow: 0 8px 16px rgba(0, 0, 0, 0.6), 0 0 30px rgba(212, 175, 55, 0.2);
  }

  .feature-icon {
    font-size: 2.5rem;
    margin-bottom: 1rem;
  }

  .feature h3 {
    margin: 0 0 0.5rem;
    font-size: 1.1rem;
    color: var(--color-gold-light);
    font-family: 'Cinzel', serif;
    font-weight: 700;
  }

  .feature p {
    margin: 0;
    font-size: 0.9rem;
    color: var(--color-text-secondary);
    font-family: 'Cinzel', serif;
  }

  header {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-bottom: 2.5rem;
    padding: 1.5rem;
    background: linear-gradient(135deg, var(--color-bg-card) 0%, var(--color-bg-secondary) 100%);
    border-radius: 8px;
    border: 2px solid var(--color-border);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.5);
    position: relative;
  }

  header::after {
    content: '';
    position: absolute;
    bottom: 0;
    left: 20%;
    right: 20%;
    height: 2px;
    background: linear-gradient(90deg, transparent 0%, var(--color-gold) 50%, transparent 100%);
    opacity: 0.4;
  }

  .header-top {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1.5rem;
    flex-wrap: wrap;
  }

  .header-top h1 {
    margin: 0;
  }

  .header-actions {
    display: flex;
    gap: 0.75rem;
  }

  .file-path {
    font-size: 0.9rem;
    color: var(--color-stone-light);
    margin: 0;
    font-family: 'Courier New', monospace;
  }

  p {
    margin: 0;
    color: var(--color-text-secondary);
    font-family: 'Cinzel', serif;
  }

  .actions {
    display: flex;
    gap: 0.75rem;
    flex-wrap: wrap;
  }

  button.large {
    font-size: 1.1rem;
    padding: 1rem 2rem;
  }

  .status {
    font-weight: 600;
    padding: 1rem 1.5rem;
    border-radius: 6px;
    border: 2px solid;
    font-family: 'Cinzel', serif;
    letter-spacing: 0.02em;
  }

  .status.error {
    color: #ff6b6b;
    background: rgba(139, 37, 0, 0.2);
    border-color: var(--color-error);
  }

  .status.success {
    color: #98fb98;
    background: rgba(107, 142, 35, 0.2);
    border-color: var(--color-success);
  }

  .status.info {
    color: var(--color-gold-light);
    background: rgba(212, 175, 55, 0.1);
    border-color: var(--color-gold);
  }

  .content {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .editor-area {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .context-card {
    background: var(--color-bg-card);
    border: 2px solid var(--color-border);
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.4);
  }

  .tabs {
    display: flex;
    gap: 0.5rem;
    padding: 0.5rem;
    background: var(--color-bg-secondary);
    border-radius: 8px;
    border: 2px solid var(--color-border);
    overflow-x: auto;
    scrollbar-width: thin;
  }

  .tabs::-webkit-scrollbar {
    height: 6px;
  }

  .tabs::-webkit-scrollbar-track {
    background: var(--color-bg-primary);
    border-radius: 4px;
  }

  .tabs::-webkit-scrollbar-thumb {
    background: var(--color-border);
    border-radius: 4px;
  }

  .tab {
    padding: 0.75rem 1.5rem;
    background: transparent;
    border: 2px solid transparent;
    border-radius: 4px;
    color: var(--color-text-muted);
    font-family: 'Cinzel', serif;
    font-weight: 600;
    font-size: 0.9rem;
    cursor: pointer;
    transition: all 0.3s ease;
    white-space: nowrap;
    flex-shrink: 0;
  }

  .tab:hover {
    background: rgba(212, 175, 55, 0.1);
    color: var(--color-gold-light);
    border-color: var(--color-border-light);
  }

  .tab.active {
    background: linear-gradient(135deg, var(--color-gold) 0%, var(--color-gold-dark) 100%);
    color: var(--color-bg-primary);
    border-color: var(--color-gold-dark);
    box-shadow: 0 4px 12px rgba(212, 175, 55, 0.4);
    font-weight: 700;
  }

  .tab-content {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .card {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 1.5rem;
    border-radius: 8px;
    background: var(--color-bg-card);
    border: 2px solid var(--color-border);
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.4);
    transition: all 0.3s ease;
    position: relative;
  }

  .card::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 2px;
    background: linear-gradient(90deg, transparent 0%, var(--color-gold) 50%, transparent 100%);
    opacity: 0.3;
  }

  .card:hover {
    border-color: var(--color-border-light);
    box-shadow: 0 6px 12px rgba(0, 0, 0, 0.5);
  }

  .card h3 {
    margin: 0;
    font-size: 1.2rem;
    font-weight: 700;
    color: var(--color-gold-light);
    font-family: 'Cinzel', serif;
    letter-spacing: 0.03em;
  }

  .muted {
    margin: 0;
    font-size: 0.85rem;
    color: var(--color-text-muted);
    font-family: 'Cinzel', serif;
  }

  .card-section {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    padding: 1rem;
    border-radius: 6px;
    background: var(--color-bg-secondary);
    border: 2px solid var(--color-border);
  }

  .card-section-header {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: 0.75rem;
    flex-wrap: wrap;
  }

  .card-section-header h4 {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
    color: var(--color-text-primary);
    font-family: 'Cinzel', serif;
    letter-spacing: 0.02em;
  }

  .current-value {
    margin: 0;
    font-size: 0.85rem;
    color: var(--color-stone-light);
    font-weight: 600;
    font-family: 'Cinzel', serif;
  }

  .form-row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: flex-end;
  }

  .form-row label {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    flex: 1 1 140px;
    font-weight: 600;
    font-size: 0.9rem;
    color: var(--color-text-secondary);
    font-family: 'Cinzel', serif;
  }

  .form-row input:not([type="checkbox"]),
  .form-row select {
    border-radius: 4px;
    border: 2px solid var(--color-border);
    padding: 0.65rem 0.85rem;
    font-size: 0.95rem;
    background: var(--color-bg-secondary);
    color: var(--color-text-primary);
    font-family: 'Cinzel', serif;
    transition: all 0.2s ease;
  }

  .form-row input:not([type="checkbox"]):focus,
  .form-row select:focus {
    outline: none;
    border-color: var(--color-gold);
    box-shadow: 0 0 0 3px rgba(212, 175, 55, 0.2);
  }

  .form-row input:not([type="checkbox"]):hover,
  .form-row select:hover {
    border-color: var(--color-border-light);
  }

  .form-row label.checkbox-label {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 0.75rem;
    cursor: pointer;
    flex: 0 0 auto;
    padding-bottom: 0.75rem;
    user-select: none;
  }

  .form-row label.checkbox-label span {
    color: var(--color-text-primary);
    font-size: 0.9rem;
    cursor: pointer;
  }

  .custom-checkbox {
    width: 20px;
    height: 20px;
    min-width: 20px;
    cursor: pointer;
    accent-color: var(--color-gold);
    border-radius: 4px;
  }

  .form-row button {
    align-self: stretch;
    padding-inline: 1.5rem;
    background: linear-gradient(135deg, var(--color-gold) 0%, var(--color-gold-dark) 100%);
    color: var(--color-bg-primary);
    min-width: 140px;
    border-color: var(--color-gold-dark);
    font-weight: 700;
  }

  .button-group {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
  }

  .button-group button {
    flex: 1 1 160px;
    background: linear-gradient(135deg, var(--color-gold) 0%, var(--color-gold-dark) 100%);
    color: var(--color-bg-primary);
    border-color: var(--color-gold-dark);
    font-weight: 700;
  }

  code {
    background: var(--color-bg-secondary);
    border-radius: 4px;
    padding: 0.3rem 0.6rem;
    font-size: 0.9rem;
    color: var(--color-gold-light);
    border: 1px solid var(--color-border);
    font-family: 'Courier New', monospace;
  }

  @media (max-width: 900px) {
    main {
      padding: 1rem;
    }

    .welcome h1 {
      font-size: 2rem;
    }

    .subtitle {
      font-size: 1rem;
    }

    .features {
      grid-template-columns: repeat(2, 1fr);
    }

    .header-top {
      flex-direction: column;
      align-items: stretch;
    }

    .header-actions {
      width: 100%;
      justify-content: stretch;
    }

    .header-actions button {
      flex: 1;
    }

    .tabs {
      overflow-x: auto;
    }

    button.large {
      width: 100%;
      justify-content: center;
    }
  }

  @media (max-width: 600px) {
    .features {
      grid-template-columns: 1fr;
    }

    .welcome h1 {
      font-size: 1.75rem;
    }
  }
  .overview-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
    gap: 1rem;
    margin-bottom: 1.5rem;
  }

  .overview-item {
    background: var(--color-bg-secondary, #2a2218);
    border: 1px solid var(--color-border, #4a3d2a);
    border-radius: 6px;
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
  }

  .overview-item.highlight {
    border-color: var(--color-gold, #d4af37);
    background: linear-gradient(180deg, rgba(212, 175, 55, 0.1) 0%, rgba(42, 34, 24, 0.9) 100%);
  }

  .overview-label {
    font-size: 0.85rem;
    color: var(--color-text-secondary, #c4b5a0);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .overview-value {
    font-size: 1.6rem;
    font-weight: 700;
    color: var(--color-text-primary, #f5f0e8);
  }

  .text-gold {
    color: var(--color-gold, #d4af37) !important;
  }

  .overview-sub {
    font-size: 0.85rem;
    color: var(--color-text-muted, #8b8070);
  }

  .special-points-bar {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem 1rem;
    background: rgba(0, 0, 0, 0.3);
    border-radius: 6px;
    border: 1px dashed var(--color-border, #4a3d2a);
  }

  .special-points-title {
    font-weight: 600;
    font-size: 0.9rem;
    color: var(--color-gold-light, #f4d03f);
  }

  .special-points-list {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .special-badge {
    padding: 0.25rem 0.6rem;
    border-radius: 4px;
    font-size: 0.8rem;
    font-weight: 600;
    border: 1px solid var(--color-border);
    background: var(--color-bg-card);
  }

  .special-badge.beggar_camp {
    background: rgba(205, 133, 63, 0.25);
    border-color: var(--color-warning, #cd853f);
    color: #f4d03f;
  }

  .special-badge.portal {
    background: rgba(138, 43, 226, 0.25);
    border-color: #9370db;
    color: #dda0dd;
  }

  .special-badge.statue {
    background: rgba(70, 130, 180, 0.25);
    border-color: #4682b4;
    color: #87ceeb;
  }

  .special-badge.mine {
    background: rgba(169, 169, 169, 0.25);
    border-color: #a9a9a9;
    color: #dcdcdc;
  }

  .special-badge.wreck {
    background: rgba(210, 105, 30, 0.25);
    border-color: #d2691e;
    color: #f4a460;
  }

  .special-badge.boat {
    background: rgba(30, 144, 255, 0.25);
    border-color: #1e90ff;
    color: #00bfff;
  }

  .card-header-flex {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
    margin-bottom: 1rem;
  }

  .batch-actions {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .btn-sm {
    padding: 0.45rem 0.9rem;
    font-size: 0.85rem;
  }

  .btn-sm.gold, .btn-xs.gold {
    background: linear-gradient(180deg, var(--color-gold) 0%, var(--color-gold-dark) 100%);
    color: #1a1410;
    border-color: var(--color-gold-light);
    font-weight: 700;
  }

  .btn-sm.secondary {
    background: var(--color-bg-card);
    border-color: var(--color-border);
  }

  .btn-xs {
    padding: 0.25rem 0.55rem;
    font-size: 0.8rem;
    border-radius: 4px;
    border: 1px solid var(--color-border);
    background: var(--color-bg-card);
    color: var(--color-text-primary);
    cursor: pointer;
    font-weight: 600;
    transition: all 0.15s ease;
  }

  .btn-xs:hover:not(:disabled) {
    border-color: var(--color-gold);
    color: var(--color-gold-light);
  }

  .btn-xs:disabled {
    opacity: 0.35;
    cursor: not-allowed;
  }

  .btn-xs.danger {
    background: rgba(139, 37, 0, 0.3);
    border-color: var(--color-error, #8b2500);
    color: #ff9980;
  }

  .filter-row {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 1rem;
    padding-bottom: 0.75rem;
    border-bottom: 1px solid var(--color-border);
  }

  .filter-label {
    font-size: 0.85rem;
    color: var(--color-text-secondary);
    font-weight: 600;
  }

  .filter-btn {
    padding: 0.3rem 0.7rem;
    font-size: 0.8rem;
    background: var(--color-bg-secondary);
    border: 1px solid var(--color-border);
    border-radius: 4px;
    color: var(--color-text-secondary);
    cursor: pointer;
  }

  .filter-btn.active {
    background: var(--color-bg-card);
    border-color: var(--color-gold);
    color: var(--color-gold-light);
    font-weight: 700;
  }

  .filter-btn.danger.active {
    border-color: #e67e22;
    color: #f39c12;
  }

  .walls-columns {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
    gap: 1.5rem;
  }

  .wall-column h4 {
    margin: 0 0 0.75rem 0;
    font-size: 1.05rem;
    color: var(--color-gold-light);
  }

  .entities-list {
    display: flex;
    flex-direction: column;
    gap: 0.6rem;
  }

  .entity-card {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 1rem;
    background: var(--color-bg-secondary);
    border: 1px solid var(--color-border);
    border-radius: 6px;
    gap: 0.75rem;
    transition: all 0.2s ease;
  }

  .entity-card.level-max {
    border-color: rgba(212, 175, 55, 0.5);
    background: linear-gradient(90deg, rgba(212, 175, 55, 0.05) 0%, rgba(42, 34, 24, 0.9) 100%);
  }

  .entity-main {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
  }

  .entity-coords {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .coord-badge {
    font-family: monospace;
    font-weight: 700;
    font-size: 0.85rem;
    color: #e0d0b0;
    background: rgba(0, 0, 0, 0.4);
    padding: 0.15rem 0.45rem;
    border-radius: 3px;
  }

  .dist-badge {
    font-size: 0.75rem;
    color: var(--color-text-muted);
  }

  .wall-level-pill {
    font-size: 0.8rem;
    font-weight: 600;
  }

  .wall-level-pill.lvl-5 {
    color: var(--color-gold-light);
  }

  .wall-level-pill.lvl-5.horn {
    color: #ffd700;
    text-shadow: 0 0 6px rgba(255, 215, 0, 0.4);
  }

  .wall-level-pill.lvl-0 {
    color: var(--color-text-muted);
  }

  .tower-type-pill {
    font-size: 0.8rem;
    font-weight: 600;
  }

  .tower-type-pill.type-tower6 {
    color: #ffd700;
    text-shadow: 0 0 6px rgba(255, 215, 0, 0.4);
  }

  .tower-type-pill.type-tower5 {
    color: var(--color-gold-light);
  }

  .tower-type-pill.type-ballista {
    color: #ff9966;
  }

  .tower-type-pill.type-baker {
    color: #f4d03f;
  }

  .tower-type-pill.type-knight {
    color: #87ceeb;
  }

  .tower-type-pill.type-tower0 {
    color: var(--color-text-muted);
  }

  .entity-actions {
    display: flex;
    gap: 0.35rem;
    flex-wrap: wrap;
  }

  .range-tool-box {
    background: rgba(0, 0, 0, 0.25);
    border: 1px solid var(--color-border);
    border-radius: 6px;
    padding: 1rem;
    margin-bottom: 1.25rem;
  }

  .range-inputs {
    display: flex;
    flex-wrap: wrap;
    align-items: flex-end;
    gap: 0.75rem;
    margin-bottom: 0.5rem;
  }

  .range-label {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    font-size: 0.85rem;
    color: var(--color-text-secondary);
  }

  .coord-input {
    width: 90px;
    padding: 0.4rem 0.6rem;
    background: #0a0a0a;
    border: 1px solid var(--color-border);
    border-radius: 4px;
    color: var(--color-text-primary);
    font-family: monospace;
    font-size: 0.9rem;
  }

  .range-hint {
    font-size: 0.8rem;
    color: var(--color-warning, #cd853f);
  }

  .trees-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(210px, 1fr));
    gap: 0.6rem;
    max-height: 480px;
    overflow-y: auto;
    padding-right: 0.5rem;
  }

  .tree-item-card {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    padding: 0.65rem 0.85rem;
    background: var(--color-bg-secondary);
    border: 1px solid var(--color-border);
    border-radius: 6px;
  }

  .tree-item-card.is-marked {
    border-color: rgba(205, 133, 63, 0.6);
    background: rgba(205, 133, 63, 0.08);
  }

  .tree-item-card.near-camp {
    border-color: #e67e22;
    box-shadow: 0 0 6px rgba(230, 126, 34, 0.2);
  }

  .tree-info {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .camp-warning {
    font-size: 0.75rem;
    color: #f39c12;
    font-weight: 700;
  }

  .marked-btn {
    width: 100%;
    background: rgba(205, 133, 63, 0.3);
    border-color: var(--color-warning);
    color: #f4d03f;
  }

  .stand-btn {
    width: 100%;
  }

  .deities-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
    gap: 1rem;
  }

  .deity-card {
    background: var(--color-bg-secondary);
    border: 1px solid var(--color-border);
    border-radius: 6px;
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    transition: all 0.2s ease;
  }

  .deity-card.active {
    border-color: var(--color-gold);
    background: linear-gradient(180deg, rgba(212, 175, 55, 0.12) 0%, rgba(42, 34, 24, 0.9) 100%);
    box-shadow: 0 0 10px rgba(212, 175, 55, 0.15);
  }

  .deity-card.unlocked {
    border-color: #4682b4;
  }

  .deity-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .deity-name {
    font-weight: 700;
    font-size: 0.95rem;
    color: var(--color-text-primary);
  }

  .deity-island-tag {
    font-size: 0.75rem;
    padding: 0.15rem 0.45rem;
    border-radius: 3px;
    background: rgba(0, 0, 0, 0.4);
    color: var(--color-gold-light);
    border: 1px solid var(--color-border);
  }

  .deity-status-pill {
    font-size: 0.85rem;
    font-weight: 700;
  }

  .deity-status-pill.status-2 {
    color: #ffd700;
    text-shadow: 0 0 6px rgba(255, 215, 0, 0.4);
  }

  .deity-status-pill.status-1 {
    color: #87ceeb;
  }

  .deity-status-pill.status-0 {
    color: var(--color-text-muted);
  }

  .deity-actions {
    display: flex;
    gap: 0.4rem;
    flex-wrap: wrap;
  }

  .asset-editor-shell {
    min-height: 100vh;
    background: var(--color-bg-primary);
  }

  .asset-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
    padding: 1.25rem 1.5rem;
    border-bottom: 1px solid var(--color-border);
    background: var(--color-bg-secondary);
  }

  .asset-header h1,
  .asset-card h2 {
    margin: 0;
  }

  .asset-content {
    width: min(1100px, calc(100% - 2rem));
    margin: 0 auto;
    padding: 1.5rem 0 3rem;
  }

  .asset-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
    gap: 1rem;
    margin-top: 1rem;
  }

  .asset-card {
    display: flex;
    flex-direction: column;
    gap: 0.85rem;
  }

  .asset-card label {
    display: grid;
    grid-template-columns: 1fr 110px;
    align-items: center;
    gap: 0.75rem;
  }

  .asset-card label.asset-checkbox-label {
    display: flex;
    align-items: flex-start;
    gap: 0.6rem;
    margin-top: 0.5rem;
    cursor: pointer;
    font-size: 0.88rem;
    line-height: 1.35;
  }

  .asset-card label.asset-checkbox-label input[type="checkbox"] {
    width: 1.1rem;
    height: 1.1rem;
    margin: 0.15rem 0 0 0;
    flex-shrink: 0;
    cursor: pointer;
  }

  .asset-card input {
    width: 100%;
    box-sizing: border-box;
  }

  .safety-card,
  .empty-assets {
    margin-top: 1rem;
  }

  .safety-card code {
    overflow-wrap: anywhere;
  }

  @media (max-width: 700px) {
    .asset-header {
      align-items: flex-start;
      flex-direction: column;
    }

    .asset-header .header-actions {
      width: 100%;
      flex-wrap: wrap;
    }
  }
</style>
