# Desktop releases

Desktop updates are signed and distributed through public GitHub Releases. A release tag triggers `.github/workflows/release.yml`, which builds Windows and macOS installers, validates the signed Tauri updater metadata, and publishes the GitHub Release. The Windows updater artifact is the signed NSIS `*-setup.exe` installer and its `.sig` file; the workflow must not expect a `.nsis.zip` asset. The Tauri Action generates updater asset URLs using GitHub's Releases Assets API, so release validation verifies each asset ID against the tagged release. The desktop updater checks `https://github.com/MxHabob/Orbis/releases/latest/download/latest.json`; the release must be published for that URL to serve update metadata.

Staging builds intentionally do not use the production updater channel. Their update check is disabled so they cannot install production updates.

Android APK and AAB packages are built and attached to the same GitHub Release. They are signed with a dedicated Android release keystore; do not use the debug keystore for public distribution.

## Configure GitHub Actions

Before publishing, configure these repository Actions settings:

| Name | GitHub setting | Purpose |
| --- | --- | --- |
| `TAURI_SIGNING_PRIVATE_KEY` | Secret | Signs Tauri updater bundles |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | Secret | Password for the signing key, if set |
| `TAURI_UPDATER_PUBLIC_KEY` | Variable | Verifies updates in the installed app; must match the private signing key |
| `MULVERSE_LICENSE_PUBLIC_KEY_B64` | Variable | Verifies application licenses in release builds |
| `MULVERSE_LICENSE_KEY_ID` | Variable | Identifies the application license key |
| `ANDROID_KEYSTORE_BASE64` | Secret | Base64-encoded Android release keystore |
| `ANDROID_KEY_ALIAS` | Secret | Alias of the release key in the keystore |
| `ANDROID_KEYSTORE_PASSWORD` | Secret | Password for the Android keystore |
| `ANDROID_KEY_PASSWORD` | Secret | Password for the release key |

Keep all private signing material and passwords in GitHub Secrets. Do not commit them. The workflow no longer requires Mulverse release API credentials to publish desktop updates.

## Prepare a release

Run release commands from an up-to-date `main` checkout with a clean working tree:

```sh
bun run release:prepare patch
```

`release:prepare` updates the synchronized desktop, mobile, Tauri, solver, and Bun lockfile versions, increments the Android `versionCode`, then generates editable notes in `releases/` and asset folders under `release-assets/`. Review the notes and add any release images or resources before continuing.

```sh
bun run release:commit
git push origin main
git push origin vX.Y.Z
```

`release:commit` validates the notes, commits the release version and assets, and creates an annotated local tag. It does not push. Push the release commit to `main` before pushing its tag; the workflow rejects tags that do not point to a commit reachable from `main`.

The workflow requires the Android signing secrets above and publishes signed APK and AAB assets alongside the desktop release. For the initial release when the manifests already contain the desired version, use `bun run release:notes` instead of `release:prepare`, edit `releases/vX.Y.Z.md`, then run `bun run release:commit`. `release:prepare` always increments the current or latest tagged version, so it is not the command for reusing an already-selected initial version.

Before reusing a version tag, verify it is absent both locally and on GitHub. Deleting a GitHub Release does not delete its Git tag; `release:commit` rejects an existing local tag, and GitHub rejects pushing a tag name that still exists remotely. A deleted old tag must be explicitly removed from both places before that exact version can be reused.
