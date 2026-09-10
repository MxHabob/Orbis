import React from "react";
import { Pressable, StyleSheet, View } from "react-native";
import { Radii } from "@/constants/theme";
import { Ionicons } from "@expo/vector-icons";
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
      style={({ pressed }) => [
        styles.track,
        {
          backgroundColor: value ? theme.primary : theme.backgroundElement,
          borderColor: value ? theme.primary : theme.border,
        },
        disabled && styles.disabled,
        pressed && styles.pressed,
      ]}
    >
      <View
        style={[
          styles.thumb,
          {
            borderColor: value ? theme.primary : theme.border,
            backgroundColor: theme.background,
            transform: [{ translateX: value ? 20 : 0 }],
          },
        ]}
      >
        <Ionicons
          name={value ? "checkmark" : "close"}
          size={12}
          color={value ? theme.primary : theme.mutedForeground}
        />
      </View>
    </Pressable>
  );
}

const styles = StyleSheet.create({
  track: {
    width: 44,
    height: 24,
    borderRadius: Radii.sm,
    borderWidth: 1,
    justifyContent: "flex-start",
  },
  thumb: {
    width: 22,
    height: 22,
    borderRadius: Radii.md,
    borderWidth: 1,
    shadowColor: "#000000",
    shadowOffset: { width: 0, height: 1 },
    shadowOpacity: 0.12,
    shadowRadius: 2,
    elevation: 1,
  },
  disabled: {
    opacity: 0.5,
  },
  pressed: {
    opacity: 0.8,
  },
});
