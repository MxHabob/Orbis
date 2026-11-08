/**
 * @module @orbis/mobile/lib/design-tokens
 *
 * Shared design-system type definitions and NativeWind class look-up maps.
 *
 * Every layout component that accepts a token-based spacing or radius prop
 * should import from here rather than re-declaring its own copy.
 */

// ---------------------------------------------------------------------------
// Scale types
// ---------------------------------------------------------------------------

/** Named spacing steps defined in `global.css` via `--spacing-*`. */
export type SpacingScale =
  | "half"
  | "one"
  | "two"
  | "three"
  | "four"
  | "five"
  | "six"
  | "eight"
  | "ten"
  | "twelve"
  | "sixteen";

/** Named radius steps defined in `global.css` via `--radius-*`. */
export type RadiusScale =
  | "none"
  | "xs"
  | "sm"
  | "md"
  | "lg"
  | "xl"
  | "2xl"
  | "3xl"
  | "full";

// ---------------------------------------------------------------------------
// Class look-up maps
// ---------------------------------------------------------------------------

export const paddingClasses: Record<SpacingScale, string> = {
  half: "p-half",
  one: "p-one",
  two: "p-two",
  three: "p-three",
  four: "p-four",
  five: "p-five",
  six: "p-six",
  eight: "p-eight",
  ten: "p-ten",
  twelve: "p-twelve",
  sixteen: "p-sixteen",
};

export const horizontalPaddingClasses: Record<SpacingScale, string> = {
  half: "px-half",
  one: "px-one",
  two: "px-two",
  three: "px-three",
  four: "px-four",
  five: "px-five",
  six: "px-six",
  eight: "px-eight",
  ten: "px-ten",
  twelve: "px-twelve",
  sixteen: "px-sixteen",
};

export const verticalPaddingClasses: Record<SpacingScale, string> = {
  half: "py-half",
  one: "py-one",
  two: "py-two",
  three: "py-three",
  four: "py-four",
  five: "py-five",
  six: "py-six",
  eight: "py-eight",
  ten: "py-ten",
  twelve: "py-twelve",
  sixteen: "py-sixteen",
};

export const marginBottomClasses: Record<SpacingScale, string> = {
  half: "mb-half",
  one: "mb-one",
  two: "mb-two",
  three: "mb-three",
  four: "mb-four",
  five: "mb-five",
  six: "mb-six",
  eight: "mb-eight",
  ten: "mb-ten",
  twelve: "mb-twelve",
  sixteen: "mb-sixteen",
};

export const gapClasses: Record<SpacingScale, string> = {
  half: "gap-half",
  one: "gap-one",
  two: "gap-two",
  three: "gap-three",
  four: "gap-four",
  five: "gap-five",
  six: "gap-six",
  eight: "gap-eight",
  ten: "gap-ten",
  twelve: "gap-twelve",
  sixteen: "gap-sixteen",
};

export const radiusClasses: Record<RadiusScale, string> = {
  none: "rounded-none",
  xs: "rounded-xs",
  sm: "rounded-sm",
  md: "rounded-md",
  lg: "rounded-lg",
  xl: "rounded-xl",
  "2xl": "rounded-2xl",
  "3xl": "rounded-3xl",
  full: "rounded-full",
};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/**
 * Resolve a spacing/radius value to its NativeWind class name.
 * Returns `undefined` when the value is numeric (the caller should set it
 * via inline style instead).
 */
export function resolveClass<T extends string>(
  value: T | number | undefined,
  classes: Record<T, string>,
): string | undefined {
  return typeof value === "string" ? classes[value] : undefined;
}
