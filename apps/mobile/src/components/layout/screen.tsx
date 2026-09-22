/**
 * @module @orbis/mobile/components/layout/screen
 *
 * Universal Screen wrapper for mobile pages.
 * Handles safe area insets, keyboard avoidance, status bar, and root backgrounds.
 */

import React from "react";
import {
  View,
  StyleSheet,
  type ViewProps,
  useColorScheme,
  KeyboardAvoidingView,
  Platform,
} from "react-native";
import { useSafeAreaInsets, type Edge } from "react-native-safe-area-context";
import { Colors } from "@/constants/theme";
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
  const colorScheme = useColorScheme();
  const theme = Colors[colorScheme === "dark" ? "dark" : "light"];
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
      style={[
        styles.container,
        { backgroundColor: resolvedBg },
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
      <View style={styles.body}>{children}</View>
      {footer}
    </View>
  );

  if (keyboardAvoiding) {
    return (
      <KeyboardAvoidingView
        style={styles.keyboardContainer}
        behavior={Platform.OS === "ios" ? "padding" : undefined}
        keyboardVerticalOffset={keyboardVerticalOffset}
      >
        {content}
      </KeyboardAvoidingView>
    );
  }

  return content;
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    width: "100%",
  },
  body: {
    flex: 1,
    width: "100%",
  },
  keyboardContainer: {
    flex: 1,
  },
});
