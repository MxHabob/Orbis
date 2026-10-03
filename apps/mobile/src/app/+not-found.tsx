/**
 * @module @orbis/mobile/app/+not-found
 *
 * 404 Fallback Screen.
 */

import { StyleSheet, View } from "react-native";
import { Stack, router } from "expo-router";
import { Header } from "@/components/header";
import { EmptyState } from "@/components/ui/empty-state";
import { useTheme } from "@/hooks/use-theme";

export default function NotFoundScreen() {
  const theme = useTheme();

  return (
    <>
      <Stack.Screen options={{ headerShown: false }} />
      <View style={[styles.container, { backgroundColor: theme.background }]}>
        <Header
          title="الصفحة غير موجودة"
          showBack
          onBack={() => router.replace("/")}
        />
        <View style={styles.center}>
          <EmptyState
            icon="alert-circle-outline"
            title="لم نتمكن من العثور على هذه الصفحة"
            description="الرابط المطلوب غير متوفر أو تم نقله. يمكنك العودة إلى الصفحة الرئيسية."
            actionTitle="العودة للرئيسية"
            onAction={() => router.replace("/")}
          />
        </View>
      </View>
    </>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  center: {
    flex: 1,
    alignItems: "center",
    justifyContent: "center",
  },
});
