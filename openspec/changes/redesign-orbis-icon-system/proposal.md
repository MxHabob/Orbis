# Proposal

## Why

The current `@orbis/icons` package exposes 206 icons to both Svelte and React Native, but their visual construction, naming, and API consistency are difficult to maintain across two separately authored implementations. A coherent, original Orbis icon system will improve recognition, legibility, and polish across desktop and mobile without copying Sentry's artwork or disrupting existing consumers.

## What Changes

- Establish a documented visual language for icons: consistent geometry, optical alignment, stroke, corner treatment, detail density, outline/filled semantics, and alignment with familiar user expectations.
- Audit and redesign the icon catalog by semantic family, prioritizing navigation, actions, status, education, and settings icons that appear most often.
- Align Svelte and React Native icon geometry and behavior while preserving the existing public component names and platform entry points.
- Standardize size, color, stroke, accessibility, and prop-forwarding behavior; clarify intentional filled variants and the Orbis brand mark.
- Validate recognition, labels, RTL behavior, contrast, focus, and interactive target sizes in context so visual polish does not reduce usability or accessibility.
- Add repeatable quality checks and a visual review catalog so future icons follow the same rules and regressions are caught before release.
- Migrate desktop and mobile usages in controlled batches, consolidating duplicate or ambiguous symbols where safe and retaining compatibility aliases where needed.

## Capabilities

### New Capabilities

- `icon-system`: A consistent, accessible, cross-platform icon language and stable component library for Orbis.

### Modified Capabilities

None.

## Impact

- `packages/icons/src/svelte/`, `packages/icons/src/native/`, package entry points, and icon documentation.
- Desktop Svelte consumers across `apps/desktop`; React Native consumers across `apps/mobile`.
- Package checks and visual-regression tooling. The plan favors existing workspace tooling and adds dependencies only if a demonstrated gap requires them.
- Existing named exports, import paths, and platform APIs remain compatible during migration; any unavoidable API break requires a separate explicit migration decision.
