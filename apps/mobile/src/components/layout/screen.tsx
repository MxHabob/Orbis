/**
 * @module @orbis/mobile/components/layout/screen
 *
 * Universal Screen wrapper for mobile pages.
 * Handles safe area insets, keyboard avoidance, and root backgrounds.
 *
 * StatusBar is managed by the root layout's ThemedStatusBar — individual
 * screens should NOT render their own StatusBar component. Use the
 * `statusBarStyle` prop when a screen needs to override the bar style
 * (e.g. light text over a dark hero header).
 */

import React from "react";
import { View, type ViewProps } from "react-native";
import { useSafeAreaInsets, type Edge } from "react-native-safe-area-context";
import { KeyboardAvoidingView } from "react-native-keyboard-controller";
import { StatusBar } from "expo-status-bar";

export interface ScreenProps extends ViewProps {
  children?: React.ReactNode;
  header?: React.ReactNode;
  footer?: React.ReactNode;
  edges?: Edge[];
  backgroundColor?: string;
  /** Override the status bar style for this screen (e.g. "light" over dark hero). */
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
  statusBarStyle,
  keyboardAvoiding = false,
  keyboardVerticalOffset = 0,
  style,
  ...props
}: ScreenProps) {
  const insets = useSafeAreaInsets();

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
      {/* Only render a StatusBar override when this screen needs a different style */}
      {statusBarStyle ? <StatusBar style={statusBarStyle} /> : null}

      {header}
      <View className="flex-1 w-full">{children}</View>
      {footer}
    </View>
  );

  if (keyboardAvoiding) {
    return (
      <KeyboardAvoidingView
        style={{ flex: 1 }}
        keyboardVerticalOffset={keyboardVerticalOffset}
      >
        {content}
      </KeyboardAvoidingView>
    );
  }

  return content;
}
