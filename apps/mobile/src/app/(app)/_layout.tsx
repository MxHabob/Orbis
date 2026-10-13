/**
 * @module @orbis/mobile/app/(app)/_layout
 *
 * Protected Route Layout for Authenticated Orbis Mobile.
 * Enforces session requirement and mounts primary App Stack.
 */

import { Redirect, Stack } from "expo-router";
import { View } from "react-native";
import { LoadingIndicator } from "@/components/ui/loading-indicator";
import { useAuth } from "@/lib/api";

export default function AppProtectedLayout() {
  const { isAuthenticated, isLoading } = useAuth();

  if (isLoading) {
    return (
      <View className="flex-1 items-center justify-center bg-background">
        <LoadingIndicator size="lg" />
      </View>
    );
  }

  // Route protection: If unauthenticated, redirect to Sign In
  if (!isAuthenticated) {
    return <Redirect href="/(auth)/sign-in" />;
  }

  return (
    <Stack
      screenOptions={{
        headerShown: false,
        animation: "slide_from_right",
      }}
    >
      <Stack.Screen name="(tabs)" options={{ headerShown: false }} />
      <Stack.Screen name="(modal)" options={{ headerShown: false }} />
      <Stack.Screen name="student/[id]" options={{ headerShown: false }} />
      <Stack.Screen name="class/[id]" options={{ headerShown: false }} />
      <Stack.Screen name="profile" options={{ headerShown: false }} />
      <Stack.Screen name="webviewer" options={{ headerShown: false }} />
    </Stack>
  );
}
