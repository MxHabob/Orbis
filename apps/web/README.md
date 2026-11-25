# Orbis Web

This workspace owns the SvelteKit interface, routes, translations, and web
assets. It runs in a browser or inside the [Tauri host](../desktop/).

- `bun run dev:web` starts Vite at `http://127.0.0.1:4173`.
- `bun run build:web` writes the static application to `apps/web/build`.
- `bun run --cwd apps/web check` checks Svelte and TypeScript.

Configure the API URL in `.env` using `.env.example`. Browser deployments should
serve `index.html` for client routes and follow the
[API origin configuration](../api-rust/deploy/README.md#client-connections).
