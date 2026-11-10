/**
 * @module @orbis/mobile/app/index
 *
 * Entry routing gate. Restores authenticated sessions into the app and opens
 * the public visitor experience when signed out.
 */

import { Redirect } from "expo-router";
import { useEffect, useState } from "react";
import { View } from "react-native";
import { LoadingIndicator } from "@/components/ui/loading-indicator";
import { WelcomeOnboarding } from "@/components/onboarding/welcome-onboarding";
import { getSecureItem, setSecureItem, STORAGE_KEYS, useAuth } from "@/lib/api";

export default function IndexGate() {
  const { isAuthenticated, isInitializing, selectedInstitution } = useAuth();
  const [hasCompletedOnboarding, setHasCompletedOnboarding] = useState(false);
  const [isCheckingOnboarding, setIsCheckingOnboarding] = useState(true);

  useEffect(() => {
    let isMounted = true;
    getSecureItem(STORAGE_KEYS.ONBOARDING_COMPLETE)
      .then((value) => {
        if (isMounted) setHasCompletedOnboarding(value === "true");
      })
      .catch((error: unknown) => {
        console.error("[Mobile Onboarding] Failed to load completion state.", error);
      })
      .finally(() => {
        if (isMounted) setIsCheckingOnboarding(false);
      });

    return () => {
      isMounted = false;
    };
  }, []);

  async function completeOnboarding() {
    await setSecureItem(STORAGE_KEYS.ONBOARDING_COMPLETE, "true");
    setHasCompletedOnboarding(true);
  }

  if (
    isInitializing ||
    (!isAuthenticated && isCheckingOnboarding)
  ) {
    return (
      <View className="flex-1 items-center justify-center bg-background">
        <LoadingIndicator size="lg" />
      </View>
    );
  }

  if (isAuthenticated) {
    return selectedInstitution ? (
      <Redirect href="/(app)/(tabs)" />
    ) : (
      <Redirect href="/institution-select" />
    );
  }

  if (!hasCompletedOnboarding) {
    return <WelcomeOnboarding onContinue={completeOnboarding} />;
  }

  if (!selectedInstitution) {
    return <Redirect href="/institution-select" />;
  }

  return <Redirect href="/(auth)/sign-in" />;
}
