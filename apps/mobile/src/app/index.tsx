/**
 * @module @orbis/mobile/app/index
 *
 * Entry routing gate. Restores authenticated sessions into the app and opens
 * the public visitor experience when signed out.
 */

import { Redirect } from "expo-router";
import { View } from "react-native";
import { LoadingIndicator } from "@/components/ui/loading-indicator";
import { useAuth } from "@/lib/api";

export default function IndexGate() {
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
  return isAuthenticated ? (
    <Redirect href="/(app)/(tabs)" />
  ) : (
    <Redirect href="/(auth)/sign-in" />
  );
}
