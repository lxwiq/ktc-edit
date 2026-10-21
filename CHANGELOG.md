# Changelog

Notable user-facing changes in this fork of [lxwiq/ktc-edit](https://github.com/lxwiq/ktc-edit). Entries summarize repository history; feature availability depends on the game build and platform described in [README.md](README.md).

## Unreleased

### Documentation

- Put the Apple Silicon macOS download and first-use instructions at the top of the README.
- Replace placeholder download and clone URLs with this repository's URLs.
- Document fork-specific features, English-only interface, and supported-profile limits.
- Correct save backup names and document the five-backup retention policy and recovery steps.
- Replace the template changelog with changes recorded in the fork's commit history.

## v0.1.0

[Release and downloads](https://github.com/nguyenphuquang1234567/ktc-edit/releases/tag/v0.1.0)

The published release includes an Apple Silicon macOS DMG. The following additions are recorded in the fork's history through `dc68387`; they do not establish gameplay verification on every platform or DLC.

### Save editor

- Campaign and Challenge mode selection with mode-specific tabs.
- Challenge resource editing, recruitment, island map and inspector, shrine/deity buffs, and catapult oil barrels.
- Ruler teleport and catapult oil barrel editing.
- Expanded save options and UI improvements.
- Wall upgrades that inherit asset base HP, with optional reset of existing wall overrides in campaign and challenge saves.

### Game asset editor

- Mount speed, stamina, and ability settings, including Griffin, Lizard, regular horse, and warhorse targets.
- Archer movement, attack preparation, cooldowns and intervals; builder movement and work time.
- Player wallet capacity; knight wallet capacity, tax threshold and coin drop probability.
- Iron wall base HP and coin/gem bag scale.
- Dynamic target lookup through Unity MonoScript and GameObject structures.
- Regular horse targets in `sharedassets0.assets` included alongside resource variants.
- Negative run stamina rates and expanded numeric ranges for relevant asset settings.

### Apple Silicon binary patches

- Warhorse buff collider limit controls.
- Visible coin limit, separate from wallet balance and gems.
- Vagrant camp preservation, including detachment from the forest list.
- Recognition of updated game build layouts for coin and camp hooks.
- Separate restore controls and binary backups for supported patches.

### Inherited functionality

- Player coins and gems, island travel, combat formations, construction upgrades, and unit recruitment.
- Automatic backups before save writes.

The current interface is English. Earlier template documentation described additional languages and platform installers that are not present in the current interface or published release.
