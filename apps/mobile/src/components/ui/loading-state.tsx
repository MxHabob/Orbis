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
    padding: 24,
    alignItems: "center",
    justifyContent: "center",
    gap: 12,
  },
  fullscreen: {
    ...StyleSheet.absoluteFill,
    zIndex: 999,
  },

  message: {
    ...{"fontSize":13,"lineHeight":18,"fontWeight":"400"},
    textAlign: "center",
  },
});
