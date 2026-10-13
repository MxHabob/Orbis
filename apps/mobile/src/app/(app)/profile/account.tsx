/**
 * @module @orbis/mobile/app/(app)/profile/account
 *
 * Read-only account details displayed in the shared settings-panel style.
 */

import React from "react";
import { Text, View } from "react-native";
import { useRouter } from "expo-router";
import { useSafeAreaInsets } from "react-native-safe-area-context";

import { HEADER_CONTENT_OFFSET, Header } from "@/components/header";
import { ScreenScrollView } from "@/components/layout/screen-scroll-view";
import { Avatar } from "@/components/ui/avatar";
import { Card } from "@/components/ui/card";
import { EmptyState } from "@/components/ui/empty-state";
import { useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";
import type { TranslationKey } from "@/lib/i18n/types";

const roleKeys: Record<string, TranslationKey> = {
  teacher: "auth_role_teacher",
  student: "auth_role_student",
  parent: "auth_role_parent",
  guardian: "auth_role_parent",
  admin: "auth_role_admin",
};

export default function AccountScreen() {
  const router = useRouter();
  const insets = useSafeAreaInsets();
  const { user } = useAuth();
  const { t, isArabic, formatDate } = useI18n();

  if (!user) {
    return (
      <View className="flex-1 bg-background">
        <Header
          title={t("profile_menu_account")}
          showBack
          onBack={() => router.back()}
        />
        <View className="flex-1 items-center justify-center p-four">
          <EmptyState
            title={t("common_error")}
            description={t("profile_account_unavailable")}
          />
        </View>
      </View>
    );
  }

  const displayName = isArabic
    ? user.full_name_ar || user.full_name
    : user.full_name;
  const roles = (user.roles ?? []).map((role) => {
    const key = roleKeys[role.toLowerCase()];
    return key ? t(key) : role;
  });

  return (
    <View className="flex-1 bg-background">
      <Header
        title={t("profile_menu_account")}
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
          <PanelTitle title={t("profile_title")} />
          <View
            className={`min-h-24 flex-row items-center gap-four px-four py-three ${
              isArabic ? "flex-row-reverse" : ""
            }`}
          >
            <Avatar
              name={displayName}
              imageUrl={user.avatar_url}
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
                {roles.length > 0
                  ? roles.join(isArabic ? "، " : ", ")
                  : t("common_unknown")}
              </Text>
            </View>
          </View>
          <PanelDivider />
          <AccountRow
            label={t("auth_full_name")}
            description={t("profile_account_description")}
            value={displayName}
          />
          <PanelDivider />
          <AccountRow
            label={t("profile_email")}
            description={
              user.is_email_verified
                ? t("profile_verified")
                : t("profile_not_verified")
            }
            value={user.email || t("common_unknown")}
          />
          <PanelDivider />
          <AccountRow
            label={t("profile_phone")}
            description={t("profile_phone")}
            value={user.phone || t("common_unknown")}
          />
          <PanelDivider />
          <AccountRow
            label={t("profile_roles")}
            description={t("profile_role")}
            value={
              roles.length > 0
                ? roles.join(isArabic ? "، " : ", ")
                : t("common_unknown")
            }
          />
          {user.created_at ? (
            <>
              <PanelDivider />
              <AccountRow
                label={t("profile_account_created")}
                description={t("profile_account_read_only")}
                value={formatDate(user.created_at)}
              />
            </>
          ) : null}
          <PanelDivider />
          <AccountRow
            label={t("profile_account_status")}
            description={t("profile_account_status")}
            value={
              user.is_active
                ? t("profile_account_active")
                : t("profile_account_inactive")
            }
          />
        </Card>
      </ScreenScrollView>
    </View>
  );
}

function PanelTitle({ title }: { title: string }) {
  return (
    <View className="border-b border-border bg-secondary/50 px-four py-three">
      <Text className="text-sm font-medium text-foreground">{title}</Text>
    </View>
  );
}

function PanelDivider() {
  return <View className="h-px bg-border" />;
}

function AccountRow({
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
      <Text
        numberOfLines={2}
        className="min-w-0 flex-1 text-sm text-foreground"
      >
        {value}
      </Text>
    </View>
  );
}
