/**
 * @module @orbis/mobile/components/ui/segmented-control
 *
 * Segmented control / tab switcher for filtering and mode selection.
 */

import React from "react";
import {
  View,
  Text,
  Pressable,
  StyleSheet,
  type ViewStyle,
} from "react-native";
import { useTheme } from "@/hooks/use-theme";
import { Radii, Shadows, Spacing, Typography } from "@/constants/theme";

export interface SegmentOption<T extends string = string> {
  value: T;
  label: string;
  icon?: React.ReactNode;
}

export interface SegmentedControlProps<T extends string = string> {
  options: SegmentOption<T>[];
  value: T;
  onChange: (value: T) => void;
  style?: ViewStyle;
}

export function SegmentedControl<T extends string = string>({
  options,
  value,
  onChange,
  style,
}: SegmentedControlProps<T>) {
  const theme = useTheme();

  return (
    <View
      style={[
        styles.container,
        {
          backgroundColor: theme.backgroundElement,
          borderColor: theme.border,
        },
        style,
      ]}
    >
      {options.map((option) => {
        const isSelected = option.value === value;

        return (
          <Pressable
            key={option.value}
            accessibilityRole="button"
            accessibilityState={{ selected: isSelected }}
            onPress={() => onChange(option.value)}
            style={[
              styles.segment,
              isSelected && [
                styles.selectedSegment,
                {
                  backgroundColor: theme.cardElevated,
                  borderColor: theme.cardBorder,
                },
              ],
            ]}
          >
            {option.icon && (
              <View style={styles.iconContainer}>{option.icon}</View>
            )}
            <Text
              style={[
                styles.label,
                {
                  color: isSelected ? theme.foreground : theme.textSecondary,
                  fontWeight: isSelected ? "600" : "500",
                },
              ]}
            >
              {option.label}
            </Text>
          </Pressable>
        );
      })}
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flexDirection: "row",
    padding: Spacing.half,
    borderRadius: Radii.lg,
    borderWidth: 1,
    alignItems: "center",
  },
  segment: {
    flex: 1,
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "center",
    paddingVertical: Spacing.two,
    paddingHorizontal: Spacing.two,
    borderRadius: Radii.md,
    gap: Spacing.one,
  },
  selectedSegment: {
    ...Shadows.sm,
    borderWidth: 1,
  },
  iconContainer: {
    alignItems: "center",
    justifyContent: "center",
  },
  label: {
    ...Typography.bodySmall,
    textAlign: "center",
  },
});
