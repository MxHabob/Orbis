/**
 * @module @orbis/mobile/components/ui/themed-status-bar
 */

import { StatusBar } from "expo-status-bar";
import { useColorScheme } from "react-native";

type ThemedStatusBarProps = {
  style?: "auto" | "light" | "dark" | "inverted";
};

export function ThemedStatusBar({ style }: ThemedStatusBarProps) {
  const colorScheme = useColorScheme();
  const isDark = colorScheme === "dark";
  const resolvedStyle = style ?? (isDark ? "light" : "dark");

  return <StatusBar style={resolvedStyle} animated />;
}
