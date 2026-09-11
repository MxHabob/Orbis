/**
 * @module @orbis/mobile/app/(auth)/_layout
 *
 * Stack navigator for unauthenticated flows (Sign In, Register, Password Recovery, MFA).
 */

import { useTheme } from "@/hooks/use-theme";
import { useAuth } from "@/lib/api";
import { Redirect, Stack } from "expo-router";

export default function AuthLayout() {
  const { isAuthenticated, isLoading } = useAuth();
  const theme = useTheme();

  // If already authenticated and not loading, redirect into app
  if (!isLoading && isAuthenticated) {
    return <Redirect href="/(app)/(tabs)" />;
  }

  return (
    <Stack
      screenOptions={{
        headerShown: false,
        animation: "slide_from_right",
      }}
    >
      <Stack.Screen name="sign-in" options={{ headerShown: false }} />
      <Stack.Screen name="register" options={{ headerShown: false }} />
      <Stack.Screen name="forgot-password" options={{ headerShown: false }} />
      <Stack.Screen name="wallet-setup" options={{ headerShown: false }} />
      <Stack.Screen
        name="mfa-challenge"
        options={{
          presentation: "modal",
          headerShown: false,
        }}
      />
    </Stack>
  );
}
