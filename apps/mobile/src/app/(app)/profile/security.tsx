/**
 * @module @orbis/mobile/app/(app)/profile/security
 *
 * Security & Active Sessions Management for Orbis Mobile.
 * Lists authorized devices, active logins, and enables session revocation.
 */

import React, { useState } from "react";
import { View, Text, Pressable, Alert } from "react-native";
import { useRouter } from "expo-router";
import { Header } from "@/components/header";
import { ScreenScrollView } from "@/components/layout/screen-scroll-view";
import { Card } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { EmptyState } from "@/components/ui/empty-state";
import { Skeleton } from "@/components/ui/skeleton";
import { useToast } from "@/components/ui/toast";
import {
  useDevicesQuery,
  useRevokeDeviceMutation,
} from "@/hooks/use-api-queries";
import { useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";
import { useTheme } from "@/hooks/use-theme";
import {
  ShieldCheck,
  Phone,
  Computer,
  Trash,
  LockClosed,
} from "@orbis/icons/native";

export default function SecurityScreen() {
  const router = useRouter();
  const theme = useTheme();
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
      toast.success(t("security_device_revoked") || "Device access revoked.");
    } catch (error) {
      console.error("[Orbis Mobile Security] Failed to revoke device.", error);
      toast.error(t("security_device_revoke_error") || "Failed to revoke device.");
    } finally {
      setRevokingId(undefined);
    }
  };

  const confirmRevoke = (deviceId: string) => {
    Alert.alert(
      t("security_revoke_device") || "Revoke Device",
      t("security_revoke_confirm") || "Are you sure you want to sign out this device?",
      [
        { text: t("common_cancel") || "Cancel", style: "cancel" },
        {
          text: t("security_revoke_device") || "Revoke",
          style: "destructive",
          onPress: () => void revokeDevice(deviceId),
        },
      ],
    );
  };

  return (
    <View className="flex-1 bg-background">
      <Header
        title={t("profile_menu_security") || "Security"}
        showBack={true}
        onBack={() => router.back()}
      />

      <ScreenScrollView
        edges={["bottom", "left", "right"]}
        scrollViewProps={{
          contentContainerClassName: "gap-six px-four pb-[100px] pt-two",
        }}
        refreshing={devicesQuery.isRefetching}
        onRefresh={() => void devicesQuery.refetch()}
      >
        {/* Security Overview Banner */}
        <Card
          variant="default"
          padding="four"
          className="bg-card border border-border/80 shadow-sm"
        >
          <View className="flex-row items-center gap-three">
            <View className="h-12 w-12 items-center justify-center rounded-2xl bg-primary/10">
              <ShieldCheck size={28} color={theme.primary} />
            </View>
            <View className="flex-1 gap-half">
              <Text className="text-base font-bold text-foreground">
                {t("security_account_status") || "Account Protection"}
              </Text>
              <Text className="text-xs text-muted-foreground">
                {user?.is_active
                  ? t("security_protected_active") || "Multi-device synchronization active"
                  : t("profile_account_inactive")}
              </Text>
            </View>
          </View>
        </Card>

        {/* Active Devices & Logins */}
        <View className="gap-three">
          <View className="flex-row items-center justify-between px-one">
            <Text className="text-xs font-bold uppercase tracking-wider text-muted-foreground">
              {t("security_devices_title") || "Authorized Devices"}
            </Text>
            {devices.length > 0 ? (
              <Badge variant="secondary" className="px-two py-half">
                <Text className="text-xs font-bold text-primary">
                  {devices.length} {t("security_active_sessions") || "Active"}
                </Text>
              </Badge>
            ) : null}
          </View>

          {devicesQuery.isPending ? (
            <View className="gap-two">
              <Skeleton className="h-20 w-full rounded-2xl" />
              <Skeleton className="h-20 w-full rounded-2xl" />
            </View>
          ) : devices.length === 0 ? (
            <Card variant="outline" padding="four">
              <Text className="text-center text-xs text-muted-foreground">
                {t("security_no_devices") || "No additional active devices."}
              </Text>
            </Card>
          ) : (
            <View className="gap-two">
              {devices.map((device) => {
                const isPhone = device.device_type?.toLowerCase().includes("mobile") || device.device_type?.toLowerCase().includes("phone");
                const isRevoking = revokingId === device.id;

                return (
                  <View
                    key={device.id}
                    className="flex-row items-center justify-between rounded-2xl border border-border/80 bg-card p-four shadow-2xs"
                  >
                    <View className="flex-row items-center gap-three flex-1">
                      <View className="h-10 w-10 items-center justify-center rounded-xl bg-secondary">
                        {isPhone ? (
                          <Phone size={20} color={theme.foreground} />
                        ) : (
                          <Computer size={20} color={theme.foreground} />
                        )}
                      </View>
                      <View className="flex-1 gap-half">
                        <View className="flex-row items-center gap-two">
                          <Text className="text-sm font-bold text-foreground" numberOfLines={1}>
                            {device.device_name || "Device"}
                          </Text>
                          {device.is_trusted ? (
                            <Badge variant="default" className="bg-primary/20 px-two py-half">
                              <Text className="text-[10px] font-bold text-primary">
                                {t("security_current_device") || "Trusted"}
                              </Text>
                            </Badge>
                          ) : null}
                        </View>
                        <Text className="text-xs text-muted-foreground">
                          {device.os || device.browser || "Active"} · {formatDate(device.last_active_at)}
                        </Text>
                      </View>
                    </View>
                  </View>
                );
              })}
            </View>
          )}
        </View>
      </ScreenScrollView>
    </View>
  );
}
