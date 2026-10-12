/**
 * @module @orbis/mobile/app/(app)/profile/preferences
 *
 * System Preferences & Appearance settings for Orbis Mobile.
 * Configures application language (Arabic / English), dynamic theme
 * (Light / Dark / System), and alert preferences.
 */

import React, { useEffect, useState } from "react";
import { View, Text, Pressable, Alert } from "react-native";
import { useRouter } from "expo-router";
import { Header } from "@/components/header";
import { ScreenScrollView } from "@/components/layout/screen-scroll-view";
import { Card } from "@/components/ui/card";
import { useToast } from "@/components/ui/toast";
import { useI18n } from "@/lib/i18n";
import { useTheme } from "@/hooks/use-theme";
import {
  applyThemePreference,
  loadThemePreference,
  saveThemePreference,
  type ThemePreference,
} from "@/lib/theme-preference";
import {
  Language,
  Sparkles,
  Bell,
  Check,
  Sun,
  Moon,
  ComputerDesktop,
} from "@orbis/icons/native";

const themeOptions: ThemePreference[] = ["system", "light", "dark"];

export default function PreferencesScreen() {
  const router = useRouter();
  const theme = useTheme();
  const toast = useToast();
  const { t, locale, setLocale } = useI18n();
  const [themePreference, setThemePreference] = useState<ThemePreference>("system");

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
      });
    return () => {
      isMounted = false;
    };
  }, []);

  const handleLocaleChange = async (nextLocale: "ar" | "en") => {
    try {
      await setLocale(nextLocale);
    } catch (error) {
      console.error("[Orbis Mobile Preferences] Failed to save language.", error);
      toast.error(t("profile_preference_save_error") || "Failed to update language.");
    }
  };

  const handleThemeChange = async (preference: ThemePreference) => {
    try {
      await saveThemePreference(preference);
      setThemePreference(preference);
    } catch (error) {
      console.error("[Orbis Mobile Theme] Failed to save theme.", error);
      toast.error(t("profile_preference_save_error") || "Failed to update theme.");
    }
  };

  return (
    <View className="flex-1 bg-background">
      <Header
        title={t("pref_title") || "Preferences"}
        showBack={true}
        onBack={() => router.back()}
      />

      <ScreenScrollView
        edges={["bottom", "left", "right"]}
        scrollViewProps={{
          contentContainerClassName: "gap-six px-four pb-[100px] pt-two",
        }}
      >
        {/* 1. Language Preference */}
        <View className="gap-two">
          <View className="flex-row items-center gap-two px-one">
            <Language size={16} color={theme.primary} />
            <Text className="text-xs font-bold uppercase tracking-wider text-muted-foreground">
              {t("pref_language") || "Language"}
            </Text>
          </View>

          <View className="overflow-hidden rounded-2xl border border-border/80 bg-card shadow-2xs divide-y divide-border/60">
            <OptionRow
              label="العربية"
              subtitle="Arabic (RTL)"
              selected={locale === "ar"}
              onPress={() => void handleLocaleChange("ar")}
            />
            <OptionRow
              label="English"
              subtitle="English (LTR)"
              selected={locale === "en"}
              onPress={() => void handleLocaleChange("en")}
            />
          </View>
        </View>

        {/* 2. Theme Preference */}
        <View className="gap-two">
          <View className="flex-row items-center gap-two px-one">
            <Sparkles size={16} color={theme.primary} />
            <Text className="text-xs font-bold uppercase tracking-wider text-muted-foreground">
              {t("pref_theme") || "Appearance"}
            </Text>
          </View>

          <View className="overflow-hidden rounded-2xl border border-border/80 bg-card shadow-2xs divide-y divide-border/60">
            {themeOptions.map((option) => (
              <OptionRow
                key={option}
                label={t(`pref_theme_${option}`) || option}
                selected={themePreference === option}
                onPress={() => void handleThemeChange(option)}
              />
            ))}
          </View>
        </View>

        {/* 3. Notification Shortcut */}
        <Pressable
          accessibilityRole="button"
          onPress={() => router.push("/(app)/notifications")}
          className="flex-row items-center justify-between rounded-2xl border border-border/80 bg-card p-four shadow-xs active:bg-secondary/40"
        >
          <View className="flex-row items-center gap-three">
            <View className="h-9 w-9 items-center justify-center rounded-xl bg-primary/10">
              <Bell size={18} color={theme.primary} />
            </View>
            <View className="gap-half">
              <Text className="text-sm font-bold text-foreground">
                {t("pref_notifications") || "Notification Settings"}
              </Text>
              <Text className="text-xs text-muted-foreground">
                {t("notifications_inbox_unavailable_description")}
              </Text>
            </View>
          </View>
        </Pressable>
      </ScreenScrollView>
    </View>
  );
}

function OptionRow({
  label,
  subtitle,
  selected,
  onPress,
}: {
  label: string;
  subtitle?: string;
  selected: boolean;
  onPress: () => void;
}) {
  const theme = useTheme();
  return (
    <Pressable
      accessibilityRole="radio"
      accessibilityState={{ checked: selected }}
      onPress={onPress}
      className="flex-row items-center justify-between p-four active:bg-secondary/40"
    >
      <View className="gap-half">
        <Text className="text-sm font-bold text-foreground">{label}</Text>
        {subtitle ? (
          <Text className="text-xs text-muted-foreground">{subtitle}</Text>
        ) : null}
      </View>

      <View
        className={`h-6 w-6 items-center justify-center rounded-full border ${
          selected
            ? "border-primary bg-primary"
            : "border-muted-foreground/30 bg-transparent"
        }`}
      >
        {selected ? <Check size={14} color="#FFFFFF" /> : null}
      </View>
    </Pressable>
  );
}
