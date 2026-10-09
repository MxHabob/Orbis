import { FeatureCard } from "@/components/feature-card";
import { ScreenScrollView } from "@/components/layout/screen-scroll-view";
import { Button } from "@/components/ui/button";
import { useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";
import { useRouter } from "expo-router";
import { Alert, Text, View } from "react-native";

export default function ProfileIndexScreen() {
  const router = useRouter();
  const { user, logout } = useAuth();
  const { t, isArabic } = useI18n();

  const displayName = user
    ? isArabic
      ? user.full_name_ar || user.full_name
      : user.full_name
    : t("profile_user_default");

  const handleLogout = () => {
    Alert.alert(t("profile_sign_out"), t("profile_sign_out_confirm"), [
      { text: t("common_cancel"), style: "cancel" },
      {
        text: t("profile_sign_out_button"),
        style: "destructive",
        onPress: () => {
          void logout().then(() => router.replace("/(auth)/sign-in"));
        },
      },
    ]);
  };

  return (
    <ScreenScrollView
      horizontalPadding="five"
      verticalPadding="four"
      scrollViewProps={{ contentContainerClassName: "gap-six pb-10" }}
    >
      <View className="gap-one">
        <Text className="text-2xl font-bold text-foreground">
          {t("profile_title")}
        </Text>
        <Text className="text-base font-semibold text-primary">
          {displayName}
        </Text>
        {user?.email ? (
          <Text className="text-sm text-muted-foreground">{user.email}</Text>
        ) : null}
      </View>

      <View className="gap-three">
        <FeatureCard
          title={t("profile_menu_account")}
          description={t("profile_account_description")}
          onPress={() => router.push("/(app)/profile/account")}
        />
        <FeatureCard
          title={t("profile_menu_security")}
          description={t("profile_security_description")}
          onPress={() => router.push("/(app)/profile/security")}
        />
        <FeatureCard
          title={t("profile_menu_preferences")}
          description={t("profile_preferences_description")}
          onPress={() => router.push("/(app)/profile/preferences")}
        />
      </View>

      <Button
        title={t("profile_sign_out_button")}
        variant="destructive"
        onPress={handleLogout}
      />
    </ScreenScrollView>
  );
}
