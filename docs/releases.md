# Desktop releases

Desktop updates are signed and distributed through the shared public Cloudflare R2 bucket. A release tag triggers `.github/workflows/release.yml`, which builds Windows and macOS installers, creates per-platform updater metadata from the signed artifacts and the release notes in `releases/vX.Y.Z.md`, validates the merged metadata, uploads versioned installers and publishes `latest.json` last. The Windows updater artifact is the signed NSIS `*-setup.exe` installer and its `.sig` file; the workflow must not expect a `.nsis.zip` asset. Orbis uses the isolated `orbis/` prefix, with its updater endpoint at `https://updates.mulverse.com/orbis/latest.json` and bundles under `/orbis/releases/vX.Y.Z/`. The public GitHub Release is still created for manually downloading installers; the installed updater checks R2, not GitHub.

After each successful R2 upload, the desktop release workflow retains the three newest Orbis versions under `/orbis/releases/` and removes older Orbis bundles. It never removes `/orbis/latest.json` or objects under another product prefix such as `/aqardesk/`.

The desktop app downloads and verifies an available signed update before offering a restart to apply it. Its title-bar download control appears only while update activity is in progress or when an update needs attention.

If a release build has already created a draft but publication fails during validation, run [Recover Draft GitHub Release](../.github/workflows/recover-release.yml) from the `main` branch and enter the draft's tag. Recovery does not rebuild or replace assets; it repeats the asset, version, updater URL, and signature checks before publishing the existing draft.

Staging builds intentionally do not use the production updater channel. Their update check is disabled so they cannot install production updates.

The mobile release workflow builds and attaches a signed arm64 Android APK to the same GitHub Release. It decodes `ANDROID_KEYSTORE_BASE64` into a temporary runner file and maps the Android signing secrets to the `ORBIS_ANDROID_*` variables consumed by the Expo config plugin. This workflow does not currently publish an AAB. Do not use the debug keystore for public distribution.

The optional [Retry Mulverse Release Sync](../.github/workflows/mulverse.yml) workflow requires the repository Actions variable `MULVERSE_API_BASE_URL` (`https://api.mulverse.com`), the repository variable `MULVERSE_PRODUCT_ID` (the canonical Orbis product UUID), and the repository secret `MULVERSE_RELEASE_API_TOKEN`. The retry validates these settings before downloading signed updater files and never prints the token. These settings are not required to build or publish desktop updates.

The scheduling engine is a separately versioned managed component. Desktop and staging installers do not contain the native Solver binary. On first use, Orbis downloads the platform-specific binary into its app-data directory, verifies its signed manifest and SHA-256 digest, and keeps a verified version available for rollback. Solver binaries are assets of their own `solver-vX.Y.Z` GitHub Release, not assets of the corresponding Orbis desktop release.

## Configure GitHub Actions

Before publishing, configure these repository Actions settings:

| Name | GitHub setting | Purpose |
| --- | --- | --- |
| `TAURI_SIGNING_PRIVATE_KEY` | Secret | Signs Tauri updater bundles |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | Secret | Password for the signing key, if set |
| `TAURI_UPDATER_PUBLIC_KEY` | Variable | Verifies updates in the installed app; must match the private signing key |
| `TAURI_UPDATER_ENDPOINT` | Variable | Set to `https://updates.mulverse.com/orbis/latest.json` |
| `CLOUDFLARE_R2_ACCOUNT_ID` | Variable | Cloudflare account ID used by the S3-compatible R2 API |
| `CLOUDFLARE_R2_BUCKET` | Variable | Shared public updates bucket; use the same bucket as AqarDesk |
| `CLOUDFLARE_R2_ACCESS_KEY_ID` | Secret | R2 S3 access key with Object Read & Write on the shared bucket |
| `CLOUDFLARE_R2_SECRET_ACCESS_KEY` | Secret | Matching R2 S3 secret access key |
| `ORBIS_COMPONENT_SIGNING_PRIVATE_KEY` | Secret | Signs the independently released component manifest |
| `ORBIS_COMPONENT_SIGNING_PRIVATE_KEY_PASSWORD` | Secret | Password for the component signing key, if set |
| `ORBIS_COMPONENT_PUBLIC_KEY` | Variable | Minisign public key matching the component signing secret |
| `ORBIS_SOLVER_MANIFEST_URL` | Variable | HTTPS URL of the stable component manifest |
| `ORBIS_SOLVER_BETA_MANIFEST_URL` | Variable | HTTPS URL of the staging/beta component manifest |
| `MULVERSE_API_BASE_URL` | Variable | HTTPS base URL for the Mulverse API (`https://api.mulverse.com`); required only by optional release sync |
| `MULVERSE_PRODUCT_ID` | Variable | Canonical Orbis product UUID in Mulverse; required only by optional release sync |
| `MULVERSE_RELEASE_API_TOKEN` | Secret | Authorized release-automation token for the Mulverse API; required only by optional release sync |
| `MULVERSE_LICENSE_PUBLIC_KEY_B64` | Variable | Verifies application licenses in release builds |
| `MULVERSE_LICENSE_KEY_ID` | Variable | Identifies the application license key |
| `ANDROID_KEYSTORE_BASE64` | Secret | Base64-encoded Android release keystore |
| `ANDROID_KEY_ALIAS` | Secret | Alias of the release key in the keystore |
| `ANDROID_KEYSTORE_PASSWORD` | Secret | Password for the Android keystore |
| `ANDROID_KEY_PASSWORD` | Secret | Password for the release key |

For the default stable catalog, set `ORBIS_SOLVER_MANIFEST_URL` to `https://raw.githubusercontent.com/MxHabob/Orbis/components/stable/components/orbis-solver.json`. The Solver component workflow updates this signed catalog after validating and publishing its release. Configure the staging variable to a separate HTTPS manifest URL; staging does not use or overwrite the stable catalog.

Keep all private signing material, R2 S3 credentials, and passwords in GitHub Secrets. Do not commit them. The R2 token must be an R2 S3 API token with Object Read & Write permission scoped to the shared updates bucket (not a Cloudflare REST API token or a read-only token). Orbis uploads only under `orbis/`; configure AqarDesk to use its own `aqardesk/` prefix. Ensure the R2 custom domain serves these objects publicly and does not cache `/orbis/latest.json`. The workflow no longer requires Mulverse release API credentials to publish desktop updates.
The desktop release workflow accepts minisign public-key text or its base64-encoded full text, validates it and writes a temporary Tauri config overlay. It fails before building if required settings are missing or malformed; public keys are never logged.

## Prepare a release

Run release commands from an up-to-date `main` checkout with a clean working tree:

```sh
bun run release:prepare patch
```

`release:prepare` updates the synchronized desktop, mobile, and Rust API versions plus Cargo/Bun lockfiles, increments the Android `versionCode`, then generates editable notes in `releases/`. Solver versions are independent and are not changed by desktop releases. Review the notes before continuing; installers, updater metadata, signatures, and release artifacts are built and attached by CI.

```sh
bun run release:commit
git push origin main
git push origin vX.Y.Z
```

`release:commit` validates the notes, commits the release version and notes, and creates an annotated local tag. It does not push. Push the release commit to `main` before pushing its tag; the workflow rejects tags that do not point to a commit reachable from `main`.

The mobile workflow requires the Android signing secrets above and publishes a signed APK. For the initial release when the manifests already contain the desired version, use `bun run release:notes` instead of `release:prepare`, edit `releases/vX.Y.Z.md`, then run `bun run release:commit`. `release:prepare` always increments the current or latest tagged version, so it is not the command for reusing an already-selected initial version.

Before reusing a version tag, verify it is absent both locally and on GitHub. Deleting a GitHub Release does not delete its Git tag; `release:commit` rejects an existing local tag, and GitHub rejects pushing a tag name that still exists remotely. A deleted old tag must be explicitly removed from both places before that exact version can be reused.

## Release the scheduling engine

After validating Solver changes, create and push a separate stable component tag:

```sh
git tag -a solver-vX.Y.Z -m "Orbis Solver X.Y.Z"
git push origin solver-vX.Y.Z
```

`.github/workflows/solver-component-release.yml` runs Solver tests, builds the supported Windows x64 and macOS Apple Silicon native binaries, computes their SHA-256 digests, and signs a versioned manifest. It publishes the component release and then updates the stable manifest catalog. Desktop version bumps do not rebuild or publish Solver.

The supported desktop targets are currently Windows x64 and macOS Apple Silicon. Add a native build and a matching target manifest entry before advertising another platform.
