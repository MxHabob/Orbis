# Desktop releases

Desktop updates are signed and distributed through public GitHub Releases. A release tag triggers `.github/workflows/release.yml`, which builds Windows and macOS installers, validates the signed Tauri updater metadata, and publishes the GitHub Release. The Tauri updater endpoint is configured through `TAURI_UPDATER_ENDPOINT`.

The desktop app downloads and verifies an available signed update before offering a restart to apply it. Its title-bar download control appears only while update activity is in progress or when an update needs attention.

Android APK and AAB packages are built and attached to the same GitHub Release. They are signed with a dedicated Android release keystore; do not use the debug keystore for public distribution.

The scheduling engine is a separately versioned managed component. Desktop and staging installers do not contain the native Solver binary. On first use, Orbis downloads the platform-specific binary into its app-data directory, verifies its signed manifest and SHA-256 digest, and keeps a verified version available for rollback. Solver binaries are assets of their own `solver-vX.Y.Z` GitHub Release, not assets of the corresponding Orbis desktop release.

## Configure GitHub Actions

Before publishing, configure these repository Actions settings:

| Name | GitHub setting | Purpose |
| --- | --- | --- |
| `TAURI_SIGNING_PRIVATE_KEY` | Secret | Signs Tauri updater bundles |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | Secret | Password for the signing key, if set |
| `TAURI_UPDATER_PUBLIC_KEY` | Variable | Verifies updates in the installed app; must match the private signing key |
| `TAURI_UPDATER_ENDPOINT` | Variable | HTTPS URL of the Tauri `latest.json` metadata |
| `ORBIS_COMPONENT_SIGNING_PRIVATE_KEY` | Secret | Signs the independently released component manifest |
| `ORBIS_COMPONENT_SIGNING_PRIVATE_KEY_PASSWORD` | Secret | Password for the component signing key, if set |
| `ORBIS_COMPONENT_PUBLIC_KEY` | Variable | Minisign public key matching the component signing secret |
| `ORBIS_SOLVER_MANIFEST_URL` | Variable | HTTPS URL of the stable component manifest |
| `ORBIS_SOLVER_BETA_MANIFEST_URL` | Variable | HTTPS URL of the staging/beta component manifest |
| `MULVERSE_LICENSE_PUBLIC_KEY_B64` | Variable | Verifies application licenses in release builds |
| `MULVERSE_LICENSE_KEY_ID` | Variable | Identifies the application license key |
| `ANDROID_KEYSTORE_BASE64` | Secret | Base64-encoded Android release keystore |
| `ANDROID_KEY_ALIAS` | Secret | Alias of the release key in the keystore |
| `ANDROID_KEYSTORE_PASSWORD` | Secret | Password for the Android keystore |
| `ANDROID_KEY_PASSWORD` | Secret | Password for the release key |

For the default stable catalog, set `ORBIS_SOLVER_MANIFEST_URL` to `https://raw.githubusercontent.com/MxHabob/Orbis/components/stable/components/orbis-solver.json`. The Solver component workflow updates this signed catalog after validating and publishing its release. Configure the staging variable to a separate HTTPS manifest URL; staging does not use or overwrite the stable catalog.

Keep all private signing material and passwords in GitHub Secrets. Do not commit them. Desktop builds validate the updater endpoint and both public keys before packaging.

## Prepare a release

Run release commands from an up-to-date `main` checkout with a clean working tree:

```sh
bun run release:prepare patch
```

`release:prepare` updates the synchronized desktop, mobile, Tauri, and Bun lockfile versions, increments the Android `versionCode`, then generates editable notes in `releases/` and asset folders under `release-assets/`. Solver versions are independent and are not changed by desktop releases.

```sh
bun run release:commit
git push origin main
git push origin vX.Y.Z
```

`release:commit` validates the notes, commits the release version and assets, and creates an annotated local tag. It does not push. Push the release commit to `main` before pushing its tag; the workflow rejects tags that do not point to a commit reachable from `main`.

The workflow requires the Android signing secrets above and publishes signed APK and AAB assets alongside the desktop release. For the initial release when the manifests already contain the desired version, use `bun run release:notes` instead of `release:prepare`, edit `releases/vX.Y.Z.md`, then run `bun run release:commit`. `release:prepare` always increments the current or latest tagged version, so it is not the command for reusing an already-selected initial version.

Before reusing a version tag, verify it is absent both locally and on GitHub. Deleting a GitHub Release does not delete its Git tag; `release:commit` rejects an existing local tag, and GitHub rejects pushing a tag name that still exists remotely. A deleted old tag must be explicitly removed from both places before that exact version can be reused.

## Release the scheduling engine

After validating Solver changes, create and push a separate stable component tag:

```sh
git tag -a solver-vX.Y.Z -m "Orbis Solver X.Y.Z"
git push origin solver-vX.Y.Z
```

`.github/workflows/solver-component-release.yml` runs Solver tests, builds the supported Windows x64 and macOS Apple Silicon native binaries, computes their SHA-256 digests, and signs a versioned manifest. It publishes the component release and then updates the stable manifest catalog. Desktop version bumps do not rebuild or publish Solver.

The supported desktop targets are currently Windows x64 and macOS Apple Silicon. Add a native build and a matching target manifest entry before advertising another platform.
