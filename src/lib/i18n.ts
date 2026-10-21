export type Language = 'en' | 'fr';

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
  fr: {
    welcome: {
      title: 'Éditeur Kingdom Two Crowns',
      subtitle: 'Modifiez vos sauvegardes facilement',
      osDetected: 'Système détecté :',
      defaultPath: 'Emplacement de sauvegarde par défaut :',
      pathHint: 'Le sélecteur de fichier ouvrira automatiquement ce dossier',
      openFile: 'Ouvrir global-v35',
      loading: 'Chargement...',
      features: {
        resources: { title: 'Ressources', desc: 'Modifiez vos pièces et gemmes' },
        navigation: { title: 'Navigation', desc: 'Voyagez instantanément entre les îles' },
        combat: { title: 'Combat', desc: 'Déployez vos formations de combat' },
        construction: { title: 'Construction', desc: 'Améliorez votre royaume' },
        recruitment: { title: 'Recrutement', desc: 'Ajoutez archers, ouvriers et plus' },
      },
    },
    header: {
      title: 'Éditeur Kingdom Two Crowns',
      changeFile: 'Changer de fichier',
      save: '💾 Sauvegarder',
      backupCreated: 'Copie de sauvegarde:',
      filePath: '📁',
    },
    context: {
      title: 'Contexte',
      description: 'Sélectionnez la campagne, le défi et l\'île à modifier.',
      mode: 'Mode',
      modeCampaign: 'Mode Campagne',
      modeChallenge: 'Mode Défi',
      campaign: 'Campagne',
      challenge: 'Île de défi',
      island: 'Île',
    },
    tabs: {
      inspector: 'Carte & inspecteur d\'île',
      resources: 'Ressources',
      navigation: 'Navigation',
      combat: 'Combat',
      construction: 'Construction',
      recruitment: 'Recrutement',
    },
    inspector: {
      title: 'Inspecteur d\'île',
      description: 'Coordonnées du château, des murs et des arbres sur l\'axe X.',
      castleInfo: 'Centre-ville / Château',
      castlePosition: 'Position du château',
      leftSide: 'Côté gauche (X < château)',
      rightSide: 'Côté droit (X > château)',
      distance: 'Dist. :',
      walls: 'Murs & fortifications',
      trees: 'Arbres & végétation',
      specialPoints: 'Lieux clés & points de repère',
      upgradeToL5: 'Améliorer en N5 (fer)',
      upgradeToL5Horn: 'Améliorer en N5 + cor',
      demoteToMound: 'Réinitialiser en butte de terre (N0)',
      batchUpgradeBuilt: 'Améliorer tous les murs construits en N5',
      batchUpgradeAll: 'Tout améliorer en N5 (buttes comprises)',
      markTree: 'Marquer à abattre',
      unmarkTree: 'Conserver l\'arbre',
      markRange: 'Marquer la plage',
      unmarkRange: 'Démarquer la plage',
      standing: 'Debout',
      markedToCut: 'Marqué à abattre',
      nearBeggarCampWarn: '⚠️ Près d\'un camp de vagabonds (ne pas abattre !)',
      totalWalls: 'Total des murs :',
      totalTrees: 'Total des arbres :',
      rangeFrom: 'De X :',
      rangeTo: 'À X :',
      applyRange: 'Appliquer la plage',
      level: 'Niveau',
      deitiesTitle: 'Sanctuaires & divinités',
      unlockAllDeities: 'Débloquer tous les sanctuaires',
      activateAllDeities: '⚡ Activer tous les sanctuaires (bonus complet)',
      lockAllDeities: 'Verrouiller tous les sanctuaires',
      deityStatusLocked: '🔒 Verrouillé',
      deityStatusUnlocked: '🔓 Débloqué',
      deityStatusActive: '✨ Bonus actif',
      towers: 'Tours d\'archers & balistes',
      upgradeToL6: 'N6 (4 archers)',
      upgradeToL5Roof: 'N5 (toit)',
      upgradeToBallista: 'Baliste',
      upgradeToBaker: 'Boulangerie',
      upgradeToKnightTower: 'Tour de chevalier',
      demoteTowerToMound: 'Réinitialiser en butte de terre (N0)',
      batchUpgradeTowersRoof: 'Améliorer toutes les tours en N5 (toit)',
      batchUpgradeTowersL6: 'Améliorer toutes les tours en N6 (4 archers)',
      upgradeCastleIron: 'Améliorer le château en fer (niveau 7)',
      upgradeCastleStone: 'Améliorer le château en pierre (niveau 4)',
    },
    resources: {
      title: 'Ressources',
      playerCoins: 'Or du joueur',
      playerGems: 'Gemmes du joueur',
      player: 'Joueur',
      coins: 'Pièces',
      gems: 'Gemmes',
      apply: 'Appliquer',
      current: 'Actuel:',
    },
    navigation: {
      title: 'Navigation & conquête',
      fastTravel: 'Voyage rapide',
      takeOver: 'Prendre le contrôle',
      destroyPortals: 'Détruire les portails',
      exterminate: 'Exterminer les ennemis',
      markTrees: 'Élaguer la forêt',
      updateDestination: 'Mettre à jour la destination',
      cleanIsland: 'Nettoyer l\'île',
      destroy: 'Détruire les portails',
      exterminateEnemies: 'Exterminer les ennemis',
      mark: 'Marquer les arbres',
      pikemen: 'Piquiers',
      farmers: 'Fermiers',
      boats: 'Bateaux',
      playerCoins: 'Pièces du joueur',
      archers: 'Archers',
      workers: 'Ouvriers',
      formationOffset: 'Décalage formation',
      teleportTitle: 'Téléporter le monarque (changer de position)',
      teleportX: 'Position X cible',
      teleportButton: '⚡ Téléporter',
      teleportToCastle: 'Centre / Château',
    },
    combat: {
      title: 'Combat',
      battleFormation: 'Formation de bataille',
      deploy: 'Déployer',
      positionX: 'Position X',
      catapultAmmo: 'Barils d\'huile des catapultes',
      catapultLeft: 'Catapulte gauche',
      catapultRight: 'Catapulte droite',
      catapultApply: 'Définir les barils',
      barrelsCount: 'Barils d\'huile',
      noCatapultOnIsland: 'Aucune catapulte trouvée de ce côté de l\'île',
    },
    construction: {
      title: 'Construction',
      pimpIsland: 'Pimper l\'île',
      upgrade: 'Mettre à niveau',
      unitsToRecruit: 'Unités à recruter',
    },
    recruitment: {
      title: 'Recrutement',
      rallyPoint: 'Point de rassemblement',
      archers: 'Archers',
      workers: 'Ouvriers',
      farmers: 'Fermiers',
      pikemen: 'Piquiers',
      knights: 'Chevaliers',
      quantity: 'Quantité',
      side: 'Côté de la frontière',
      sideLeft: 'Gauche',
      sideRight: 'Droite',
      withArchers: 'Inclure les 4 archers d\'escouade',
      add: 'Ajouter',
      combinedRecruitment: 'Recrutement combiné',
      addUnits: 'Ajouter les unités',
      current: 'Actuel:',
    },
    status: {
      coinsUpdated: 'Pièces mises à jour',
      gemsUpdated: 'Gemmes mises à jour',
      archersAdded: 'Archers ajoutés',
      workersAdded: 'Ouvriers ajoutés',
      farmersAdded: 'Fermiers ajoutés',
      pikemenAdded: 'Piquiers ajoutés',
      knightsAdded: 'Chevaliers ajoutés au château',
      destinationUpdated: 'Destination mise à jour',
      islandSecured: 'Île sécurisée',
      portalsDestroyed: 'Portails détruits',
      enemiesRemoved: 'Ennemis supprimés',
      formationDeployed: 'Formation déployée',
      islandUpgraded: 'Île améliorée',
      treesMarked: 'Arbres marqués',
      unitsAdded: 'Unités ajoutées',
      noFileLoaded: 'Aucun fichier chargé',
      saveApplied: 'Sauvegarde appliquée',
      addAtLeastOneUnit: 'Saisissez au moins une unité à ajouter',
      wallUpdated: 'Mur amélioré avec succès',
      wallsBatchUpdated: 'Tous les murs sélectionnés ont été améliorés',
      castleUpdated: 'Château amélioré avec succès',
      towerUpdated: 'Tour améliorée avec succès',
      towersBatchUpdated: 'Toutes les tours sélectionnées ont été améliorées',
      treeMarkUpdated: 'Marquage des arbres mis à jour',
      deityUpdated: 'Statut du sanctuaire mis à jour',
      deitiesAllUpdated: 'Tous les sanctuaires ont été mis à jour',
      catapultOilUpdated: 'Barils d\'huile des catapultes mis à jour',
      playerTeleported: 'Joueur téléporté avec succès',
    },
  },
};

export function detectLanguage(): Language {
  const browserLang = navigator.language.toLowerCase();

  if (browserLang.startsWith('fr')) {
    return 'fr';
  }

  return 'en';
}

export function getTranslations(lang: Language): Translations {
  return translations[lang];
}
