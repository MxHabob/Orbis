# Proposal

## Why

Orbis Mobile requires an API endpoint that remains available, but customers currently have no release workflow for obtaining and running the API on infrastructure they operate. A supported multi-platform distribution lets customers host the service themselves without requiring Orbis to pay for or operate their servers.

## What Changes

- Add CI release builds for native API server packages targeting Linux x64 and ARM64, Windows x64, and macOS Apple Silicon.
- Publish a multi-architecture Linux Docker image and Compose deployment bundle alongside the existing desktop and mobile release assets.
- Provide platform-specific setup materials for running the API as a persistent service, configuring the existing database connection, and safely verifying downloaded artifacts.
- Add a separate API setup/download flow to Orbis Mobile settings. Enabling the mobile integration must not implicitly download, install, or remotely start a server.
- Let administrators select their server deployment target and choose where to save the matching release package; provide Docker as a deployment option.

## Capabilities

### New Capabilities

- `api-server-distribution`: Build, publish, verify, download, and prepare customer-operated API server packages across supported platforms and Docker.

### Modified Capabilities

None.

## Impact

- GitHub Actions release workflow and GitHub Release assets.
- Rust API server build configuration and platform-specific packaging/service setup.
- Orbis Mobile integration settings in the desktop app and localized messages.
- Artifact checksums/signatures, download UX, and customer-facing deployment documentation.
- Assumption for this proposal: native targets are Linux x64/ARM64, Windows x64, and macOS Apple Silicon; Docker targets Linux x64/ARM64. Native macOS Intel is not included initially.
