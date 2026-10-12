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
  return (
    <View
      className="flex-row items-center rounded-lg border border-border bg-background-element p-half"
      style={style}
    >
      {options.map((option) => {
        const isSelected = option.value === value;

        return (
          <Pressable
            key={option.value}
            accessibilityRole="button"
            accessibilityState={{ selected: isSelected }}
            onPress={() => onChange(option.value)}
            className={`flex-1 flex-row items-center justify-center gap-one rounded-md px-two py-two ${isSelected ? "border border-card-border bg-card-elevated" : ""}`}
            style={isSelected ? styles.selectedElevation : undefined}
          >
            {option.icon && (
              <View className="items-center justify-center">{option.icon}</View>
            )}
            <Text
              className={`text-center text-[13px] leading-[18px] ${isSelected ? "font-semibold text-foreground" : "font-medium text-muted-foreground"}`}
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
  selectedElevation: {
    ...{"shadowColor":"#000000","shadowOffset":{"width":0,"height":1},"shadowOpacity":0.05,"shadowRadius":2,"elevation":1},
  },
});
