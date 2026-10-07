# Orbis icon system

The icon package ships paired Svelte and React Native components. Existing names
and package entry points are stable; visual updates are made to both
implementations together.

## Current catalog

The inventory and maintained manifest cover 206 public icon names and 206
components per platform. The inventory records source files, viewBox, default
properties, primitives, and desktop/mobile import frequency. The manifest
classifies each name by semantic family, variant, and compatibility kind, and
records the six native default aliases separately.

The most-used desktop imports in the current inventory are `Plus` (51),
`Refresh` (49), `AdjustmentsHorizontal` (45), `Search` (43), and
`CheckCircle`/`InfoCircle` (42 each), followed by `ArrowPath` (41),
`Sparkles` (39), `Trash` (36), and `XMark` (34). The mobile app imports 25
distinct icons; all 27 rendered icon instances set an explicit size. Static
import counts do not reveal whether every icon-only control has an accessible
name, so interactive controls still require in-context review.

Redesign batches are prioritized as navigation/actions, status/feedback and
account/security, education/organization, then records/data and remaining
utilities; this keeps high-frequency shared symbols ahead of low-usage
families. Counts come from the reproducible audit command below.

Representative icon-only control checks found two useful accessibility
patterns: the desktop app sidebar places a 20px navigation glyph in a button
with a localized `aria-label` and a visible-on-hover tooltip; mobile's shared
`IconButton` requires an `accessibilityLabel` and expands targets smaller than
44 points with hit slop. The underlying interactive component—not the glyph—
owns the label and target size. These focused examples are not an exhaustive
audit of every importing control; the rest of the consumers still need review
in their family rollout.

The Svelte API uses SVG attributes, `class`/`className`, optional `size`, and
`strokeWidth`; its default rendered size is 16px. Native components use
React-Native-SVG props and `style`, with `size`, `color`, and `strokeWidth`;
the old default was 20pt (with a small set of 24pt exceptions). Both platforms
default to `currentColor` and decorative accessibility behavior.

Use size by visual hierarchy rather than applying one size globally: 12–14px
is reserved for dense inline metadata, 16px is the standard control glyph,
18–20px is for prominent navigation/toolbars, and 24px is for prominent
states. The desktop inventory has 1,187 rendered icon instances: 889 specify a
size and 298 rely on the component default. Its many 12–14px instances must be
reviewed in context; they should not be bulk-enlarged without checking row
height, alignment, and legibility.

## Baseline checks

Before the drawing/default changes, the inventory found 206 exports and source
files on each platform with no missing pairs or viewBox mismatches. The
following checks passed:

```sh
bun packages/icons/scripts/audit-icons.mjs --output packages/icons/docs/icon-inventory.json
bun packages/icons/scripts/audit-icons.mjs --output packages/icons/docs/icon-inventory.json --manifest-output packages/icons/docs/icon-manifest.json
bun run --cwd packages/icons audit
bun run --cwd packages/icons gallery
bun run --cwd packages/icons test
bunx tsc --noEmit -p packages/icons/tsconfig.json
bun run --cwd apps/mobile typecheck
bun run --cwd apps/desktop build
```

The desktop build completed successfully. Baseline screenshots and bundle-size
measurements were not captured; do not use the build result as a substitute for
either visual or bundle-size comparison.

The generated [icon gallery](./icon-gallery.html) renders all 206 exported
symbols at 16, 20, and 24px on light and dark surfaces. Rebuild it with
`bun run --cwd packages/icons gallery`; use the family filter and name search to
review related concepts together. The catalog was checked in-browser at 206
cards (six previews per symbol) and family filtering was verified. For React
Native, compare the same names in the existing Expo app and its rendered
consumer controls; the static gallery does not claim to replace a native-device
rendering review.

The inherited cap/join defaults now use rounded joins on all 203 non-brand icon
components per platform. Individual paths may override these defaults where
their depicted form requires a different finish.

## Design and review

See [the drawing guidelines](./icon-guidelines.md) and
[the initial paired pilot](./icon-pilot.json). Validate icons at 16, 20, and 24
against light and dark surfaces, then review them in the real control that uses
them. Directional icons should only be mirrored by consumers when they express
logical navigation; physical directions and fixed spatial relationships remain
unchanged.

The first redraw batch now covers the paired `ArrowLeft`, `Plus`, `CheckCircle`,
`BookOpen`, `AdjustmentsHorizontal`, and `Pencil`/`PencilSolid` pilot, as well
as high-frequency `Sparkles`/`SparklesSolid` and `UserBadgeCheck`. Each changed
glyph has matched Svelte and native geometry. The 16/20/24 light/dark and
in-context visual review is still pending.

The current focused redesign also updates `GlobeAltSolid`,
`DocumentArrowUpSolid`, and `Grid` in both implementations. The two `Solid`
icons preserve filled semantics with negative-space details; `Grid` uses four
rounded cells, with diagonally opposed taller panels matching the dashboard
reference. All three retain the 16px default and 1.75 nominal stroke contract.

The current high-frequency batch redraws `Refresh`, `ArrowPath`, `Trash`,
`InfoCircle`, `XMark`, `Users`, and `DocumentPage` in both implementations.
Their consumer size overrides remain intact; the paired paths, rather than a
blanket size change, carry the visual update.
