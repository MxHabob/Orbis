/**
 * @module @orbis/mobile/components/ui/loading-state
 *
 * Full-screen or container loading state with spinner and descriptive label.
 */

import React from "react";
import {
  View,
  Text,
  ActivityIndicator,
  StyleSheet,
  type ViewStyle,
} from "react-native";
import { useTheme } from "@/hooks/use-theme";
import { Spacing, Typography } from "@/constants/theme";

export interface LoadingStateProps {
  message?: string;
  fullscreen?: boolean;
  style?: ViewStyle;
}

export function LoadingState({
  message,
  fullscreen = false,
  style,
}: LoadingStateProps) {
  const theme = useTheme();

  return (
    <View
      style={[
        styles.container,
        fullscreen && [
          styles.fullscreen,
          { backgroundColor: theme.background },
        ],
        style,
      ]}
    >
      <ActivityIndicator size="large" color={theme.primary} />
      {message && (
        <Text style={[styles.message, { color: theme.textSecondary }]}>
          {message}
        </Text>
      )}
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    padding: Spacing.six,
    alignItems: "center",
    justifyContent: "center",
    gap: Spacing.three,
  },
  fullscreen: {
    ...StyleSheet.absoluteFill,
    zIndex: 999,
  },

  message: {
    ...Typography.bodySmall,
    textAlign: "center",
  },
});
