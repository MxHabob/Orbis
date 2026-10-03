/**
 * @module @orbis/mobile/components/ui/floating-action-button
 *
 * Floating Action Button (FAB) with tactile depth and elevation.
 */

import React from "react";
import {
  Pressable,
  StyleSheet,
  type ViewStyle,
  type TextStyle,
  Text,
  View,
  ActivityIndicator,
} from "react-native";
import { useTheme } from "@/hooks/use-theme";
import { Radii, Shadows, Spacing, Typography } from "@/constants/theme";

export interface FloatingActionButtonProps {
  icon: React.ReactNode;
  label?: string;
  onPress: () => void;
  accessibilityLabel: string;
  position?: "bottom-right" | "bottom-left" | "bottom-center";
  offsetBottom?: number;
  offsetHorizontal?: number;
  loading?: boolean;
  disabled?: boolean;
  style?: ViewStyle;
}

export const FloatingActionButton = React.memo<FloatingActionButtonProps>(
  ({
    icon,
    label,
    onPress,
    accessibilityLabel,
    position = "bottom-right",
    offsetBottom = 24,
    offsetHorizontal = 20,
    loading = false,
    disabled = false,
    style,
  }) => {
    const theme = useTheme();
    const isDisabled = disabled || loading;

    const positionStyle: ViewStyle = {
      position: "absolute",
      bottom: offsetBottom,
      ...(position === "bottom-right" ? { right: offsetHorizontal } : {}),
      ...(position === "bottom-left" ? { left: offsetHorizontal } : {}),
      ...(position === "bottom-center"
        ? { left: "50%", transform: [{ translateX: -28 }] }
        : {}),
    };

    return (
      <Pressable
        accessibilityRole="button"
        accessibilityLabel={accessibilityLabel}
        disabled={isDisabled}
        onPress={isDisabled ? undefined : onPress}
        style={({ pressed }) => [
          styles.container,
          positionStyle,
          {
            backgroundColor: theme.primary,
            borderColor: theme.primaryDark,
            borderBottomColor: theme.btnShadowPrimary,
            opacity: isDisabled ? 0.6 : 1,
            transform: [
              ...(position === "bottom-center" ? [{ translateX: -28 }] : []),
              { scale: pressed && !isDisabled ? 0.95 : 1 },
            ],
          },
          label ? styles.extendedContainer : styles.circleContainer,
          style,
        ]}
      >
        {loading ? (
          <ActivityIndicator size="small" color={theme.primaryForeground} />
        ) : (
          <View style={styles.innerRow}>
            {icon}
            {label ? (
              <Text
                style={[
                  styles.label,
                  { color: theme.primaryForeground },
                ]}
              >
                {label}
              </Text>
            ) : null}
          </View>
        )}
      </Pressable>
    );
  },
);

FloatingActionButton.displayName = "FloatingActionButton";

const styles = StyleSheet.create({
  container: {
    zIndex: 90,
    alignItems: "center",
    justifyContent: "center",
    borderWidth: 1,
    borderBottomWidth: 3,
    ...Shadows.lg,
  },
  circleContainer: {
    width: 56,
    height: 56,
    borderRadius: Radii.full,
  },
  extendedContainer: {
    height: 48,
    paddingHorizontal: Spacing.four,
    borderRadius: Radii.full,
  },
  innerRow: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "center",
    gap: Spacing.two,
  },
  label: {
    ...Typography.bodyBold,
  },
});
