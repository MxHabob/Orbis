/**
 * @module @orbis/mobile/app/_layout
 *
 * Root Layout for Orbis Mobile.
 * Configures Theme, Query Provider, Auth Provider, Effective Context Provider,
 * and declares root Stack navigation with synchronized StatusBar background and SystemUI window color.
 */

import React, { useEffect, useMemo, useRef, useState } from "react";
import "../global.css";
import { Stack, DarkTheme, DefaultTheme, ThemeProvider } from "expo-router";
import * as SplashScreen from "expo-splash-screen";
import * as SystemUI from "expo-system-ui";
import { useColorScheme } from "react-native";

import { useTheme } from "@/hooks/use-theme";
import { AppProviders } from "@/providers/app-providers";
import { ThemedStatusBar } from "@/components/ui/themed-status-bar";
import {
  applyThemePreference,
  loadThemePreference,
} from "@/lib/theme-preference";
import { ToastProvider } from "@/components/ui/toast";
import { AppErrorBoundary } from "@/components/ui/error-boundary";
import { OfflineStatusBanner } from "@/components/ui/offline-status-banner";

void SplashScreen.preventAutoHideAsync().catch((error: unknown) => {
  console.error(
    "[Orbis Mobile Startup] Failed to retain the native splash screen.",
    error,
  );
});

export default function RootLayout() {
  return (
    <AppProviders>
      <RootLayoutContent />
    </AppProviders>
  );
}

function RootLayoutContent() {
  const [isThemeReady, setIsThemeReady] = useState(false);
  const hasHiddenSplash = useRef(false);
  const colorScheme = useColorScheme();
  const isDark = colorScheme === "dark";
  const currentColors = useTheme();

  const navigationTheme = useMemo(() => {
    const base = isDark ? DarkTheme : DefaultTheme;
    return {
      ...base,
      colors: {
        ...base.colors,
        background: currentColors.background,
        card: currentColors.card,
        text: currentColors.text,
        border: currentColors.border,
        primary: currentColors.primary,
      },
    };
  }, [
    isDark,
    currentColors.background,
    currentColors.border,
    currentColors.card,
    currentColors.primary,
    currentColors.text,
  ]);

  useEffect(() => {
    loadThemePreference()
      .then(applyThemePreference)
      .catch((error: unknown) => {
        console.error("[Orbis Mobile Theme] Failed to load saved theme.", error);
      })
      .finally(() => {
        setIsThemeReady(true);
      });
  }, []);

  useEffect(() => {
    if (hasHiddenSplash.current) return;

    hasHiddenSplash.current = true;
    void SplashScreen.hideAsync().catch((error: unknown) => {
      console.error(
        "[Orbis Mobile Startup] Failed to hide the native splash screen.",
        error,
      );
    });
  }, []);

  useEffect(() => {
    if (!isThemeReady) return;

    SystemUI.setBackgroundColorAsync(currentColors.background).catch(
      (error: unknown) => {
        console.error(
          "[Orbis Mobile Theme] Failed to synchronize the system background.",
          error,
        );
      },
    );
  }, [currentColors.background, isThemeReady]);

  return (
    <ThemeProvider value={navigationTheme}>
      <ToastProvider>
        <AppErrorBoundary>
          <ThemedStatusBar backgroundColor={currentColors.background} />
          <OfflineStatusBanner />
          <Stack
            screenOptions={{
              headerShown: false,
              animation: "slide_from_right",
              contentStyle: {
                backgroundColor: currentColors.background,
              },
            }}
          >
            <Stack.Screen name="(auth)" options={{ headerShown: false }} />
            <Stack.Screen name="(app)" options={{ headerShown: false }} />
            <Stack.Screen name="institution-select" options={{ headerShown: false }} />
            <Stack.Screen
              name="institution-scan"
              options={{
                presentation: "modal",
                animation: "slide_from_bottom",
                headerShown: false,
                contentStyle: { backgroundColor: "#000000" },
              }}
            />
            <Stack.Screen
              name="+not-found"
              options={{ title: "Oops!", headerShown: true }}
            />
          </Stack>
        </AppErrorBoundary>
      </ToastProvider>
    </ThemeProvider>
  );
}
