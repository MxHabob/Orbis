/**
 * @module @orbis/mobile/app/+not-found
 *
 * 404 Fallback Screen.
 */

import { View } from "react-native";
import { Stack, router } from "expo-router";
import { Header } from "@/components/header";
import { EmptyState } from "@/components/ui/empty-state";

export default function NotFoundScreen() {
  return (
    <>
      <Stack.Screen options={{ headerShown: false }} />
      <View flex-1>
        <Header
          title="الصفحة غير موجودة"
          showBack
          onBack={() => router.replace("/")}
        />
        <View className="flex-1 items-center justify-center">
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
