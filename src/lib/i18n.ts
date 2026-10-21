export type Language = 'en';

export interface Translations {
  welcome: {
    title: string;
    subtitle: string;
    osDetected: string;
    defaultPath: string;
    pathHint: string;
    openFile: string;
    loading: string;
    features: {
      resources: { title: string; desc: string };
      navigation: { title: string; desc: string };
      combat: { title: string; desc: string };
      construction: { title: string; desc: string };
      recruitment: { title: string; desc: string };
    };
  };
  header: {
    title: string;
    changeFile: string;
    save: string;
    backupCreated: string;
    filePath: string;
  };
  context: {
    title: string;
    description: string;
    mode: string;
    modeCampaign: string;
    modeChallenge: string;
    campaign: string;
    challenge: string;
    island: string;
  };
  tabs: {
    inspector: string;
    resources: string;
    navigation: string;
    combat: string;
    construction: string;
    recruitment: string;
  };
  inspector: {
    title: string;
    description: string;
    castleInfo: string;
    castlePosition: string;
    leftSide: string;
    rightSide: string;
    distance: string;
    walls: string;
    trees: string;
    specialPoints: string;
    upgradeToL5: string;
    upgradeToL5Horn: string;
    demoteToMound: string;
    batchUpgradeBuilt: string;
    batchUpgradeAll: string;
    markTree: string;
    unmarkTree: string;
    markRange: string;
    unmarkRange: string;
    standing: string;
    markedToCut: string;
    nearBeggarCampWarn: string;
    totalWalls: string;
    totalTrees: string;
    rangeFrom: string;
    rangeTo: string;
    applyRange: string;
    level: string;
    deitiesTitle: string;
    unlockAllDeities: string;
    activateAllDeities: string;
    lockAllDeities: string;
    deityStatusLocked: string;
    deityStatusUnlocked: string;
    deityStatusActive: string;
    towers: string;
    upgradeToL6: string;
    upgradeToL5Roof: string;
    upgradeToBallista: string;
    upgradeToBaker: string;
    upgradeToKnightTower: string;
    demoteTowerToMound: string;
    batchUpgradeTowersRoof: string;
    batchUpgradeTowersL6: string;
    upgradeCastleIron: string;
    upgradeCastleStone: string;
  };
  resources: {
    title: string;
    playerCoins: string;
    playerGems: string;
    player: string;
    coins: string;
    gems: string;
    apply: string;
    current: string;
  };
  navigation: {
    title: string;
    fastTravel: string;
    takeOver: string;
    destroyPortals: string;
    exterminate: string;
    markTrees: string;
    updateDestination: string;
    cleanIsland: string;
    destroy: string;
    exterminateEnemies: string;
    mark: string;
    pikemen: string;
    farmers: string;
    boats: string;
    playerCoins: string;
    archers: string;
    workers: string;
    formationOffset: string;
    teleportTitle: string;
    teleportX: string;
    teleportButton: string;
    teleportToCastle: string;
  };
  combat: {
    title: string;
    battleFormation: string;
    deploy: string;
    positionX: string;
    catapultAmmo: string;
    catapultLeft: string;
    catapultRight: string;
    catapultApply: string;
    barrelsCount: string;
    noCatapultOnIsland: string;
  };
  construction: {
    title: string;
    pimpIsland: string;
    upgrade: string;
    unitsToRecruit: string;
  };
  recruitment: {
    title: string;
    rallyPoint: string;
    archers: string;
    workers: string;
    farmers: string;
    pikemen: string;
    knights: string;
    quantity: string;
    side: string;
    sideLeft: string;
    sideRight: string;
    withArchers: string;
    add: string;
    combinedRecruitment: string;
    addUnits: string;
    current: string;
  };
  status: {
    coinsUpdated: string;
    gemsUpdated: string;
    archersAdded: string;
    workersAdded: string;
    farmersAdded: string;
    pikemenAdded: string;
    knightsAdded: string;
    destinationUpdated: string;
    islandSecured: string;
    portalsDestroyed: string;
    enemiesRemoved: string;
    formationDeployed: string;
    islandUpgraded: string;
    treesMarked: string;
    unitsAdded: string;
    noFileLoaded: string;
    saveApplied: string;
    addAtLeastOneUnit: string;
    wallUpdated: string;
    wallsBatchUpdated: string;
    castleUpdated: string;
    towerUpdated: string;
    towersBatchUpdated: string;
    treeMarkUpdated: string;
    deityUpdated: string;
    deitiesAllUpdated: string;
    catapultOilUpdated: string;
    playerTeleported: string;
  };
}

const translations: Record<Language, Translations> = {
  en: {
    welcome: {
      title: 'Kingdom Two Crowns Save Editor',
      subtitle: 'Edit your save files easily',
      osDetected: 'System detected:',
      defaultPath: 'Default save location:',
      pathHint: 'The file picker will automatically open this folder',
      openFile: 'Open global-v35',
      loading: 'Loading...',
      features: {
        resources: { title: 'Resources', desc: 'Edit your coins and gems' },
        navigation: { title: 'Navigation', desc: 'Travel instantly between islands' },
        combat: { title: 'Combat', desc: 'Deploy your battle formations' },
        construction: { title: 'Construction', desc: 'Upgrade your kingdom' },
        recruitment: { title: 'Recruitment', desc: 'Add archers, workers and more' },
      },
    },
    header: {
      title: 'Kingdom Two Crowns Save Editor',
      changeFile: 'Change file',
      save: '💾 Save',
      backupCreated: 'Backup copy:',
      filePath: '📁',
    },
    context: {
      title: 'Context',
      description: 'Select the campaign, challenge, and island to edit.',
      mode: 'Mode',
      modeCampaign: 'Campaign Mode',
      modeChallenge: 'Challenge Mode',
      campaign: 'Campaign',
      challenge: 'Challenge Island',
      island: 'Island',
    },
    tabs: {
      inspector: 'Island Map & Inspector',
      resources: 'Resources',
      navigation: 'Navigation',
      combat: 'Combat',
      construction: 'Construction',
      recruitment: 'Recruitment',
    },
    inspector: {
      title: 'Island Inspector',
      description: 'Coordinates of Castle, Walls and Trees along the X axis.',
      castleInfo: 'Town Center / Castle',
      castlePosition: 'Castle Position',
      leftSide: 'Left Side (X < Castle)',
      rightSide: 'Right Side (X > Castle)',
      distance: 'Dist:',
      walls: 'Walls & Fortifications',
      trees: 'Trees & Vegetation',
      specialPoints: 'Key Locations & Landmarks',
      upgradeToL5: 'Upgrade to L5 (Iron)',
      upgradeToL5Horn: 'Upgrade to L5 + Horn',
      demoteToMound: 'Reset to Dirt Mound (L0)',
      batchUpgradeBuilt: 'Upgrade All Built Walls to L5',
      batchUpgradeAll: 'Upgrade All (including Mounds) to L5',
      markTree: 'Mark to Cut',
      unmarkTree: 'Keep Tree',
      markRange: 'Mark range',
      unmarkRange: 'Unmark range',
      standing: 'Standing',
      markedToCut: 'Marked to cut',
      nearBeggarCampWarn: '⚠️ Near Beggar Camp (Do not cut!)',
      totalWalls: 'Total Walls:',
      totalTrees: 'Total Trees:',
      rangeFrom: 'From X:',
      rangeTo: 'To X:',
      applyRange: 'Apply Range',
      level: 'Level',
      deitiesTitle: 'Shrines & Deities',
      unlockAllDeities: 'Unlock All Shrines',
      activateAllDeities: '⚡ Activate All Shrines (Full Buff)',
      lockAllDeities: 'Lock All Shrines',
      deityStatusLocked: '🔒 Locked',
      deityStatusUnlocked: '🔓 Unlocked',
      deityStatusActive: '✨ Active Buff',
      towers: 'Archer Towers & Ballistas',
      upgradeToL6: 'L6 (4 Archers)',
      upgradeToL5Roof: 'L5 (Roof)',
      upgradeToBallista: 'Ballista',
      upgradeToBaker: 'Bakery',
      upgradeToKnightTower: 'Knight Tower',
      demoteTowerToMound: 'Reset to Dirt Mound (L0)',
      batchUpgradeTowersRoof: 'Upgrade All Towers to L5 (Roof)',
      batchUpgradeTowersL6: 'Upgrade All Towers to L6 (4 Archers)',
      upgradeCastleIron: 'Upgrade Castle to Iron (Level 7)',
      upgradeCastleStone: 'Upgrade Castle to Stone (Level 4)',
    },
    resources: {
      title: 'Resources',
      playerCoins: 'Player gold',
      playerGems: 'Player gems',
      player: 'Player',
      coins: 'Coins',
      gems: 'Gems',
      apply: 'Apply',
      current: 'Current:',
    },
    navigation: {
      title: 'Navigation & conquest',
      fastTravel: 'Fast travel',
      takeOver: 'Take over',
      destroyPortals: 'Destroy portals',
      exterminate: 'Exterminate enemies',
      markTrees: 'Mark trees',
      updateDestination: 'Update destination',
      cleanIsland: 'Clean island',
      destroy: 'Destroy portals',
      exterminateEnemies: 'Exterminate enemies',
      mark: 'Mark trees',
      pikemen: 'Pikemen',
      farmers: 'Farmers',
      boats: 'Boats',
      playerCoins: 'Player coins',
      archers: 'Archers',
      workers: 'Workers',
      formationOffset: 'Formation offset',
      teleportTitle: 'Teleport Ruler (Change Position)',
      teleportX: 'Target Position X',
      teleportButton: '⚡ Teleport',
      teleportToCastle: 'Center / Castle',
    },
    combat: {
      title: 'Combat',
      battleFormation: 'Battle formation',
      deploy: 'Deploy',
      positionX: 'Position X',
      catapultAmmo: 'Catapult Oil Barrels',
      catapultLeft: 'Left Catapult',
      catapultRight: 'Right Catapult',
      catapultApply: 'Set Barrels',
      barrelsCount: 'Oil Barrels',
      noCatapultOnIsland: 'No Catapult found on this side of the island',
    },
    construction: {
      title: 'Construction',
      pimpIsland: 'Pimp my island',
      upgrade: 'Upgrade',
      unitsToRecruit: 'Units to recruit',
    },
    recruitment: {
      title: 'Recruitment',
      rallyPoint: 'Rally point',
      archers: 'Archers',
      workers: 'Workers',
      farmers: 'Farmers',
      pikemen: 'Pikemen',
      knights: 'Knights',
      quantity: 'Quantity',
      side: 'Border Side',
      sideLeft: 'Left',
      sideRight: 'Right',
      withArchers: 'Include 4 Squad Archers',
      add: 'Add',
      combinedRecruitment: 'Combined recruitment',
      addUnits: 'Add units',
      current: 'Current:',
    },
    status: {
      coinsUpdated: 'Coins updated',
      gemsUpdated: 'Gems updated',
      archersAdded: 'Archers added',
      workersAdded: 'Workers added',
      farmersAdded: 'Farmers added',
      pikemenAdded: 'Pikemen added',
      knightsAdded: 'Knights added to Castle',
      destinationUpdated: 'Destination updated',
      islandSecured: 'Island secured',
      portalsDestroyed: 'Portals destroyed',
      enemiesRemoved: 'Enemies removed',
      formationDeployed: 'Formation deployed',
      islandUpgraded: 'Island upgraded',
      treesMarked: 'Trees marked',
      unitsAdded: 'Units added',
      noFileLoaded: 'No file loaded',
      saveApplied: 'Save applied',
      addAtLeastOneUnit: 'Enter at least one unit to add',
      wallUpdated: 'Wall upgraded successfully',
      wallsBatchUpdated: 'All selected walls upgraded',
      castleUpdated: 'Castle upgraded successfully',
      towerUpdated: 'Tower upgraded successfully',
      towersBatchUpdated: 'All selected towers upgraded',
      treeMarkUpdated: 'Tree marking updated',
      deityUpdated: 'Shrine status updated',
      deitiesAllUpdated: 'All shrines updated successfully',
      catapultOilUpdated: 'Catapult oil barrels updated successfully',
      playerTeleported: 'Player teleported successfully',
    },
  },
};

export function detectLanguage(): Language {
  return 'en';
}

export function getTranslations(lang: Language): Translations {
  return translations[lang];
}
