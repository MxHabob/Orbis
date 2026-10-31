# Design

## Context

See `proposal.md` for motivation and `specs/api-server-distribution/spec.md` for observable requirements. The existing tagged release workflow publishes desktop and Android assets. The API reads a protected `server.json`, requires an already-initialized database schema, and does not currently have a release distribution or installer. The Orbis Mobile integration settings already manage customer deployment endpoints, but integration activation itself is a separate operation.

## Goals / Non-Goals

**Goals:**

- Build release assets for Linux x64/ARM64, Windows x64, macOS Apple Silicon, and Docker on Linux x64/ARM64.
- Make native packages and Docker deployment materials discoverable and downloadable from the desktop settings flow, with a native save-location picker.
- Verify downloaded artifacts before presenting them as ready to use.
- Keep server installation, database configuration, and startup under the customer's control.
- Require platform signing for Windows/macOS artifacts and a verifiable release manifest for all downloaded assets.

**Non-Goals:**

- Hosting or remotely provisioning customer infrastructure.
- Installing or starting a service on a customer server from the Orbis desktop application.
- Bundling, creating, migrating, or backing up the customer's database as part of the API package.
- Native macOS Intel support in the initial release.
- Supporting Windows or macOS container hosts as Docker targets; Docker distribution targets Linux containers on x64/ARM64.

## Decisions

### Build platform-specific native release archives and persistent service definitions

Build the API executable for Linux x64 (`x86_64-unknown-linux-gnu`), Linux ARM64 (`aarch64-unknown-linux-gnu`), Windows x64 (`x86_64-pc-windows-msvc`), and macOS Apple Silicon (`aarch64-apple-darwin`). Package each executable with platform-specific setup materials: a least-privilege `systemd` unit and installer/uninstaller for Linux; a signed Windows executable and service-wrapper configuration/scripts; and a signed, notarized macOS executable with a `launchd` plist and setup/removal scripts.

Keep the API binary independent from customer configuration. Installers create service definitions and protected configuration-file locations, but require the administrator to supply the database URL and explicitly start the service. Linux uses the existing `/etc/orbis/server.json` default; Windows uses `%PROGRAMDATA%\\Orbis\\server.json`; macOS uses `/Library/Application Support/Orbis/server.json`. Document that the database must already be available and migrated before the service starts.

Native installers were preferred to a universal executable because service management and trust prompts differ by operating system. The first release intentionally excludes macOS Intel to limit signing and CI matrix costs until demand is established.

### Publish a versioned multi-architecture Linux image to GHCR

Build one API image for `linux/amd64` and `linux/arm64` with Docker Buildx and publish a version-immutable tag for each application release. Include a Compose bundle that references that versioned image, persists only required API state, and accepts a customer-provided configuration/secrets file. Do not include a database container or publish database ports. Run the API as a non-root user and expose deployment behind a customer-managed HTTPS reverse proxy.

GHCR was selected to keep image publication next to the existing GitHub release workflow and avoid operating another registry. Compose is the single-host deployment interface; Kubernetes and automated host provisioning are out of scope.

### Make API release assets a required release gate

Add native and Docker build jobs to the tagged release workflow. The native job uses a target matrix and stages archives; Windows binaries are Authenticode-signed and macOS binaries are Developer ID-signed and notarized. The Docker job publishes the versioned multi-architecture image. A finalization step generates a manifest containing version, target, artifact URL, and SHA-256 for each native archive and Docker bundle, then signs the manifest with a dedicated release signing key. The desktop application pins the corresponding public key and verifies the manifest signature and artifact hash before accepting a download.

The final release publication depends on successful build, signing, notarization, image publication, and artifact validation for every required target. Required signing credentials are configured as protected CI secrets; missing credentials or failed signing checks fail the release rather than silently publishing unsigned Windows/macOS assets. Checksums alone were rejected as the trust mechanism because an attacker able to replace both an artifact and its checksum could defeat them.

### Keep download and server setup separate from mobile integration activation

Add a distinct API distribution section to the Orbis Mobile deployment settings. It retrieves the signed release manifest, presents supported native and Docker targets with release versions, and uses the desktop's native save dialog to download the selected archive or Compose bundle. Show integrity verification, progress, and explicit error states. Do not download during app activation and do not issue commands to a remote host.

The initial delivery UX supplies an archive/bundle and setup guidance rather than executing privileged installation on the administrator's workstation. Endpoint registration remains a later, separate action after the customer has configured and started the API.

### Release signing and key lifecycle

Store Windows and Apple signing material only in protected CI secrets and scope it to release jobs. Keep the manifest signing private key separate from platform certificates; embed only its public verification key in the desktop app. Document key rotation as a versioned public-key trust update shipped with the desktop application. Never place customer database credentials or private keys in release assets, logs, or image layers.

## Risks / Trade-offs

- [Platform signing credentials increase setup cost and can expire] → Provision and validate the certificates before the first release that includes native Windows/macOS packages; alert on expiry and fail closed.
- [A release signing key compromise could authenticate malicious assets] → Restrict access to protected release workflows, keep the signing key separate from source, verify the signature before download acceptance, and document key rotation.
- [Cross-compiled Linux ARM64 packages may fail due to native dependencies or ABI differences] → Build/test on native or compatible ARM64 runners and validate artifacts in target-architecture containers before publishing.
- [Customer may expose the API without TLS or expose the database publicly] → Installation guidance requires HTTPS at the public boundary, firewalling, and a private database endpoint; endpoint setup rejects non-HTTPS public URLs.
- [Service installers require elevated privileges and can be misused] → The desktop downloads packages only; the customer explicitly reviews and runs platform instructions with their own administrator privileges.
- [GHCR availability or permissions can interrupt image pulls] → Validate the published image digest and document registry access requirements; keep native release assets independently downloadable.
- [A schema migration mismatch prevents startup] → Preserve the API's existing fail-fast schema checks and document applying migrations before enabling the service.

## Migration Plan

1. Provision protected platform-signing and manifest-signing secrets and validate their presence in a release preflight.
2. Add target builds, packaging, Docker publication, and signed-manifest generation to the tagged release pipeline; keep publication blocked until every required target passes.
3. Add the separate distribution download and verification flow to desktop Orbis Mobile settings without changing integration activation behavior.
4. Publish the first release with native archives, Compose bundle, versioned GHCR image, checksums, and signed manifest; verify a clean install and rollback on each target.
5. To roll back a customer deployment, stop the service/container and install the previous versioned package/image while preserving the separately stored customer configuration and database.

No automatic customer server upgrade is introduced. Customers choose when to replace their installed version.

## Open Questions

None. The target set and signed-native-package policy are fixed for this change.
