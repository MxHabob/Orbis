/**
 * @module @orbis/mobile/app/_layout
 *
 * Root Layout for Orbis Mobile.
 * Configures Theme, Query Provider, Auth Provider, Effective Context Provider,
 * and declares root Stack navigation.
 */

import React, { useEffect } from 'react';
import { Stack, DarkTheme, DefaultTheme, ThemeProvider } from 'expo-router';
import * as SplashScreen from 'expo-splash-screen';
import { StatusBar } from 'expo-status-bar';
import { useColorScheme } from 'react-native';

import { AnimatedSplashOverlay } from '@/components/animated-icon';
import { AuthProvider } from '@/lib/api';
import { AppQueryProvider } from '@/providers/query-provider';
import { EffectiveContextProvider } from '@/providers/effective-context';

SplashScreen.preventAutoHideAsync().catch(() => {});

export default function RootLayout() {
  const colorScheme = useColorScheme();
  const theme = colorScheme === 'dark' ? DarkTheme : DefaultTheme;

  useEffect(() => {
    // Hide native splash screen after mount
    SplashScreen.hideAsync().catch(() => {});
  }, []);

  return (
    <AuthProvider>
      <AppQueryProvider>
        <EffectiveContextProvider>
          <ThemeProvider value={theme}>
            <StatusBar style={colorScheme === 'dark' ? 'light' : 'dark'} />
            <AnimatedSplashOverlay />
            <Stack
              screenOptions={{
                headerShown: false,
                animation: 'slide_from_right',
              }}>
              <Stack.Screen name="(auth)" options={{ headerShown: false }} />
              <Stack.Screen name="(app)" options={{ headerShown: false }} />
              <Stack.Screen name="+not-found" options={{ title: 'Oops!', headerShown: true }} />
            </Stack>
          </ThemeProvider>
        </EffectiveContextProvider>
      </AppQueryProvider>
    </AuthProvider>
  );
}
