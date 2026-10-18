/**
 * @module @orbis/mobile/components/ui/loading-state
 *
 * Full-screen or container loading state with spinner and descriptive label.
 */

import { Text, View, type ViewStyle } from "react-native";
import { LoadingIndicator } from "./loading-indicator";

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
  return (
    <View
      className={`items-center justify-center gap-three p-six ${fullscreen ? "absolute inset-0 z-[999] bg-background" : ""}`}
      style={style}
    >
      <LoadingIndicator size="lg" />
      {message && (
        <Text className="text-center text-[13px] leading-[18px] text-muted-foreground">
          {message}
        </Text>
      )}
    </View>
  );
}
