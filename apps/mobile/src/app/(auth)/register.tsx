import { useRouter } from "expo-router";
import { View } from "react-native";

import { ScreenScrollView } from "@/components/layout/screen-scroll-view";
import { Button } from "@/components/ui/button";
import { Text } from "@/components/ui/text";
import { useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";

export default function RegisterScreen() {
  const router = useRouter();
  const { selectedInstitution } = useAuth();
  const { t, isArabic } = useI18n();

  return (
    <ScreenScrollView
      edges={["top", "bottom", "left", "right"]}
      horizontalPadding="six"
      verticalPadding="eight"
      contentContainerStyle={{ direction: isArabic ? "rtl" : "ltr" }}
    >
      <View className="flex-1 justify-between gap-eight">
        <View className="gap-three">
          <Text className="text-sm font-medium text-muted-foreground">
            {selectedInstitution?.name}
          </Text>
          <Text
            accessibilityRole="header"
            className="text-[32px] leading-[38px] font-bold text-foreground"
          >
            {t("visitor_register")}
          </Text>
          <Text className="text-base leading-6 text-muted-foreground">
            {t("auth_account_access_description")}
          </Text>
        </View>
        <Button
          title={t("auth_sign_in")}
          onPress={() => router.replace("/(auth)/sign-in")}
          fullWidth
          size="lg"
        />
      </View>
    </ScreenScrollView>
  );
}
