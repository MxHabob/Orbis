import React from "react";
import { Pressable, View } from "react-native";
import { Check, XMark } from "@orbis/icons/native";
import { useTheme } from "@/hooks/use-theme";

export interface SwitchProps {
  value: boolean;
  onValueChange?: (value: boolean) => void;
  disabled?: boolean;
  accessibilityLabel?: string;
}

export function Switch({
  value,
  onValueChange,
  disabled = false,
  accessibilityLabel,
}: SwitchProps) {
  const theme = useTheme();

  return (
    <Pressable
      accessibilityRole="switch"
      accessibilityState={{ checked: value, disabled }}
      accessibilityLabel={accessibilityLabel}
      disabled={disabled}
      onPress={() => onValueChange?.(!value)}
      className={`h-6 w-11 justify-start rounded-sm border ${value ? "border-primary bg-primary" : "border-border bg-background-element"} ${disabled ? "opacity-50" : ""}`}
      style={({ pressed }) => (pressed && !disabled ? { opacity: 0.8 } : undefined)}
    >
      <View
        className={`h-[22px] w-[22px] rounded-md border bg-background shadow-sm ${value ? "border-primary" : "border-border"}`}
        style={{ transform: [{ translateX: value ? 20 : 0 }] }}
      >
        {value ? (
          <Check size={12} color={theme.primary} />
        ) : (
          <XMark size={12} color={theme.mutedForeground} />
        )}
      </View>
    </Pressable>
  );
}
