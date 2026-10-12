/**
 * @module @orbis/mobile/components/layout/screen
 *
 * Universal Screen wrapper for mobile pages.
 * Handles safe area insets, keyboard avoidance, status bar, and root backgrounds.
 */

import React from "react";
import {
  View,
  type ViewProps,
  KeyboardAvoidingView,
  Platform,
} from "react-native";
import { useSafeAreaInsets, type Edge } from "react-native-safe-area-context";
import { useTheme } from "@/hooks/use-theme";
import { ThemedStatusBar } from "@/components/ui/themed-status-bar";

export interface ScreenProps extends ViewProps {
  children?: React.ReactNode;
  header?: React.ReactNode;
  footer?: React.ReactNode;
  edges?: Edge[];
  backgroundColor?: string;
  statusBarColor?: string;
  statusBarStyle?: "auto" | "light" | "dark" | "inverted";
  keyboardAvoiding?: boolean;
  keyboardVerticalOffset?: number;
}

export function Screen({
  children,
  header,
  footer,
  edges = ["top", "bottom", "left", "right"],
  backgroundColor,
  statusBarColor,
  statusBarStyle,
  keyboardAvoiding = false,
  keyboardVerticalOffset = 0,
  style,
  ...props
}: ScreenProps) {
  const theme = useTheme();
  const insets = useSafeAreaInsets();

  const resolvedBg = backgroundColor ?? theme.background;

  const edgePadding = {
    paddingTop: edges.includes("top") ? insets.top : 0,
    paddingBottom: edges.includes("bottom") ? insets.bottom : 0,
    paddingLeft: edges.includes("left") ? insets.left : 0,
    paddingRight: edges.includes("right") ? insets.right : 0,
  };

  const content = (
    <View
      className="flex-1 w-full bg-background"
      style={[
        backgroundColor ? { backgroundColor } : undefined,
        edgePadding,
        style,
      ]}
      {...props}
    >
      <ThemedStatusBar
        backgroundColor={statusBarColor ?? resolvedBg}
        style={statusBarStyle}
      />

      {header}
      <View className="flex-1 w-full">{children}</View>
      {footer}
    </View>
  );

  if (keyboardAvoiding) {
    return (
      <KeyboardAvoidingView
        className="flex-1"
        behavior={Platform.OS === "ios" ? "padding" : undefined}
        keyboardVerticalOffset={keyboardVerticalOffset}
      >
        {content}
      </KeyboardAvoidingView>
    );
  }

  return content;
}
