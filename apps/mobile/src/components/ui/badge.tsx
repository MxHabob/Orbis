/**
 * @module @orbis/mobile/components/ui/badge
 *
 * S.C.R.A.P.S. Badge Pill component for Orbis Mobile.
 * Synchronized with @orbis/ui badge variants (default, secondary, outline, destructive, lime, pink, purple).
 */

import React from "react";
import {
  View,
  Text,
  type ViewStyle,
  type TextStyle,
} from "react-native";
import { useTheme } from "@/hooks/use-theme";

export type BadgeVariant =
  | "default"
  | "primary"
  | "secondary"
  | "outline"
  | "destructive"
  | "error"
  | "warning"
  | "success"
  | "info"
  | "lime"
  | "pink"
  | "purple"
  | "neutral";

export interface BadgeProps {
  label: string;
  variant?: BadgeVariant;
  size?: "sm" | "md";
  icon?: React.ReactNode;
  style?: ViewStyle;
  textStyle?: TextStyle;
}

export const Badge = React.memo<BadgeProps>(
  ({ label, variant = "default", size = "md", icon, style, textStyle }) => {
    const theme = useTheme();

    const getColors = (): { bg: string; text: string; border?: string } => {
      switch (variant) {
        case "secondary":
        case "neutral":
          return {
            bg: theme.secondary,
            text: theme.secondaryForeground,
            border: theme.border,
          };
        case "outline":
          return {
            bg: "transparent",
            text: theme.foreground,
            border: theme.border,
          };
        case "destructive":
        case "error":
          return {
            bg: theme.errorLight,
            text: theme.destructive,
            border: theme.errorLight,
          };
        case "warning":
          return {
            bg: theme.warningLight,
            text: theme.warning,
            border: theme.warningLight,
          };
        case "success":
          return {
            bg: theme.successLight,
            text: theme.success,
            border: theme.successLight,
          };
        case "info":
          return {
            bg: theme.infoLight,
            text: theme.info,
            border: theme.infoLight,
          };
        case "lime":
          return { bg: "#ECFCCB", text: "#3F6212", border: "#D9F99D" };
        case "pink":
          return { bg: "#FCE7F3", text: "#BE185D", border: "#FBCFE8" };
        case "purple":
        case "primary":
        case "default":
        default:
          return {
            bg: theme.primaryLight,
            text: theme.primary,
            border: theme.primaryLight,
          };
      }
    };

    const colors = getColors();
    const isSm = size === "sm";

    return (
      <View
        className="flex-row items-center self-start gap-1 rounded-full"
        style={[
          {
            backgroundColor: colors.bg,
            borderColor: colors.border || "transparent",
            borderWidth: colors.border ? 1 : 0,
            paddingVertical: isSm ? 2 : 4,
            paddingHorizontal: isSm ? 8 : 10,
            borderRadius: 9999,
          },
          style,
        ]}
      >
        {icon && <View className="mr-0.5">{icon}</View>}
        <Text
          className="text-center"
          style={[
            isSm ? {"fontSize":11,"lineHeight":14,"fontWeight":"500"} : {"fontSize":12,"lineHeight":16,"fontWeight":"600"},
            { color: colors.text },
            textStyle,
          ]}
        >
          {label}
        </Text>
      </View>
    );
  },
);

Badge.displayName = "Badge";
