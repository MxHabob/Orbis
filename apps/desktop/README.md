# Orbis Desktop

This workspace owns the Tauri host, native integrations, and installer tooling.
It loads the [web workspace](../web/) during development and bundles its static
build for release.

Run `bun run dev:desktop` from the repository root to start Tauri and its Vite
server. Run `bun run build:desktop:prod` to build the production installer.

Desktop installer, updater, and independently managed Solver component release configuration is documented in [docs/releases.md](../../docs/releases.md).
