/**
 * @module @orbis/mobile/app/(app)/profile/index
 *
 * User Profile & Settings Hub for Orbis Mobile.
 * Features user identity header, role badges, inset grouped settings rows,
 * and secure session termination.
 */

import React from "react";
import { View, Text, Pressable, Alert } from "react-native";
import { useRouter } from "expo-router";
import { Header } from "@/components/header";
import { ScreenScrollView } from "@/components/layout/screen-scroll-view";
import { Avatar } from "@/components/ui/avatar";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { useToast } from "@/components/ui/toast";
import { useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";
import { useTheme } from "@/hooks/use-theme";
import {
  User,
  ShieldCheck,
  AdjustmentsHorizontal,
  SignOut,
  ChevronLeft,
  ChevronRight,
  AcademicCap,
  BuildingLibrary,
} from "@orbis/icons/native";

export default function ProfileIndexScreen() {
  const router = useRouter();
  const theme = useTheme();
  const { user, effectiveContext, logout } = useAuth();
  const { t, isArabic } = useI18n();
  const toast = useToast();

  const DirectionChevron = isArabic ? ChevronLeft : ChevronRight;

  const displayName = user
    ? isArabic
      ? user.full_name_ar || user.full_name
      : user.full_name
    : t("profile_user_default") || "Orbis User";

  const orgName = isArabic
    ? effectiveContext?.organization_name_ar || effectiveContext?.organization_name
    : effectiveContext?.organization_name;

  const userRoles = [
    ...(user?.roles ?? []),
    effectiveContext?.effective_role ?? "",
  ].filter(Boolean);

  const handleLogout = () => {
    Alert.alert(
      t("profile_sign_out") || "Sign Out",
      t("profile_sign_out_confirm") || "Are you sure you want to sign out?",
      [
        { text: t("common_cancel") || "Cancel", style: "cancel" },
        {
          text: t("profile_sign_out_button") || "Sign Out",
          style: "destructive",
          onPress: () => {
            void logout()
              .then(() => router.replace("/(auth)/sign-in"))
              .catch((error: unknown) => {
                console.error("[Orbis Mobile Auth] Failed to sign out.", error);
                toast.error(t("profile_logout_error") || "Failed to sign out.");
              });
          },
        },
      ],
    );
  };

  return (
    <View className="flex-1 bg-background">
      <Header
        title={t("profile_title") || "Settings"}
        showBack={true}
        onBack={() => router.back()}
      />

      <ScreenScrollView
        edges={["bottom", "left", "right"]}
        scrollViewProps={{
          contentContainerClassName: "gap-six px-four pb-[120px] pt-two",
        }}
      >
        {/* 1. Hero User Identity Card */}
        <View className="items-center gap-three rounded-3xl border border-border/80 bg-card p-six shadow-sm">
          <Avatar
            fallback={(displayName || "U").charAt(0)}
            size="xl"
            className="h-20 w-20 border-2 border-primary/20 bg-primary/10"
          />

          <View className="items-center gap-one">
            <Text className="text-xl font-extrabold text-foreground text-center">
              {displayName}
            </Text>
            {user?.email ? (
              <Text className="text-xs text-muted-foreground">{user.email}</Text>
            ) : null}

            {orgName ? (
              <View className="mt-one flex-row items-center gap-one">
                <BuildingLibrary size={12} color={theme.primary} />
                <Text className="text-xs font-semibold text-primary">{orgName}</Text>
              </View>
            ) : null}
          </View>

          {/* Role Badges */}
          {userRoles.length > 0 ? (
            <View className="flex-row flex-wrap justify-center gap-two pt-one">
              {Array.from(new Set(userRoles)).map((role) => (
                <Badge key={role} variant="secondary" className="px-three py-half">
                  <Text className="text-xs font-bold text-foreground capitalize">
                    {role}
                  </Text>
                </Badge>
              ))}
            </View>
          ) : null}
        </View>

        {/* 2. Inset Grouped Settings Section */}
        <View className="gap-two">
          <Text className="text-xs font-bold uppercase tracking-wider text-muted-foreground px-one">
            {t("settings_preferences") || "Preferences & Security"}
          </Text>

          <View className="overflow-hidden rounded-2xl border border-border/80 bg-card shadow-2xs divide-y divide-border/60">
            <SettingsItemRow
              icon={<User size={18} color={theme.primary} />}
              title={t("profile_menu_account") || "Account Details"}
              subtitle={t("profile_account_description") || "Personal info & contact"}
              onPress={() => router.push("/(app)/profile/account")}
              chevron={DirectionChevron}
            />
            <SettingsItemRow
              icon={<ShieldCheck size={18} color={theme.primary} />}
              title={t("profile_menu_security") || "Security"}
              subtitle={t("profile_security_description") || "Password & MFA"}
              onPress={() => router.push("/(app)/profile/security")}
              chevron={DirectionChevron}
            />
            <SettingsItemRow
              icon={<AdjustmentsHorizontal size={18} color={theme.primary} />}
              title={t("profile_menu_preferences") || "Preferences"}
              subtitle={t("profile_preferences_description") || "Language, theme & alerts"}
              onPress={() => router.push("/(app)/profile/preferences")}
              chevron={DirectionChevron}
            />
          </View>
        </View>

        {/* 3. Sign Out Button */}
        <View className="gap-four pt-two">
          <Button
            variant="destructive"
            onPress={handleLogout}
            className="h-12 rounded-2xl flex-row items-center justify-center gap-two shadow-xs"
          >
            <SignOut size={18} color="#FFFFFF" />
            <Text className="text-sm font-bold text-destructive-foreground">
              {t("profile_sign_out_button") || "Sign Out"}
            </Text>
          </Button>

          <Text className="text-center font-mono text-[10px] text-muted-foreground">
            Orbis Mobile v1.0.0 (Build 57)
          </Text>
        </View>
      </ScreenScrollView>
    </View>
  );
}

function SettingsItemRow({
  icon,
  title,
  subtitle,
  onPress,
  chevron: Chevron,
}: {
  icon: React.ReactNode;
  title: string;
  subtitle: string;
  onPress: () => void;
  chevron: any;
}) {
  const theme = useTheme();
  return (
    <Pressable
      accessibilityRole="button"
      onPress={onPress}
      className="flex-row items-center justify-between p-four active:bg-secondary/40"
    >
      <View className="flex-row items-center gap-three">
        <View className="h-9 w-9 items-center justify-center rounded-xl bg-primary/10">
          {icon}
        </View>
        <View className="gap-half">
          <Text className="text-sm font-bold text-foreground">{title}</Text>
          <Text className="text-xs text-muted-foreground">{subtitle}</Text>
        </View>
      </View>
      <Chevron size={18} color={theme.mutedForeground} />
    </Pressable>
  );
}
