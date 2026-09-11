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

  const shouldRenderAndroidBar =
    Platform.OS === "android" && backgroundColor !== undefined;

  return (
    <>
      <ExpoStatusBar style={isDark ? "light" : "dark"} animated />

      {shouldRenderAndroidBar && (
        <RNStatusBar
          translucent={backgroundColor === "transparent"}
          backgroundColor={backgroundColor}
          barStyle={isDark ? "light-content" : "dark-content"}
          animated
        />
      )}
    </>
  );
}
