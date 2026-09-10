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
};

export function ThemedStatusBar({ backgroundColor }: ThemedStatusBarProps) {
  const colorScheme = useColorScheme();
  const isDark = colorScheme === "dark";

  return (
    <>
      <ExpoStatusBar style={isDark ? "light" : "dark"} animated />

      {Platform.OS === "android" && (
        <RNStatusBar
          translucent
          backgroundColor={backgroundColor ?? "transparent"}
          barStyle={isDark ? "light-content" : "dark-content"}
          animated
        />
      )}
    </>
  );
}
