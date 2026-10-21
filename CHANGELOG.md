# Changelog

Notable user-facing changes to Kingdom Two Crowns Save & Asset Editor. Feature availability depends on the game build and platform described in [README.md](README.md).

## Unreleased

### Contributed by [Nguyen Phu Quang](https://github.com/nguyenphuquang1234567)

Merged from the fork [nguyenphuquang1234567/ktc-edit](https://github.com/nguyenphuquang1234567/ktc-edit) (history through `447848c`). These entries summarize the fork's commit history; they do not establish gameplay verification on every platform or DLC.

#### Save editor

- Campaign and Challenge mode selection with mode-specific tabs.
- Challenge resource editing, recruitment, island map and inspector, shrine/deity buffs, and catapult oil barrels.
- Ruler teleport and catapult oil barrel editing.
- Expanded save options and UI improvements.
- Wall upgrades that inherit asset base HP, with optional reset of existing wall overrides in campaign and challenge saves.

#### Game asset editor

- Mount speed, stamina, and ability settings, including Griffin, Lizard, regular horse, and warhorse targets.
- Archer movement, attack preparation, cooldowns and intervals; builder movement and work time.
- Player wallet capacity; knight wallet capacity, tax threshold and coin drop probability.
- Iron wall base HP and coin/gem bag scale.
- Dynamic target lookup through Unity MonoScript and GameObject structures.
- Regular horse targets in `sharedassets0.assets` included alongside resource variants.
- Negative run stamina rates and expanded numeric ranges for relevant asset settings.

#### Apple Silicon binary patches

- Warhorse buff collider limit controls.
- Visible coin limit, separate from wallet balance and gems.
- Vagrant camp preservation, including detachment from the forest list.
- Recognition of updated game build layouts for coin and camp hooks.
- Separate restore controls and binary backups for supported patches.

#### Documentation

- Rewritten README with quick start, compatibility limits, backup naming and recovery steps.

### Changed

- French translation restored and extended to the new save editor features (the fork had made the interface English only). The game asset editor screen remains English only.
- README and changelog credit the fork's contributions.

## Initial version

- Player coins and gems, island travel, combat formations, construction upgrades, and unit recruitment.
- Automatic backups before save writes.
- English and French interface with automatic language detection.
