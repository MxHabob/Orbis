/**
 * @module @orbis/mobile/components/ui/themed-status-bar
 */

import {
  Platform,
  useColorScheme,
  StatusBar as RNStatusBar,
} from "react-native";

import { StatusBar as ExpoStatusBar } from "expo-status-bar";

type ThemedStatusBarProps = {
  backgroundColor?: string;
  style?: "auto" | "light" | "dark" | "inverted";
};

export function ThemedStatusBar({ backgroundColor, style }: ThemedStatusBarProps) {
  const colorScheme = useColorScheme();
  const isDark = colorScheme === "dark";
  const resolvedStyle = style ?? (isDark ? "light" : "dark");

  const shouldRenderAndroidBar =
    Platform.OS === "android" && backgroundColor !== undefined;

  const androidBarStyle =
    resolvedStyle === "light"
      ? "light-content"
      : resolvedStyle === "dark"
        ? "dark-content"
        : isDark
          ? "light-content"
          : "dark-content";

  return (
    <>
      <ExpoStatusBar style={resolvedStyle} animated />

      {shouldRenderAndroidBar && (
        <RNStatusBar
          translucent={backgroundColor === "transparent"}
          backgroundColor={backgroundColor}
          barStyle={androidBarStyle}
          animated
        />
      )}
    </>
  );
}

