/**
 * @module @orbis/mobile/app/_layout
 *
 * Root Layout for Orbis Mobile.
 * Configures Theme, Query Provider, Auth Provider, Effective Context Provider,
 * and declares root Stack navigation with synchronized StatusBar background and SystemUI window color.
 */

import React, { useEffect, useMemo } from "react";
import "../global.css";
import { Stack, DarkTheme, DefaultTheme, ThemeProvider } from "expo-router";
import * as SplashScreen from "expo-splash-screen";
import * as SystemUI from "expo-system-ui";
import { useColorScheme } from "react-native";

import { useTheme } from "@/hooks/use-theme";
import { AuthProvider } from "@/lib/api";
import { AppQueryProvider } from "@/providers/query-provider";
import { EffectiveContextProvider } from "@/providers/effective-context";
import { I18nProvider } from "@/lib/i18n";
import { ThemedStatusBar } from "@/components/ui/themed-status-bar";
import { applyThemePreference, loadThemePreference } from "@/lib/theme-preference";
import { ToastProvider } from "@/components/ui/toast";
import { AppErrorBoundary } from "@/components/ui/error-boundary";
import { SafeAreaProvider } from "react-native-safe-area-context";
import { GestureHandlerRootView } from "react-native-gesture-handler";

SplashScreen.preventAutoHideAsync().catch(() => {});

export default function RootLayout() {
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
    // Hide native splash screen after mount
    SplashScreen.hideAsync().catch(() => {});
  }, []);

  useEffect(() => {
    loadThemePreference()
      .then(applyThemePreference)
      .catch((error: unknown) => {
        console.error("[Orbis Mobile Theme] Failed to load saved theme.", error);
      });
  }, []);

  useEffect(() => {
    // Dynamically synchronize the root OS window background color with active theme
    SystemUI.setBackgroundColorAsync(currentColors.background).catch(() => {});
  }, [currentColors.background]);

  return (
    <GestureHandlerRootView className="flex-1">
      <SafeAreaProvider>
        <I18nProvider>
          <AuthProvider>
            <AppQueryProvider>
              <EffectiveContextProvider>
                <ThemeProvider value={navigationTheme}>
                  <ToastProvider>
                    <AppErrorBoundary>
                      <ThemedStatusBar
                        backgroundColor={currentColors.background}
                      />
                      <Stack
                        screenOptions={{
                          headerShown: false,
                          animation: "slide_from_right",
                          contentStyle: {
                            backgroundColor: currentColors.background,
                          },
                        }}
                      >
                        <Stack.Screen
                          name="(auth)"
                          options={{ headerShown: false }}
                        />
                        <Stack.Screen
                          name="(app)"
                          options={{ headerShown: false }}
                        />
                        <Stack.Screen
                          name="+not-found"
                          options={{ title: "Oops!", headerShown: true }}
                        />
                      </Stack>
                    </AppErrorBoundary>
                  </ToastProvider>
                </ThemeProvider>
              </EffectiveContextProvider>
            </AppQueryProvider>
          </AuthProvider>
        </I18nProvider>
      </SafeAreaProvider>
    </GestureHandlerRootView>
  );
}
