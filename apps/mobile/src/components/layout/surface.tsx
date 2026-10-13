/**
 * @module @orbis/mobile/components/layout/surface
 *
 * Theme-aware surface container with elevation, borders, and rounding.
 */

import React from "react";
import {
  View,
  type ViewProps,
} from "react-native";
import type { ViewStyle } from "react-native";

type SpacingScale =
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
type RadiusScale = "none" | "xs" | "sm" | "md" | "lg" | "xl" | "2xl" | "3xl" | "full";

const paddingClasses: Record<SpacingScale, string> = {
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

const radiusClasses: Record<RadiusScale, string> = {
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

export interface SurfaceProps extends ViewProps {
  variant?: "flat" | "elevated" | "outlined" | "sunken";
  radius?: RadiusScale;
  padding?: SpacingScale | number;
  children?: React.ReactNode;
}

export function Surface({
  variant = "flat",
  radius = "md",
  padding,
  style,
  children,
  ...props
}: SurfaceProps) {
  const variantClasses: Record<SurfaceProps["variant"] & string, string> = {
    flat: "bg-card",
    elevated: "border border-card-border bg-card-elevated shadow-md",
    outlined: "border border-border bg-card",
    sunken: "bg-background-element",
  };

  const computedStyle: ViewStyle = {
    padding: typeof padding === "number" ? padding : undefined,
  };
  const paddingClass =
    typeof padding === "string" ? paddingClasses[padding] : undefined;

  return (
    <View
      className={`${radiusClasses[radius]} ${paddingClass ?? ""} ${variantClasses[variant]}`}
      style={[computedStyle, style]}
      {...props}
    >
      {children}
    </View>
  );
}
