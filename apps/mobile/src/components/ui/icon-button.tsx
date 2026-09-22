/**
 * @module @orbis/mobile/components/ui/icon-button
 *
 * Tactile Icon Button with accessible hitSlop, active state animation, and theme support.
 */

import React from "react";
import {
  Pressable,
  StyleSheet,
  type ViewStyle,
  ActivityIndicator,
  View,
} from "react-native";
import { useTheme } from "@/hooks/use-theme";
import { Radii, Shadows } from "@/constants/theme";

export type IconButtonVariant =
  | "default"
  | "secondary"
  | "outline"
  | "ghost"
  | "destructive";

export type IconButtonSize = "xs" | "sm" | "md" | "lg";

export interface IconButtonProps {
  icon: React.ReactNode;
  onPress: () => void;
  accessibilityLabel: string;
  variant?: IconButtonVariant;
  size?: IconButtonSize;
  rounded?: boolean;
  loading?: boolean;
  disabled?: boolean;
  style?: ViewStyle;
}

const SIZE_MAP: Record<IconButtonSize, { size: number; iconSize: number }> = {
  xs: { size: 28, iconSize: 14 },
  sm: { size: 36, iconSize: 18 },
  md: { size: 44, iconSize: 22 },
  lg: { size: 52, iconSize: 26 },
};

export const IconButton = React.memo<IconButtonProps>(
  ({
    icon,
    onPress,
    accessibilityLabel,
    variant = "default",
    size = "md",
    rounded = true,
    loading = false,
    disabled = false,
    style,
  }) => {
    const theme = useTheme();
    const isDisabled = disabled || loading;
    const dimensions = SIZE_MAP[size];

    const getVariantStyles = (): {
      container: ViewStyle;
      indicatorColor: string;
    } => {
      switch (variant) {
        case "secondary":
          return {
            container: {
              backgroundColor: theme.secondary,
              borderColor: theme.border,
              borderWidth: 1,
              borderBottomWidth: 2,
              borderBottomColor: theme.btnShadowSecondary,
            },
            indicatorColor: theme.secondaryForeground,
          };
        case "outline":
          return {
            container: {
              backgroundColor: theme.card,
              borderColor: theme.border,
              borderWidth: 1,
              borderBottomWidth: 2,
              borderBottomColor: theme.border,
            },
            indicatorColor: theme.foreground,
          };
        case "destructive":
          return {
            container: {
              backgroundColor: theme.destructive,
              borderColor: theme.destructive,
              borderWidth: 1,
              borderBottomWidth: 2,
              borderBottomColor: theme.btnShadowDestructive,
            },
            indicatorColor: theme.destructiveForeground,
          };
        case "ghost":
          return {
            container: {
              backgroundColor: "transparent",
            },
            indicatorColor: theme.foreground,
          };
        case "default":
        default:
          return {
            container: {
              backgroundColor: theme.primary,
              borderColor: theme.primary,
              borderWidth: 1,
              borderBottomWidth: 2,
              borderBottomColor: theme.btnShadowPrimary,
            },
            indicatorColor: theme.primaryForeground,
          };
      }
    };

    const vStyles = getVariantStyles();
    const borderRadius = rounded ? dimensions.size / 2 : Radii.md;

    // Hit slop expands smaller buttons to minimum 44px touch area
    const hitSlopAmount = Math.max(0, (44 - dimensions.size) / 2);

    return (
      <Pressable
        accessibilityRole="button"
        accessibilityLabel={accessibilityLabel}
        disabled={isDisabled}
        hitSlop={hitSlopAmount}
        onPress={isDisabled ? undefined : onPress}
        style={({ pressed }) => [
          styles.base,
          vStyles.container,
          {
            width: dimensions.size,
            height: dimensions.size,
            borderRadius,
            opacity: isDisabled ? 0.45 : 1,
            transform: [{ scale: pressed && !isDisabled ? 0.94 : 1 }],
          },
          style,
        ]}
      >
        {loading ? (
          <ActivityIndicator size="small" color={vStyles.indicatorColor} />
        ) : (
          <View style={styles.center}>{icon}</View>
        )}
      </Pressable>
    );
  },
);

IconButton.displayName = "IconButton";

const styles = StyleSheet.create({
  base: {
    alignItems: "center",
    justifyContent: "center",
  },
  center: {
    alignItems: "center",
    justifyContent: "center",
  },
});
