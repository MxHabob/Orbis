/**
 * @module @orbis/mobile/components/ui/button
 *
 * S.C.R.A.P.S. Tactile Button Component for Orbis Mobile.
 * 100% synchronized with @orbis/ui (packages/ui/src/components/ui/button/variants.ts & globals.css).
 *
 * Implements tactile 3D depth, variants (primary/default, secondary, outline, ghost, destructive, lime, link),
 * multiple sizes (xs, sm, default/md, lg, icon), and smooth active transitions.
 */

import React from "react";
import {
  Pressable,
  Text,
  ActivityIndicator,
  StyleSheet,
  type ViewStyle,
  type TextStyle,
  View,
} from "react-native";
import { useTheme } from "@/hooks/use-theme";
import { Radii, Spacing, Typography } from "@/constants/theme";

export type ButtonVariant =
  | "default"
  | "primary"
  | "secondary"
  | "outline"
  | "destructive"
  | "danger"
  | "lime"
  | "ghost"
  | "link";

export type ButtonSize =
  | "xs"
  | "sm"
  | "default"
  | "md"
  | "lg"
  | "icon"
  | "icon-xs"
  | "icon-sm"
  | "icon-lg";

export interface ButtonProps {
  title?: string;
  onPress: () => void;
  variant?: ButtonVariant;
  size?: ButtonSize;
  loading?: boolean;
  disabled?: boolean;
  icon?: React.ReactNode;
  iconPosition?: "left" | "right";
  style?: ViewStyle;
  textStyle?: TextStyle;
  fullWidth?: boolean;
  children?: React.ReactNode;
}

export const Button = React.memo<ButtonProps>(
  ({
    title,
    onPress,
    variant = "default",
    size = "default",
    loading = false,
    disabled = false,
    icon,
    iconPosition = "left",
    style,
    textStyle,
    fullWidth = false,
    children,
  }) => {
    const theme = useTheme();

    const normalizedVariant: ButtonVariant =
      variant === "primary"
        ? "default"
        : variant === "danger"
          ? "destructive"
          : variant;

    // 1. Variant Styles (Tactile 3D Depth + Backgrounds + Borders)
    const getVariantStyles = (): {
      container: ViewStyle;
      text: TextStyle;
      indicatorColor: string;
    } => {
      switch (normalizedVariant) {
        case "secondary":
          return {
            container: {
              backgroundColor: theme.secondary,
              borderColor: theme.border,
              borderWidth: 1,
              borderBottomWidth: 2,
              borderBottomColor: theme.btnShadowSecondary,
            },
            text: {
              color: theme.secondaryForeground,
              fontWeight: "600",
            },
            indicatorColor: theme.secondaryForeground,
          };

        case "outline":
          return {
            container: {
              backgroundColor: theme.card,
              borderColor: theme.border,
              borderWidth: 1.5,
              borderBottomWidth: 2.5,
              borderBottomColor: theme.border,
            },
            text: {
              color: theme.foreground,
              fontWeight: "600",
            },
            indicatorColor: theme.foreground,
          };

        case "destructive":
          return {
            container: {
              backgroundColor: theme.destructive,
              borderWidth: 1,
              borderColor: theme.destructive,
              borderBottomWidth: 2.5,
              borderBottomColor: theme.btnShadowDestructive,
            },
            text: {
              color: theme.destructiveForeground,
              fontWeight: "700",
            },
            indicatorColor: theme.destructiveForeground,
          };

        case "lime":
          return {
            container: {
              backgroundColor: theme.brandLime,
              borderWidth: 1,
              borderColor: theme.brandLime,
              borderBottomWidth: 2.5,
              borderBottomColor: theme.btnShadowLime,
            },
            text: {
              color: "#152E05",
              fontWeight: "700",
            },
            indicatorColor: "#152E05",
          };

        case "ghost":
          return {
            container: {
              backgroundColor: "transparent",
              borderWidth: 0,
            },
            text: {
              color: theme.foreground,
              fontWeight: "500",
            },
            indicatorColor: theme.foreground,
          };

        case "link":
          return {
            container: {
              backgroundColor: "transparent",
              borderWidth: 0,
              paddingHorizontal: 0,
            },
            text: {
              color: theme.primary,
              fontWeight: "600",
              textDecorationLine: "underline",
            },
            indicatorColor: theme.primary,
          };

        case "default":
        default:
          return {
            container: {
              backgroundColor: theme.primary,
              borderWidth: 1,
              borderColor: theme.primary,
              borderBottomWidth: 2.5,
              borderBottomColor: theme.btnShadowPrimary,
            },
            text: {
              color: theme.primaryForeground,
              fontWeight: "600",
            },
            indicatorColor: theme.primaryForeground,
          };
      }
    };

    // 2. Size Dimensions matching packages/ui
    const getSizeStyles = (): {
      height: number;
      paddingHorizontal: number;
      borderRadius: number;
      fontSize: number;
      gap: number;
      isIconOnly: boolean;
    } => {
      switch (size) {
        case "xs":
          return {
            height: 26,
            paddingHorizontal: 8,
            borderRadius: Radii.sm,
            fontSize: 11,
            gap: 4,
            isIconOnly: false,
          };
        case "sm":
          return {
            height: 32,
            paddingHorizontal: 12,
            borderRadius: Radii.sm,
            fontSize: 12,
            gap: 5,
            isIconOnly: false,
          };
        case "lg":
          return {
            height: 46,
            paddingHorizontal: 18,
            borderRadius: Radii.md,
            fontSize: 15,
            gap: 8,
            isIconOnly: false,
          };
        case "icon-xs":
          return {
            height: 26,
            paddingHorizontal: 0,
            borderRadius: Radii.sm,
            fontSize: 11,
            gap: 0,
            isIconOnly: true,
          };
        case "icon-sm":
          return {
            height: 32,
            paddingHorizontal: 0,
            borderRadius: Radii.sm,
            fontSize: 12,
            gap: 0,
            isIconOnly: true,
          };
        case "icon-lg":
          return {
            height: 46,
            paddingHorizontal: 0,
            borderRadius: Radii.md,
            fontSize: 15,
            gap: 0,
            isIconOnly: true,
          };
        case "icon":
          return {
            height: 38,
            paddingHorizontal: 0,
            borderRadius: Radii.md,
            fontSize: 14,
            gap: 0,
            isIconOnly: true,
          };
        case "md":
        case "default":
        default:
          return {
            height: 38,
            paddingHorizontal: 14,
            borderRadius: Radii.md,
            fontSize: 13.5,
            gap: 6,
            isIconOnly: false,
          };
      }
    };

    const vStyles = getVariantStyles();
    const sStyles = getSizeStyles();
    const isDisabled = disabled || loading;

    return (
      <Pressable
        onPress={isDisabled ? undefined : onPress}
        disabled={isDisabled}
        style={({ pressed }) => [
          styles.base,
          vStyles.container,
          {
            height: sStyles.height,
            width: sStyles.isIconOnly
              ? sStyles.height
              : fullWidth
                ? "100%"
                : undefined,
            paddingHorizontal: sStyles.paddingHorizontal,
            borderRadius: sStyles.borderRadius,
            opacity: isDisabled ? 0.45 : 1,
            transform: [{ translateY: pressed && !isDisabled ? 1 : 0 }],
          },
          style,
        ]}
      >
        {loading ? (
          <ActivityIndicator size="small" color={vStyles.indicatorColor} />
        ) : children ? (
          children
        ) : (
          <View style={[styles.contentRow, { gap: sStyles.gap }]}>
            {icon && iconPosition === "left" && (
              <View style={styles.iconBox}>{icon}</View>
            )}
            {title && (
              <Text
                style={[
                  styles.text,
                  { fontSize: sStyles.fontSize },
                  vStyles.text,
                  textStyle,
                ]}
              >
                {title}
              </Text>
            )}
            {icon && iconPosition === "right" && (
              <View style={styles.iconBox}>{icon}</View>
            )}
          </View>
        )}
      </Pressable>
    );
  },
);

Button.displayName = "Button";

const styles = StyleSheet.create({
  base: {
    alignItems: "center",
    justifyContent: "center",
    flexDirection: "row",
  },
  contentRow: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "center",
  },
  text: {
    textAlign: "center",
  },
  iconBox: {
    alignItems: "center",
    justifyContent: "center",
  },
});
