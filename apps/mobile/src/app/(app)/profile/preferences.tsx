/**
 * @module @orbis/mobile/app/(app)/profile/preferences
 *
 * Language, appearance, and notification settings.
 */

import React, { useEffect, useState } from "react";
import { Pressable, Text, View } from "react-native";
import { useRouter } from "expo-router";
import { useSafeAreaInsets } from "react-native-safe-area-context";

import { HEADER_CONTENT_OFFSET, Header } from "@/components/header";
import { ScreenScrollView } from "@/components/layout/screen-scroll-view";
import { Card } from "@/components/ui/card";
import {
  SettingsPanelDivider,
  SettingsPanelTitle,
} from "@/components/ui/settings-panel";
import { useToast } from "@/components/ui/toast";
import { useTheme } from "@/hooks/use-theme";
import { useI18n } from "@/lib/i18n";
import {
  loadThemePreference,
  saveThemePreference,
  type ThemePreference,
} from "@/lib/theme-preference";
import { Check, ChevronLeft, ChevronRight } from "@orbis/icons/native";

const themeOptions: ThemePreference[] = ["system", "light", "dark"];

export default function PreferencesScreen() {
  const router = useRouter();
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const toast = useToast();
  const { t, locale, setLocale, isArabic } = useI18n();
  const [themePreference, setThemePreference] =
    useState<ThemePreference>("system");
  const [isThemeLoading, setIsThemeLoading] = useState(true);
  const DirectionChevron = isArabic ? ChevronLeft : ChevronRight;

  useEffect(() => {
    let isMounted = true;
    loadThemePreference()
      .then((preference) => {
        if (isMounted) setThemePreference(preference);
      })
      .catch((error: unknown) => {
        console.error(
          "[Orbis Mobile Preferences] Failed to load theme preference.",
          error,
        );
        toast.error(t("profile_theme_load_error"));
      })
      .finally(() => {
        if (isMounted) setIsThemeLoading(false);
      });

    return () => {
      isMounted = false;
    };
  }, [t, toast]);

  const handleLocaleChange = async (nextLocale: "ar" | "en") => {
    try {
      await setLocale(nextLocale);
    } catch (error) {
      console.error(
        "[Orbis Mobile Preferences] Failed to save language.",
        error,
      );
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
    <View className="flex-1 bg-background">
      <Header
        title={t("pref_title")}
        showBack
        onBack={() => router.back()}
      />
      <ScreenScrollView
        edges={["bottom", "left", "right"]}
        contentContainerStyle={{
          paddingTop: insets.top + HEADER_CONTENT_OFFSET,
        }}
        scrollViewProps={{
          contentContainerClassName: "gap-six px-four pb-eight",
        }}
      >
        <Card variant="default" padding="none">
          <SettingsPanelTitle title={t("pref_language")} />
          <PreferenceRow
            label={t("pref_language_ar")}
            description={t("pref_language_ar_sub")}
            selected={locale === "ar"}
            checkColor={theme.primaryForeground}
            onPress={() => void handleLocaleChange("ar")}
          />
          <SettingsPanelDivider />
          <PreferenceRow
            label={t("pref_language_en")}
            description={t("pref_language_en_sub")}
            selected={locale === "en"}
            checkColor={theme.primaryForeground}
            onPress={() => void handleLocaleChange("en")}
          />
        </Card>

        <Card variant="default" padding="none">
          <SettingsPanelTitle title={t("pref_theme")} />
          {themeOptions.map((option, index) => (
            <React.Fragment key={option}>
              {index > 0 ? <SettingsPanelDivider /> : null}
              <PreferenceRow
                label={t(`pref_theme_${option}`)}
                selected={themePreference === option}
                disabled={isThemeLoading}
                checkColor={theme.primaryForeground}
                onPress={() => void handleThemeChange(option)}
              />
            </React.Fragment>
          ))}
        </Card>

        <Card variant="default" padding="none">
          <SettingsPanelTitle title={t("pref_notifications")} />
          <Pressable
            accessibilityRole="button"
            accessibilityLabel={t("pref_notifications")}
            onPress={() => router.push("/(app)/notifications")}
            className="min-h-16 flex-row items-center justify-between gap-four px-four py-three active:bg-secondary/50"
          >
            <View className="min-w-0 flex-1 gap-half">
              <Text className="text-sm font-medium text-foreground">
                {t("pref_notifications")}
              </Text>
              <Text className="text-[11px] leading-4 text-muted-foreground">
                {t("home_notifications_description")}
              </Text>
            </View>
            <DirectionChevron size={17} color={theme.mutedForeground} />
          </Pressable>
        </Card>

      </ScreenScrollView>
    </View>
  );
}

function PreferenceRow({
  label,
  description,
  selected,
  disabled = false,
  checkColor,
  onPress,
}: {
  label: string;
  description?: string;
  selected: boolean;
  disabled?: boolean;
  checkColor: string;
  onPress: () => void;
}) {
  return (
    <Pressable
      accessibilityRole="radio"
      accessibilityState={{ checked: selected, disabled }}
      disabled={disabled}
      onPress={onPress}
      className={`min-h-16 flex-row items-center justify-between gap-four px-four py-three ${
        disabled ? "opacity-60" : "active:bg-secondary/50"
      }`}
    >
      <View className="min-w-0 flex-1 items-start gap-half">
        <Text className="text-start text-sm font-medium text-foreground">
          {label}
        </Text>
        {description ? (
          <Text className="text-start text-xs leading-4 text-muted-foreground">
            {description}
          </Text>
        ) : null}
      </View>
      <View
        className={`h-6 w-6 items-center justify-center rounded-sm border ${
          selected
            ? "border-primary bg-primary"
            : "border-input bg-background"
        }`}
      >
        {selected ? <Check size={14} color={checkColor} /> : null}
      </View>
    </Pressable>
  );
}
