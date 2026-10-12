<!-- BEGIN:turborepo-agent-rules -->

# This is NOT the Turborepo you know

Turborepo configuration, task behavior, and CLI commands can vary between installed versions and may differ from your training data. Resolve the `turbo` package from this file's directory or relevant workspace; in monorepos, it may not be visible from the repository root. For example, run `node -p "require.resolve('turbo/package.json')"` from a workspace that depends on `turbo`.

Read `docs/README.md` inside that installed package first, then read the relevant pages from its `docs/` directory before changing Turborepo configuration or commands. Heed deprecation notices. These bundled docs match the installed package version and are available without network access.

This block is written and re-added by `turbo` before repository-scoped commands when an AI agent is detected. In the Turborepo source repository, its template is defined in `crates/turborepo-cli/src/cli/agent_guidance.rs`. Removing the managed block while updates are enabled means a later qualifying invocation will add it again. Set `"agentGuidance": false` in the root `turbo.json` or `turbo.jsonc` to opt out; this does not remove an existing block. Keep the block committed with your work to avoid an uncommitted change on the next agent invocation.
<!-- END:turborepo-agent-rules -->

# Orbis Workspace Engineering & Antigravity Guide

This repository is a Turborepo monorepo powered by Bun containing the Orbis school management and learning platform.

## Monorepo Architecture
- **`apps/mobile` (`@orbis/mobile`)**: React Native application powered by Expo SDK 57, Expo Router, NativeWind 5 (RC), Tailwind CSS 4, and React Native Reanimated 4.
- **`apps/desktop`**: Desktop application powered by Svelte / Tauri.
- **`apps/api-rust`**: Backend API server in Rust.
- **`apps/cli`**: CLI administration utility.
- **`packages/*`**: Shared domain SDK, icons (`@orbis/icons`), and utilities.

## Antigravity Customizations
Antigravity automatically discovers project rules and skills in `.agents/`:
- **Project Rules (`.agents/rules/`)**:
  - [`expo-router.md`](file:///d:/projects/Orbis/.agents/rules/expo-router.md): Expo Router navigation, layout ownership, and typed routes.
  - [`mobile-design-system.md`](file:///d:/projects/Orbis/.agents/rules/mobile-design-system.md): NativeWind v5 / Tailwind v4 tokens in `global.css` and shared component library.
  - [`mobile-layout-safety.md`](file:///d:/projects/Orbis/.agents/rules/mobile-layout-safety.md): Safe areas, keyboard avoidance, FloatingTabBar insets, RTL, and accessibility.
  - [`mobile-performance.md`](file:///d:/projects/Orbis/.agents/rules/mobile-performance.md): React Native 0.86, Reanimated 4, FlatList virtualization, and query caching.
  - [`mobile-visual-quality.md`](file:///d:/projects/Orbis/.agents/rules/mobile-visual-quality.md): World-class product design principles; banishing generic AI cards and templates.
  - [`screen-review-checklist.md`](file:///d:/projects/Orbis/.agents/rules/screen-review-checklist.md): Pre-flight verification checklist for screens.
- **Skills (`.agents/skills/`)**:
  - [`mobile-product-design`](file:///d:/projects/Orbis/.agents/skills/mobile-product-design/SKILL.md): Comprehensive product thinking, ergonomic screen architecture, states, and responsive UX.
  - [`mobile-design-intelligence`](file:///d:/projects/Orbis/.agents/skills/mobile-design-intelligence/SKILL.md): Visual exploration, reference benchmarking, multi-direction generation, and visual review workflows.
  - [`react-native-best-practices`](file:///d:/projects/Orbis/.agents/skills/react-native-best-practices/SKILL.md): Low-level performance optimization guidelines.

## Verification & Tooling Reality
- Always run `bun run typecheck` or `npm run typecheck` inside `apps/mobile` to verify TypeScript contracts.
- Run `bun run lint` inside `apps/mobile` to verify code quality.
- Never claim visual or on-device verification without running the app and inspecting screenshots via available preview tools.

