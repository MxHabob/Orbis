/**
 * @module @orbis/mobile/components/layout/surface
 *
 * Theme-aware surface container with elevation, borders, and rounding.
 */

import React from "react";
import { View, type ViewProps, type ViewStyle } from "react-native";
import {
  type SpacingScale,
  type RadiusScale,
  paddingClasses,
  radiusClasses,
} from "@/lib/design-tokens";

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
