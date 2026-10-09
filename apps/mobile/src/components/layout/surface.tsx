/**
 * @module @orbis/mobile/components/layout/surface
 *
 * Theme-aware surface container with elevation, borders, and rounding.
 */

import React from "react";
import {
  View,
  type ViewProps,
  type ViewStyle,
} from "react-native";
import { useTheme } from "@/hooks/use-theme";

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
  radius = "xl",
  padding,
  style,
  children,
  ...props
}: SurfaceProps) {
  const theme = useTheme();

  let variantStyle: ViewStyle = {};
  let variantClass = "";
  switch (variant) {
    case "elevated":
      variantStyle = {
        backgroundColor: theme.cardElevated,
        borderWidth: 1,
        borderColor: theme.cardBorder,
      };
      variantClass = "shadow-md";
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
    padding: typeof padding === "number" ? padding : undefined,
    ...variantStyle,
  };
  const paddingClass =
    typeof padding === "string" ? paddingClasses[padding] : undefined;

  return (
    <View
      className={`${radiusClasses[radius]} ${paddingClass ?? ""} ${variantClass}`}
      style={[computedStyle, style]}
      {...props}
    >
      {children}
    </View>
  );
}
