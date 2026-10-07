# Tasks

## 1. Catalog and Baseline

- [x] 1.1 Inventory every Svelte and React Native icon, export name, viewBox, default size/color/stroke, SVG primitives, and existing variant; verify the report accounts for all package exports and both platform directories.
- [x] 1.2 Measure icon import frequency across `apps/desktop` and `apps/mobile`, identify icon-only interactive controls, and rank redesign batches; verify the report names the most-used symbols and representative accessibility risks.
- [ ] 1.3 Capture current package typecheck/build results, representative desktop/mobile screenshots, and bundle-size baselines; verify the baseline can be reproduced before icon files change.
- [x] 1.4 Document the inventory findings, usage ranking, API differences, and baseline results in the icon-package documentation; verify the report links each finding to a reproducible check or artifact.

## 2. Visual Language and Pilot

- [x] 2.1 Write the Orbis icon guidelines covering 24-unit canvas, 16/20/24 previews, common 16-unit default, nominal 1.75 stroke, optical alignment, line/corner treatment, detail limits, color inheritance, familiar metaphors, RTL direction classes, original filled silhouettes/counters, intentional variants, and logo exceptions; verify each rule is illustrated with examples and exceptions.
- [x] 2.2 Select a high-frequency pilot spanning navigation, action, status, education, a dense symbol, and an intentional solid/outline pair; verify every selected glyph has both Svelte and native implementations listed in the pilot manifest.
- [ ] 2.3 Redraw the pilot glyphs as original Orbis artwork in both platform directories, including a deliberately shaped filled/outline pair, and align documented component defaults; verify imports still resolve and previews are materially equivalent at 16, 20, and 24 in light and dark themes.
- [ ] 2.4 Review the pilot in realistic desktop and mobile controls with representative users or a documented task-based usability method; verify recognition issues, 16-pixel legibility, accessible names, and accepted visual exceptions are recorded before approving full-catalog rollout.

## 3. Catalog and Regression Tooling

- [x] 3.1 Add a maintained icon manifest classifying every symbol as canonical, intentional variant, brand asset, or compatibility alias with its semantic family; verify manifest entries reconcile exactly with public exports.
- [ ] 3.2 Add a Svelte icon gallery showing every icon's public name, family, variant, and 16/20/24 previews in light and dark themes; verify all manifest entries render without runtime or console errors.
- [ ] 3.3 Add a native review surface or reproducible Expo screenshot procedure for the same manifest and sizes; verify the pilot's Svelte/native gallery entries can be compared side by side.
- [ ] 3.4 Add automated package checks for paired platform coverage, manifest/export drift, alias targets, standard component defaults, and invalid icon source; verify the checks fail on deliberate missing-pair, missing-export, and broken-alias fixtures.
- [ ] 3.5 Document how to add, classify, review, and test an icon, including familiar metaphor selection, RTL direction classification, when a solid variant or compatibility alias is allowed, and how to validate in-context recognition; verify a maintainer can follow the guide to add a test icon without undocumented steps.

## 4. Navigation and Core Actions

- [ ] 4.1 Redesign navigation, menu, directional, and layout symbols according to the approved guide; verify each icon is reviewed in both galleries at 16/20/24, logical-direction behavior is checked in RTL and LTR, absolute-direction symbols are not mirrored, and package parity checks pass.
- [ ] 4.2 Redesign search, edit, selection, confirmation, and common action symbols; verify each inventory-assigned icon is visually approved in both platform implementations and remains legible at 16.
- [ ] 4.3 Review all consumers of the navigation/action batches and update only incorrect semantic choices, explicit size overrides, or missing accessible names; verify ambiguous icon-only controls have visible or discoverable labels, keyboard focus is perceivable, and interactive targets meet WCAG 2.2 AA or a documented exception plus platform guidance.

## 5. Status, Feedback, and Security

- [ ] 5.1 Redesign status, notification, loading, warning, error, help, and feedback symbols; verify semantic distinctions remain clear without relying on color and all paired exports pass gallery review.
- [ ] 5.2 Redesign people, account, permission, privacy, and security symbols; verify silhouette and detail remain distinguishable at 16 and accessible consumers retain explicit labels.
- [ ] 5.3 Review status/security consumers for color inheritance, adjacent-color contrast, state cues, and accessible names, correcting only identified issues; verify WCAG 2.2 AA-relevant contrast and representative light/dark and screen-reader checks pass on web and native.

## 6. Academic and Organization Families

- [ ] 6.1 Redesign education, academics, classes, calendars, schedules, grades, and assessment symbols; verify every assigned icon passes both-platform 16/20/24 visual review and family consistency checks.
- [ ] 6.2 Redesign organization, buildings, groups, locations, and academic operations symbols; verify every assigned icon passes the manifest coverage check and has no unexplained geometry/style exceptions.
- [ ] 6.3 Review the corresponding desktop and mobile consumers for symbol meaning and sizing, updating only confirmed mismatches; verify affected application checks and representative screen screenshots pass.

## 7. Records, Data, and Business Families

- [ ] 7.1 Redesign documents, folders, uploads, downloads, attachments, and archive symbols; verify common file states remain distinguishable at 16 and every icon has a matching native/web design.
- [ ] 7.2 Redesign tables, charts, finance, payments, metrics, databases, and reporting symbols; verify related concepts are distinguishable as a family and previews remain legible at all standard sizes.
- [ ] 7.3 Redesign settings, tools, integrations, devices, and remaining system/utility symbols; verify every remaining manifest entry is reviewed and no placeholder or legacy unclassified glyph remains.
- [ ] 7.4 Review consumers of the records/data/business batches for semantic ambiguity, explicit sizing, and accessibility; verify targeted application checks pass and any consumer changes are documented.

## 8. Brand and Variant Consolidation

- [ ] 8.1 Review the Orbis logo and other brand assets separately from interface-icon geometry; verify brand colors, proportions, and special properties render consistently across platforms and remain covered by focused fixtures.
- [ ] 8.2 Consolidate visually duplicate icons into canonical designs and redirect legacy public exports through compatibility aliases; verify all old imports still resolve and each alias has a documented target and reason.
- [ ] 8.3 Redraw intentional outline/solid pairs as related but optically tuned original forms, preserving clear counters and negative space in filled variants; verify naming, silhouette, and meaning are consistent at 16/20/24 in both galleries without deleting public exports.

## 9. API, Accessibility, and Performance

- [ ] 9.1 Align and document Svelte/native size, color, stroke-width, class/style, and property-forwarding behavior with framework-appropriate types; verify representative components typecheck and defaults match the agreed effective size/color contract.
- [ ] 9.2 Verify decorative defaults and explicit accessible-name forwarding on both platforms; verify a decorative icon is hidden from assistive technology while an icon-only action exposes its consumer-provided accessible name.
- [ ] 9.3 Verify direct named imports remain tree-shakeable and compare final package/app bundle sizes with the baseline; verify no unexpected dependency is added and any material bundle increase is explained and approved.
- [ ] 9.4 Audit interactive consumers rather than glyph dimensions for touch-target size, spacing exceptions, and platform-specific touch guidance; verify web controls meet WCAG 2.2 AA 2.5.8 or document a valid exception and mobile controls meet the host platform's guidance.

## 10. Integration and Release Readiness

- [ ] 10.1 Run icon package parity/source checks and package typecheck/build; verify the full catalog passes with no missing platform pair or unclassified export.
- [ ] 10.2 Run the desktop production build and mobile typecheck/build; verify both apps resolve their existing `@orbis/icons` imports without consumer-wide migration.
- [ ] 10.3 Complete a final paired visual and in-context usability review of the entire icon catalog at standard sizes, in light/dark themes, RTL/LTR, on desktop and mobile; verify every manifest entry has an approval, accessibility checks pass, and every exception is documented.
- [ ] 10.4 Publish the final icon guidelines, manifest, compatibility/alias policy, gallery workflow, and before/after baseline summary; verify the documentation matches the released components and checks.
