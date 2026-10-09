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
  return (
    <Pressable
      accessibilityRole="button"
      accessibilityState={{ selected, disabled }}
      disabled={disabled}
      onPress={disabled ? undefined : onPress}
      className={`flex-row items-center gap-1 rounded-full border px-3 py-1.5 ${selected ? "border-primary bg-accent-light" : "border-border bg-background-element"}`}
      style={({ pressed }) => [
        disabled && { opacity: 0.5 },
        pressed && !disabled && { opacity: 0.8 },
        style,
      ]}
    >
      {icon && <View className="items-center justify-center">{icon}</View>}

      <Text
        className={`text-xs ${selected ? "font-semibold text-primary" : "font-medium text-foreground"}`}
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
            className={`relative h-3.5 w-3.5 items-center justify-center rounded-full ${selected ? "bg-primary" : "bg-muted-foreground"}`}
          >
            <View className="absolute h-[1.2px] w-2 rotate-45 bg-white" />
            <View className="absolute h-[1.2px] w-2 -rotate-45 bg-white" />
          </View>
        </Pressable>
      )}
    </Pressable>
  );
}
