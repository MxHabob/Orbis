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
      className={`items-center justify-center gap-three p-six ${fullscreen ? "absolute inset-0 z-[999] bg-background" : ""}`}
      style={style}
    >
      <ActivityIndicator size="large" color={theme.primary} />
      {message && (
        <Text className="text-center text-[13px] leading-[18px] text-muted-foreground">
          {message}
        </Text>
      )}
    </View>
  );
}
