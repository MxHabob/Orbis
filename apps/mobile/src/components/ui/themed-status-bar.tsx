/**
 * @module @orbis/mobile/components/ui/themed-status-bar
 */

import { StatusBar, useColorScheme } from "react-native";

type ThemedStatusBarProps = {
  style?: "auto" | "light" | "dark" | "inverted";
};

export function ThemedStatusBar({ style }: ThemedStatusBarProps) {
  const colorScheme = useColorScheme();
  const isDark = colorScheme === "dark";
  const resolvedStyle = style ?? (isDark ? "light" : "dark");
  const barStyle =
    resolvedStyle === "light" || (resolvedStyle === "inverted" && !isDark)
      ? "light-content"
      : "dark-content";

  return <StatusBar barStyle={barStyle} animated />;
}
