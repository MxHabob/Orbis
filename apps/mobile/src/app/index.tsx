/**
 * @module @orbis/mobile/app/index
 *
 * Entry routing gate. Automatically redirects to (app)/(tabs) if authenticated
 * or (auth)/sign-in if unauthenticated.
 */

import { View, ActivityIndicator } from "react-native";
import { Redirect } from "expo-router";
import { useAuth } from "@/lib/api";
import { useTheme } from "@/hooks/use-theme";

export default function IndexGate() {
  const { isAuthenticated, isLoading } = useAuth();
  const theme = useTheme();

  if (isLoading) {
    return (
      <View
        className="flex-1 items-center justify-center bg-background"
      >
        <ActivityIndicator size="large" color={theme.primary} />
      </View>
    );
  }

  if (isAuthenticated) {
    return <Redirect href="/(app)/(tabs)" />;
  }

  return <Redirect href="/(auth)/sign-in" />;
}
