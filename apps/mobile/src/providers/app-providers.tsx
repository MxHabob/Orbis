import { AuthProvider } from "@/lib/api";
import { I18nProvider } from "@/lib/i18n";
import { EffectiveContextProvider } from "@/providers/effective-context";
import { AppQueryProvider } from "@/providers/query-provider";
import React from "react";
import { GestureHandlerRootView } from "react-native-gesture-handler";
import { KeyboardProvider } from "react-native-keyboard-controller";
import { SafeAreaProvider } from "react-native-safe-area-context";

export function AppProviders({ children }: { children: React.ReactNode }) {
  return (
    <GestureHandlerRootView className="flex-1">
      <KeyboardProvider>
        <SafeAreaProvider>
          <I18nProvider>
            <AuthProvider>
              <AppQueryProvider>
                <EffectiveContextProvider>{children}</EffectiveContextProvider>
              </AppQueryProvider>
            </AuthProvider>
          </I18nProvider>
        </SafeAreaProvider>
      </KeyboardProvider>
    </GestureHandlerRootView>
  );
}
