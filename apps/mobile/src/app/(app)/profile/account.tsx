/**
 * @module @orbis/mobile/app/(app)/profile/account
 *
 * Account Profile details for Orbis Mobile.
 * Displays authoritative user attributes, organization affiliation, and roles.
 */

import React from "react";
import { View, Text } from "react-native";
import { useRouter } from "expo-router";
import { Header } from "@/components/header";
import { ScreenScrollView } from "@/components/layout/screen-scroll-view";
import { Card } from "@/components/ui/card";
import { Avatar } from "@/components/ui/avatar";
import { Badge } from "@/components/ui/badge";
import { EmptyState } from "@/components/ui/empty-state";
import { useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";
import { useTheme } from "@/hooks/use-theme";
import type { TranslationKey } from "@/lib/i18n/types";
import {
  User,
  Envelope,
  Phone,
  ShieldCheck,
  BuildingLibrary,
  Calendar,
} from "@orbis/icons/native";

const roleKeys: Record<string, TranslationKey> = {
  teacher: "auth_role_teacher",
  student: "auth_role_student",
  parent: "auth_role_parent",
  guardian: "auth_role_parent",
  admin: "auth_role_admin",
};

export default function AccountScreen() {
  const router = useRouter();
  const theme = useTheme();
  const { user, effectiveContext } = useAuth();
  const { t, isArabic, formatDate } = useI18n();

  if (!user) {
    return (
      <View className="flex-1 bg-background">
        <Header
          title={t("profile_menu_account") || "Account"}
          showBack={true}
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
        title={t("profile_menu_account") || "Account Details"}
        showBack={true}
        onBack={() => router.back()}
      />

      <ScreenScrollView
        edges={["bottom", "left", "right"]}
        scrollViewProps={{
          contentContainerClassName: "gap-six px-four pb-[100px] pt-two",
        }}
      >
        {/* Identity Snapshot Card */}
        <View className="flex-row items-center gap-four rounded-2xl border border-border/80 bg-card p-four shadow-xs">
          <Avatar
            fallback={(displayName || "U").charAt(0)}
            size="lg"
            className="h-16 w-16 border border-primary/20 bg-primary/10"
          />
          <View className="flex-1 gap-half">
            <Text className="text-lg font-extrabold text-foreground">
              {displayName}
            </Text>
            <View className="flex-row items-center gap-two">
              <Badge variant="secondary" className="px-two py-half">
                <Text className="text-xs font-bold text-primary">
                  {user.is_active ? t("profile_account_active") || "Active" : t("profile_account_inactive") || "Inactive"}
                </Text>
              </Badge>
              {user.is_email_verified ? (
                <Badge variant="outline" className="px-two py-half">
                  <Text className="text-[10px] font-semibold text-muted-foreground">
                    {t("profile_verified") || "Verified"}
                  </Text>
                </Badge>
              ) : null}
            </View>
          </View>
        </View>

        {/* Detailed Attributes Inset Group */}
        <View className="gap-two">
          <Text className="text-xs font-bold uppercase tracking-wider text-muted-foreground px-one">
            {t("profile_title") || "General Information"}
          </Text>

          <View className="overflow-hidden rounded-2xl border border-border/80 bg-card shadow-2xs divide-y divide-border/60">
            <InfoFieldRow
              icon={<User size={16} color={theme.primary} />}
              label={t("auth_full_name") || "Full Name"}
              value={displayName}
            />
            <InfoFieldRow
              icon={<Envelope size={16} color={theme.primary} />}
              label={t("profile_email") || "Email Address"}
              value={user.email || t("common_unknown")}
            />
            <InfoFieldRow
              icon={<Phone size={16} color={theme.primary} />}
              label={t("profile_phone") || "Phone Number"}
              value={user.phone || t("common_unknown")}
            />
            <InfoFieldRow
              icon={<ShieldCheck size={16} color={theme.primary} />}
              label={t("profile_roles") || "Roles"}
              value={
                roles.length > 0
                  ? roles.join(isArabic ? "، " : ", ")
                  : t("common_unknown")
              }
            />
            {user.created_at ? (
              <InfoFieldRow
                icon={<Calendar size={16} color={theme.primary} />}
                label={t("profile_account_created") || "Member Since"}
                value={formatDate(user.created_at)}
              />
            ) : null}
          </View>
        </View>

        <Text className="text-center text-xs text-muted-foreground px-four">
          {t("profile_account_read_only") || "To change institutional credentials or official legal name, please contact school administration."}
        </Text>
      </ScreenScrollView>
    </View>
  );
}

function InfoFieldRow({
  icon,
  label,
  value,
}: {
  icon: React.ReactNode;
  label: string;
  value: string;
}) {
  return (
    <View className="flex-row items-center justify-between p-four">
      <View className="flex-row items-center gap-three">
        <View className="h-8 w-8 items-center justify-center rounded-lg bg-secondary">
          {icon}
        </View>
        <Text className="text-xs font-medium text-muted-foreground">{label}</Text>
      </View>
      <Text className="text-xs font-bold text-foreground text-end max-w-[50%]">
        {value}
      </Text>
    </View>
  );
}
