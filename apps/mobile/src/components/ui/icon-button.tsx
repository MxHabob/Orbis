/**
 * @module @orbis/mobile/components/ui/icon-button
 *
 * Tactile Icon Button with accessible hitSlop, active state animation, and theme support.
 */

import React from "react";
import {
  type ViewStyle,
  ActivityIndicator,
  View,
} from "react-native";
import { useTheme } from "@/hooks/use-theme";
import { AnimatedPressable, useTactilePress } from "./tactile-press";

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

    const variantClasses: Record<
      IconButtonVariant,
      { container: string; indicatorColor: string }
    > = {
      default: {
        container:
          "border border-primary border-b-2 border-b-btn-shadow-primary bg-primary",
        indicatorColor: theme.primaryForeground,
      },
      secondary: {
        container:
          "border border-border border-b-2 border-b-btn-shadow-secondary bg-secondary",
        indicatorColor: theme.secondaryForeground,
      },
      outline: {
        container: "border border-border border-b-2 border-b-border bg-card",
        indicatorColor: theme.foreground,
      },
      destructive: {
        container:
          "border border-destructive border-b-2 border-b-btn-shadow-destructive bg-destructive",
        indicatorColor: theme.destructiveForeground,
      },
      ghost: {
        container: "bg-transparent",
        indicatorColor: theme.foreground,
      },
    };

    const variantStyle = variantClasses[variant];
    const shadowColor =
      variant === "destructive"
        ? theme.btnShadowDestructive
        : variant === "secondary" || variant === "outline"
          ? theme.btnShadowSecondary
          : theme.btnShadowPrimary;
    const tactilePress = useTactilePress(
      variant !== "ghost",
      isDisabled,
      shadowColor,
    );

    // Hit slop expands smaller buttons to minimum 44px touch area
    const hitSlopAmount = Math.max(0, (44 - dimensions.size) / 2);

    return (
      <AnimatedPressable
        accessibilityRole="button"
        accessibilityLabel={accessibilityLabel}
        disabled={isDisabled}
        hitSlop={hitSlopAmount}
        onPress={isDisabled ? undefined : onPress}
        onPressIn={tactilePress.onPressIn}
        onPressOut={tactilePress.onPressOut}
        className={`items-center justify-center ${rounded ? "rounded-full" : "rounded-md"} ${variantStyle.container}`}
        style={[
          tactilePress.style,
          {
            width: dimensions.size,
            height: dimensions.size,
            opacity: isDisabled ? 0.45 : 1,
          },
          style,
        ]}
      >
        {loading ? (
          <ActivityIndicator size="small" color={variantStyle.indicatorColor} />
        ) : (
          <View className="items-center justify-center">{icon}</View>
        )}
      </AnimatedPressable>
    );
  },
);

IconButton.displayName = "IconButton";
