/**
 * @module @orbis/mobile/components/layout/divider
 *
 * Semantic divider for separating content sections.
 */

import React from "react";
import { StyleSheet, View, type ViewProps } from "react-native";
import { useTheme } from "@/hooks/use-theme";

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
  const theme = useTheme();

  const dividerColor = color ?? theme.border;

  if (orientation === "vertical") {
    return (
      <View
        className="h-full"
        style={[
          {
            width: thickness,
            backgroundColor: dividerColor,
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
      className="w-full"
      style={[
        {
          height: thickness,
          backgroundColor: dividerColor,
          marginVertical: spacing,
        },
        style,
      ]}
      {...props}
    />
  );
}
