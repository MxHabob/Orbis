/**
 * @module @orbis/mobile/components/ui/themed-status-bar
 */

import { useColorScheme } from "react-native";

import { StatusBar as ExpoStatusBar } from "expo-status-bar";

type ThemedStatusBarProps = {
  backgroundColor?: string;
  style?: "auto" | "light" | "dark" | "inverted";
};

export function ThemedStatusBar({ style }: ThemedStatusBarProps) {
  const colorScheme = useColorScheme();
  const isDark = colorScheme === "dark";
  const resolvedStyle = style ?? (isDark ? "light" : "dark");

  return <ExpoStatusBar style={resolvedStyle} animated />;
}
