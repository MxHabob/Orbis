/**
 * @module @orbis/mobile/components/ui/chip
 *
 * Filter chip / tag component with selected and deletable states.
 */

import React from "react";
import {
  View,
  Text,
  Pressable,
  type ViewStyle,
} from "react-native";
import { useTheme } from "@/hooks/use-theme";

export interface ChipProps {
  label: string;
  selected?: boolean;
  onPress?: () => void;
  onDelete?: () => void;
  icon?: React.ReactNode;
  disabled?: boolean;
  style?: ViewStyle;
}

export function Chip({
  label,
  selected = false,
  onPress,
  onDelete,
  icon,
  disabled = false,
  style,
}: ChipProps) {
  const theme = useTheme();

  return (
    <Pressable
      accessibilityRole="button"
      accessibilityState={{ selected, disabled }}
      disabled={disabled}
      onPress={disabled ? undefined : onPress}
      className="flex-row items-center gap-1 rounded-full border px-3 py-1.5"
      style={({ pressed }) => [
        {
          backgroundColor: selected ? theme.accentLight : theme.backgroundElement,
          borderColor: selected ? theme.primary : theme.border,
          opacity: disabled ? 0.5 : 1,
        },
        pressed && !disabled && { opacity: 0.8 },
        style,
      ]}
    >
      {icon && <View className="items-center justify-center">{icon}</View>}

      <Text
        className="text-xs"
        style={[
          {
            color: selected ? theme.primary : theme.foreground,
            fontWeight: selected ? "600" : "500",
          },
        ]}
      >
        {label}
      </Text>

      {onDelete && (
        <Pressable
          accessibilityRole="button"
          accessibilityLabel={`Remove ${label}`}
          onPress={onDelete}
          hitSlop={6}
          className="ml-0.5 items-center justify-center"
        >
          <View
            className="relative h-3.5 w-3.5 items-center justify-center rounded-full"
            style={[
              { backgroundColor: selected ? theme.primary : theme.textMuted },
            ]}
          >
            <View className="absolute h-[1.2px] w-2 rotate-45 bg-white" />
            <View className="absolute h-[1.2px] w-2 -rotate-45 bg-white" />
          </View>
        </Pressable>
      )}
    </Pressable>
  );
}
