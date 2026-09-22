/**
 * @module @orbis/mobile/components/layout/surface
 *
 * Theme-aware surface container with elevation, borders, and rounding.
 */

import React from "react";
import {
  View,
  type ViewProps,
  StyleSheet,
  useColorScheme,
  type ViewStyle,
} from "react-native";
import { Colors, Radii, Shadows, Spacing } from "@/constants/theme";

export interface SurfaceProps extends ViewProps {
  variant?: "flat" | "elevated" | "outlined" | "sunken";
  radius?: keyof typeof Radii;
  padding?: keyof typeof Spacing | number;
  children?: React.ReactNode;
}

function resolvePadding(padding?: keyof typeof Spacing | number): number | undefined {
  if (padding === undefined) return undefined;
  if (typeof padding === "number") return padding;
  return Spacing[padding];
}

export function Surface({
  variant = "flat",
  radius = "xl",
  padding,
  style,
  children,
  ...props
}: SurfaceProps) {
  const colorScheme = useColorScheme();
  const isDark = colorScheme === "dark";
  const theme = Colors[isDark ? "dark" : "light"];

  let variantStyle: ViewStyle = {};
  switch (variant) {
    case "elevated":
      variantStyle = {
        backgroundColor: theme.cardElevated,
        ...Shadows.md,
        borderWidth: 1,
        borderColor: theme.cardBorder,
      };
      break;
    case "outlined":
      variantStyle = {
        backgroundColor: theme.card,
        borderWidth: 1,
        borderColor: theme.border,
      };
      break;
    case "sunken":
      variantStyle = {
        backgroundColor: theme.backgroundElement,
      };
      break;
    case "flat":
    default:
      variantStyle = {
        backgroundColor: theme.card,
      };
      break;
  }

  const computedStyle: ViewStyle = {
    borderRadius: Radii[radius],
    padding: resolvePadding(padding),
    ...variantStyle,
  };

  return (
    <View style={[computedStyle, style]} {...props}>
      {children}
    </View>
  );
}
