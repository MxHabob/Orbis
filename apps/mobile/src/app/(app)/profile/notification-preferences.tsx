/**
 * @module @orbis/mobile/app/(app)/profile/notification-preferences
 *
 * Per-category notification channel toggles (in-app, email, push).
 */

import React, { useCallback, useEffect, useState } from "react";
import { Text, View } from "react-native";
import { useRouter } from "expo-router";
import { useSafeAreaInsets } from "react-native-safe-area-context";

import { HEADER_CONTENT_OFFSET, Header } from "@/components/header";
import { ScreenScrollView } from "@/components/layout/screen-scroll-view";
import { Card } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { LoadingState } from "@/components/ui/loading-state";
import {
  SettingsPanelDivider,
  SettingsPanelTitle,
} from "@/components/ui/settings-panel";
import { Switch } from "@/components/ui/switch";
import { useToast } from "@/components/ui/toast";
import { useTheme } from "@/hooks/use-theme";
import { useI18n } from "@/lib/i18n";
import type { TranslationKey } from "@/lib/i18n";
import {
  fetchNotificationPreferences,
  updateNotificationPreferences,
} from "@/lib/api/notifications";
import { ExclamationCircle } from "@orbis/icons/native";

// ── Types ──────────────────────────────────────────────────────

const CATEGORIES = [
  "enrollment",
  "grades",
  "attendance",
  "finance",
  "behavior",
  "schedule",
  "approval",
  "system",
  "announcement",
] as const;

type NotifCategory = (typeof CATEGORIES)[number];

const CATEGORY_LABEL_MAP: Record<NotifCategory, TranslationKey> = {
  enrollment: "notifications_category_enrollment",
  grades: "notifications_category_grades",
  attendance: "notifications_category_attendance",
  finance: "notifications_category_finance",
  behavior: "notifications_category_behavior",
  schedule: "notifications_category_schedule",
  approval: "notifications_category_approval",
  system: "notifications_category_system",
  announcement: "notifications_category_announcement",
};

interface PrefState {
  in_app_enabled: boolean;
  email_enabled: boolean;
  push_enabled: boolean;
}

const DEFAULT_PREF: PrefState = {
  in_app_enabled: true,
  email_enabled: true,
  push_enabled: true,
};

type PrefsMap = Record<string, PrefState>;

// ── Screen ─────────────────────────────────────────────────────

export default function NotificationPreferencesScreen() {
  const router = useRouter();
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const toast = useToast();
  const { t } = useI18n();

  const [prefs, setPrefs] = useState<PrefsMap>({});
  const [isLoading, setIsLoading] = useState(true);
  const [isError, setIsError] = useState(false);

  const loadPrefs = useCallback(async () => {
    try {
      const data = await fetchNotificationPreferences();
      const map: PrefsMap = {};
      for (const pref of data) {
        map[pref.category] = {
          in_app_enabled: pref.in_app_enabled,
          email_enabled: pref.email_enabled,
          push_enabled: pref.push_enabled,
        };
      }
      setPrefs(map);
    } catch {
      setIsError(true);
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    void loadPrefs();
  }, [loadPrefs]);

  const toggle = useCallback(
    async (category: string, channel: keyof PrefState, value: boolean) => {
      const current = prefs[category] ?? { ...DEFAULT_PREF };
      const updated: PrefState = { ...current, [channel]: value };

      // Optimistic update
      setPrefs((prev) => ({ ...prev, [category]: updated }));

      try {
        await updateNotificationPreferences([{ category, ...updated }]);
      } catch {
        // Revert on failure
        setPrefs((prev) => ({ ...prev, [category]: current }));
        toast.error(t("profile_preference_save_error"));
      }
    },
    [prefs, toast, t],
  );

  const getPref = (category: string): PrefState =>
    prefs[category] ?? DEFAULT_PREF;

  return (
    <View className="flex-1 bg-background">
      <Header
        title={t("pref_notifications")}
        showBack
        onBack={() => router.back()}
      />
      {isLoading ? (
        <View
          className="flex-1"
          style={{ paddingTop: insets.top + HEADER_CONTENT_OFFSET }}
        >
          <LoadingState message={t("common_loading")} />
        </View>
      ) : isError ? (
        <View
          className="flex-1 items-center justify-center px-four"
          style={{ paddingTop: insets.top + HEADER_CONTENT_OFFSET }}
        >
          <ExclamationCircle size={28} color={theme.destructive} />
          <Text className="mt-three mb-two text-center text-base font-semibold text-foreground">
            {t("notifications_error_title")}
          </Text>
          <Button
            title={t("notifications_error_retry")}
            variant="outline"
            size="sm"
            onPress={() => void loadPrefs()}
          />
        </View>
      ) : (
        <ScreenScrollView
          edges={["bottom", "left", "right"]}
          contentContainerStyle={{
            paddingTop: insets.top + HEADER_CONTENT_OFFSET,
          }}
          scrollViewProps={{
            contentContainerClassName: "gap-six px-four pb-eight",
          }}
        >
          {CATEGORIES.map((category) => {
            const pref = getPref(category);
            return (
              <Card key={category} variant="default" padding="none">
                <SettingsPanelTitle title={t(CATEGORY_LABEL_MAP[category])} />
                <ChannelRow
                  label={t("notif_pref_in_app")}
                  value={pref.in_app_enabled}
                  onValueChange={(v) =>
                    void toggle(category, "in_app_enabled", v)
                  }
                />
                <SettingsPanelDivider />
                <ChannelRow
                  label={t("notif_pref_email")}
                  value={pref.email_enabled}
                  onValueChange={(v) =>
                    void toggle(category, "email_enabled", v)
                  }
                />
                <SettingsPanelDivider />
                <ChannelRow
                  label={t("notif_pref_push")}
                  value={pref.push_enabled}
                  onValueChange={(v) =>
                    void toggle(category, "push_enabled", v)
                  }
                />
              </Card>
            );
          })}
        </ScreenScrollView>
      )}
    </View>
  );
}

// ── Channel toggle row ─────────────────────────────────────────

function ChannelRow({
  label,
  value,
  onValueChange,
}: {
  label: string;
  value: boolean;
  onValueChange: (v: boolean) => void;
}) {
  return (
    <View className="min-h-12 flex-row items-center justify-between px-four py-two">
      <Text className="text-sm text-foreground">{label}</Text>
      <Switch
        value={value}
        onValueChange={onValueChange}
        accessibilityLabel={label}
      />
    </View>
  );
}
