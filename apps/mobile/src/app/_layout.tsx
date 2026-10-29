/**
 * @module @orbis/mobile/app/_layout
 *
 * Root Layout for Orbis Mobile.
 * Configures Theme, Query Provider, Auth Provider, Effective Context Provider,
 * and declares root Stack navigation with synchronized StatusBar background and SystemUI window color.
 */

import React, { useEffect, useMemo, useState } from "react";
import "../global.css";
import { Stack, DarkTheme, DefaultTheme, ThemeProvider } from "expo-router";
import * as SplashScreen from "expo-splash-screen";
import * as SystemUI from "expo-system-ui";
import { Pressable, Text, View, useColorScheme } from "react-native";

import { useTheme } from "@/hooks/use-theme";
import { AppProviders } from "@/providers/app-providers";
import { ThemedStatusBar } from "@/components/ui/themed-status-bar";
import {
  applyThemePreference,
  loadThemePreference,
} from "@/lib/theme-preference";
import { ToastProvider } from "@/components/ui/toast";
import {
  AppErrorBoundary,
  type ErrorFallbackProps,
} from "@/components/ui/error-boundary";
import { OfflineStatusBanner } from "@/components/ui/offline-status-banner";

void SplashScreen.preventAutoHideAsync().catch((error: unknown) => {
  console.error(
    "[Orbis Mobile Startup] Failed to retain the native splash screen.",
    error,
  );
});

export default function RootLayout() {
  useEffect(() => {
    void SplashScreen.hideAsync().catch((error: unknown) => {
      console.error(
        "[Orbis Mobile Startup] Failed to hide the native splash screen.",
        error,
      );
    });
  }, []);

  return (
    <AppErrorBoundary fallback={StartupErrorFallback}>
      <AppProviders>
        <RootLayoutContent />
      </AppProviders>
    </AppErrorBoundary>
  );
}

function StartupErrorFallback({
  error,
  resetErrorBoundary,
}: ErrorFallbackProps) {
  return (
    <View
      style={{
        flex: 1,
        alignItems: "center",
        justifyContent: "center",
        backgroundColor: "#1f1f1f",
        padding: 24,
      }}
    >
      <Text style={{ color: "#ffffff", fontSize: 20, fontWeight: "600" }}>
        Orbis could not start
      </Text>
      <Text style={{ marginTop: 12, color: "#d1d5db", textAlign: "center" }}>
        Please try again. If the problem continues, reinstall the app.
      </Text>
      {__DEV__ && (
        <Text style={{ marginTop: 12, color: "#d1d5db", textAlign: "center" }}>
          {error.message}
        </Text>
      )}
      <Pressable
        accessibilityRole="button"
        onPress={resetErrorBoundary}
        style={{
          marginTop: 24,
          borderRadius: 8,
          backgroundColor: "#7c3aed",
          paddingHorizontal: 20,
          paddingVertical: 12,
        }}
      >
        <Text style={{ color: "#ffffff", fontWeight: "600" }}>Try again</Text>
      </Pressable>
    </View>
  );
}

function RootLayoutContent() {
  const [isThemeReady, setIsThemeReady] = useState(false);
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
        console.error(
          "[Orbis Mobile Theme] Failed to load saved theme.",
          error,
        );
      })
      .finally(() => {
        setIsThemeReady(true);
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
          <Stack.Screen
            name="institution-select"
            options={{ headerShown: false }}
          />
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
      </ToastProvider>
    </ThemeProvider>
  );
}
