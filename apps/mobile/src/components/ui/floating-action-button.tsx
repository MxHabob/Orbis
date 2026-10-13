/**
 * @module @orbis/mobile/components/ui/floating-action-button
 *
 * Floating Action Button (FAB) with tactile depth and elevation.
 */

import React from "react";
import { StyleSheet, Text, View, type ViewStyle } from "react-native";
import { useTheme } from "@/hooks/use-theme";
import { LoadingIndicator } from "./loading-indicator";
import { AnimatedPressable, useTactilePress } from "./tactile-press";

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
    const tactilePress = useTactilePress(
      true,
      isDisabled,
      theme.btnShadowPrimary,
    );

    const positionStyle: ViewStyle = {
      position: "absolute",
      bottom: offsetBottom,
      ...(position === "bottom-right" ? { right: offsetHorizontal } : {}),
      ...(position === "bottom-left" ? { left: offsetHorizontal } : {}),
      ...(position === "bottom-center" ? { left: "50%", marginLeft: -28 } : {}),
    };

    return (
      <AnimatedPressable
        accessibilityRole="button"
        accessibilityLabel={accessibilityLabel}
        disabled={isDisabled}
        onPress={isDisabled ? undefined : onPress}
        onPressIn={tactilePress.onPressIn}
        onPressOut={tactilePress.onPressOut}
        className={`absolute z-[90] items-center justify-center border border-b-[3px] border-primary border-b-btn-shadow-primary bg-primary ${label ? "h-12 rounded-full px-four" : "h-14 w-14 rounded-full"}`}
        style={[
          styles.elevation,
          positionStyle,
          tactilePress.style,
          {
            opacity: isDisabled ? 0.6 : 1,
          },
          style,
        ]}
      >
        {loading ? (
          <LoadingIndicator
            size="sm"
            color={theme.primaryForeground}
            containerColor="transparent"
          />
        ) : (
          <View className="flex-row items-center justify-center gap-two">
            {icon}
            {label ? (
              <Text className="text-sm leading-5 font-semibold text-primary-foreground">
                {label}
              </Text>
            ) : null}
          </View>
        )}
      </AnimatedPressable>
    );
  },
);

FloatingActionButton.displayName = "FloatingActionButton";

const styles = StyleSheet.create({
  elevation: {
    ...{
      shadowColor: "#000000",
      shadowOffset: { width: 0, height: 4 },
      shadowOpacity: 0.12,
      shadowRadius: 8,
      elevation: 6,
    },
  },
});
