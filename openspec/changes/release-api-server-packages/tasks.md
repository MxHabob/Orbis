# Tasks

## 1. Native API Packages

- [ ] 1.1 Add release target builds and versioned archive staging for Linux x64/ARM64, Windows x64, and macOS Apple Silicon; verify each package contains the expected executable and version metadata.
- [ ] 1.2 Add least-privilege Linux systemd install/remove assets and protected server configuration setup; verify service files and installer behavior with Linux package smoke tests.
- [ ] 1.3 Add Windows service-wrapper configuration and PowerShell install/remove scripts; verify service registration, configuration path, and removal using Windows CI.
- [ ] 1.4 Add macOS launchd setup/removal assets and signed executable packaging; verify plist validity and package startup/teardown on Apple Silicon CI.
- [ ] 1.5 Document native prerequisites, customer database configuration, schema/migration requirement, TLS/firewall guidance, and manual rollback; verify commands and paths against the generated packages.

## 2. Docker Distribution

- [ ] 2.1 Add a minimal non-root API Docker image build for Linux AMD64 and ARM64; verify both architecture manifests and container startup in CI.
- [ ] 2.2 Add a version-pinned Docker Compose bundle with customer-provided configuration/secrets and no database service or public database port; verify it with `docker compose config` and a container smoke test.
- [ ] 2.3 Document Docker prerequisites, secure secret/configuration handling, HTTPS reverse-proxy requirements, upgrades, and rollback; verify the documented Compose invocation works with the staged bundle.

## 3. Signed Release Metadata and Gates

- [ ] 3.1 Add protected CI preflight checks for Windows Authenticode, Apple Developer ID/notarization, and manifest-signing credentials; verify missing or expired signing configuration blocks publication.
- [ ] 3.2 Generate a versioned manifest with target URLs, checksums, and Docker image digest, then sign it with the release key; verify signature and hashes in CI and reject tampered fixtures.
- [ ] 3.3 Integrate native assets, Docker image publication, manifest signing, and target validation as required dependencies of release publication; verify a failed target prevents a release from being marked complete.
- [ ] 3.4 Document protected signing-secret setup and manifest-key rotation; verify the documented key/public-key update procedure matches the release workflow.

## 4. Orbis Mobile Server Setup UX

- [ ] 4.1 Add a server distribution section separate from Orbis Mobile activation and endpoint registration, with supported targets and release versions; verify activation alone triggers no download or remote operation.
- [ ] 4.2 Implement signed-manifest retrieval, target-specific artifact selection, native save-location selection, download progress, and explicit error states; verify successful, cancelled, and failed downloads.
- [ ] 4.3 Verify the manifest signature and downloaded artifact checksum before showing a package as ready; verify invalid signatures and mismatched hashes are rejected.
- [ ] 4.4 Add Arabic and English localization and user-facing package/setup guidance; verify locale key parity and desktop typecheck/lint.

## 5. End-to-End Release Verification

- [ ] 5.1 Run the complete release validation against staged artifacts and verify each native target, Docker architecture, signature, manifest entry, and save/download path is represented exactly once.
- [ ] 5.2 Perform clean-install and previous-version rollback smoke tests for supported native services and Docker while preserving customer configuration and database state; record verified steps in deployment documentation.
