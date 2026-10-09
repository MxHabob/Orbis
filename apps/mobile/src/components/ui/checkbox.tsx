import React from "react";
import { Pressable } from "react-native";
import { useTheme } from "@/hooks/use-theme";

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
  const theme = useTheme();

  return (
    <Pressable
      accessibilityRole="checkbox"
      accessibilityState={{ checked: value, disabled }}
      accessibilityLabel={accessibilityLabel}
      disabled={disabled}
      onPress={() => onValueChange?.(!value)}
      className="h-4 w-4 items-center justify-center rounded-full"
      style={({ pressed }) => [
        {
          backgroundColor: "transparent",
          borderColor: value ? theme.primary : theme.input,
          borderWidth: value ? 4 : 1,
        },
        disabled && { opacity: 0.5 },
        pressed && { opacity: 0.75 },
      ]}
    ></Pressable>
  );
}
