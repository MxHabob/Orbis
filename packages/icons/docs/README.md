# Orbis icon system

The icon package ships paired Svelte and React Native components. Existing names
and package entry points are stable; visual updates are made to both
implementations together.

## Current catalog

The baseline inventory covers 206 public icon names and 206 components per
platform. It records the source files, viewBox, default properties, primitives,
and desktop/mobile import frequency.

The most-used desktop imports in the baseline were `Plus` (51), `Refresh` (48),
`AdjustmentsHorizontal` (45), `Search` (43), and `InfoCircle` (42). The mobile
app imported 25 distinct icons; all 27 rendered icon instances in the baseline
set an explicit size. Static import counts do not reveal whether every
icon-only control has an accessible name, so interactive controls still require
in-context review.

The Svelte API uses SVG attributes, `class`/`className`, optional `size`, and
`strokeWidth`; its default rendered size is 16px. Native components use
React-Native-SVG props and `style`, with `size`, `color`, and `strokeWidth`;
the old default was 20pt (with a small set of 24pt exceptions). Both platforms
default to `currentColor` and decorative accessibility behavior.

## Baseline checks

Before the drawing/default changes, the inventory found 206 exports and source
files on each platform with no missing pairs or viewBox mismatches. The
following checks passed:

```sh
bun packages/icons/scripts/audit-icons.mjs --output packages/icons/docs/icon-inventory.json
bunx tsc --noEmit -p packages/icons/tsconfig.json
bun run --cwd apps/mobile typecheck
bun run --cwd apps/desktop build
```

The desktop build completed successfully. Baseline screenshots and bundle-size
measurements were not captured; do not use the build result as a substitute for
either visual or bundle-size comparison.

## Design and review

See [the drawing guidelines](./icon-guidelines.md) and
[the initial paired pilot](./icon-pilot.json). Validate icons at 16, 20, and 24
against light and dark surfaces, then review them in the real control that uses
them. Directional icons should only be mirrored by consumers when they express
logical navigation; physical directions and fixed spatial relationships remain
unchanged.
