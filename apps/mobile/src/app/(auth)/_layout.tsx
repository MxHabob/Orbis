/**
 * @module @orbis/mobile/app/(auth)/_layout
 *
 * Stack navigator for unauthenticated flows (Sign In, Register, Password Recovery, MFA).
 */

import { useAuth } from "@/lib/api";
import { Redirect, Stack } from "expo-router";
import { View } from "react-native";
import { LoadingIndicator } from "@/components/ui/loading-indicator";

export default function AuthLayout() {
  const { isAuthenticated, isInitializing, selectedInstitution } = useAuth();

  if (isInitializing) {
    return (
      <View className="flex-1 items-center justify-center bg-background">
        <LoadingIndicator size="lg" />
      </View>
    );
  }

  if (!selectedInstitution) {
    return <Redirect href="/institution-select" />;
  }

  if (isAuthenticated) {
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
      <Stack.Screen name="explore" options={{ headerShown: false }} />
      <Stack.Screen name="register" options={{ headerShown: false }} />
      <Stack.Screen name="forgot-password" options={{ headerShown: false }} />
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
