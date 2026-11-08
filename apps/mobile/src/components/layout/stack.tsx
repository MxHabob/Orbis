/**
 * @module @orbis/mobile/components/layout/stack
 *
 * Flexbox layout primitives for structured spacing and alignment.
 */

import React from "react";
import { View, type ViewProps, type ViewStyle } from "react-native";
import { type SpacingScale, gapClasses } from "@/lib/design-tokens";

export interface StackProps extends ViewProps {
  direction?: "column" | "row" | "column-reverse" | "row-reverse";
  gap?: SpacingScale | number;
  align?: ViewStyle["alignItems"];
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
    gap: typeof gap === "number" ? gap : undefined,
    alignItems: align,
    justifyContent: justify,
    flexWrap: wrap,
    ...(fullWidth ? { width: "100%" } : {}),
    ...(flex !== undefined ? { flex } : {}),
  };
  const gapClass = typeof gap === "string" ? gapClasses[gap] : undefined;

  return (
    <View
      className={`flex ${gapClass ?? ""}`}
      style={[computedStyle, style]}
      {...props}
    >
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
