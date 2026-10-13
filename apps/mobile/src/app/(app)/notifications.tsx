/**
 * @module @orbis/mobile/app/(app)/notifications
 *
 * Notification center empty state for the currently unavailable inbox.
 */

import { Bell } from "@orbis/icons/native";
import { useRouter } from "expo-router";
import { Text, View } from "react-native";
import { useSafeAreaInsets } from "react-native-safe-area-context";
import { HEADER_CONTENT_OFFSET, Header } from "@/components/header";
import { ScreenScrollView } from "@/components/layout/screen-scroll-view";
import { useTheme } from "@/hooks/use-theme";
import { useI18n } from "@/lib/i18n";

export default function NotificationsScreen() {
  const { t } = useI18n();
  const router = useRouter();
  const insets = useSafeAreaInsets();
  const theme = useTheme();

  return (
    <View className="flex-1 bg-background">
      <Header
        title={t("home_notifications")}
        showBack
        onBack={() => router.back()}
      />

      <ScreenScrollView
        edges={["bottom", "left", "right"]}
        contentContainerStyle={{
          paddingTop: insets.top + HEADER_CONTENT_OFFSET,
        }}
        scrollViewProps={{
          contentContainerClassName: "flex-grow px-four pb-eight",
        }}
      >
        <View className="flex-1 items-center justify-center px-two">
          <View className="mb-six h-20 w-20 items-center justify-center rounded-full bg-primary/10">
            <Bell size={36} color={theme.primary} />
          </View>
          <Text className="mb-two text-center text-lg leading-6 font-bold text-foreground">
            {t("notifications_inbox_unavailable_title")}
          </Text>
          <Text className="max-w-[340px] text-center text-sm leading-6 text-muted-foreground">
            {t("notifications_inbox_unavailable_description")}
          </Text>
        </View>
      </ScreenScrollView>
    </View>
  );
}
