# Kingdom Two Crowns Save & Asset Editor

Edit coins and gems, manage islands and units, and customize mounts and game settings in a desktop app built with Tauri and SvelteKit.

[Releases](https://github.com/lxwiq/ktc-edit/releases) · [What's changed](CHANGELOG.md) · [Report a bug](https://github.com/lxwiq/ktc-edit/issues)

> The game asset editor, Campaign/Challenge mode, island inspector and Apple Silicon patches were contributed by **[Nguyen Phu Quang](https://github.com/nguyenphuquang1234567)** in the fork [nguyenphuquang1234567/ktc-edit](https://github.com/nguyenphuquang1234567/ktc-edit). Thank you! See [Contributors](#contributors).

## Quick start

1. Download a build from [Releases](https://github.com/lxwiq/ktc-edit/releases) or [build from source](#build-from-source).
2. Close Kingdom Two Crowns and keep a separate copy of your save before editing.
3. Launch the app and open your `global-v35` save file.
4. Select **Campaign** or **Challenge**, then the relevant island or challenge.
5. Change the values you need and click **Save**. The app creates a backup before writing.
6. Start the game and check your changes.

For asset settings, choose **Edit Game Assets** from the welcome screen. Select your game's Data folder if the default Steam path is unavailable, adjust settings, and use **Apply**. The Apple Silicon binary patches have their own Apply and Restore controls.

## What you can edit

| Area | Examples |
| --- | --- |
| Resources | Player 1 and Player 2 coins and gems |
| Navigation | Island travel, ruler teleport, island map and inspector |
| Combat and recruitment | Unit spawning, formations, enemy and portal actions |
| Construction | Castle and wall upgrades, tree actions, catapult oil barrels |
| Game assets | Mounts, archers, builders, knight and peasant speeds, unit invulnerability, wallets, iron walls, bag scale |
| Apple Silicon patches | Warhorse collider limit, visible coin limit, vagrant camp preservation, clock speed, experimental Blood Moon |

Some actions depend on the selected mode and game build. The app hides non-applicable tabs in Challenge mode.

These are implemented features, not a claim that every setting has been tested in gameplay on every game version or DLC. Multiplayer behavior for camp preservation is unverified.

## Compatibility

- **Save editing:** targets the `global-v35` save format (gzip-compressed JSON).
- **Game assets:** accepts only the file profiles recognized by the current code. A different game update may be rejected; do not assume all DLC or versions are supported.
- **Binary patches:** require supported macOS Apple Silicon builds of `GameAssembly.dylib`; these are separate from ordinary save edits.
- **Interface:** English and French, detected from the system language. The game asset editor screen is currently English only.

The default macOS save location is:

```text
~/Library/Application Support/nl.noio.kingdom-two-crowns/Release/global-v35
```

If automatic detection fails on your platform, use the file picker to select your actual `global-v35`. Save locations can vary by installation.

## Backups and recovery

Save backups are written beside the original file as `global-v35_<unix-timestamp>.bak`. Ordinary save and asset backups retain the five most recent backups per file stem, so keep a separate copy for long-term recovery.

To restore a save:

1. Close the game and editor.
2. Copy your current `global-v35` somewhere safe.
3. Copy the chosen backup into the save folder and name the copy `global-v35`.
4. Restart the game.

Asset edits back up `resources.assets` and `sharedassets0.assets` before writing. Use **Restore** in the asset editor while its backup paths are available, or restore the matching file pair from your saved backups with the game closed. Binary patches create separate `GameAssembly.dylib` backups and expose their own restore controls.

## Troubleshooting

**The macOS app reports that it is damaged:** remove the quarantine attribute from the app installed in Applications:

```sh
xattr -cr /Applications/ktcedit.app
```

**The game assets are rejected:** the selected files do not match a supported profile. Report the game version, platform, and exact error; do not force an unsupported patch.

**Changes do not appear:** close the game before editing, confirm you saved the correct campaign or challenge, and restart the game. Asset settings can interact with existing save and runtime state.

For bug reports, include your app version, OS and architecture, game version/DLC, selected mode, exact error, and steps to reproduce. Remove personal information from attachments.

## Build from source

You need Node.js 20 or later, Rust, and the platform dependencies for Tauri 2. The [build workflow](.github/workflows/build.yml) lists the CI dependencies and targets.

```sh
git clone https://github.com/lxwiq/ktc-edit.git
cd ktc-edit
npm ci
npm run tauri dev
```

To check the frontend or build an installer:

```sh
npm run check
npm run tauri build
```

Built bundles are placed in `src-tauri/target/release/bundle/`. The included GitHub Actions workflow builds draft releases on `v*` tags or manual dispatch; publishing a release is a separate step.

## Contributing

Bug reports and contributions are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md). For compatibility changes, describe the game build and platform used and distinguish code checks from actual gameplay testing.

If this tool helped you, consider leaving a ⭐ so other players can find it.

## Contributors

- **[lxwiq](https://github.com/lxwiq)** — original Tauri/SvelteKit save editor.
- **[Nguyen Phu Quang](https://github.com/nguyenphuquang1234567)** — game asset editor (mounts, archers, builders, wallets, iron walls, bag scale), Campaign/Challenge mode, island map and inspector, shrines and deities, ruler teleport, catapult oil barrels, Apple Silicon binary patches (warhorse collider limit, visible coin limit, vagrant camp preservation) and the documentation rewrite. Originally developed in [nguyenphuquang1234567/ktc-edit](https://github.com/nguyenphuquang1234567/ktc-edit).

## Credits and license

- Original Python implementation: [bitwitch/kingdom-edit](https://github.com/bitwitch/kingdom-edit).
- Kingdom Two Crowns by Noio and Raw Fury.

[MIT License](LICENSE). This is an unofficial project, unaffiliated with Noio or Raw Fury. Keep backups before editing saves or game files.
