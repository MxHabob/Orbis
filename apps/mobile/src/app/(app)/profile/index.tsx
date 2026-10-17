import React, { useState } from "react";
import { Alert, Pressable, Text, View } from "react-native";
import { useRouter } from "expo-router";
import Constants from "expo-constants";
import * as Updates from "expo-updates";

import { HEADER_CONTENT_OFFSET, Header } from "@/components/header";
import { ScreenScrollView } from "@/components/layout/screen-scroll-view";
import { SupportDrawer } from "@/components/support-drawer";
import { Avatar } from "@/components/ui/avatar";
import { Card } from "@/components/ui/card";
import {
  SettingsPanelDivider,
  SettingsPanelTitle,
} from "@/components/ui/settings-panel";
import { useToast } from "@/components/ui/toast";
import { useTheme } from "@/hooks/use-theme";
import { useCurrentOrgQuery } from "@/hooks/use-api-queries";
import { useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";
import { getOrganizationWebsite } from "@/lib/web-urls";
import type { TranslationKey } from "@/lib/i18n/types";
import { ChevronLeft, ChevronRight } from "@orbis/icons/native";

const roleKeys: Record<string, TranslationKey> = {
  teacher: "auth_role_teacher",
  student: "auth_role_student",
  parent: "auth_role_parent",
  guardian: "auth_role_parent",
  admin: "auth_role_admin",
};

type ProfileMenuItem = {
  id: string;
  title: string;
  subtitle: string;
  trailingText?: string;
  onPress: () => void;
};

export default function ProfileScreen() {
  const router = useRouter();
  const theme = useTheme();
  const toast = useToast();
  const { user, logout, institutions } = useAuth();
  const { t, isArabic, locale } = useI18n();
  const organizationQuery = useCurrentOrgQuery();
  const [isSupportDrawerVisible, setIsSupportDrawerVisible] = useState(false);
  const DirectionChevron = isArabic ? ChevronLeft : ChevronRight;
  const organizationWebsite = getOrganizationWebsite(organizationQuery.data);
  const [isCheckingUpdates, setIsCheckingUpdates] = useState(false);
  const appVersion =
    Constants.nativeAppVersion ?? Constants.expoConfig?.version ?? "—";

  const displayName = user
    ? isArabic
      ? user.full_name_ar || user.full_name
      : user.full_name
    : t("profile_user_default");
  const roles = (user?.roles ?? []).map((role) => {
    const key = roleKeys[role.toLowerCase()];
    return key ? t(key) : role;
  });
  const roleSummary =
    roles.length > 0 ? roles.join(isArabic ? "، " : ", ") : t("common_unknown");

  const openWebViewer = (params: { page: string } | { url: string; title: string }) => {
    router.push({
      pathname: "/(app)/webviewer",
      params,
    });
  };

  const openOrganizationWebsite = () => {
    if (!organizationWebsite) return;
    openWebViewer({
      url: organizationWebsite,
      title: t("profile_organization_website"),
    });
  };

  const handleSignOut = () => {
    Alert.alert(t("profile_sign_out"), t("profile_sign_out_confirm"), [
      { text: t("common_cancel"), style: "cancel" },
      {
        text: t("profile_sign_out_button"),
        style: "destructive",
        onPress: () => {
          void logout()
            .then(() => router.replace("/(auth)/sign-in"))
            .catch((error: unknown) => {
              console.error("[Orbis Mobile Profile] Failed to sign out.", error);
              toast.error(t("profile_logout_error"));
            });
        },
      },
    ]);
  };

  const handleCheckForUpdates = async () => {
    if (!Updates.isEnabled) {
      toast.warning(t("updates_not_configured"));
      return;
    }

    setIsCheckingUpdates(true);
    let isDownloadingUpdate = false;
    try {
      const update = await Updates.checkForUpdateAsync();
      if (!update.isAvailable) {
        toast.success(t("updates_none_available"));
        return;
      }

      isDownloadingUpdate = true;
      const downloadedUpdate = await Updates.fetchUpdateAsync();
      if (!downloadedUpdate.isNew) {
        throw new Error("The available update could not be downloaded.");
      }

      Alert.alert(t("updates_available_title"), t("updates_available_message"), [
        { text: t("common_cancel"), style: "cancel" },
        {
          text: t("updates_restart_now"),
          onPress: () => {
            void Updates.reloadAsync().catch((error: unknown) => {
              console.error(
                "[Orbis Mobile Updates] Failed to reload after downloading update.",
                error,
              );
              toast.error(t("updates_install_error"));
            });
          },
        },
      ]);
    } catch (error: unknown) {
      console.error("[Orbis Mobile Updates] Failed to check for updates.", error);
      toast.error(
        t(isDownloadingUpdate ? "updates_download_error" : "updates_check_error"),
      );
    } finally {
      setIsCheckingUpdates(false);
    }
  };

  const settingsItems: ProfileMenuItem[] = [
    {
      id: "institutions",
      title: t("auth_institution_linked"),
      subtitle: t("auth_institution_switch"),
      trailingText: String(institutions.length),
      onPress: () => router.push("/institution-select"),
    },
    {
      id: "security",
      title: t("profile_menu_security"),
      subtitle: t("profile_security_description"),
      onPress: () => router.push("/(app)/profile/security"),
    },
    {
      id: "preferences",
      title: t("profile_menu_preferences"),
      subtitle: t("profile_preferences_description"),
      trailingText: locale === "ar" ? "العربية" : "English",
      onPress: () => router.push("/(app)/profile/preferences"),
    },
    {
      id: "notifications",
      title: t("pref_notifications"),
      subtitle: t("home_notifications_description"),
      onPress: () => router.push("/(app)/notifications"),
    },
  ];
  const helpItems: ProfileMenuItem[] = [
    {
      id: "support",
      title: t("profile_help_title"),
      subtitle: t("profile_help_description"),
      onPress: () => setIsSupportDrawerVisible(true),
    },
    {
      id: "about",
      title: t("profile_about_title"),
      subtitle: t("app_tagline"),
      onPress: () => openWebViewer({ page: "about" }),
    },
    {
      id: "mulverse",
      title: t("profile_about_mulverse"),
      subtitle: t("profile_about_mulverse_description"),
      onPress: () => openWebViewer({ page: "mulverse" }),
    },
  ];
  if (organizationWebsite) {
    helpItems.push({
      id: "organization-website",
      title: t("profile_organization_website"),
      subtitle: t("profile_organization_website_description"),
      onPress: openOrganizationWebsite,
    });
  }

  return (
    <>
      <Header
        title={t("profile_settings_title")}
        showBack
        onBack={() => router.back()}
      />
      <ScreenScrollView
        edges={["top", "bottom", "left", "right"]}
        contentContainerStyle={{ paddingTop: HEADER_CONTENT_OFFSET }}
        scrollViewProps={{
          contentContainerClassName: "gap-six px-four pb-eight",
        }}
      >
      <Card variant="default" padding="none">
        <SettingsPanelTitle title={t("profile_title")} />
        <Pressable
          accessibilityRole="button"
          accessibilityLabel={`${displayName}. ${roleSummary}. ${t("profile_menu_account")}`}
          onPress={() => router.push("/(app)/profile/account")}
          className={`min-h-24 flex-row items-center gap-four px-four py-three ${
            isArabic ? "flex-row-reverse" : ""
          } active:bg-secondary/50`}
        >
          <Avatar
            name={displayName}
            imageUrl={user?.avatar_url}
            size="xl"
          />
          <View className="min-w-0 flex-1 gap-half">
            <Text numberOfLines={2} className="text-start text-base font-semibold text-foreground">
              {displayName}
            </Text>
            <Text numberOfLines={2} className="text-start text-xs text-muted-foreground">
              {roleSummary}
            </Text>
        </View>
        </Pressable>
        <SettingsPanelDivider />
        <ProfileDataRow
          label={t("profile_email")}
          description={
            user?.is_email_verified
              ? t("profile_verified")
              : t("profile_not_verified")
          }
          value={user?.email || t("common_unknown")}
        />
      </Card>

      <ProfileMenuGroup
        title={t("profile_settings_title")}
        items={settingsItems}
        directionChevron={DirectionChevron}
        mutedForeground={theme.mutedForeground}
      />

      <Card variant="default" padding="none">
        <SettingsPanelTitle title={t("updates_title")} />
        <Pressable
          accessibilityRole="button"
          accessibilityLabel={`${t("updates_app_version")}: ${appVersion}. ${t("updates_check_button")}`}
          accessibilityState={{ busy: isCheckingUpdates, disabled: isCheckingUpdates }}
          disabled={isCheckingUpdates}
          onPress={() => void handleCheckForUpdates()}
          className={`min-h-16 flex-row items-center justify-between gap-four px-four py-three ${
            isCheckingUpdates ? "opacity-60" : "active:bg-secondary/50"
          }`}
        >
          <View className="min-w-0 flex-1 items-start gap-half">
            <Text className="text-start text-sm font-medium text-foreground">
              {t("updates_app_version")}
            </Text>
            <Text className="text-start text-xs leading-4 text-muted-foreground">
              {isCheckingUpdates
                ? t("updates_checking")
                : t("updates_check_button")}
            </Text>
          </View>
          <Text className="text-sm font-medium text-foreground">
            {appVersion}
          </Text>
        </Pressable>
      </Card>

      <ProfileMenuGroup
        title={t("profile_help_title")}
        items={helpItems}
        directionChevron={DirectionChevron}
        mutedForeground={theme.mutedForeground}
      />

      <Card variant="default" padding="none">
        <Pressable
          accessibilityRole="button"
          accessibilityLabel={t("profile_sign_out_button")}
          onPress={handleSignOut}
          className="min-h-14 flex-row items-center justify-between px-four py-three active:bg-secondary/50"
        >
          <View className="flex-1 gap-half">
            <Text className="text-sm font-medium text-destructive">
              {t("profile_sign_out_button")}
            </Text>
          </View>
          <DirectionChevron size={17} color={theme.destructive} />
        </Pressable>
      </Card>
      </ScreenScrollView>
      <SupportDrawer
        visible={isSupportDrawerVisible}
        onClose={() => setIsSupportDrawerVisible(false)}
      />
    </>
  );
}

function ProfileDataRow({
  label,
  description,
  value,
}: {
  label: string;
  description: string;
  value: string;
}) {
  return (
    <View className="min-h-16 flex-row items-center justify-between gap-four px-four py-three">
      <View className="min-w-0 flex-1 items-start gap-half">
        <Text
          numberOfLines={2}
          className="text-start text-xs font-medium text-foreground"
        >
          {label}
        </Text>
        <Text
          numberOfLines={2}
          className="text-start text-xs leading-4 text-muted-foreground"
        >
          {description}
        </Text>
      </View>
      <Text numberOfLines={2} className="min-w-0 flex-1 text-sm text-foreground">
        {value}
      </Text>
    </View>
  );
}

function ProfileMenuGroup({
  title,
  items,
  directionChevron: DirectionChevron,
  mutedForeground,
}: {
  title: string;
  items: ProfileMenuItem[];
  directionChevron: typeof ChevronLeft;
  mutedForeground: string;
}) {
  return (
    <Card variant="default" padding="none">
      <SettingsPanelTitle title={title} />
      {items.map((item, index) => (
        <React.Fragment key={item.id}>
          {index > 0 ? <SettingsPanelDivider /> : null}
          <Pressable
            accessibilityRole="button"
            accessibilityLabel={`${item.title}. ${item.subtitle}`}
            onPress={item.onPress}
            className="min-h-16 flex-row items-center justify-between gap-three px-four py-three active:bg-secondary/50"
          >
            <View className="min-w-0 flex-1 gap-half">
              <Text
                numberOfLines={2}
                className="text-sm font-medium text-foreground"
              >
                {item.title}
              </Text>
              <Text
                numberOfLines={2}
                className="text-xs leading-4 text-muted-foreground"
              >
                {item.subtitle}
              </Text>
            </View>
            <View className="flex-row items-center gap-two">
              {item.trailingText ? (
                <Text className="text-xs text-muted-foreground">
                  {item.trailingText}
                </Text>
              ) : null}
              <DirectionChevron size={17} color={mutedForeground} />
            </View>
          </Pressable>
        </React.Fragment>
      ))}
    </Card>
  );
}
