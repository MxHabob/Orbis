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
    padding: 2,
    borderRadius: 8,
    borderWidth: 1,
    alignItems: "center",
  },
  segment: {
    flex: 1,
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "center",
    paddingVertical: 8,
    paddingHorizontal: 8,
    borderRadius: 6,
    gap: 4,
  },
  selectedSegment: {
    ...{"shadowColor":"#000000","shadowOffset":{"width":0,"height":1},"shadowOpacity":0.05,"shadowRadius":2,"elevation":1},
    borderWidth: 1,
  },
  iconContainer: {
    alignItems: "center",
    justifyContent: "center",
  },
  label: {
    ...{"fontSize":13,"lineHeight":18,"fontWeight":"400"},
    textAlign: "center",
  },
});
