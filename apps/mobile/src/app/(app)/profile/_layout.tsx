/**
 * @module @orbis/mobile/app/(app)/settings/_layout
 *
 * Nested Stack Navigator for Settings and Security Workflows.
 */

import { Stack } from "expo-router";

export default function SettingsLayout() {
  return (
    <Stack
      screenOptions={{
        headerShown: false,
        animation: "slide_from_right",
      }}
    >
      <Stack.Screen name="index" options={{ headerShown: false }} />
      <Stack.Screen name="account" options={{ headerShown: false }} />
      <Stack.Screen name="security" options={{ headerShown: false }} />
      <Stack.Screen name="preferences" options={{ headerShown: false }} />
      <Stack.Screen name="institutions" options={{ headerShown: false }} />
    </Stack>
  );
}
