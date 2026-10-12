import { ScreenScrollView } from "@/components/layout/screen-scroll-view";
import { Card } from "@/components/ui/card";
import { EmptyState } from "@/components/ui/empty-state";
import { useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";
import type { TranslationKey } from "@/lib/i18n/types";
import { Text, View } from "react-native";

const roleKeys: Record<string, TranslationKey> = {
  teacher: "auth_role_teacher",
  student: "auth_role_student",
  parent: "auth_role_parent",
  guardian: "auth_role_parent",
  admin: "auth_role_admin",
};

export default function AccountScreen() {
  const { user } = useAuth();
  const { t, isArabic, formatDate } = useI18n();

  if (!user) {
    return (
      <ScreenScrollView
        horizontalPadding="five"
        verticalPadding="four"
        scrollViewProps={{ contentContainerClassName: "flex-grow justify-center" }}
      >
        <EmptyState
          title={t("common_error")}
          description={t("profile_account_unavailable")}
        />
      </ScreenScrollView>
    );
  }

  const roles = (user.roles ?? []).map((role) => {
    const key = roleKeys[role.toLowerCase()];
    return key ? t(key) : role;
  });

  return (
    <ScreenScrollView
      horizontalPadding="five"
      verticalPadding="four"
      scrollViewProps={{ contentContainerClassName: "gap-four pb-10" }}
    >
      <View className="gap-one">
        <Text className="text-2xl font-bold text-foreground">
          {t("profile_menu_account")}
        </Text>
        <Text className="text-sm text-muted-foreground">
          {t("profile_account_read_only")}
        </Text>
      </View>

      <Card variant="outline" padding="four">
        <View className="gap-three">
          <AccountField
            label={t("auth_full_name")}
            value={
              isArabic
                ? user.full_name_ar || user.full_name
                : user.full_name
            }
          />
          <AccountField
            label={t("profile_email")}
            value={user.email || t("common_unknown")}
          />
          <AccountField
            label={t("profile_phone")}
            value={user.phone || t("common_unknown")}
          />
          <AccountField
            label={t("profile_roles")}
            value={
              roles.length > 0
                ? roles.join(isArabic ? "، " : ", ")
                : t("common_unknown")
            }
          />
          <AccountField
            label={t("profile_account_status")}
            value={
              user.is_active
                ? t("profile_account_active")
                : t("profile_account_inactive")
            }
          />
          <AccountField
            label={t("profile_email_verified")}
            value={
              user.is_email_verified
                ? t("profile_verified")
                : t("profile_not_verified")
            }
          />
          <AccountField
            label={t("profile_account_created")}
            value={formatDate(user.created_at) || t("common_unknown")}
          />
        </View>
      </Card>
    </ScreenScrollView>
  );
}

function AccountField({ label, value }: { label: string; value: string }) {
  return (
    <View className="flex-row items-start justify-between gap-three border-b border-border pb-two">
      <Text className="flex-1 text-sm text-muted-foreground">{label}</Text>
      <Text className="flex-1 text-end text-sm font-medium text-foreground">
        {value}
      </Text>
    </View>
  );
}
