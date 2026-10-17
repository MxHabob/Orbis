import React, { useState } from "react";
import { Alert, Linking, Pressable, Text, View } from "react-native";
import { useRouter } from "expo-router";
import Constants from "expo-constants";
import {
  AppNotification,
  ArrowUpCircle,
  BuildingOffice,
  Cog6,
  GlobeAlt,
  Headphone,
  InfoCircle,
  LockClosed,
  SignOut,
} from "@orbis/icons/native";

import { Header } from "@/components/header";
import { ScreenScrollView } from "@/components/layout/screen-scroll-view";
import { SupportDrawer } from "@/components/support-drawer";
import { Avatar } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { SettingsPanelDivider } from "@/components/ui/settings-panel";
import { useToast } from "@/components/ui/toast";
import { useTheme } from "@/hooks/use-theme";
import { useCurrentOrgQuery } from "@/hooks/use-api-queries";
import { useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";
import { checkForMobileAppUpdate } from "@/lib/app-updates";
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
  icon: React.ReactNode;
  trailingText?: string;
  onPress: () => void;
};

export default function ProfileScreen() {
  const router = useRouter();
  const theme = useTheme();
  const toast = useToast();
  const { user, logout, institutions, selectedInstitution } = useAuth();
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
  const linkedInstitutionCount =
    institutions.length +
    (selectedInstitution &&
    !institutions.some(
      (institution) =>
        institution.deployment_id === selectedInstitution.deployment_id,
    )
      ? 1
      : 0);

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
    setIsCheckingUpdates(true);
    try {
      if (!appVersion || appVersion === "—") {
        throw new Error("The installed app version is unavailable.");
      }

      const result = await checkForMobileAppUpdate(appVersion);
      if (result.status === "no_mobile_release") {
        toast.warning(t("updates_mobile_release_missing"));
        return;
      }

      if (result.status === "up_to_date") {
        toast.success(t("updates_none_available"));
        return;
      }

      Alert.alert(
        t("updates_available_title"),
        t("updates_available_message", { version: result.latestVersion }),
        [
          { text: t("common_cancel"), style: "cancel" },
          {
            text: t("updates_open_release"),
            onPress: () => {
              void Linking.openURL(result.releaseUrl).catch(
                (error: unknown) => {
                  console.error(
                    "[Orbis Mobile Updates] Failed to open the mobile release.",
                    error,
                  );
                  toast.error(t("updates_open_release_error"));
                },
              );
            },
          },
        ],
      );
    } catch (error: unknown) {
      console.error("[Orbis Mobile Updates] Failed to check for updates.", error);
      toast.error(t("updates_check_error"));
    } finally {
      setIsCheckingUpdates(false);
    }
  };

  const accountItems: ProfileMenuItem[] = [
    {
      id: "institutions",
      title: t("auth_institution_linked"),
      subtitle: t("auth_institution_linked_description"),
      icon: <BuildingOffice size={20} color={theme.mutedForeground} />,
      trailingText: String(linkedInstitutionCount),
      onPress: () => router.push("/(app)/profile/institutions"),
    },
    {
      id: "security",
      title: t("profile_menu_security"),
      subtitle: t("profile_security_description"),
      icon: <LockClosed size={20} color={theme.mutedForeground} />,
      onPress: () => router.push("/(app)/profile/security"),
    },
  ];
  const personalizationItems: ProfileMenuItem[] = [
    {
      id: "preferences",
      title: t("profile_menu_preferences"),
      subtitle: t("profile_preferences_description"),
      icon: <Cog6 size={20} color={theme.mutedForeground} />,
      trailingText: locale === "ar" ? "العربية" : "English",
      onPress: () => router.push("/(app)/profile/preferences"),
    },
    {
      id: "notifications",
      title: t("pref_notifications"),
      subtitle: t("home_notifications_description"),
      icon: <AppNotification size={20} color={theme.mutedForeground} />,
      onPress: () => router.push("/(app)/notifications"),
    },
  ];
  const helpItems: ProfileMenuItem[] = [
    {
      id: "support",
      title: t("profile_help_title"),
      subtitle: t("profile_help_description"),
      icon: <Headphone size={20} color={theme.mutedForeground} />,
      onPress: () => setIsSupportDrawerVisible(true),
    },
    {
      id: "about",
      title: t("profile_about_title"),
      subtitle: t("app_tagline"),
      icon: <InfoCircle size={20} color={theme.mutedForeground} />,
      onPress: () => openWebViewer({ page: "about" }),
    },
    {
      id: "mulverse",
      title: t("profile_about_mulverse"),
      subtitle: t("profile_about_mulverse_description"),
      icon: <GlobeAlt size={20} color={theme.mutedForeground} />,
      onPress: () => openWebViewer({ page: "mulverse" }),
    },
  ];
  if (organizationWebsite) {
    helpItems.push({
      id: "organization-website",
      title: t("profile_organization_website"),
      subtitle: t("profile_organization_website_description"),
      icon: <GlobeAlt size={20} color={theme.mutedForeground} />,
      onPress: openOrganizationWebsite,
    });
  }
  helpItems.push({
    id: "updates",
    title: t("updates_app_version"),
    subtitle: isCheckingUpdates
      ? t("updates_checking")
      : t("updates_check_button"),
    icon: <ArrowUpCircle size={20} color={theme.mutedForeground} />,
    trailingText: appVersion,
    onPress: () => void handleCheckForUpdates(),
  });

  return (
    <>
      <Header
        title={t("profile_settings_title")}
        showBack
        onBack={() => router.back()}
      />
      <ScreenScrollView
        edges={["bottom", "left", "right"]}
        scrollViewProps={{
          contentContainerClassName: "gap-five px-four pb-eight",
        }}
      >
        <ProfileMenuGroup
          title={t("profile_account_section_title")}
          items={accountItems}
          directionChevron={DirectionChevron}
          mutedForeground={theme.mutedForeground}
          leadingContent={
            <>
              <Pressable
                accessibilityRole="button"
                accessibilityLabel={`${displayName}. ${roleSummary}. ${t("profile_menu_account")}`}
                onPress={() => router.push("/(app)/profile/account")}
                className="min-h-20 flex-row items-center gap-four px-four py-three active:bg-secondary/50"
              >
                <Avatar
                  name={displayName}
                  imageUrl={user?.avatar_url}
                  size="xl"
                />
                <View className="min-w-0 flex-1 gap-half">
                  <Text
                    numberOfLines={2}
                    className="text-start text-base font-semibold text-foreground"
                  >
                    {displayName}
                  </Text>
                  <Text
                    numberOfLines={2}
                    className="text-start text-xs text-muted-foreground"
                  >
                    {roleSummary}
                  </Text>
                </View>
                <DirectionChevron size={17} color={theme.mutedForeground} />
              </Pressable>
              <SettingsPanelDivider />
            </>
          }
        />

        <ProfileMenuGroup
          title={t("profile_personalization_title")}
          items={personalizationItems}
          directionChevron={DirectionChevron}
          mutedForeground={theme.mutedForeground}
        />

        <ProfileMenuGroup
          title={t("profile_help_center_title")}
          items={helpItems}
          directionChevron={DirectionChevron}
          mutedForeground={theme.mutedForeground}
          isCheckingUpdates={isCheckingUpdates}
        />

        <Button
          title={t("profile_sign_out_button")}
          onPress={handleSignOut}
          variant="destructive"
          size="lg"
          fullWidth
          icon={
            <SignOut size={18} color={theme.destructiveForeground} />
          }
          className="rounded-full"
        />

        <View className="items-center gap-one px-four pb-two pt-one">
          <Text className="text-xs font-medium text-muted-foreground">
            {t("profile_footer_app_name")}
          </Text>
          <Text className="text-center text-[11px] leading-4 text-muted-foreground">
            {t("profile_footer_app_details", { version: appVersion })}
          </Text>
        </View>
      </ScreenScrollView>
      <SupportDrawer
        visible={isSupportDrawerVisible}
        onClose={() => setIsSupportDrawerVisible(false)}
      />
    </>
  );
}

function ProfileMenuGroup({
  title,
  items,
  directionChevron: DirectionChevron,
  mutedForeground,
  leadingContent,
  isCheckingUpdates = false,
}: {
  title: string;
  items: ProfileMenuItem[];
  directionChevron: typeof ChevronLeft;
  mutedForeground: string;
  leadingContent?: React.ReactNode;
  isCheckingUpdates?: boolean;
}) {
  return (
    <View className="gap-two">
      <Text className="px-one text-sm font-medium text-muted-foreground">
        {title}
      </Text>
      <Card variant="default" padding="none">
        {leadingContent}
        {items.map((item, index) => (
          <React.Fragment key={item.id}>
            {leadingContent || index > 0 ? (
              <View style={{ marginStart: 52 }}>
                <SettingsPanelDivider />
              </View>
            ) : null}
            <Pressable
              accessibilityRole="button"
              accessibilityLabel={`${item.title}. ${item.subtitle}`}
              accessibilityState={{
                busy: item.id === "updates" && isCheckingUpdates,
                disabled: item.id === "updates" && isCheckingUpdates,
              }}
              disabled={item.id === "updates" && isCheckingUpdates}
              onPress={item.onPress}
              className={`min-h-16 flex-row items-center justify-between gap-three px-four py-three active:bg-secondary/50 ${
                item.id === "updates" && isCheckingUpdates ? "opacity-60" : ""
              }`}
            >
              <View className="flex-1 flex-row items-center gap-three">
                <View className="h-8 w-8 items-center justify-center">
                  {item.icon}
                </View>
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
    </View>
  );
}
