/**
 * @module @orbis/mobile/components/ui/button
 *
 * S.C.R.A.P.S. tactile button variants styled with the shared global tokens.
 */

import React from "react";
import {
  Pressable,
  Text,
  ActivityIndicator,
  type ViewStyle,
  type TextStyle,
  View,
} from "react-native";
import { useTheme } from "@/hooks/use-theme";

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
  className?: string;
  children?: React.ReactNode;
}

const variantClasses: Record<
  Exclude<ButtonVariant, "primary" | "danger">,
  { container: string; text: string }
> = {
  default: {
    container:
      "border border-primary border-b-[2.5px] border-b-btn-shadow-primary bg-primary",
    text: "font-semibold text-primary-foreground",
  },
  secondary: {
    container:
      "border border-border border-b-2 border-b-btn-shadow-secondary bg-secondary",
    text: "font-semibold text-secondary-foreground",
  },
  outline: {
    container:
      "border border-border border-b-[2.5px] border-b-border bg-card",
    text: "font-semibold text-foreground",
  },
  destructive: {
    container:
      "border border-destructive border-b-[2.5px] border-b-btn-shadow-destructive bg-destructive",
    text: "font-bold text-destructive-foreground",
  },
  lime: {
    container:
      "border border-brand-lime border-b-[2.5px] border-b-btn-shadow-lime bg-brand-lime",
    text: "font-bold text-brand-lime-foreground",
  },
  ghost: {
    container: "border-0 bg-transparent",
    text: "font-medium text-foreground",
  },
  link: {
    container: "border-0 bg-transparent px-0",
    text: "font-semibold text-primary underline",
  },
};

const sizeClasses: Record<
  ButtonSize,
  { container: string; contentGap: string; text: string; iconOnly: boolean }
> = {
  xs: {
    container: "h-[26px] gap-1 rounded-sm px-2",
    contentGap: "gap-1",
    text: "text-[11px]",
    iconOnly: false,
  },
  sm: {
    container: "h-8 gap-[5px] rounded-sm px-3",
    contentGap: "gap-[5px]",
    text: "text-xs",
    iconOnly: false,
  },
  default: {
    container: "h-[38px] gap-1.5 rounded-md px-[14px]",
    contentGap: "gap-1.5",
    text: "text-[13.5px]",
    iconOnly: false,
  },
  md: {
    container: "h-[38px] gap-1.5 rounded-md px-[14px]",
    contentGap: "gap-1.5",
    text: "text-[13.5px]",
    iconOnly: false,
  },
  lg: {
    container: "h-[46px] gap-2 rounded-md px-[18px]",
    contentGap: "gap-2",
    text: "text-[15px]",
    iconOnly: false,
  },
  icon: {
    container: "h-[38px] w-[38px] rounded-md",
    contentGap: "gap-0",
    text: "text-sm",
    iconOnly: true,
  },
  "icon-xs": {
    container: "h-[26px] w-[26px] rounded-sm",
    contentGap: "gap-0",
    text: "text-[11px]",
    iconOnly: true,
  },
  "icon-sm": {
    container: "h-8 w-8 rounded-sm",
    contentGap: "gap-0",
    text: "text-xs",
    iconOnly: true,
  },
  "icon-lg": {
    container: "h-[46px] w-[46px] rounded-md",
    contentGap: "gap-0",
    text: "text-[15px]",
    iconOnly: true,
  },
};

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
    className,
    children,
  }) => {
    const theme = useTheme();
    const normalizedVariant =
      variant === "primary"
        ? "default"
        : variant === "danger"
          ? "destructive"
          : variant;
    const variantStyle = variantClasses[normalizedVariant];
    const sizeStyle = sizeClasses[size];
    const isDisabled = disabled || loading;
    const indicatorColor =
      normalizedVariant === "secondary"
        ? theme.secondaryForeground
        : normalizedVariant === "outline" || normalizedVariant === "ghost"
          ? theme.foreground
          : normalizedVariant === "destructive"
            ? theme.destructiveForeground
            : normalizedVariant === "lime"
              ? "#152E05"
              : normalizedVariant === "link"
                ? theme.primary
                : theme.primaryForeground;

    return (
      <Pressable
        accessibilityRole="button"
        accessibilityState={{ disabled: isDisabled, busy: loading }}
        onPress={isDisabled ? undefined : onPress}
        disabled={isDisabled}
        className={`flex-row items-center justify-center ${sizeStyle.container} ${variantStyle.container} ${fullWidth ? "w-full" : ""} ${className ?? ""}`}
        style={({ pressed }) => [
          {
            opacity: isDisabled ? 0.45 : 1,
            transform: [{ translateY: pressed && !isDisabled ? 1 : 0 }],
          },
          style,
        ]}
      >
        {loading ? (
          <ActivityIndicator size="small" color={indicatorColor} />
        ) : children !== undefined ? (
          typeof children === "string" || typeof children === "number" ? (
            <Text
              className={`text-center ${sizeStyle.text} ${variantStyle.text}`}
              style={textStyle}
            >
              {children}
            </Text>
          ) : (
            children
          )
        ) : (
          <View
            className={`flex-row items-center justify-center ${sizeStyle.contentGap}`}
          >
            {icon && iconPosition === "left" && (
              <View className="items-center justify-center">{icon}</View>
            )}
            {title && (
              <Text
                className={`text-center ${sizeStyle.text} ${variantStyle.text}`}
                style={textStyle}
              >
                {title}
              </Text>
            )}
            {icon && iconPosition === "right" && (
              <View className="items-center justify-center">{icon}</View>
            )}
          </View>
        )}
      </Pressable>
    );
  },
);

Button.displayName = "Button";
