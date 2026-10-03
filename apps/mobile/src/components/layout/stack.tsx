/**
 * @module @orbis/mobile/components/layout/stack
 *
 * Flexbox layout primitives for structured spacing and alignment.
 */

import React from "react";
import {
  View,
  type ViewProps,
  type FlexAlignType,
  type ViewStyle,
  StyleSheet,
} from "react-native";
import { Spacing } from "@/constants/theme";

export interface StackProps extends ViewProps {
  direction?: "column" | "row" | "column-reverse" | "row-reverse";
  gap?: keyof typeof Spacing | number;
  align?: FlexAlignType;
  justify?:
    | "flex-start"
    | "flex-end"
    | "center"
    | "space-between"
    | "space-around"
    | "space-evenly";
  wrap?: "wrap" | "nowrap" | "wrap-reverse";
  fullWidth?: boolean;
  flex?: number;
}

function resolveGap(gap?: keyof typeof Spacing | number): number | undefined {
  if (gap === undefined) return undefined;
  if (typeof gap === "number") return gap;
  return Spacing[gap];
}

export function Stack({
  direction = "column",
  gap,
  align,
  justify,
  wrap,
  fullWidth = false,
  flex,
  style,
  children,
  ...props
}: StackProps) {
  const computedStyle: ViewStyle = {
    flexDirection: direction,
    gap: resolveGap(gap),
    alignItems: align,
    justifyContent: justify,
    flexWrap: wrap,
    ...(fullWidth ? { width: "100%" } : {}),
    ...(flex !== undefined ? { flex } : {}),
  };

  return (
    <View style={[computedStyle, style]} {...props}>
      {children}
    </View>
  );
}

export function Row(props: Omit<StackProps, "direction">) {
  return <Stack direction="row" align="center" {...props} />;
}

export function Col(props: Omit<StackProps, "direction">) {
  return <Stack direction="column" {...props} />;
}
