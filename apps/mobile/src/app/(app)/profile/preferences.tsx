import { FeatureCard } from "@/components/feature-card";
import { ScreenScrollView } from "@/components/layout/screen-scroll-view";
import { Card } from "@/components/ui/card";
import { useToast } from "@/components/ui/toast";
import { useI18n } from "@/lib/i18n";
import {
  applyThemePreference,
  loadThemePreference,
  saveThemePreference,
  type ThemePreference,
} from "@/lib/theme-preference";
import { useRouter } from "expo-router";
import { useEffect, useState } from "react";
import { Alert, Pressable, Text, View } from "react-native";

const themeOptions: ThemePreference[] = ["system", "light", "dark"];

export default function PreferencesScreen() {
  const router = useRouter();
  const toast = useToast();
  const { t, locale, setLocale } = useI18n();
  const [themePreference, setThemePreference] =
    useState<ThemePreference>("system");

  useEffect(() => {
    let isMounted = true;
    loadThemePreference()
      .then((preference) => {
        if (isMounted) {
          setThemePreference(preference);
          applyThemePreference(preference);
        }
      })
      .catch((error: unknown) => {
        console.error("[Orbis Mobile Theme] Failed to load saved theme.", error);
        Alert.alert(t("common_error"), t("profile_theme_load_error"));
      });
    return () => {
      isMounted = false;
    };
  }, [t]);

  const handleLocaleChange = async (nextLocale: "ar" | "en") => {
    try {
      await setLocale(nextLocale);
    } catch (error) {
      console.error("[Orbis Mobile Preferences] Failed to save language.", error);
      toast.error(t("profile_preference_save_error"));
    }
  };

  const handleThemeChange = async (preference: ThemePreference) => {
    try {
      await saveThemePreference(preference);
      setThemePreference(preference);
    } catch (error) {
      console.error("[Orbis Mobile Theme] Failed to save theme.", error);
      toast.error(t("profile_preference_save_error"));
    }
  };

  return (
    <ScreenScrollView
      horizontalPadding="five"
      verticalPadding="four"
      scrollViewProps={{ contentContainerClassName: "gap-four pb-10" }}
    >
      <View className="gap-one">
        <Text className="text-2xl font-bold text-foreground">
          {t("pref_title")}
        </Text>
      </View>

      <Card variant="outline" padding="four">
        <View className="gap-three">
          <Text className="text-base font-semibold text-foreground">
            {t("pref_language")}
          </Text>
          <PreferenceOption
            label={t("pref_language_ar")}
            selected={locale === "ar"}
            onPress={() => void handleLocaleChange("ar")}
          />
          <PreferenceOption
            label={t("pref_language_en")}
            selected={locale === "en"}
            onPress={() => void handleLocaleChange("en")}
          />
        </View>
      </Card>

      <Card variant="outline" padding="four">
        <View className="gap-three">
          <Text className="text-base font-semibold text-foreground">
            {t("pref_theme")}
          </Text>
          {themeOptions.map((option) => (
            <PreferenceOption
              key={option}
              label={t(`pref_theme_${option}`)}
              selected={themePreference === option}
              onPress={() => void handleThemeChange(option)}
            />
          ))}
        </View>
      </Card>

      <FeatureCard
        title={t("pref_notifications")}
        description={t("notifications_inbox_unavailable_description")}
        onPress={() => router.push("/(app)/notifications")}
      />
    </ScreenScrollView>
  );
}

function PreferenceOption({
  label,
  selected,
  onPress,
}: {
  label: string;
  selected: boolean;
  onPress: () => void;
}) {
  return (
    <Pressable
      accessibilityRole="radio"
      accessibilityState={{ checked: selected }}
      onPress={onPress}
      className={`min-h-12 flex-row items-center justify-between rounded-lg border px-four py-three ${
        selected ? "border-primary bg-primary/10" : "border-border bg-card"
      }`}
    >
      <Text className="flex-1 text-sm font-medium text-foreground">{label}</Text>
      <View
        className={`h-5 w-5 items-center justify-center rounded-full border ${
          selected ? "border-primary" : "border-muted-foreground"
        }`}
      >
        {selected ? (
          <View className="h-2.5 w-2.5 rounded-full bg-primary" />
        ) : null}
      </View>
    </Pressable>
  );
}
