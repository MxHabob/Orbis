/**
 * @module @orbis/mobile/app/+not-found
 *
 * 404 Fallback Screen.
 */

import { View } from "react-native";
import { Stack, router } from "expo-router";
import { useSafeAreaInsets } from "react-native-safe-area-context";
import { HEADER_CONTENT_OFFSET, Header } from "@/components/header";
import { EmptyState } from "@/components/ui/empty-state";

export default function NotFoundScreen() {
  const insets = useSafeAreaInsets();

  return (
    <>
      <Stack.Screen options={{ headerShown: false }} />
      <View className="flex-1 bg-background">
        <Header
          title="الصفحة غير موجودة"
          showBack
          onBack={() => router.replace("/")}
        />
        <View
          className="flex-1 items-center justify-center px-four"
          style={{ paddingTop: insets.top + HEADER_CONTENT_OFFSET }}
        >
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
