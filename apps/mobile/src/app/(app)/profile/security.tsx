/**
 * @module @orbis/mobile/app/(app)/profile/security
 *
 * Device sessions and account protection in the shared settings-panel style.
 */

import React, { useState } from "react";
import { Alert, Pressable, Text, View } from "react-native";
import { useRouter } from "expo-router";
import { useSafeAreaInsets } from "react-native-safe-area-context";

import { HEADER_CONTENT_OFFSET, Header } from "@/components/header";
import { ScreenScrollView } from "@/components/layout/screen-scroll-view";
import { Card } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import {
  SettingsPanelDivider,
  SettingsPanelTitle,
} from "@/components/ui/settings-panel";
import { useToast } from "@/components/ui/toast";
import {
  useDevicesQuery,
  useRevokeDeviceMutation,
} from "@/hooks/use-api-queries";
import { useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";

export default function SecurityScreen() {
  const router = useRouter();
  const insets = useSafeAreaInsets();
  const { user } = useAuth();
  const { t, formatDate } = useI18n();
  const toast = useToast();
  const devicesQuery = useDevicesQuery();
  const revokeMutation = useRevokeDeviceMutation();
  const [revokingId, setRevokingId] = useState<string>();
  const devices = devicesQuery.data ?? [];

  const revokeDevice = async (deviceId: string) => {
    setRevokingId(deviceId);
    try {
      await revokeMutation.mutateAsync(deviceId);
      toast.success(t("security_device_revoked"));
    } catch (error) {
      console.error("[Orbis Mobile Security] Failed to revoke device.", error);
      toast.error(t("security_device_revoke_error"));
    } finally {
      setRevokingId(undefined);
    }
  };

  const confirmRevoke = (deviceId: string) => {
    Alert.alert(t("security_revoke_device"), t("security_revoke_confirm"), [
      { text: t("common_cancel"), style: "cancel" },
      {
        text: t("security_revoke_device"),
        style: "destructive",
        onPress: () => void revokeDevice(deviceId),
      },
    ]);
  };

  return (
    <View className="flex-1 bg-background">
      <Header
        title={t("profile_menu_security")}
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
        refreshing={devicesQuery.isRefetching}
        onRefresh={() => void devicesQuery.refetch()}
      >
        <Card variant="default" padding="none">
          <SettingsPanelTitle title={t("security_account_status")} />
          <SecurityRow
            label={t("security_description")}
            value={
              user?.is_active
                ? t("security_protected_active")
                : t("profile_account_inactive")
            }
          />
        </Card>

        <Card variant="default" padding="none">
          <SettingsPanelTitle
            title={t("security_devices_title")}
            trailing={
              devices.length > 0
                ? `${devices.length} ${t("security_active_sessions")}`
                : undefined
            }
          />
          {devicesQuery.isPending ? (
            <View className="gap-three px-four py-three">
              <Skeleton className="h-9 w-full rounded-sm" />
              <SettingsPanelDivider />
              <Skeleton className="h-9 w-full rounded-sm" />
            </View>
          ) : devicesQuery.isError && devices.length === 0 ? (
            <View className="min-h-16 flex-row items-center justify-between gap-four px-four py-three">
              <Text className="flex-1 text-start text-xs text-muted-foreground">
                {t("security_devices_error")}
              </Text>
              <Pressable
                accessibilityRole="button"
                onPress={() => void devicesQuery.refetch()}
                className="min-h-11 justify-center rounded-sm border border-border px-three active:bg-secondary/50"
              >
                <Text className="text-xs font-medium text-foreground">
                  {t("common_retry")}
                </Text>
              </Pressable>
            </View>
          ) : devices.length === 0 ? (
            <View className="min-h-16 justify-center px-four py-three">
              <Text className="text-end text-xs text-muted-foreground">
                {t("security_no_devices")}
              </Text>
            </View>
          ) : (
            devices.map((device, index) => {
              const isRevoking = revokingId === device.id;
              const deviceDescription = [
                device.os || device.browser || t("common_unknown"),
                t("security_last_active"),
                formatDate(device.last_active_at),
                device.is_trusted ? t("security_device_trusted") : undefined,
              ]
                .filter(Boolean)
                .join(" · ");

              return (
                <React.Fragment key={device.id}>
                  {index > 0 ? <SettingsPanelDivider /> : null}
                  <View className="min-h-16 flex-row items-center justify-between gap-four px-four py-three">
                    <View className="min-w-0 flex-1 items-start gap-half">
                      <Text
                        numberOfLines={1}
                        className="text-start text-sm font-medium text-foreground"
                      >
                        {device.device_name || t("common_unknown")}
                      </Text>
                      <Text
                        numberOfLines={2}
                        className="text-start text-xs leading-4 text-muted-foreground"
                      >
                        {deviceDescription}
                      </Text>
                    </View>
                    <Pressable
                      accessibilityRole="button"
                      accessibilityLabel={`${t("security_revoke_device")}: ${device.device_name || t("common_unknown")}`}
                      disabled={isRevoking}
                      onPress={() => confirmRevoke(device.id)}
                      className={`min-h-11 justify-center rounded-sm border border-destructive px-three ${
                        isRevoking ? "opacity-50" : "active:bg-destructive/10"
                      }`}
                    >
                      <Text className="text-xs font-medium text-destructive">
                        {isRevoking
                          ? t("common_loading")
                          : t("security_revoke_device")}
                      </Text>
                    </Pressable>
                  </View>
                </React.Fragment>
              );
            })
          )}
        </Card>
      </ScreenScrollView>
    </View>
  );
}

function SecurityRow({ label, value }: { label: string; value: string }) {
  return (
    <View className="min-h-16 flex-row items-center justify-between gap-four px-four py-three">
      <Text className="min-w-0 flex-1 text-start text-xs text-muted-foreground">
        {label}
      </Text>
      <Text className="min-w-0 flex-1 text-sm text-foreground">{value}</Text>
    </View>
  );
}
