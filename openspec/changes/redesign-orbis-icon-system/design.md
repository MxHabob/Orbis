# Design

## Context

See `proposal.md` for motivation and `specs/icon-system/spec.md` for the behavioral contract. The package currently has 206 paired icon components under `src/svelte/` and `src/native/`. The Svelte and React Native implementations are separately authored; their default sizes and property types differ, and the catalog includes multiple filled/outline conventions. Desktop has hundreds of icon import sites, while mobile uses the native entry point.

## Goals / Non-Goals

**Goals:**

- Establish a distinct Orbis visual language informed by the reference's legibility and consistency, without reproducing Sentry icon shapes or identity.
- Keep web and native symbols recognizably equivalent and make consumer-facing defaults predictable.
- Redesign the complete maintained catalog in reviewed, reversible batches.
- Preserve existing public import paths and names while removing internal visual duplication and documenting intentional variants.
- Make the catalog, compatibility, and cross-platform review practical for future contributors.

**Non-Goals:**

- Replacing icons with a third-party icon package or copying another product's artwork.
- Rebuilding navigation, button, or other consumer components solely to apply icon styling.
- Introducing icon animation, multicolor decoration, or per-icon runtime configuration as a general design requirement.
- Changing consumer imports en masse where existing imports remain clear and compatible.

## Decisions

### 1. Establish measurable drawing and sizing rules before redrawing the catalog

Use a 24-by-24 coordinate canvas for interface icons, with optical rather than mechanically identical alignment. Define standard preview/render sizes at 16, 20, and 24 CSS pixels/points. Adopt 16 as the common default because it matches the current Svelte default; review and adjust native consumers that relied on the current 20-point default before aligning that default. Use a shared nominal outline weight (target 1.75 at 24 units), rounded joins/caps where semantically appropriate, and explicit documented exceptions for dense or solid symbols. Keep the Orbis logo outside the generic grid/stroke rules.

**Alternative considered:** Enforce one stroke width and identical geometry on every symbol. Rejected because optical correction and filled/complex pictograms need controlled exceptions for clarity.

### 2. Keep platform components independently authored for this change

Retain Svelte SVG components and React Native SVG components as the shipped implementation. Define their shared contract in documentation and test their icon-name inventory, default properties, and visual appearance together. Do not add a generator or SVG parsing tool in this redesign: it would create a second transformation toolchain for the current mixed catalog and expand release risk. Reconsider code generation only if catalog parity checks show recurring drift that cannot be controlled with review and fixtures.

**Alternative considered:** Make standalone SVG files the source of truth and generate both component targets. Deferred because it adds generation/dependency and build/debugging complexity before proving that the simpler parity workflow is insufficient.

### 3. Preserve API compatibility and use canonical symbols internally

Retain the root, `./svelte`, and `./native` package entry points and all current named exports. Create a catalog manifest classifying canonical icons, intentional paired variants, brand assets, and compatibility aliases. Redirect duplicate legacy exports to a canonical design where appropriate; add a deprecation note rather than deleting or renaming a public export. Standardize documented `size`, color, stroke-width, class/style, SVG/prop forwarding, and accessibility behavior with platform-appropriate types. Align default size semantics, but keep platform-native color/type semantics where the frameworks differ.

**Alternative considered:** Rename the catalog to a new naming scheme and migrate every consumer immediately. Rejected because it introduces breaking imports and broad unrelated churn without improving the visual result.

### 4. Use an original, minimal Orbis language with deliberately crafted filled variants

Design a restrained, original Orbis family with simple silhouettes, intentional negative space, and low detail at small sizes. Redraws must be visibly authored for Orbis rather than merely restyled or mechanically normalized copies of stock glyphs. Filled variants receive equal design attention: use a clear outer silhouette, deliberate negative-space cutouts, balanced optical weight, and small-size legibility instead of applying a solid fill to an outline path. Use filled silhouettes for deliberate semantic hierarchy or state, not indiscriminately; a suffix such as `Solid` is reserved for an intentional, documented pair whose filled form remains recognizable as the same concept. Consolidate duplicate files through compatibility aliases rather than maintaining visibly divergent copies. Apply semantic families (navigation, actions, status, education, records/data, people, settings, and brand) as review batches. Sentry is a quality reference for clarity and sidebar consistency only; each Orbis glyph must be independently authored.

**Alternative considered:** Use one treatment everywhere or make each solid variant by filling its outline counterpart. Rejected because some meanings need a filled silhouette, while simple fill conversion often closes essential counters and makes the symbol heavy or unclear at small sizes.

### 5. Add a complete visual catalog and lightweight quality gates

Provide a review gallery for Svelte with names, semantic family, variants, and 16/20/24 previews in light/dark themes. Provide an equivalent native review surface or documented screenshot procedure using the existing Expo app. Add a deterministic script/test that checks paired filename/export coverage, manifest classifications, alias targets, and standard component defaults. Record approved baseline screenshots for high-visibility and complex icons; review the full catalog in family batches rather than requiring brittle pixel-perfect snapshots for every platform on every CI run. Validate focused screen-reader semantics and color inheritance with representative tests.

**Alternative considered:** Add a new visual testing framework immediately. Rejected unless current tooling cannot produce reliable gallery/screenshot review; first reuse workspace and platform tooling.

### 6. Roll out by frequency and risk, not by filesystem order

Before editing glyphs, generate a usage inventory from desktop and mobile imports and establish baseline package/app build results and bundle sizes. Review shared high-frequency navigation/action/status icons first, then education and data families, then low-frequency/brand/complex assets. Keep each batch independently reviewable and buildable. If a batch causes a visual or API regression, revert that batch without backing out already approved families.

### 7. Optimize for recognition and task completion, not novelty

Prefer familiar platform conventions for common actions and states. Use custom Orbis symbols only where they improve recognition or represent domain concepts that lack a clear convention. Avoid ornamental detail, ambiguous metaphors, and icons that require users to memorize a private symbol language. Review icons in their consuming control and surrounding layout, not only as isolated artwork. For ambiguous actions, require a visible label or a discoverable tooltip/contextual label; icon-only destructive or consequential actions require a clear accessible name and confirmation or explanatory context as appropriate. Evaluate the pilot with representative users or a documented task-based review, record misinterpretations, and revise repeatedly misunderstood symbols.

**Alternative considered:** Favor novel, distinctive glyphs throughout the interface to maximize brand uniqueness. Rejected because visual distinctiveness that slows recognition increases cognitive load and interaction errors.

### 8. Treat accessibility and target size as interface-level acceptance criteria

Check meaningful icon graphics and selected/focused indicators against WCAG 2.2 AA non-text contrast (3:1 where Success Criterion 1.4.11 applies); never use color as the sole state cue. Check focus visibility and screen-reader names in the consuming controls. The icon glyph's drawing size is independent of its control's hit area: web targets must satisfy WCAG 2.2 AA 2.5.8 (24 by 24 CSS pixels or a permitted exception), while native controls follow the platform's touch-target guidance. Test RTL navigation icons semantically: mirror logical previous/next or back/forward presentation where appropriate, but do not mirror absolute directions or fixed spatial data.

**Alternative considered:** Encode target size, labels, and mirroring behavior inside every icon component. Rejected because these concerns belong to the control and usage context; embedding them in a glyph would couple symbols to unrelated layout and semantic decisions.

## Risks / Trade-offs

- [A common default size change can make native controls feel smaller] → Inventory actual call sites first; adjust affected mobile call sites explicitly and verify screenshots before changing the default.
- [A global stroke rule can make dense pictograms muddy] → Use the rule as a nominal target and require documented optical exceptions with 16-pixel review.
- [Manual implementations can drift between Svelte and React Native] → Maintain paired manifest checks, compare the two galleries in the same review, and add regression fixtures for each family.
- [Compatibility aliases can preserve confusing names indefinitely] → Mark aliases in the manifest, route them to canonical designs, and prohibit adding new aliases without a migration reason.
- [Visual cleanup can accidentally disturb accessibility or color inheritance] → Keep icons decorative by default, support explicit accessible names, inherit current color by default, and validate named/icon-only actions at consumer level.
- [An icon may look consistent in isolation but remain ambiguous or inaccessible in its actual control] → Review representative contexts, use task-based recognition review, check contrast and keyboard focus, and audit target size and RTL behavior at consumer level.
- [Redrawing 206 icons is substantial and subjective] → Prioritize by measured usage, define family acceptance criteria, and require design review at each batch boundary.

## Migration Plan

1. Inventory the catalog, exports, variants, consumer frequency, actual default sizes, and baseline build/bundle behavior.
2. Approve the visual rules using a representative pilot spanning navigation, actions, status, dense education pictograms, and a solid variant; validate both platforms at all standard sizes and themes.
3. Create the manifest, review galleries, parity checks, and compatibility/alias rules; verify these against the pilot before scaling.
4. Redraw and review the catalog by semantic family. Keep old export names and import paths resolvable throughout.
5. Audit and tune mobile usages affected by a default-size alignment. Update consumer usages only for accessibility names, inappropriate semantic choices, or explicit size corrections.
6. Run package checks, desktop production build, mobile typecheck/build, cross-platform visual review, accessibility checks, and tree-shaking/bundle comparison.
7. Roll back an individual family batch if acceptance checks fail. Do not remove compatibility exports in this change; any later breaking cleanup requires a separately versioned migration.

## Open Questions

None. The initial design assumes a 16-unit common default, a nominal 1.75 stroke target, separately authored platform components, and no copied Sentry artwork; the pilot is the acceptance gate for those visual rules before they are applied to the full catalog.

## Standards References

- [WCAG 2.2 - Non-text Contrast (1.4.11)](https://www.w3.org/WAI/WCAG22/Understanding/non-text-contrast.html)
- [WCAG 2.2 - Target Size (Minimum) (2.5.8)](https://www.w3.org/WAI/WCAG22/Understanding/target-size-minimum.html)
- [Apple Human Interface Guidelines - Icons](https://developer.apple.com/design/human-interface-guidelines/icons)
- [Material Design 3 - Icons](https://m3.material.io/styles/icons/overview)
