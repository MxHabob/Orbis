import React from "react";
import { Pressable } from "react-native";

export interface CheckboxProps {
  value: boolean;
  onValueChange?: (value: boolean) => void;
  disabled?: boolean;
  accessibilityLabel?: string;
}

export function Checkbox({
  value,
  onValueChange,
  disabled = false,
  accessibilityLabel,
}: CheckboxProps) {
  return (
    <Pressable
      accessibilityRole="checkbox"
      accessibilityState={{ checked: value, disabled }}
      accessibilityLabel={accessibilityLabel}
      disabled={disabled}
      onPress={() => onValueChange?.(!value)}
      className={`h-4 w-4 items-center justify-center rounded-full ${value ? "border-4 border-primary" : "border border-input"}`}
      style={({ pressed }) => [
        disabled && { opacity: 0.5 },
        pressed && { opacity: 0.75 },
      ]}
    ></Pressable>
  );
}
