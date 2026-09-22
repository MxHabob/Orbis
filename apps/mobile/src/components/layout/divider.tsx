/**
 * @module @orbis/mobile/components/layout/divider
 *
 * Semantic divider for separating content sections.
 */

import React from "react";
import { StyleSheet, View, type ViewProps, useColorScheme } from "react-native";
import { Colors } from "@/constants/theme";

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
  const colorScheme = useColorScheme();
  const theme = Colors[colorScheme === "dark" ? "dark" : "light"];

  const dividerColor = color ?? theme.border;

  if (orientation === "vertical") {
    return (
      <View
        style={[
          styles.vertical,
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
      style={[
        styles.horizontal,
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

const styles = StyleSheet.create({
  horizontal: {
    width: "100%",
  },
  vertical: {
    height: "100%",
  },
});
