# Orbis icon drawing guidelines

## Grid and weight

- Draw interface symbols on a `24 × 24` viewBox. Keep ordinary outlines within
  a 2–22 unit optical area; let strokes and optically corrected forms extend
  only when needed.
- Use `16`, `20`, and `24` as review sizes. The component default is `16` in
  both Svelte and React Native; a control may request a larger size when its
  hierarchy calls for it.
- Use a nominal `1.75` stroke at 24 units. Adjustments are allowed only when
  needed for a dense form or a deliberate filled/brand shape.
- Use rounded caps and joins for open strokes. Keep corners square only when
  the depicted object requires them.
- Center by visual mass, not by bounding-box arithmetic. Check optical balance
  at 16px, where unnecessary detail and narrow gaps disappear first.

## Shape and meaning

- Prefer one clear, familiar metaphor over ornamental or redundant detail.
- Keep silhouettes distinct within related families; simplify dense symbols
  rather than shrinking their details.
- Filled symbols are intentional variants, not a way to add visual emphasis to
  an otherwise unrelated glyph. A paired outline and filled icon must depict
  the same object and preserve its meaning.
- Use consumer-selected color (`currentColor` on web) rather than embedding
  interface colors. The Orbis logo and other brand assets are exceptions to
  interface-grid and stroke rules.
- Icon artwork itself does not infer locale direction. Consumers mirror an
  icon only when it represents logical backward/forward navigation; absolute
  directions, charts, and fixed spatial relationships must not be mirrored.

## Platform parity and accessibility

- Keep each public symbol paired between `src/svelte/` and `src/native/`.
  Equivalent size, color, and stroke settings must preserve the same silhouette
  and visual weight.
- Decorative icons remain hidden from assistive technology by default.
  Icon-only controls must provide an accessible name at the control level.
- Do not use color as the only state cue. Check meaningful forms against their
  adjacent surface and review the containing interactive target separately
  from the glyph dimensions.

## Review checklist

1. Compare the icon at 16, 20, and 24 on light and dark surfaces.
2. Check it beside related symbols and in the actual desktop/mobile control.
3. Confirm that its meaning is familiar without relying on color or a private
   convention; add a visible or discoverable label if it is ambiguous.
4. Check RTL and LTR behavior at the consumer, and verify an absolute-direction
   symbol is not mirrored.
5. Keep the Svelte and native paths equivalent and run the inventory,
   typechecks, and relevant application checks before approval.
