/**
 * @module @orbis/mobile/components/layout/divider
 *
 * Semantic divider for separating content sections.
 */

import React from "react";
import { StyleSheet, View, type ViewProps } from "react-native";

export interface DividerProps extends ViewProps {
  orientation?: "horizontal" | "vertical";
  thickness?: number;
  color?: string;
  spacing?: number;
}

export function Divider({
  orientation = "horizontal",
  thickness = StyleSheet.hairlineWidth,
  color,
  spacing = 0,
  style,
  ...props
}: DividerProps) {
  if (orientation === "vertical") {
    return (
      <View
        className="h-full bg-border"
        style={[
          {
            width: thickness,
            ...(color ? { backgroundColor: color } : {}),
            marginHorizontal: spacing,
          },
          style,
        ]}
        {...props}
      />
    );
  }

  return (
    <View
      className="w-full bg-border"
      style={[
        {
          height: thickness,
          ...(color ? { backgroundColor: color } : {}),
          marginVertical: spacing,
        },
        style,
      ]}
      {...props}
    />
  );
}
