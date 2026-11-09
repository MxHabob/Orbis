/**
 * @module @orbis/mobile/components/ui/badge
 *
 * S.C.R.A.P.S. Badge Pill component for Orbis Mobile.
 * Synchronized with @orbis/ui badge variants (default, secondary, outline, destructive, lime, pink, purple).
 */

import React from "react";
import { View, Text, type ViewStyle, type TextStyle } from "react-native";

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
  label?: string;
  variant?: BadgeVariant;
  size?: "sm" | "md";
  icon?: React.ReactNode;
  style?: ViewStyle;
  textStyle?: TextStyle;
  className?: string;
  children?: React.ReactNode;
}

const badgeVariantClasses: Record<
  BadgeVariant,
  { container: string; text: string }
> = {
  default: {
    container: "border-primary/20 bg-primary/10",
    text: "text-primary",
  },
  primary: {
    container: "border-primary/20 bg-primary/10",
    text: "text-primary",
  },
  secondary: {
    container: "border-border bg-secondary",
    text: "text-secondary-foreground",
  },
  outline: {
    container: "border-border bg-transparent",
    text: "text-foreground",
  },
  destructive: {
    container: "border-destructive/20 bg-destructive/10",
    text: "text-destructive",
  },
  error: {
    container: "border-destructive/20 bg-destructive/10",
    text: "text-destructive",
  },
  warning: {
    container: "border-warning/30 bg-warning-light",
    text: "text-warning",
  },
  success: {
    container: "border-success/30 bg-success-light",
    text: "text-success",
  },
  info: { container: "border-info/30 bg-info-light", text: "text-info" },
  lime: {
    container: "border-brand-lime/30 bg-brand-lime/15",
    text: "text-brand-lime-dark",
  },
  pink: {
    container: "border-brand-pink/20 bg-brand-pink/10",
    text: "text-brand-pink",
  },
  purple: {
    container: "border-primary/20 bg-primary/10",
    text: "text-primary",
  },
  neutral: {
    container: "border-border bg-secondary",
    text: "text-secondary-foreground",
  },
};

export const Badge = React.memo<BadgeProps>(
  ({
    label,
    variant = "default",
    size = "md",
    icon,
    style,
    textStyle,
    className,
    children,
  }) => {
    const isSm = size === "sm";
    const variantClasses = badgeVariantClasses[variant];

    return (
      <View
        className={`flex-row items-center self-start gap-1 rounded-full border ${isSm ? "px-two py-half" : "px-[10px] py-one"} ${variantClasses.container} ${className ?? ""}`}
        style={style}
      >
        {icon && <View className="mr-0.5">{icon}</View>}
        {children !== undefined ? (
          typeof children === "string" || typeof children === "number" ? (
            <Text
              className={`text-center ${isSm ? "text-[11px] leading-[14px] font-medium" : "text-xs leading-4 font-semibold"} ${variantClasses.text}`}
              style={textStyle}
            >
              {children}
            </Text>
          ) : (
            children
          )
        ) : label ? (
          <Text
            className={`text-center ${isSm ? "text-[11px] leading-[14px] font-medium" : "text-xs leading-4 font-semibold"} ${variantClasses.text}`}
            style={textStyle}
          >
            {label}
          </Text>
        ) : null}
      </View>
    );
  },
);

Badge.displayName = "Badge";
