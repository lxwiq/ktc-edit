# Contributing to Kingdom Two Crowns Save Editor

Thank you for considering contributing to this project! This document provides guidelines and instructions for contributing.

## Code of Conduct

- Be respectful and constructive in all interactions
- Focus on what is best for the community
- Show empathy towards other community members

## How to Contribute

### Reporting Bugs

Before creating a bug report, please check existing issues to avoid duplicates.

When creating a bug report, include:
- A clear and descriptive title
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Screenshots if applicable
- Your operating system and version
- Application version

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, include:
- A clear and descriptive title
- Detailed description of the proposed feature
- Explanation of why this enhancement would be useful
- Possible implementation approach (if you have ideas)

### Pull Requests

1. Fork the repository
2. Create a new branch from `main`:
   ```bash
   git checkout -b feat/your-feature-name
   # or
   git checkout -b fix/your-bug-fix
   ```
3. Make your changes following the coding standards
4. Test your changes thoroughly
5. Commit your changes using conventional commits:
   ```bash
   git commit -m "feat: add new feature"
   git commit -m "fix: resolve issue with coins"
   ```
6. Push to your fork:
   ```bash
   git push origin feat/your-feature-name
   ```
7. Open a Pull Request

## Development Setup

### Prerequisites

- Node.js 20 or higher
- Rust toolchain (from rustup.rs)
- Platform-specific dependencies (see README.md)

### Setup Steps

1. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/ktcedit.git
   cd ktcedit
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Run in development mode:
   ```bash
   npm run tauri dev
   ```

4. Run type checking:
   ```bash
   npm run check
   ```

## Coding Standards

### TypeScript/Svelte

- Use TypeScript for all new code
- Follow existing code style
- Add type definitions for all functions and variables
- Use Svelte 5 runes API ($state, $derived, $effect)
- Avoid using `any` type

### Rust

- Follow Rust standard formatting (use `cargo fmt`)
- Add documentation comments for public functions
- Handle errors appropriately (use `Result<T, E>`)

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>: <description>

[optional body]

[optional footer]
```

Types:
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `style:` Code style changes (formatting, missing semicolons, etc.)
- `refactor:` Code refactoring
- `perf:` Performance improvements
- `test:` Adding or updating tests
- `chore:` Build process or auxiliary tool changes

Examples:
```
feat: add support for knight spawning

Implements knight spawning functionality similar to other unit types.
Adds template for knight character with proper components.

Closes #42
```

```
fix: correct coin calculation for player 2

The coin calculation was using player 1 index instead of the
selected player index.
```

### Code Review Process

1. All submissions require review
2. Reviewers will check:
   - Code quality and style
   - Test coverage
   - Documentation
   - Performance implications
3. Address review comments
4. Once approved, maintainers will merge

## Testing

### Manual Testing

Before submitting:
1. Test on your platform (macOS, Windows, or Linux)
2. Test with actual Kingdom Two Crowns save files
3. Verify backup creation works
4. Test all modified features

### Automated Testing

Currently, the project relies on:
- TypeScript type checking: `npm run check`
- Rust compilation: `cargo check` in `src-tauri/`

Future: Unit tests and integration tests are planned.

## Translation

To add a new language:

1. Edit `src/lib/i18n.ts`
2. Add your language to the `Language` type
3. Add translations to the `translations` object
4. Update `detectLanguage()` function to detect your language
5. Test all UI elements display correctly

Example:
```typescript
export type Language = 'en' | 'fr' | 'de'; // Added German

const translations: Record<Language, Translations> = {
  // ... existing translations
  de: {
    welcome: {
      title: 'Kingdom Two Crowns Speichereditor',
      // ... rest of translations
    }
  }
};
```

## Documentation

- Update README.md for user-facing changes
- Update BUILD.md for build process changes
- Add inline code comments for complex logic
- Update CHANGELOG.md following Keep a Changelog format

## Release Process

Releases are automatic. When a merge to `main` passes CI, the [release workflow](.github/workflows/build.yml):

1. Picks the next version from the conventional commits since the last `v*` tag:
   - breaking change (`feat!:`, `BREAKING CHANGE`) → major
   - `feat:` → minor
   - anything else → patch
2. Writes it to `package.json`, `package-lock.json`, `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock` and `src-tauri/tauri.conf.json`, then commits `chore(release): vX.Y.Z` and tags it
3. Builds the macOS (universal) and Windows bundles, signs the in-app update artifacts and `latest.json`, and publishes the GitHub release with generated notes

The `release` environment holds the secrets: `RELEASE_DEPLOY_KEY` (pushes the version commit past the `main` ruleset) and `TAURI_SIGNING_PRIVATE_KEY` / `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` (signs updates; the matching public key is in `tauri.conf.json`). Losing the signing key means installed apps can no longer update.

Merges that only change Markdown files do not create a release. To force one, or to choose the bump yourself, run the **Release** workflow manually from the Actions tab. Do not edit the version numbers by hand.

Keep CHANGELOG.md up to date for user-facing changes.

## Questions?

Feel free to open an issue with your question or reach out to maintainers.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
