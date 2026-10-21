import type { JSONValue } from "$lib/components/FieldEditor.svelte";

type MutableJSONObject = Record<string, any>;

export interface SelectionOptions {
  campaignIndex: number;
  islandIndex: number;
  playerIndex: number;
  mode?: "campaign" | "challenge";
  challengeIndex?: number;
}

export interface CoinsOptions extends SelectionOptions {
  coins: number;
}



export type CoinsQueryOptions = SelectionOptions;

export interface GemsOptions extends SelectionOptions {
  gems: number;
}

export type GemsQueryOptions = SelectionOptions;

export interface SpawnArchersOptions extends SelectionOptions {
  count: number;
}

export interface SpawnKnightsOptions extends SelectionOptions {
  count: number;
  side: -1 | 1; // -1: Left, 1: Right
  withArchers?: boolean;
}

interface CampaignIslandOptions {
  campaignIndex: number;
  islandIndex: number;
  targetMode?: "campaign" | "challenge";
  challengeIndex?: number;
}

export interface GotoOptions extends CampaignIslandOptions {
  coins?: number;
  gems?: number;
  pikemen?: number;
  farmers?: number;
  boats?: number;
}

export interface TakeOverOptions extends CampaignIslandOptions {
  coins?: number;
  archerCount?: number;
  workerCount?: number;
  formationOffset?: number;
}

export interface DestroyPortalsOptions extends CampaignIslandOptions { }

export interface ExterminateOptions extends CampaignIslandOptions { }

export interface FormationOptions extends CampaignIslandOptions {
  playerIndex: number;
  position: number;
  archers?: number;
  pikemen?: number;
}

export interface PimpOptions extends CampaignIslandOptions {
  spawnCount?: number;
  coins?: number;
}

export interface TreesOptions extends CampaignIslandOptions {
  playerIndex?: number;
  coins?: number;
}

export interface SpawnUnitsOptions extends SelectionOptions {
  archers?: number;
  workers?: number;
  pikemen?: number;
}

export interface WallInfo {
  id: string;
  name: string;
  level: number;
  hasHorn: boolean;
  x: number;
  side: "left" | "right";
  distanceToCastle: number;
}

export interface TreeInfo {
  id: string;
  x: number;
  side: "left" | "right";
  marked: boolean;
  distanceToCastle: number;
  isNearBeggarCamp: boolean;
  nearCampDist?: number;
}

export interface SpecialPointInfo {
  name: string;
  type: "beggar_camp" | "portal" | "statue" | "mine" | "wreck" | "boat" | "other";
  x: number;
}

export interface DeityInfo {
  index: number;
  name: string;
  island: number;
  status: number; // 0: Locked, 1: Unlocked, 2: Active
}

export interface SetDeityStatusOptions {
  campaignIndex?: number;
  challengeIndex?: number;
  targetMode?: "campaign" | "challenge";
  deityIndex: number;
  status: number;
}

export interface UnlockAllDeitiesOptions {
  campaignIndex?: number;
  challengeIndex?: number;
  targetMode?: "campaign" | "challenge";
  status?: number; // 0, 1, 2
}

export interface PlayerInfo {
  name: string;
  x: number;
  y: number;
}

export interface TeleportPlayerOptions extends SelectionOptions {
  x: number;
}

export interface UpgradeCastleOptions extends CampaignIslandOptions {
  targetLevel: number;
}

export interface CatapultInfo {
  id: string;
  side: -1 | 1;
  sideLabel: "left" | "right";
  x: number;
  oilBarrels: number;
}

export interface SetCatapultOilBarrelsOptions extends CampaignIslandOptions {
  side: -1 | 1;
  oilBarrels: number;
}

export interface IslandOverview {
  castleX: number;
  castleLevel: number;
  castleName: string;
  players: PlayerInfo[];
  walls: WallInfo[];
  towers: TowerInfo[];
  catapults: CatapultInfo[];
  trees: TreeInfo[];
  specialPoints: SpecialPointInfo[];
  deities: DeityInfo[];
}

export type TowerType =
  | "tower0"
  | "tower1"
  | "tower2"
  | "tower3"
  | "tower4"
  | "tower5"
  | "tower6"
  | "ballista"
  | "baker"
  | "knight";

export interface TowerInfo {
  id: string;
  name: string;
  type: TowerType;
  typeLabel: string;
  x: number;
  side: "left" | "right";
  distanceToCastle: number;
}

export interface UpgradeSpecificWallOptions extends CampaignIslandOptions {
  wallId: string;
  targetLevel: number;
  addHorn?: boolean;
}

export interface UpgradeSpecificTowerOptions extends CampaignIslandOptions {
  towerId: string;
  targetType: TowerType;
}

export interface BatchUpgradeTowersOptions extends CampaignIslandOptions {
  targetType: TowerType;
  mode: "all" | "built_only" | "mounds_only";
}

export interface BatchUpgradeWallsOptions extends CampaignIslandOptions {
  targetLevel: number;
  addHorn?: boolean;
  mode: "all" | "built_only" | "mounds_only";
}

export interface SetTreeMarkOptions extends CampaignIslandOptions {
  treeId?: string;
  xMin?: number;
  xMax?: number;
  marked: boolean;
}

function toObject(value: unknown, message: string): MutableJSONObject {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new Error(message);
  }

  return value as MutableJSONObject;
}

function toArray(value: unknown, message: string): MutableJSONObject[] {
  if (!Array.isArray(value)) {
    throw new Error(message);
  }

  return value as MutableJSONObject[];
}

function cloneJson<T extends JSONValue>(value: T): T {
  return JSON.parse(JSON.stringify(value)) as T;
}

function updateWallet(componentList: MutableJSONObject[], coins: number) {
  let updated = false;

  for (const component of componentList) {
    if (component.name === "Wallet" && typeof component.data === "string") {
      try {
        const payload = JSON.parse(component.data) as MutableJSONObject;
        payload.usesCurrencySystem = true;
        payload.coins = coins;

        if (typeof payload.currency === "object" && payload.currency !== null) {
          (payload.currency as MutableJSONObject).Coins = coins;
        }

        component.data = JSON.stringify(payload);
        updated = true;
      } catch (error) {
        throw new Error("Unable to update the player wallet");
      }
    }
  }

  if (!updated) {
    throw new Error("No Wallet component found for the selected player");
  }
}

function extractWalletCoins(componentList: MutableJSONObject[]): number | null {
  for (const component of componentList) {
    if (component.name === "Wallet" && typeof component.data === "string") {
      try {
        const payload = JSON.parse(component.data) as MutableJSONObject;
        if (typeof payload.currency === "object" && payload.currency !== null) {
          const coins = (payload.currency as MutableJSONObject).Coins;
          return typeof coins === "number" ? coins : null;
        }
      } catch (error) {
        return null;
      }
    }
  }

  return null;
}

function randomId() {
  if (typeof crypto !== "undefined" && "randomUUID" in crypto) {
    return crypto.randomUUID();
  }

  return `uid-${Date.now()}-${Math.random().toString(16).slice(2)}`;
}

function findPlayerObject(objects: MutableJSONObject[], playerIndex: number) {
  const name = `Player ${playerIndex + 1}`;
  return objects.find((entry) => entry.name === name) ?? null;
}
/*
  Đây là comment
  trên nhiều dòng khác nhau
*/
function nextNumericValue(objects: MutableJSONObject[], key: string, fallback: number): number {
  let maxValue = fallback;

  for (const obj of objects) {
    const value = obj[key];
    if (typeof value === "number" && value > maxValue) {
      maxValue = value;
    }
  }

  return maxValue + 1;
}

export function setPlayerCoins(doc: JSONValue, options: CoinsOptions): JSONValue {
  const root = cloneJson(doc);
  const { campaignIndex, islandIndex, playerIndex, coins, mode, challengeIndex } = options;

  if (mode === "challenge") {
    const rootObj = toObject(root, "Invalid save data");
    const challenges = toArray(rootObj.challenges, "The save contains no challenges");
    const chIdx = challengeIndex ?? 0;
    const challenge = challenges[chIdx];
    if (!challenge) {
      throw new Error("Challenge not found");
    }
    const chObj = toObject(challenge, "Invalid challenge");
    const islands = chObj._islands ? toArray(chObj._islands, "No islands in challenge") : [];
    const island = islands[0];
    if (island) {
      const islandObj = toObject(island, "Invalid island data");
      if (Array.isArray(islandObj.objects)) {
        const objects = islandObj.objects as MutableJSONObject[];
        const player = findPlayerObject(objects, playerIndex);
        if (player) {
          const components = Array.isArray(player.componentData2)
            ? (player.componentData2 as MutableJSONObject[])
            : [];
          updateWallet(components, coins);
        }
      }
    }
    return root;
  }

  const campaigns = toArray(toObject(root, "Invalid save data").campaigns, "The save contains no campaigns");
  const campaign = campaigns[campaignIndex];
  if (!campaign) {
    throw new Error("Campaign not found");
  }

  const campaignObject = toObject(campaign, "Invalid campaign");

  if (campaignObject.carryForward) {
    const carry = toObject(campaignObject.carryForward, "Invalid carryForward data");
    const key = playerIndex === 0 ? "currencyP1" : "currencyP2";
    if (carry[key]) {
      const currency = toObject(carry[key], "Invalid player currency");
      currency.Coins = coins;
      carry.usesCurrencySystem = true;
    }

    const legacyKey = playerIndex === 0 ? "coins1" : "coins2";
    if (legacyKey in carry) {
      carry[legacyKey] = coins;
    }
  }

  const islands = campaignObject._islands ? toArray(campaignObject._islands, "No islands found for the campaign") : [];
  const island = islands[islandIndex];
  if (island) {
    const islandObj = toObject(island, "Invalid island data");
    if (Array.isArray(islandObj.monarchy?.characters)) {
      const characters = islandObj.monarchy.characters as MutableJSONObject[];
      const target = characters[playerIndex];
      if (target) {
        toObject(target, "Invalid character").currency = coins;
      }
    }

    if (Array.isArray(islandObj.objects)) {
      const objects = islandObj.objects as MutableJSONObject[];
      const player = findPlayerObject(objects, playerIndex);
      if (player) {
        const components = Array.isArray(player.componentData2)
          ? (player.componentData2 as MutableJSONObject[])
          : [];
        updateWallet(components, coins);
      }
    }
  }

  return root;
}

export function setPlayerGems(doc: JSONValue, options: GemsOptions): JSONValue {
  const root = cloneJson(doc);
  const { campaignIndex, islandIndex, playerIndex, gems, mode, challengeIndex } = options;

  if (mode === "challenge") {
    const rootObj = toObject(root, "Invalid save data");
    const challenges = toArray(rootObj.challenges, "The save contains no challenges");
    const chIdx = challengeIndex ?? 0;
    const challenge = challenges[chIdx];
    if (!challenge) {
      throw new Error("Challenge not found");
    }
    const chObj = toObject(challenge, "Invalid challenge");
    const islands = chObj._islands ? toArray(chObj._islands, "No islands in challenge") : [];
    const island = islands[0];
    if (island) {
      const islandObj = toObject(island, "Invalid island data");
      if (Array.isArray(islandObj.objects)) {
        const objects = islandObj.objects as MutableJSONObject[];
        const player = findPlayerObject(objects, playerIndex);
        if (player && Array.isArray(player.componentData2)) {
          for (const component of player.componentData2 as MutableJSONObject[]) {
            if (component.name === "Wallet" && typeof component.data === "string") {
              try {
                const payload = JSON.parse(component.data) as MutableJSONObject;
                payload.usesCurrencySystem = true;
                if (typeof payload.currency === "object" && payload.currency !== null) {
                  (payload.currency as MutableJSONObject).Gems = gems;
                }
                component.data = JSON.stringify(payload);
              } catch (error) {
                console.warn("Unable to update wallet gems", error);
              }
            }
          }
        }
      }
    }
    return root;
  }

  const campaigns = toArray(toObject(root, "Invalid save data").campaigns, "The save contains no campaigns");
  const campaign = campaigns[campaignIndex];
  if (!campaign) {
    throw new Error("Campaign not found");
  }

  const campaignObject = toObject(campaign, "Invalid campaign");

  if (campaignObject.carryForward) {
    const carry = toObject(campaignObject.carryForward, "Invalid carryForward data");
    const key = playerIndex === 0 ? "currencyP1" : "currencyP2";
    if (carry[key]) {
      const currency = toObject(carry[key], "Invalid player currency");
      currency.Gems = gems;
      carry.usesCurrencySystem = true;
    }

    const legacyKey = playerIndex === 0 ? "gems1" : "gems2";
    if (legacyKey in carry) {
      carry[legacyKey] = gems;
    }
  }

  const islands = campaignObject._islands ? toArray(campaignObject._islands, "No islands found for the campaign") : [];
  const island = islands[islandIndex];
  if (island) {
    const islandObj = toObject(island, "Invalid island data");
    if (Array.isArray(islandObj.objects)) {
      const objects = islandObj.objects as MutableJSONObject[];
      const player = findPlayerObject(objects, playerIndex);
      if (player && Array.isArray(player.componentData2)) {
        for (const component of player.componentData2 as MutableJSONObject[]) {
          if (component.name === "Wallet" && typeof component.data === "string") {
            try {
              const payload = JSON.parse(component.data) as MutableJSONObject;
              payload.usesCurrencySystem = true;
              if (typeof payload.currency === "object" && payload.currency !== null) {
                (payload.currency as MutableJSONObject).Gems = gems;
              }
              component.data = JSON.stringify(payload);
            } catch (error) {
              console.warn("Unable to update wallet gems", error);
            }
          }
        }
      }
    }
  }

  return root;
}

export function spawnArchers(doc: JSONValue, options: SpawnArchersOptions): JSONValue {
  return spawnUnitsByConfig(doc, options, {
    prefabs: [
      "Prefabs/Characters/Archer",
      "Prefabs/Characters/norselands/Archer_norselands"
    ],
    namePrefix: "Archer",
    positionError: "Invalid archer position",
    duplicateName: "Archer"
  });
}

interface SpawnConfig {
  prefabs: readonly string[];
  namePrefix: string;
  positionError: string;
  duplicateName: string;
}

const DEFAULT_CONFIGS = {
  worker: {
    prefabs: ["Prefabs/Characters/Worker", "Prefabs/Characters/norselands/Worker_norselands"] as const,
    namePrefix: "Worker",
    positionError: "Invalid worker position",
    duplicateName: "Worker",
  },
  farmer: {
    prefabs: ["Prefabs/Characters/Farmer", "Prefabs/Characters/norselands/Farmer_norselands"] as const,
    namePrefix: "Farmer",
    positionError: "Invalid farmer position",
    duplicateName: "Farmer",
  },
  pikeman: {
    prefabs: ["Prefabs/Characters/Pikeman", "Prefabs/Characters/norselands/Knight_norselands", "Prefabs/Characters/norselands/Pikeman_norselands"] as const,
    namePrefix: "Pikeman",
    positionError: "Invalid pikeman position",
    duplicateName: "Pikeman",
  },
} satisfies Record<string, SpawnConfig>;

const archerTemplate: MutableJSONObject = {
  name: "Archer P1 [B]",
  parentObject: { linkedObjectID: "" },
  hierarchyPath: "Level/GameLayer/",
  prefabPath: "Prefabs/Characters/Archer",
  uniqueID: "Archer P1 [B]--6969",
  mode: 0,
  createOrder: 12345,
  linkOrder: 0,
  decayHint: 0,
  decayResistanceDays: 2,
  decayedVersionPrefabPath: "Prefabs/Characters/Peasant",
  netID: 1666,
  crpcType: 1,
  localPosition: { x: 120.98619079589844, y: 0.8799997568130493, z: 0.7556657791137695 },
  localScale: { x: -1.0, y: 1.0, z: 1.0 },
  componentData2: [
    {
      name: "Wallet",
      type: "WalletData",
      data: "{\"coins\":0,\"gems\":0,\"usesCurrencySystem\":true,\"currency\":{\"Coins\":1,\"Gems\":0,\"Crown\":0,\"Skulls\":0,\"Shades\":0,\"Merchandise\":0}}",
    },
    {
      name: "Character",
      type: "CharacterData",
      data: "{\"isGrabbed\":false,\"inert\":false}",
    },
    {
      name: "Archer",
      type: "ArcherData",
      data: "{\"tower\":{\"linkedObjectID\":\"\"},\"knight\":{\"linkedObjectID\":\"\"},\"guardSide\":1,\"guardDepth\":0,\"desiredAttackMode\":0,\"despawnOnLoad\":false}",
    },
    {
      name: "Damageable",
      type: "DamageableData",
      data: "{\"hitPoints\":0,\"invulnerable\":false}",
    },
    {
      name: "GenderAnimatorSelector",
      type: "GenderSelectorSaveData",
      data: "{\"IsFemale\":false}",
    },
    {
      name: "Petrifiable",
      type: "PetrifiableSaveData",
      data: "{\"RemainingHP\":0,\"IsPetrified\":false,\"RemainingDuration\":-106.38296508789063}",
    },
    {
      name: "Embarkee",
      type: "EmbarkeeSaveData",
      data: "{\"IsEmbarked\":false,\"SlotId\":-1,\"Embarkable\":{\"linkedObjectID\":\"\"}}",
    },
  ],
};

const workerTemplate: MutableJSONObject = {
  name: "Worker P1 [A]",
  parentObject: { linkedObjectID: "" },
  hierarchyPath: "Level/GameLayer/",
  prefabPath: "Prefabs/Characters/Worker",
  uniqueID: "Worker P1 [A]--6969",
  mode: 0,
  createOrder: 22222,
  linkOrder: 0,
  decayHint: 4,
  decayResistanceDays: 1,
  decayedVersionPrefabPath: "Prefabs/Characters/Peasant",
  netID: 1098,
  crpcType: 1,
  localPosition: { x: 127.18162536621094, y: 0.875, z: 0.5052622556686401 },
  localScale: { x: -1.0, y: 1.0, z: 1.0 },
  componentData2: [
    {
      name: "Wallet",
      type: "WalletData",
      data: "{\"coins\":0,\"gems\":0,\"usesCurrencySystem\":true,\"currency\":{\"Coins\":1,\"Gems\":0,\"Crown\":0,\"Skulls\":0,\"Shades\":0,\"Merchandise\":0}}",
    },
    {
      name: "Character",
      type: "CharacterData",
      data: "{\"isGrabbed\":false,\"inert\":false}",
    },
    {
      name: "Worker",
      type: "WorkerData",
      data: "{\"despawnOnLoad\":false}",
    },
    {
      name: "Damageable",
      type: "DamageableData",
      data: "{\"hitPoints\":0,\"invulnerable\":false}",
    },
    {
      name: "GenderAnimatorSelector",
      type: "GenderSelectorSaveData",
      data: "{\"IsFemale\":false}",
    },
    {
      name: "Petrifiable",
      type: "PetrifiableSaveData",
      data: "{\"RemainingHP\":0,\"IsPetrified\":false,\"RemainingDuration\":-106.38296508789063}",
    },
    {
      name: "Embarkee",
      type: "EmbarkeeSaveData",
      data: "{\"IsEmbarked\":false,\"SlotId\":-1,\"Embarkable\":{\"linkedObjectID\":\"\"}}",
    },
  ],
};

const pikemanTemplate: MutableJSONObject = {
  name: "Pikeman P1 [8]",
  parentObject: { linkedObjectID: "" },
  hierarchyPath: "Level/GameLayer/",
  prefabPath: "Prefabs/Characters/Pikeman",
  uniqueID: "Pikeman P1 [8]--59924",
  mode: 0,
  createOrder: 20668,
  linkOrder: 0,
  decayHint: 32,
  decayResistanceDays: 2,
  decayedVersionPrefabPath: "Prefabs/Characters/Peasant",
  netID: 1101,
  crpcType: 1,
  localPosition: { x: -7.411938667297363, y: 0.8804292678833008, z: 0.9069020748138428 },
  localScale: { x: -1.0, y: 1.0, z: 1.0 },
  componentData2: [
    {
      name: "Wallet",
      type: "WalletData",
      data: "{\"coins\":0,\"gems\":0,\"usesCurrencySystem\":true,\"currency\":{\"Coins\":3,\"Gems\":0,\"Crown\":0,\"Skulls\":0,\"Shades\":0,\"Merchandise\":0}}",
    },
    {
      name: "Character",
      type: "CharacterData",
      data: "{\"isGrabbed\":false,\"inert\":false}",
    },
    {
      name: "Pikeman",
      type: "PikemanData",
      data: "{\"fishingSide\":-1,\"assignedSide\":-1,\"assignedWall\":{\"linkedObjectID\":\"\"},\"assignedReadiness\":0,\"remainingPikeUses\":30,\"summonedToBoat\":false}",
    },
    {
      name: "Damageable",
      type: "DamageableData",
      data: "{\"hitPoints\":1,\"invulnerable\":false}",
    },
    {
      name: "GenderAnimatorSelector",
      type: "GenderSelectorSaveData",
      data: "{\"IsFemale\":true}",
    },
    {
      name: "Petrifiable",
      type: "PetrifiableSaveData",
      data: "{\"RemainingHP\":0,\"IsPetrified\":false,\"RemainingDuration\":-26.941898345947267}",
    },
    {
      name: "Embarkee",
      type: "EmbarkeeSaveData",
      data: "{\"IsEmbarked\":false,\"SlotId\":-1,\"Embarkable\":{\"linkedObjectID\":\"\"}}",
    },
  ],
};

const farmerTemplate: MutableJSONObject = {
  name: "Farmer P1 [A]",
  parentObject: { linkedObjectID: "" },
  hierarchyPath: "Level/GameLayer/",
  prefabPath: "Prefabs/Characters/Farmer",
  uniqueID: "Farmer P1 [A]--6969",
  mode: 0,
  createOrder: 23333,
  linkOrder: 0,
  decayHint: 4,
  decayResistanceDays: 1,
  decayedVersionPrefabPath: "Prefabs/Characters/Peasant",
  netID: 1111,
  crpcType: 1,
  localPosition: { x: 0, y: 0.88, z: 0.81 },
  localScale: { x: -1.0, y: 1.0, z: 1.0 },
  componentData2: [
    {
      name: "Wallet",
      type: "WalletData",
      data: "{\"coins\":0,\"gems\":0,\"usesCurrencySystem\":true,\"currency\":{\"Coins\":1,\"Gems\":0,\"Crown\":0,\"Skulls\":0,\"Shades\":0,\"Merchandise\":0}}",
    },
    {
      name: "Character",
      type: "CharacterData",
      data: "{\"isGrabbed\":false,\"inert\":false}",
    },
    {
      name: "Farmer",
      type: "FarmerData",
      data: "{\"despawnOnLoad\":false}",
    },
    {
      name: "Damageable",
      type: "DamageableData",
      data: "{\"hitPoints\":0,\"invulnerable\":false}",
    },
    {
      name: "GenderAnimatorSelector",
      type: "GenderSelectorSaveData",
      data: "{\"IsFemale\":false}",
    },
    {
      name: "Petrifiable",
      type: "PetrifiableSaveData",
      data: "{\"RemainingHP\":0,\"IsPetrified\":false,\"RemainingDuration\":-100.0}",
    },
    {
      name: "Embarkee",
      type: "EmbarkeeSaveData",
      data: "{\"IsEmbarked\":false,\"SlotId\":-1,\"Embarkable\":{\"linkedObjectID\":\"\"}}",
    },
  ],
};

const knightTemplate: MutableJSONObject = {
  name: "Knight P1 [A]",
  parentObject: { linkedObjectID: "" },
  hierarchyPath: "Level/GameLayer/",
  prefabPath: "Prefabs/Characters/Knight",
  uniqueID: "Knight P1 [A]--6969",
  mode: 0,
  createOrder: 24444,
  linkOrder: 10,
  decayHint: 0,
  decayResistanceDays: -1,
  decayedVersionPrefabPath: "",
  netID: 1388,
  crpcType: 1,
  localPosition: { x: 0, y: 0.88, z: 0.755 },
  localScale: { x: 1.0, y: 1.0, z: 1.0 },
  componentData2: [
    {
      name: "Knight",
      type: "KnightData",
      data: "{\"side\":1,\"rank\":1,\"state\":2,\"shieldShop\":{\"linkedObjectID\":\"\"},\"bannerless\":false,\"additionalFollowers\":0,\"despawnOnLoad\":false}",
    },
    {
      name: "Character",
      type: "CharacterData",
      data: "{\"isGrabbed\":false,\"inert\":false}",
    },
    {
      name: "Wallet",
      type: "WalletData",
      data: "{\"coins\":0,\"gems\":0,\"usesCurrencySystem\":true,\"currency\":{\"Coins\":15,\"Gems\":0,\"Crown\":0,\"Skulls\":0,\"Shades\":0,\"Merchandise\":0}}",
    },
    {
      name: "Damageable",
      type: "DamageableData",
      data: "{\"hitPoints\":3,\"invulnerable\":false}",
    },
    {
      name: "GenderAnimatorSelector",
      type: "GenderSelectorSaveData",
      data: "{\"IsFemale\":false}",
    },
    {
      name: "Petrifiable",
      type: "PetrifiableSaveData",
      data: "{\"RemainingHP\":0,\"IsPetrified\":false,\"RemainingDuration\":-100.0}",
    },
    {
      name: "Embarkee",
      type: "EmbarkeeSaveData",
      data: "{\"IsEmbarked\":false,\"SlotId\":-1,\"Embarkable\":{\"linkedObjectID\":\"\"}}",
    },
  ],
};

const ENEMY_COMPONENTS = ["GreedArcher", "GreedKnight", "CrownStealer", "Troll", "Squid", "Boss"] as const;

const MAX_CASTLE_LEVEL = 7;
const MAX_WALL_LEVEL = 5;

function findTemplateByPrefab(objects: MutableJSONObject[], prefabs: readonly string[]): MutableJSONObject | null {
  return (
    objects.find((obj) => {
      const prefab = obj.prefabPath;
      if (typeof prefab !== "string") {
        return false;
      }

      return prefabs.some((candidate) => prefab.includes(candidate));
    }) ?? null
  );
}

function getFallbackTemplate(config: SpawnConfig): MutableJSONObject {
  if (config.duplicateName === "Worker") return workerTemplate;
  if (config.duplicateName === "Farmer") return farmerTemplate;
  if (config.duplicateName === "Pikeman") return pikemanTemplate;
  return archerTemplate;
}

function spawnUnitsByConfig(
  doc: JSONValue,
  options: SpawnArchersOptions,
  config: SpawnConfig
): JSONValue {
  const root = cloneJson(doc);
  const { campaignIndex, islandIndex, playerIndex, count } = options;

  if (count <= 0) {
    throw new Error("The count must be positive");
  }

  const { objects } = getIslandContext(root, {
    campaignIndex,
    islandIndex,
    targetMode: options.mode,
    challengeIndex: options.challengeIndex
  });

  const template = findTemplateByPrefab(objects, config.prefabs) ?? getFallbackTemplate(config);

  const playerObj = findPlayerObject(objects, playerIndex);
  const spawnSource = toObject(playerObj ?? template, "Invalid source object");
  const position = spawnSource.localPosition && typeof spawnSource.localPosition === "object"
    ? toObject(spawnSource.localPosition, config.positionError)
    : { x: 0, y: 0, z: 0 };

  let nextNetId = nextNumericValue(objects, "netID", 2000);
  let nextCreateOrder = nextNumericValue(objects, "createOrder", 20000);

  const baseWallet = Array.isArray(template.componentData2)
    ? extractWalletCoins(template.componentData2 as MutableJSONObject[])
    : null;

  for (let i = 0; i < count; i += 1) {
    const clone = cloneJson(template as JSONValue) as MutableJSONObject;
    clone.name = `${config.namePrefix} ${randomId()}`;
    clone.uniqueID = randomId();
    clone.netID = nextNetId++;
    clone.createOrder = nextCreateOrder++;

    if (clone.localPosition && typeof clone.localPosition === "object") {
      const loc = toObject(clone.localPosition, config.positionError);
      loc.x = typeof position.x === "number" ? position.x + i * 0.5 : i * 0.5;
      loc.y = typeof position.y === "number" ? position.y : 0.88;
      loc.z = typeof position.z === "number" ? position.z : 0.75;
    }

    const components = Array.isArray(clone.componentData2)
      ? (clone.componentData2 as MutableJSONObject[])
      : [];
    if (baseWallet !== null) {
      updateWallet(components, baseWallet);
    }
    objects.push(clone);
  }

  return root;
}

export function spawnWorkers(doc: JSONValue, options: SpawnArchersOptions): JSONValue {
  return spawnUnitsByConfig(doc, options, DEFAULT_CONFIGS.worker);
}

export function spawnFarmers(doc: JSONValue, options: SpawnArchersOptions): JSONValue {
  return spawnUnitsByConfig(doc, options, DEFAULT_CONFIGS.farmer);
}

export function spawnPikemen(doc: JSONValue, options: SpawnArchersOptions): JSONValue {
  return spawnUnitsByConfig(doc, options, DEFAULT_CONFIGS.pikeman);
}

export function spawnKnights(doc: JSONValue, options: SpawnKnightsOptions): JSONValue {
  const root = cloneJson(doc);
  const { campaignIndex, islandIndex, count, side, withArchers = true } = options;

  if (count <= 0) {
    throw new Error("The count must be positive");
  }

  const { objects } = getIslandContext(root, {
    campaignIndex,
    islandIndex,
    targetMode: options.mode,
    challengeIndex: options.challengeIndex
  });

  // Locate Castle position as the spawn location
  let spawnX = 0;
  const castle = findObjectByNameContains(objects, "Castle");
  if (castle) {
    const pos = getLocalPosition(castle);
    if (typeof pos.x === "number") {
      spawnX = pos.x;
    }
  }

  const existingKnight = findTemplateByPrefab(objects, [
    "Prefabs/Characters/Knight",
    "Prefabs/Characters/norselands/Knight_norselands"
  ]);
  const existingArcher = findTemplateByPrefab(objects, [
    "Prefabs/Characters/Archer",
    "Prefabs/Characters/norselands/Archer_norselands"
  ]);

  const knightBase = existingKnight ?? knightTemplate;
  const archerBase = existingArcher ?? archerTemplate;

  let nextNetId = nextNumericValue(objects, "netID", 2000);
  let nextCreateOrder = nextNumericValue(objects, "createOrder", 20000);

  for (let k = 0; k < count; k += 1) {
    const knight = cloneJson(knightBase as JSONValue) as MutableJSONObject;
    const knightUid = randomId();
    knight.uniqueID = knightUid;
    knight.name = `Knight P${k + 1} [Editor]`;
    knight.netID = nextNetId++;
    knight.createOrder = nextCreateOrder++;

    // Position knight slightly offset towards their side
    const kPos = getLocalPosition(knight);
    kPos.x = spawnX + (side > 0 ? (2.0 + k * 1.5) : (-2.0 - k * 1.5));
    kPos.y = 0.88;
    kPos.z = 0.755;

    // Set knight side and rank
    mutateComponentData(knight, "Knight", (payload) => {
      payload.side = side;
      payload.rank = 1;
      payload.state = 2; // Guarding/Patrolling
    });

    // Ensure 3 HP and full 50 coins protection
    setDamageableHP(knight, 3);
    const kComps = ensureComponentList(knight);
    updateWallet(kComps, 50);

    objects.push(knight);

    // Spawn 4 archers attached to this knight if requested
    if (withArchers) {
      for (let a = 0; a < 4; a += 1) {
        const archer = cloneJson(archerBase as JSONValue) as MutableJSONObject;
        archer.uniqueID = randomId();
        archer.name = `Archer Knight Squad [${k + 1}-${a + 1}]`;
        archer.netID = nextNetId++;
        archer.createOrder = nextCreateOrder++;

        const aPos = getLocalPosition(archer);
        aPos.x = kPos.x + (side > 0 ? -(0.5 + a * 0.4) : (0.5 + a * 0.4));
        aPos.y = 0.88;
        aPos.z = 0.755;

        // Link archer directly to knight
        mutateComponentData(archer, "Archer", (payload) => {
          if (!payload.knight || typeof payload.knight !== "object") {
            payload.knight = {};
          }
          payload.knight.linkedObjectID = knightUid;
          payload.guardSide = side;
        });

        objects.push(archer);
      }
    }
  }

  return root;
}

function objHasComponent(obj: MutableJSONObject, componentName: string): boolean {
  if (!Array.isArray(obj.componentData2)) {
    return false;
  }

  return (obj.componentData2 as MutableJSONObject[]).some((entry) => entry?.name === componentName);
}

function ensureComponentList(obj: MutableJSONObject): MutableJSONObject[] {
  if (!Array.isArray(obj.componentData2)) {
    obj.componentData2 = [];
  }

  return obj.componentData2 as MutableJSONObject[];
}

function mutateComponentData(
  obj: MutableJSONObject,
  componentName: string,
  mutation: (payload: MutableJSONObject) => void
): boolean {
  if (!Array.isArray(obj.componentData2)) {
    return false;
  }

  let updated = false;

  for (const entry of obj.componentData2 as MutableJSONObject[]) {
    if (entry?.name === componentName && typeof entry.data === "string") {
      try {
        const payload = JSON.parse(entry.data) as MutableJSONObject;
        mutation(payload);
        entry.data = JSON.stringify(payload);
        updated = true;
      } catch (error) {
        console.warn(`Unable to update ${componentName}`, error);
      }
    }
  }

  return updated;
}

function getLocalPosition(obj: MutableJSONObject): MutableJSONObject {
  if (!obj.localPosition || typeof obj.localPosition !== "object" || Array.isArray(obj.localPosition)) {
    obj.localPosition = { x: 0, y: 0, z: 0 };
  }

  return obj.localPosition as MutableJSONObject;
}

function findObjectByName(objects: MutableJSONObject[], name: string): MutableJSONObject | null {
  return objects.find((entry) => entry?.name === name) ?? null;
}

function findObjectByNameContains(objects: MutableJSONObject[], needle: string): MutableJSONObject | null {
  return objects.find((entry) => typeof entry?.name === "string" && entry.name.includes(needle)) ?? null;
}

function markTree(obj: MutableJSONObject) {
  mutateComponentData(obj, "WorkableTree", (payload) => {
    payload.marked = true;
  });
}

function markTrees(objects: MutableJSONObject[]) {
  for (const obj of objects) {
    if (objHasComponent(obj, "WorkableTree")) {
      markTree(obj);
    }
  }
}

function setWalletCoinsOnObject(obj: MutableJSONObject, coins: number) {
  const components = ensureComponentList(obj);
  updateWallet(components, coins);
}

function setDamageableHP(obj: MutableJSONObject, hp: number) {
  mutateComponentData(obj, "Damageable", (payload) => {
    payload.hitPoints = hp;
  });
}

function setPlayerFormationActive(obj: MutableJSONObject, active: boolean) {
  mutateComponentData(obj, "Player", (payload) => {
    payload.isFormationActive = active;
  });
}

function createIdGenerators(objects: MutableJSONObject[]) {
  let nextNet = nextNumericValue(objects, "netID", 2999);
  let nextCreate = nextNumericValue(objects, "createOrder", 29999);
  return {
    nextNetId: () => nextNet++,
    nextCreateOrder: () => nextCreate++,
  };
}

type IdGenerators = ReturnType<typeof createIdGenerators>;

function createCharacterFromTemplate(
  template: MutableJSONObject,
  name: string,
  x: number,
  generators: IdGenerators
): MutableJSONObject {
  const clone = cloneJson(template as JSONValue) as MutableJSONObject;
  clone.name = name;
  clone.uniqueID = randomId();
  clone.netID = generators.nextNetId();
  clone.createOrder = generators.nextCreateOrder();
  const position = getLocalPosition(clone);
  position.x = x;
  if (typeof position.y !== "number") {
    position.y = 0;
  }
  if (typeof position.z !== "number") {
    position.z = 0;
  }
  return clone;
}

function makeArcher(name: string, x: number, generators: IdGenerators): MutableJSONObject {
  return createCharacterFromTemplate(archerTemplate, name, x, generators);
}

function makeWorker(name: string, x: number, generators: IdGenerators): MutableJSONObject {
  return createCharacterFromTemplate(workerTemplate, name, x, generators);
}

function makePikeman(name: string, x: number, generators: IdGenerators): MutableJSONObject {
  return createCharacterFromTemplate(pikemanTemplate, name, x, generators);
}

function initShop(
  kind: "bow" | "hammer" | "scythe",
  objects: MutableJSONObject[],
  generators: IdGenerators,
  theme: ThemeInfo = { prefix: "", suffix: "" }
) {
  const identifier =
    kind === "bow"
      ? "ShopBow"
      : kind === "hammer"
        ? "ShopHammer"
        : kind === "scythe"
          ? "ShopScythe"
          : null;

  if (!identifier) {
    return;
  }

  const placeholder = findObjectByNameContains(objects, `${identifier} Placeholder`);
  if (!placeholder) {
    return;
  }

  const netId = generators.nextNetId();
  placeholder.name = `${identifier}${theme.suffix}(Clone)`;
  placeholder.uniqueID = randomId();
  placeholder.prefabPath = `Prefabs/Buildings and Interactive/${theme.prefix}${identifier}${theme.suffix}`;
  placeholder.netID = netId;
}

interface ThemeInfo {
  prefix: string;
  suffix: string;
}

function detectTheme(objects: MutableJSONObject[], campaignObject?: MutableJSONObject): ThemeInfo {
  // Primary source of truth: biomeIndex in campaign object
  // Biomes: 0 = Default / Medieval, 1 = Shogun, 2 = Deadlands, 3 = Norselands, 4 = Greece / Call of Olympus
  if (campaignObject && typeof campaignObject.biomeIndex === "number") {
    switch (campaignObject.biomeIndex) {
      case 1:
        return { prefix: "shogun/", suffix: "_shogun" };
      case 2:
        return { prefix: "deadlands/", suffix: "_deadlands" };
      case 3:
        return { prefix: "norselands/", suffix: "_norselands" };
      case 4:
        return { prefix: "greece/", suffix: "_greece" };
      default:
        return { prefix: "", suffix: "" };
    }
  }

  // Fallback heuristic: inspect existing objects
  for (const obj of objects) {
    const prefab = String(obj?.prefabPath ?? "");
    const name = String(obj?.name ?? "");
    if (prefab.includes("/greece/") || name.includes("_greece")) {
      return { prefix: "greece/", suffix: "_greece" };
    }
    if (prefab.includes("/norselands/") || name.includes("_norselands")) {
      return { prefix: "norselands/", suffix: "_norselands" };
    }
    if (prefab.includes("/shogun/") || name.includes("_shogun")) {
      return { prefix: "shogun/", suffix: "_shogun" };
    }
    if (prefab.includes("/deadlands/") || name.includes("_deadlands")) {
      return { prefix: "deadlands/", suffix: "_deadlands" };
    }
  }
  return { prefix: "", suffix: "" };
}

function applyWallUpgrade(wallObj: MutableJSONObject, targetLevel: number, addHorn: boolean, theme: ThemeInfo) {
  const level = Math.min(Math.max(0, targetLevel), 5);
  const withHorn = level === 5 && addHorn;

  if (level === 0) {
    wallObj.name = `Wall0${theme.suffix}(Clone)`;
    wallObj.prefabPath = `Prefabs/Buildings and Interactive/${theme.prefix}Wall0${theme.suffix}`;
    wallObj.decayedVersionPrefabPath = "";
    const components = ensureComponentList(wallObj);
    wallObj.componentData2 = components.filter(c => c.name === "PayableUpgrade" || c.name === "CRPCStamp");
    return;
  }

  const nameSuffix = withHorn ? `5_horn${theme.suffix}` : `${level}${theme.suffix}`;
  const prefabName = withHorn ? `Wall5_horn${theme.suffix}` : `Wall${level}${theme.suffix}`;
  const wreckName = `Wall${level} Wreck${theme.suffix}`;

  wallObj.name = `Wall${nameSuffix}(Clone)`;
  wallObj.prefabPath = `Prefabs/Buildings and Interactive/${theme.prefix}${prefabName}`;
  wallObj.decayedVersionPrefabPath = `Prefabs/Buildings and Interactive/${theme.prefix}${wreckName}`;

  const components = ensureComponentList(wallObj);

  function setOrUpdateComponent(cName: string, cType: string, cData: string) {
    const existing = components.find(c => c.name === cName);
    if (existing) {
      existing.type = cType;
      existing.data = cData;
    } else {
      components.push({ name: cName, type: cType, data: cData });
    }
  }

  setOrUpdateComponent("WorkableBuilding", "WorkableBuildingData", JSON.stringify({ currentBuildPoints: 0.0, usesConstructionBuildingComponent: true }));
  setOrUpdateComponent("PayableUpgrade", "PayableUpgradeData", JSON.stringify({ cooldown: 0.0 }));
  const dmgIdx = components.findIndex(c => c.name === "Damageable");
  if (dmgIdx !== -1) {
    components.splice(dmgIdx, 1);
  }
  setOrUpdateComponent("ConstructionBuildingComponent", "ConstructionBuildingComponentData", JSON.stringify({ CurrentBuildPoints: 90.0 }));

  if (level === 5 && !withHorn) {
    setOrUpdateComponent("GhostHintTrigger", "GhostHintTriggerData", JSON.stringify({ IsGhostHintEnabled: true, hintDataPath: "HermitHorn" }));
  } else {
    const hintIdx = components.findIndex(c => c.name === "GhostHintTrigger");
    if (hintIdx !== -1) {
      components.splice(hintIdx, 1);
    }
  }
}

function parseWallInfo(obj: MutableJSONObject, castleX: number): WallInfo | null {
  const name = String(obj?.name ?? "");
  const prefab = String(obj?.prefabPath ?? "");

  const isWallPrefab = prefab.includes("Buildings and Interactive/") && prefab.includes("Wall");
  const isWallName = /^Wall\d/.test(name);
  if (!isWallPrefab && !isWallName) {
    return null;
  }

  let level = 0;
  let hasHorn = false;

  if (name.includes("Wall5_horn") || prefab.includes("Wall5_horn")) {
    level = 5;
    hasHorn = true;
  } else if (name.includes("Wall5") || prefab.includes("Wall5")) {
    level = 5;
  } else if (name.includes("Wall4") || prefab.includes("Wall4")) {
    level = 4;
  } else if (name.includes("Wall3") || prefab.includes("Wall3")) {
    level = 3;
  } else if (name.includes("Wall2") || prefab.includes("Wall2")) {
    level = 2;
  } else if (name.includes("Wall1") || prefab.includes("Wall1")) {
    level = 1;
  } else if (name.includes("Wall0") || prefab.includes("Wall0")) {
    level = 0;
  } else {
    return null;
  }

  const pos = getLocalPosition(obj);
  const x = Number(pos.x ?? 0);
  const side = x < castleX ? "left" : "right";
  const distanceToCastle = Math.round(Math.abs(x - castleX) * 100) / 100;

  return {
    id: String(obj.uniqueID ?? ""),
    name,
    level,
    hasHorn,
    x: Math.round(x * 100) / 100,
    side,
    distanceToCastle,
  };
}

function parseTowerInfo(obj: MutableJSONObject, castleX: number): TowerInfo | null {
  const name = String(obj?.name ?? "");
  const prefab = String(obj?.prefabPath ?? "");

  const isTowerPrefab = prefab.includes("Buildings and Interactive/Tower");
  const isTowerName = /Tower\d|Tower Ballista|Tower Baker|Tower Knight/i.test(name);
  if (!isTowerPrefab && !isTowerName) {
    return null;
  }

  let type: TowerType = "tower0";
  let typeLabel = "Dirt Mound (L0)";

  if (prefab.includes("Tower Ballista") || name.includes("Tower Ballista")) {
    type = "ballista";
    typeLabel = "Ballista (Hermit)";
  } else if (prefab.includes("Tower Baker") || name.includes("Tower Baker")) {
    type = "baker";
    typeLabel = "Bakery (Hermit)";
  } else if (prefab.includes("Tower Knight") || name.includes("Tower Knight")) {
    type = "knight";
    typeLabel = "Knight Tower (Hermit)";
  } else if (prefab.includes("Tower6") || name.includes("Tower6")) {
    type = "tower6";
    typeLabel = "Level 6 (4 Archers)";
  } else if (prefab.includes("Tower5") || name.includes("Tower5")) {
    type = "tower5";
    typeLabel = "Level 5 (Roof)";
  } else if (prefab.includes("Tower4") || name.includes("Tower4")) {
    type = "tower4";
    typeLabel = "Level 4 (3 Archers)";
  } else if (prefab.includes("Tower3") || name.includes("Tower3")) {
    type = "tower3";
    typeLabel = "Level 3 (2 Archers)";
  } else if (prefab.includes("Tower2") || name.includes("Tower2")) {
    type = "tower2";
    typeLabel = "Level 2 (1 Archer)";
  } else if (prefab.includes("Tower1") || name.includes("Tower1")) {
    type = "tower1";
    typeLabel = "Level 1 (1 Archer)";
  } else if (prefab.includes("Tower0") || name.includes("Tower0")) {
    type = "tower0";
    typeLabel = "Dirt Mound (L0)";
  } else {
    return null;
  }

  const pos = getLocalPosition(obj);
  const x = Number(pos.x ?? 0);
  const side = x < castleX ? "left" : "right";
  const distanceToCastle = Math.round(Math.abs(x - castleX) * 100) / 100;

  return {
    id: String(obj.uniqueID ?? ""),
    name,
    type,
    typeLabel,
    x: Math.round(x * 100) / 100,
    side,
    distanceToCastle,
  };
}

function applyTowerUpgrade(towerObj: MutableJSONObject, targetType: TowerType, theme: ThemeInfo) {
  const components = ensureComponentList(towerObj);

  function setOrUpdateComponent(cName: string, cType: string, cData: string) {
    const existing = components.find(c => c.name === cName);
    if (existing) {
      existing.type = cType;
      existing.data = cData;
    } else {
      components.push({ name: cName, type: cType, data: cData });
    }
  }

  function removeComponent(cName: string) {
    const idx = components.findIndex(c => c.name === cName);
    if (idx !== -1) {
      components.splice(idx, 1);
    }
  }

  // Remove specialized components by default
  removeComponent("PayableShop");
  removeComponent("TowerKnight");

  if (targetType === "tower0") {
    towerObj.name = `Tower0${theme.suffix}(Clone)`;
    towerObj.prefabPath = `Prefabs/Buildings and Interactive/${theme.prefix}Tower0${theme.suffix}`;
    towerObj.decayedVersionPrefabPath = "";
    towerObj.componentData2 = components.filter(c => c.name === "PayableUpgrade" || c.name === "CRPCStamp");
    return;
  }

  if (targetType === "tower6") {
    towerObj.name = `Tower6${theme.suffix}(Clone)`;
    towerObj.prefabPath = `Prefabs/Buildings and Interactive/${theme.prefix}Tower6${theme.suffix}`;
    towerObj.decayedVersionPrefabPath = "";
    setOrUpdateComponent("WorkableBuilding", "WorkableBuildingData", JSON.stringify({ currentBuildPoints: 0.0, usesConstructionBuildingComponent: true }));
    setOrUpdateComponent("PayableUpgrade", "PayableUpgradeData", JSON.stringify({ cooldown: 0.0 }));
    setOrUpdateComponent("ConstructionBuildingComponent", "ConstructionBuildingComponentSaveData", JSON.stringify({ CurrentBuildPoints: 180.0 }));
  } else if (targetType === "tower5") {
    towerObj.name = `Tower5${theme.suffix}(Clone)`;
    towerObj.prefabPath = `Prefabs/Buildings and Interactive/${theme.prefix}Tower5${theme.suffix}`;
    towerObj.decayedVersionPrefabPath = "";
    setOrUpdateComponent("WorkableBuilding", "WorkableBuildingData", JSON.stringify({ currentBuildPoints: 0.0, usesConstructionBuildingComponent: true }));
    setOrUpdateComponent("PayableUpgrade", "PayableUpgradeData", JSON.stringify({ cooldown: 0.0 }));
    setOrUpdateComponent("ConstructionBuildingComponent", "ConstructionBuildingComponentSaveData", JSON.stringify({ CurrentBuildPoints: 150.0 }));
  } else if (targetType === "ballista") {
    towerObj.name = `Tower Ballista(Clone)`;
    towerObj.prefabPath = `Prefabs/Buildings and Interactive/Tower Ballista`;
    towerObj.decayedVersionPrefabPath = "";
    removeComponent("PayableUpgrade");
    setOrUpdateComponent("WorkableBuilding", "WorkableBuildingData", JSON.stringify({ currentBuildPoints: 0.0, usesConstructionBuildingComponent: true }));
    setOrUpdateComponent("ConstructionBuildingComponent", "ConstructionBuildingComponentSaveData", JSON.stringify({ CurrentBuildPoints: 120.0 }));
  } else if (targetType === "baker") {
    towerObj.name = `Tower Baker(Clone)`;
    towerObj.prefabPath = `Prefabs/Buildings and Interactive/Tower Baker`;
    towerObj.decayedVersionPrefabPath = "";
    removeComponent("PayableUpgrade");
    setOrUpdateComponent("PayableShop", "PayableShopData", JSON.stringify({ items: [] }));
    setOrUpdateComponent("WorkableBuilding", "WorkableBuildingData", JSON.stringify({ currentBuildPoints: 0.0, usesConstructionBuildingComponent: true }));
    setOrUpdateComponent("ConstructionBuildingComponent", "ConstructionBuildingComponentSaveData", JSON.stringify({ CurrentBuildPoints: 40.0 }));
  } else if (targetType === "knight") {
    towerObj.name = `Tower Knight(Clone)`;
    towerObj.prefabPath = `Prefabs/Buildings and Interactive/Tower Knight`;
    towerObj.decayedVersionPrefabPath = "";
    removeComponent("PayableUpgrade");
    setOrUpdateComponent("WorkableBuilding", "WorkableBuildingData", JSON.stringify({ currentBuildPoints: 0.0, usesConstructionBuildingComponent: true }));
    setOrUpdateComponent("TowerKnight", "TowerKnightData", "{}");
    setOrUpdateComponent("ConstructionBuildingComponent", "ConstructionBuildingComponentSaveData", JSON.stringify({ CurrentBuildPoints: 20.0 }));
  } else {
    // tower1, tower2, tower3, tower4
    const lvlStr = targetType.replace("tower", "");
    towerObj.name = `Tower${lvlStr}${theme.suffix}(Clone)`;
    towerObj.prefabPath = `Prefabs/Buildings and Interactive/${theme.prefix}Tower${lvlStr}${theme.suffix}`;
    towerObj.decayedVersionPrefabPath = "";
    setOrUpdateComponent("WorkableBuilding", "WorkableBuildingData", JSON.stringify({ currentBuildPoints: 0.0, usesConstructionBuildingComponent: true }));
    setOrUpdateComponent("PayableUpgrade", "PayableUpgradeData", JSON.stringify({ cooldown: 0.0 }));
    setOrUpdateComponent("ConstructionBuildingComponent", "ConstructionBuildingComponentSaveData", JSON.stringify({ CurrentBuildPoints: 30.0 * Number(lvlStr) }));
  }
}

function upgradeCastle(obj: MutableJSONObject, level: number, theme: ThemeInfo) {
  const targetLevel = Math.min(level, MAX_CASTLE_LEVEL);
  obj.name = `Castle${targetLevel}${theme.suffix}(Clone)`;
  obj.uniqueID = randomId();
  obj.prefabPath = `Prefabs/Buildings and Interactive/${theme.prefix}Castle${targetLevel}${theme.suffix}`;
  const components = ensureComponentList(obj);
  if (!objHasComponent(obj, "Castle")) {
    components.push({ name: "Castle", type: "CastleData", data: "{}" });
  }
  if (!objHasComponent(obj, "GhostHintTrigger")) {
    components.push({ name: "GhostHintTrigger", type: "GhostHintTriggerData", data: "{\"IsGhostHintEnabled\":true,\"hintDataPath\":\"UpgradeTheCastle1\"}" });
  }
}

function upgradeWalls(objects: MutableJSONObject[], level: number, generators: IdGenerators, theme: ThemeInfo = { prefix: "", suffix: "" }) {
  const targetLevel = Math.min(level, MAX_WALL_LEVEL);

  for (const obj of objects.slice()) {
    if (typeof obj.name === "string" && obj.name.includes("Wall0")) {
      applyWallUpgrade(obj, targetLevel, false, theme);
      const position = getLocalPosition(obj);
      const baseX = typeof position.x === "number" ? position.x : 0;
      for (let i = 0; i < 2; i += 1) {
        const worker = makeWorker("pimp my walls worker", baseX, generators);
        objects.push(worker);
      }
    }
  }
}

function getIslandContext(root: JSONValue, options: CampaignIslandOptions) {
  const rootObj = toObject(root, "Invalid save data");
  if (options.targetMode === "challenge") {
    const challenges = toArray(rootObj.challenges, "The save contains no challenges");
    const chIdx = options.challengeIndex ?? 0;
    const challenge = challenges[chIdx];
    if (!challenge) {
      throw new Error("Challenge not found");
    }
    const challengeObject = toObject(challenge, "Invalid challenge");
    const islands = challengeObject._islands
      ? toArray(challengeObject._islands, "No islands found for the challenge")
      : [];
    const island = islands[0];
    if (!island) {
      throw new Error("Island not found");
    }
    const islandObject = toObject(island, "Invalid island data");
    const objects = islandObject.objects
      ? toArray(islandObject.objects, "No objects found on the island")
      : [];
    return { campaigns: [], campaignObject: challengeObject, islandObject, objects } as const;
  }

  const campaigns = toArray(rootObj.campaigns, "The save contains no campaigns");
  const campaign = campaigns[options.campaignIndex];
  if (!campaign) {
    throw new Error("Campaign not found");
  }

  const campaignObject = toObject(campaign, "Invalid campaign");
  const islands = campaignObject._islands
    ? toArray(campaignObject._islands, "No islands found for the campaign")
    : [];
  const island = islands[options.islandIndex];
  if (!island) {
    throw new Error("Island not found");
  }

  const islandObject = toObject(island, "Invalid island data");
  const objects = islandObject.objects
    ? toArray(islandObject.objects, "No objects found on the island")
    : [];

  return { campaigns, campaignObject, islandObject, objects } as const;
}

function updateLandPortalCount(campaignObject: MutableJSONObject, islandIndex: number, portals: number) {
  try {
    if (!campaignObject.currentReign || typeof campaignObject.currentReign !== "object" || Array.isArray(campaignObject.currentReign)) {
      return;
    }

    const reign = toObject(campaignObject.currentReign, "Invalid reign");
    if (!Array.isArray(reign.landData)) {
      return;
    }

    const land = reign.landData[islandIndex];
    if (land && typeof land === "object" && !Array.isArray(land)) {
      (land as MutableJSONObject).portals = portals;
    }
  } catch (error) {
    console.warn("Unable to update the portal count", error);
  }
}

function ensureCurrencyBag(target: MutableJSONObject, key: string): MutableJSONObject {
  if (!target[key] || typeof target[key] !== "object" || Array.isArray(target[key])) {
    target[key] = {};
  }

  return target[key] as MutableJSONObject;
}

export function getPlayerCoins(doc: JSONValue | null, options: CoinsQueryOptions): number | null {
  if (!doc) {
    return null;
  }

  try {
    const { campaignIndex, islandIndex, playerIndex, mode, challengeIndex } = options;
    const root = toObject(doc, "Invalid save data");

    if (mode === "challenge") {
      const challenges = Array.isArray(root.challenges) ? (root.challenges as MutableJSONObject[]) : null;
      if (!challenges) return null;
      const chIdx = challengeIndex ?? 0;
      const challenge = challenges[chIdx];
      if (!challenge) return null;
      const chObj = toObject(challenge, "Invalid challenge");
      const islands = chObj._islands ? toArray(chObj._islands, "Invalid islands data") : [];
      const island = islands[0];
      if (island) {
        const islandObj = toObject(island, "Invalid island data");
        if (Array.isArray(islandObj.objects)) {
          const objects = islandObj.objects as MutableJSONObject[];
          const player = findPlayerObject(objects, playerIndex);
          if (player) {
            const components = Array.isArray(player.componentData2)
              ? (player.componentData2 as MutableJSONObject[])
              : [];
            const coins = extractWalletCoins(components);
            if (typeof coins === "number") {
              return coins;
            }
          }
        }
      }
      return null;
    }

    const campaigns = Array.isArray(root.campaigns) ? (root.campaigns as MutableJSONObject[]) : null;
    if (!campaigns) {
      return null;
    }

    const campaign = campaigns[campaignIndex];
    if (!campaign) {
      return null;
    }

    const campaignObject = toObject(campaign, "Invalid campaign");

    if (campaignObject._islands) {
      const islands = toArray(campaignObject._islands, "Invalid islands data");
      const island = islands[islandIndex];
      if (island) {
        const islandObj = toObject(island, "Invalid island data");

        if (islandObj.monarchy && Array.isArray(islandObj.monarchy.characters)) {
          const characters = islandObj.monarchy.characters as MutableJSONObject[];
          const target = characters[playerIndex];
          if (target) {
            const charObj = toObject(target, "Invalid character");
            if (typeof charObj.currency === "number") {
              return charObj.currency;
            }
          }
        }

        if (Array.isArray(islandObj.objects)) {
          const objects = islandObj.objects as MutableJSONObject[];
          const player = findPlayerObject(objects, playerIndex);
          if (player) {
            const components = Array.isArray(player.componentData2)
              ? (player.componentData2 as MutableJSONObject[])
              : [];
            const coins = extractWalletCoins(components);
            if (typeof coins === "number") {
              return coins;
            }
          }
        }
      }
    }

    if (campaignObject.carryForward) {
      const carry = toObject(campaignObject.carryForward, "Invalid carryForward data");
      const key = playerIndex === 0 ? "currencyP1" : "currencyP2";
      if (carry[key]) {
        const currency = toObject(carry[key], "Invalid player currency");
        const coins = currency.Coins;
        if (typeof coins === "number") {
          return coins;
        }
      }
    }

    return null;
  } catch (error) {
    console.warn("Unable to read the coin count", error);
    return null;
  }
}

export function getPlayerGems(doc: JSONValue | null, options: GemsQueryOptions): number | null {
  if (!doc) {
    return null;
  }

  try {
    const { campaignIndex, islandIndex, playerIndex, mode, challengeIndex } = options;
    const root = toObject(doc, "Invalid save data");

    if (mode === "challenge") {
      const challenges = Array.isArray(root.challenges) ? (root.challenges as MutableJSONObject[]) : null;
      if (!challenges) return null;
      const chIdx = challengeIndex ?? 0;
      const challenge = challenges[chIdx];
      if (!challenge) return null;
      const chObj = toObject(challenge, "Invalid challenge");
      const islands = chObj._islands ? toArray(chObj._islands, "Invalid islands data") : [];
      const island = islands[0];
      if (island) {
        const islandObj = toObject(island, "Invalid island data");
        if (Array.isArray(islandObj.objects)) {
          const objects = islandObj.objects as MutableJSONObject[];
          const player = findPlayerObject(objects, playerIndex);
          if (player && Array.isArray(player.componentData2)) {
            for (const component of player.componentData2 as MutableJSONObject[]) {
              if (component.name === "Wallet" && typeof component.data === "string") {
                try {
                  const payload = JSON.parse(component.data) as MutableJSONObject;
                  if (typeof payload.currency === "object" && payload.currency !== null) {
                    const gems = (payload.currency as MutableJSONObject).Gems;
                    if (typeof gems === "number") {
                      return gems;
                    }
                  }
                } catch (error) {
                  console.warn("Unable to read wallet gems", error);
                }
              }
            }
          }
        }
      }
      return null;
    }

    const campaigns = Array.isArray(root.campaigns) ? (root.campaigns as MutableJSONObject[]) : null;
    if (!campaigns) {
      return null;
    }

    const campaign = campaigns[campaignIndex];
    if (!campaign) {
      return null;
    }

    const campaignObject = toObject(campaign, "Invalid campaign");

    if (campaignObject._islands) {
      const islands = toArray(campaignObject._islands, "Invalid islands data");
      const island = islands[islandIndex];
      if (island) {
        const islandObj = toObject(island, "Invalid island data");

        if (Array.isArray(islandObj.objects)) {
          const objects = islandObj.objects as MutableJSONObject[];
          const player = findPlayerObject(objects, playerIndex);
          if (player && Array.isArray(player.componentData2)) {
            for (const component of player.componentData2 as MutableJSONObject[]) {
              if (component.name === "Wallet" && typeof component.data === "string") {
                try {
                  const payload = JSON.parse(component.data) as MutableJSONObject;
                  if (typeof payload.currency === "object" && payload.currency !== null) {
                    const gems = (payload.currency as MutableJSONObject).Gems;
                    if (typeof gems === "number") {
                      return gems;
                    }
                  }
                } catch (error) {
                  console.warn("Unable to read wallet gems", error);
                }
              }
            }
          }
        }
      }
    }

    if (campaignObject.carryForward) {
      const carry = toObject(campaignObject.carryForward, "Invalid carryForward data");
      const key = playerIndex === 0 ? "currencyP1" : "currencyP2";
      if (carry[key]) {
        const currency = toObject(carry[key], "Invalid player currency");
        const gems = currency.Gems;
        if (typeof gems === "number") {
          return gems;
        }
      }
    }

    return null;
  } catch (error) {
    console.warn("Unable to read the gem count", error);
    return null;
  }
}

export function gotoIsland(doc: JSONValue, options: GotoOptions): JSONValue {
  const root = cloneJson(doc);
  const { campaignObject } = getIslandContext(root, options);

  const previousLand = campaignObject.currentLand;
  campaignObject.PreviousLand = previousLand ?? options.islandIndex;
  campaignObject.currentLand = options.islandIndex;

  if (campaignObject.currentReign && typeof campaignObject.currentReign === "object" && !Array.isArray(campaignObject.currentReign)) {
    const reign = toObject(campaignObject.currentReign, "Invalid reign");
    reign.currentLand = options.islandIndex;
  }

  if (campaignObject.carryForward) {
    try {
      const carry = toObject(campaignObject.carryForward, "Invalid carryForward data");
      const coins = options.coins ?? 69;
      const gems = options.gems ?? 3;
      const pikemen = options.pikemen ?? 4;
      const farmers = options.farmers ?? 4;
      const boats = options.boats ?? 4;

      carry.present = true;
      carry.usesCurrencySystem = true;

      const currency = ensureCurrencyBag(carry, "currencyP1");
      currency.Coins = coins;
      currency.Gems = gems;
      currency.Crown = 1;

      carry.numPikeman = pikemen;
      carry.numFarmers = farmers;
      carry.numFleetBoats = boats;

      const legacyCoins = "coins1";
      const legacyGems = "gems1";
      carry[legacyCoins] = coins;
      carry[legacyGems] = gems;
    } catch (error) {
      console.warn("Unable to update carryForward", error);
    }
  }

  return root;
}

export function takeOverIsland(doc: JSONValue, options: TakeOverOptions): JSONValue {
  const root = cloneJson(doc);
  const { campaignObject, islandObject, objects } = getIslandContext(root, options);
  const generators = createIdGenerators(objects);

  let portalX = 0;
  const filtered: MutableJSONObject[] = [];

  for (const obj of objects) {
    if (objHasComponent(obj, "Portal")) {
      if (objHasComponent(obj, "QuestTransitionTrigger")) {
        setDamageableHP(obj, 1);
        const position = getLocalPosition(obj);
        portalX = typeof position.x === "number" ? position.x : 0;
        filtered.push(obj);
      }
    } else {
      filtered.push(obj);
    }
  }

  const targetX = portalX > 0 ? portalX - (options.formationOffset ?? 3) : portalX + (options.formationOffset ?? 3);
  const coins = options.coins ?? 69;

  const player = findObjectByName(filtered, "Player 1");
  if (player) {
    const position = getLocalPosition(player);
    position.x = targetX;
    setPlayerFormationActive(player, true);
    setWalletCoinsOnObject(player, coins);
  }

  const idol = findObjectByNameContains(filtered, "Idol");
  if (idol) {
    const position = getLocalPosition(idol);
    position.x = portalX;
  }

  const archerCount = Math.max(0, options.archerCount ?? 1);
  const workerCount = Math.max(0, options.workerCount ?? 1);

  for (let i = 0; i < archerCount; i += 1) {
    const name = archerCount === 1 ? "Steve" : `TakeOver Archer ${i + 1}`;
    filtered.push(makeArcher(name, targetX, generators));
  }

  for (let i = 0; i < workerCount; i += 1) {
    const name = workerCount === 1 ? "Joey" : `TakeOver Worker ${i + 1}`;
    filtered.push(makeWorker(name, targetX, generators));
  }

  islandObject.objects = filtered;
  updateLandPortalCount(campaignObject, options.islandIndex, 1);

  return root;
}

export function destroyPortals(doc: JSONValue, options: DestroyPortalsOptions): JSONValue {
  const root = cloneJson(doc);
  const { campaignObject, islandObject, objects } = getIslandContext(root, options);

  const remaining = objects.filter((obj) => !objHasComponent(obj, "Portal"));
  islandObject.objects = remaining;
  updateLandPortalCount(campaignObject, options.islandIndex, 0);

  return root;
}

export function exterminateEnemies(doc: JSONValue, options: ExterminateOptions): JSONValue {
  const root = cloneJson(doc);
  const { islandObject, objects } = getIslandContext(root, options);

  const survivors = objects.filter((obj) => !ENEMY_COMPONENTS.some((component) => objHasComponent(obj, component)));
  islandObject.objects = survivors;

  return root;
}

export function startFormationAssault(doc: JSONValue, options: FormationOptions): JSONValue {
  const root = cloneJson(doc);
  const { islandObject, objects } = getIslandContext(root, options);
  const generators = createIdGenerators(objects);

  const playerName = `Player ${options.playerIndex + 1}`;
  const player = findObjectByName(objects, playerName);
  if (!player) {
    throw new Error(`Unable to find ${playerName}`);
  }

  const position = getLocalPosition(player);
  position.x = options.position;
  setPlayerFormationActive(player, true);

  const archerTotal = Math.max(0, options.archers ?? 5);
  const pikemanTotal = Math.max(0, options.pikemen ?? 5);

  for (let i = 0; i < archerTotal; i += 1) {
    objects.push(makeArcher(`Formation Archer ${i + 1}`, options.position, generators));
  }

  for (let i = 0; i < pikemanTotal; i += 1) {
    objects.push(makePikeman(`Formation Pikeman ${i + 1}`, options.position, generators));
  }

  islandObject.objects = objects;

  return root;
}

export function pimpIsland(doc: JSONValue, options: PimpOptions): JSONValue {
  const root = cloneJson(doc);
  const { campaignObject, islandObject, objects } = getIslandContext(root, options);
  const generators = createIdGenerators(objects);

  const player = findObjectByName(objects, "Player 1");
  const playerPosition = player ? getLocalPosition(player) : null;
  if (player && playerPosition) {
    playerPosition.x = 0;
    setWalletCoinsOnObject(player, options.coins ?? 69);
  }

  const castle = findObjectByNameContains(objects, "Castle");
  if (!castle) {
    throw new Error("Castle not found on the selected island");
  }

  const theme = detectTheme(objects, campaignObject);

  if (typeof castle.name === "string" && castle.name.includes("Castle0")) {
    initShop("bow", objects, generators, theme);
    initShop("hammer", objects, generators, theme);
    initShop("scythe", objects, generators, theme);
  }

  upgradeCastle(castle, MAX_CASTLE_LEVEL, theme);
  upgradeWalls(objects, MAX_WALL_LEVEL, generators, theme);
  markTrees(objects);

  const spawnCount = Math.max(0, options.spawnCount ?? 10);
  const baseX = playerPosition && typeof playerPosition.x === "number" ? playerPosition.x : 0;

  for (let i = 0; i < spawnCount; i += 1) {
    objects.push(makeArcher(`Shooty Boy ${i + 1}`, baseX, generators));
    objects.push(makeWorker(`Good Ole Boy ${i + 1}`, baseX, generators));
    objects.push(makePikeman(`Pokey Boy ${i + 1}`, baseX, generators));
  }

  islandObject.objects = objects;

  return root;
}

export function markTreesForRemoval(doc: JSONValue, options: TreesOptions): JSONValue {
  const root = cloneJson(doc);
  const { islandObject, objects } = getIslandContext(root, options);

  markTrees(objects);

  const playerIndex = options.playerIndex ?? 0;
  const player = findObjectByName(objects, `Player ${playerIndex + 1}`);
  if (player) {
    setWalletCoinsOnObject(player, options.coins ?? 69);
  }

  islandObject.objects = objects;

  return root;
}

export function spawnUnits(doc: JSONValue, options: SpawnUnitsOptions): JSONValue {
  let current = doc;

  const archerCount = Math.max(0, options.archers ?? 0);
  if (archerCount > 0) {
    current = spawnArchers(current, { ...options, count: archerCount });
  }

  const workerCount = Math.max(0, options.workers ?? 0);
  if (workerCount > 0) {
    current = spawnWorkers(current, { ...options, count: workerCount });
  }

  const pikemanCount = Math.max(0, options.pikemen ?? 0);
  if (pikemanCount > 0) {
    current = spawnPikemen(current, { ...options, count: pikemanCount });
  }

  return current;
}

export function getIslandOverview(doc: JSONValue | null, options: CampaignIslandOptions): IslandOverview | null {
  if (!doc) return null;
  try {
    const { objects } = getIslandContext(doc, options);

    // Find Castle
    let castleX = 0;
    let castleLevel = 0;
    let castleName = "Castle";
    const castle = findObjectByNameContains(objects, "Castle");
    if (castle) {
      castleName = String(castle.name ?? "Castle");
      const pos = getLocalPosition(castle);
      castleX = Math.round(Number(pos.x ?? 0) * 100) / 100;
      const match = castleName.match(/Castle(\d+)/i);
      if (match) {
        castleLevel = parseInt(match[1], 10);
      }
    }

    // Players
    const players: PlayerInfo[] = [];
    for (const obj of objects) {
      const prefab = String(obj?.prefabPath ?? "");
      if (prefab.includes("Characters/Player")) {
        const pName = String(obj?.name ?? "Player");
        const pos = getLocalPosition(obj);
        players.push({
          name: pName,
          x: Math.round(Number(pos.x ?? 0) * 100) / 100,
          y: Math.round(Number(pos.y ?? 0) * 100) / 100,
        });
      }
    }
    players.sort((a, b) => a.name.localeCompare(b.name));

    // Special points
    const specialPoints: SpecialPointInfo[] = [];
    for (const obj of objects) {
      const name = String(obj?.name ?? "");
      const prefab = String(obj?.prefabPath ?? "");
      const lowerName = name.toLowerCase();
      const lowerPrefab = prefab.toLowerCase();
      const pos = getLocalPosition(obj);
      const x = Math.round(Number(pos.x ?? 0) * 100) / 100;

      if (lowerName.includes("beggar camp") || lowerPrefab.includes("beggar camp")) {
        specialPoints.push({ name: "Beggar Camp", type: "beggar_camp", x });
      } else if (lowerName.includes("portal") || lowerPrefab.includes("portal")) {
        specialPoints.push({ name, type: "portal", x });
      } else if (lowerName.includes("statue") || lowerPrefab.includes("statue")) {
        specialPoints.push({ name, type: "statue", x });
      } else if (lowerName.includes("mine") || lowerPrefab.includes("mine")) {
        specialPoints.push({ name, type: "mine", x });
      } else if (lowerName.includes("wreck") || lowerPrefab.includes("wreck")) {
        specialPoints.push({ name: "Shipwreck", type: "wreck", x });
      } else if (
        (lowerName.includes("boat") || lowerPrefab.includes("boat")) &&
        !lowerName.includes("position") &&
        !lowerPrefab.includes("position")
      ) {
        specialPoints.push({ name: "Boat", type: "boat", x });
      }
    }
    specialPoints.sort((a, b) => a.x - b.x);

    // Walls
    const walls: WallInfo[] = [];
    for (const obj of objects) {
      const info = parseWallInfo(obj, castleX);
      if (info) {
        walls.push(info);
      }
    }
    walls.sort((a, b) => a.x - b.x);

    // Trees
    const trees: TreeInfo[] = [];
    for (const obj of objects) {
      if (objHasComponent(obj, "WorkableTree")) {
        const pos = getLocalPosition(obj);
        const x = Math.round(Number(pos.x ?? 0) * 100) / 100;
        const side = x < castleX ? "left" : "right";
        const distanceToCastle = Math.round(Math.abs(x - castleX) * 100) / 100;

        let marked = false;
        if (Array.isArray(obj.componentData2)) {
          for (const c of obj.componentData2) {
            if (c?.name === "WorkableTree" && typeof c.data === "string") {
              try {
                const parsed = JSON.parse(c.data);
                marked = Boolean(parsed.marked);
              } catch {
                // ignore
              }
            }
          }
        }

        let isNearBeggarCamp = false;
        let nearCampDist: number | undefined = undefined;
        for (const pt of specialPoints) {
          if (pt.type === "beggar_camp") {
            const d = Math.abs(x - pt.x);
            if (d <= 15) {
              isNearBeggarCamp = true;
              nearCampDist = Math.round(d * 10) / 10;
              break;
            }
          }
        }

        trees.push({
          id: String(obj.uniqueID ?? ""),
          x,
          side,
          marked,
          distanceToCastle,
          isNearBeggarCamp,
          nearCampDist,
        });
      }
    }
    trees.sort((a, b) => a.x - b.x);

    // Towers
    const towers: TowerInfo[] = [];
    for (const obj of objects) {
      const info = parseTowerInfo(obj, castleX);
      if (info) {
        towers.push(info);
      }
    }
    towers.sort((a, b) => a.x - b.x);

    // Catapults
    const catapults: CatapultInfo[] = [];
    for (const obj of objects) {
      const name = String(obj?.name ?? "");
      if (name.includes("Catapult")) {
        const comps = Array.isArray(obj?.componentData2) ? obj.componentData2 : [];
        for (const comp of comps) {
          if (comp?.name === "Catapult" && typeof comp?.data === "string") {
            try {
              const cData = JSON.parse(comp.data);
              const sideVal = cData.side === 1 ? 1 : -1;
              const pos = getLocalPosition(obj);
              catapults.push({
                id: String(obj.uniqueID ?? ""),
                side: sideVal,
                sideLabel: sideVal === -1 ? "left" : "right",
                x: Math.round(Number(pos.x ?? 0) * 10) / 10,
                oilBarrels: typeof cData.oilBarrels === "number" ? cData.oilBarrels : 0,
              });
            } catch {
              // ignore malformed data
            }
          }
        }
      }
    }
    catapults.sort((a, b) => a.x - b.x);

    const deities = getCampaignDeities(doc, options.campaignIndex, options.targetMode, options.challengeIndex);

    return {
      castleX,
      castleLevel,
      castleName,
      players,
      walls,
      towers,
      catapults,
      trees,
      specialPoints,
      deities,
    };
  } catch (error) {
    console.error("getIslandOverview failed:", error);
    return null;
  }
}

export function upgradeSpecificWall(doc: JSONValue, options: UpgradeSpecificWallOptions): JSONValue {
  const root = cloneJson(doc);
  const { campaignObject, islandObject, objects } = getIslandContext(root, options);
  const theme = detectTheme(objects, campaignObject);

  const wall = objects.find((obj) => String(obj?.uniqueID ?? "") === options.wallId);
  if (!wall) {
    throw new Error(`Wall with ID ${options.wallId} not found`);
  }

  applyWallUpgrade(wall, options.targetLevel, Boolean(options.addHorn), theme);
  islandObject.objects = objects;
  return root;
}

export function batchUpgradeWalls(doc: JSONValue, options: BatchUpgradeWallsOptions): JSONValue {
  const root = cloneJson(doc);
  const { campaignObject, islandObject, objects } = getIslandContext(root, options);
  const theme = detectTheme(objects, campaignObject);
  const targetLevel = Math.min(Math.max(0, options.targetLevel), 5);
  const addHorn = Boolean(options.addHorn);

  for (const obj of objects) {
    const name = String(obj?.name ?? "");
    const prefab = String(obj?.prefabPath ?? "");
    const isWall = (prefab.includes("Buildings and Interactive/") && prefab.includes("Wall")) || /^Wall\d/.test(name);
    if (!isWall) continue;

    const isMound = name.includes("Wall0") || prefab.includes("Wall0");

    if (options.mode === "mounds_only" && !isMound) continue;
    if (options.mode === "built_only" && isMound) continue;

    applyWallUpgrade(obj, targetLevel, addHorn, theme);
  }

  islandObject.objects = objects;
  return root;
}

export function upgradeIslandCastle(doc: JSONValue, options: UpgradeCastleOptions): JSONValue {
  const root = cloneJson(doc);
  const { campaignObject, islandObject, objects } = getIslandContext(root, options);
  const theme = detectTheme(objects, campaignObject);
  const generators = createIdGenerators(objects);

  const castle = findObjectByNameContains(objects, "Castle");
  if (!castle) {
    throw new Error("Castle not found on the selected island");
  }

  const targetLevel = Math.min(Math.max(1, options.targetLevel), MAX_CASTLE_LEVEL);

  // Initialize standard shops if needed
  if (targetLevel >= 1) {
    initShop("bow", objects, generators, theme);
    initShop("hammer", objects, generators, theme);
  }
  if (targetLevel >= 4) {
    initShop("scythe", objects, generators, theme);
  }

  upgradeCastle(castle, targetLevel, theme);
  islandObject.objects = objects;
  return root;
}

export function upgradeSpecificTower(doc: JSONValue, options: UpgradeSpecificTowerOptions): JSONValue {
  const root = cloneJson(doc);
  const { campaignObject, islandObject, objects } = getIslandContext(root, options);
  const theme = detectTheme(objects, campaignObject);

  const tower = objects.find((obj) => String(obj?.uniqueID ?? "") === options.towerId);
  if (!tower) {
    throw new Error(`Tower with ID ${options.towerId} not found`);
  }

  applyTowerUpgrade(tower, options.targetType, theme);
  islandObject.objects = objects;
  return root;
}

export function batchUpgradeTowers(doc: JSONValue, options: BatchUpgradeTowersOptions): JSONValue {
  const root = cloneJson(doc);
  const { campaignObject, islandObject, objects } = getIslandContext(root, options);
  const theme = detectTheme(objects, campaignObject);

  for (const obj of objects) {
    const info = parseTowerInfo(obj, 0);
    if (!info) continue;

    const isMound = info.type === "tower0";
    if (options.mode === "mounds_only" && !isMound) continue;
    if (options.mode === "built_only" && isMound) continue;

    applyTowerUpgrade(obj, options.targetType, theme);
  }

  islandObject.objects = objects;
  return root;
}

export function setTreeMark(doc: JSONValue, options: SetTreeMarkOptions): JSONValue {
  const root = cloneJson(doc);
  const { islandObject, objects } = getIslandContext(root, options);

  for (const obj of objects) {
    if (!objHasComponent(obj, "WorkableTree")) continue;

    if (options.treeId) {
      if (String(obj.uniqueID ?? "") === options.treeId) {
        mutateComponentData(obj, "WorkableTree", (payload) => {
          payload.marked = options.marked;
        });
      }
    } else {
      const pos = getLocalPosition(obj);
      const x = Number(pos.x ?? 0);
      const inRange = (options.xMin === undefined || x >= options.xMin) &&
        (options.xMax === undefined || x <= options.xMax);
      if (inRange) {
        mutateComponentData(obj, "WorkableTree", (payload) => {
          payload.marked = options.marked;
        });
      }
    }
  }

  islandObject.objects = objects;
  return root;
}

const DEITY_DEFINITIONS = [
  { name: "Archer Shrine", island: 1 },
  { name: "Farmer Shrine", island: 2 },
  { name: "Builder Shrine", island: 3 },
  { name: "Knight Shrine", island: 5 },
];

export function getCampaignDeities(
  doc: JSONValue | null,
  campaignIndex: number = 0,
  targetMode: "campaign" | "challenge" = "campaign",
  challengeIndex: number = 0
): DeityInfo[] {
  if (!doc) return [];
  try {
    const rootObj = toObject(doc, "Invalid save data");
    let container: Record<string, JSONValue> | undefined;

    if (targetMode === "challenge") {
      const challenges = toArray(rootObj.challenges, "No challenges found");
      container = challenges[challengeIndex] as Record<string, JSONValue>;
    } else {
      const campaigns = toArray(rootObj.campaigns, "No campaigns found");
      container = campaigns[campaignIndex] as Record<string, JSONValue>;
    }

    if (!container) return [];

    const statuses = Array.isArray(container.deityStatuses)
      ? (container.deityStatuses as number[])
      : [0, 0, 0, 0, 0, 0];

    return DEITY_DEFINITIONS.map((d, i) => ({
      index: i,
      name: d.name,
      island: d.island,
      status: typeof statuses[i] === "number" ? statuses[i] : 0,
    }));
  } catch (err) {
    console.error("getCampaignDeities failed:", err);
    return [];
  }
}

export function setDeityStatus(doc: JSONValue, options: SetDeityStatusOptions): JSONValue {
  const root = cloneJson(doc);
  const rootObj = toObject(root, "Invalid save data");
  let container: Record<string, JSONValue> | undefined;

  if (options.targetMode === "challenge") {
    const challenges = toArray(rootObj.challenges, "No challenges found");
    const chIdx = options.challengeIndex ?? 0;
    container = challenges[chIdx] as Record<string, JSONValue>;
    if (!container) throw new Error("Challenge not found");
  } else {
    const campaigns = toArray(rootObj.campaigns, "No campaigns found");
    const campIdx = options.campaignIndex ?? 0;
    container = campaigns[campIdx] as Record<string, JSONValue>;
    if (!container) throw new Error("Campaign not found");
  }

  if (!Array.isArray(container.deityStatuses)) {
    container.deityStatuses = [0, 0, 0, 0, 0, 0];
  }

  const idx = options.deityIndex;
  if (idx >= 0 && idx < container.deityStatuses.length) {
    container.deityStatuses[idx] = options.status;
  }

  if (options.status === 2) {
    if (!Array.isArray(container.StatueExpirationDay)) {
      container.StatueExpirationDay = [0, 0, 0, 0, 0, 0];
    }
    const currentDays = typeof container.totalGameDays === "number" ? container.totalGameDays : 100;
    container.StatueExpirationDay[idx] = currentDays + 999;
  } else if (options.status === 0) {
    if (Array.isArray(container.StatueExpirationDay)) {
      container.StatueExpirationDay[idx] = 0;
    }
  }

  return root;
}

export function unlockAllDeities(doc: JSONValue, options: UnlockAllDeitiesOptions): JSONValue {
  const root = cloneJson(doc);
  const rootObj = toObject(root, "Invalid save data");
  let container: Record<string, JSONValue> | undefined;

  if (options.targetMode === "challenge") {
    const challenges = toArray(rootObj.challenges, "No challenges found");
    const chIdx = options.challengeIndex ?? 0;
    container = challenges[chIdx] as Record<string, JSONValue>;
    if (!container) throw new Error("Challenge not found");
  } else {
    const campaigns = toArray(rootObj.campaigns, "No campaigns found");
    const campIdx = options.campaignIndex ?? 0;
    container = campaigns[campIdx] as Record<string, JSONValue>;
    if (!container) throw new Error("Campaign not found");
  }

  const targetStatus = options.status !== undefined ? options.status : 2;

  if (!Array.isArray(container.deityStatuses)) {
    container.deityStatuses = [0, 0, 0, 0, 0, 0];
  }

  for (let i = 0; i < 4; i++) {
    container.deityStatuses[i] = targetStatus;
  }

  if (targetStatus === 2) {
    if (!Array.isArray(container.StatueExpirationDay)) {
      container.StatueExpirationDay = [0, 0, 0, 0, 0, 0];
    }
    const currentDays = typeof container.totalGameDays === "number" ? container.totalGameDays : 100;
    for (let i = 0; i < 4; i++) {
      container.StatueExpirationDay[i] = currentDays + 999;
    }
  } else if (targetStatus === 0) {
    if (Array.isArray(container.StatueExpirationDay)) {
      for (let i = 0; i < 4; i++) {
        container.StatueExpirationDay[i] = 0;
      }
    }
  }

  return root;
}

export function setCatapultOilBarrels(doc: JSONValue, options: SetCatapultOilBarrelsOptions): JSONValue {
  const root = cloneJson(doc);
  const { objects, islandObject } = getIslandContext(root, options);
  const { side, oilBarrels } = options;

  let updatedCount = 0;
  for (const obj of objects) {
    const name = String(obj?.name ?? "");
    if (name.includes("Catapult")) {
      const comps = Array.isArray(obj?.componentData2) ? obj.componentData2 : [];
      for (const comp of comps) {
        if (comp?.name === "Catapult" && typeof comp?.data === "string") {
          try {
            const cData = JSON.parse(comp.data);
            const catapultSide = cData.side === 1 ? 1 : -1;
            if (catapultSide === side) {
              cData.oilBarrels = Math.max(0, oilBarrels);
              comp.data = JSON.stringify(cData);
              updatedCount++;
            }
          } catch {
            // ignore malformed data
          }
        }
      }
    }
  }

  if (updatedCount === 0) {
    const sideName = side === -1 ? "Left" : "Right";
    throw new Error(`No Catapult found on the ${sideName} side of this island`);
  }

  islandObject.objects = objects;
  return root;
}

export function teleportPlayer(doc: JSONValue, options: TeleportPlayerOptions): JSONValue {
  const root = cloneJson(doc);
  const { objects, islandObject } = getIslandContext(root, options);
  const { playerIndex, x } = options;

  let player = findPlayerObject(objects, playerIndex);
  if (!player) {
    for (const obj of objects) {
      const prefab = String(obj?.prefabPath ?? "");
      const name = String(obj?.name ?? "");
      if (prefab.includes("Characters/Player") && (name === `Player ${playerIndex + 1}` || name.includes(`Player`))) {
        player = obj;
        break;
      }
    }
  }

  if (!player) {
    throw new Error(`Player ${playerIndex + 1} not found on this island`);
  }

  if (!player.localPosition || typeof player.localPosition !== "object") {
    player.localPosition = { x, y: 0.88, z: 1.0 };
  } else {
    player.localPosition.x = x;
  }

  islandObject.objects = objects;
  return root;
}
