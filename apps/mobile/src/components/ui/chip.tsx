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
  StyleSheet,
  type ViewStyle,
} from "react-native";
import { useTheme } from "@/hooks/use-theme";
import { Radii, Spacing, Typography } from "@/constants/theme";

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
      style={({ pressed }) => [
        styles.chip,
        {
          backgroundColor: selected ? theme.accentLight : theme.backgroundElement,
          borderColor: selected ? theme.primary : theme.border,
          opacity: disabled ? 0.5 : 1,
        },
        pressed && !disabled && styles.pressed,
        style,
      ]}
    >
      {icon && <View style={styles.iconContainer}>{icon}</View>}

      <Text
        style={[
          styles.label,
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
          style={styles.deleteButton}
        >
          <View
            style={[
              styles.deleteIcon,
              { backgroundColor: selected ? theme.primary : theme.textMuted },
            ]}
          >
            <View style={styles.clearBar1} />
            <View style={styles.clearBar2} />
          </View>
        </Pressable>
      )}
    </Pressable>
  );
}

const styles = StyleSheet.create({
  chip: {
    flexDirection: "row",
    alignItems: "center",
    paddingVertical: 6,
    paddingHorizontal: Spacing.three,
    borderRadius: Radii.full,
    borderWidth: 1,
    gap: Spacing.one,
  },
  iconContainer: {
    alignItems: "center",
    justifyContent: "center",
  },
  label: {
    ...Typography.caption,
  },
  deleteButton: {
    marginLeft: 2,
    alignItems: "center",
    justifyContent: "center",
  },
  deleteIcon: {
    width: 14,
    height: 14,
    borderRadius: 7,
    alignItems: "center",
    justifyContent: "center",
    position: "relative",
  },
  clearBar1: {
    position: "absolute",
    width: 8,
    height: 1.2,
    backgroundColor: "#FFFFFF",
    transform: [{ rotate: "45deg" }],
  },
  clearBar2: {
    position: "absolute",
    width: 8,
    height: 1.2,
    backgroundColor: "#FFFFFF",
    transform: [{ rotate: "-45deg" }],
  },
  pressed: {
    opacity: 0.8,
  },
});
