import { ScreenScrollView } from "@/components/layout/screen-scroll-view";
import { Card } from "@/components/ui/card";
import { EmptyState } from "@/components/ui/empty-state";
import { LoadingState } from "@/components/ui/loading-state";
import { Button } from "@/components/ui/button";
import { useToast } from "@/components/ui/toast";
import {
  useDevicesQuery,
  useRevokeDeviceMutation,
} from "@/hooks/use-api-queries";
import { useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";
import { useState } from "react";
import { Alert, Text, View } from "react-native";

export default function SecurityScreen() {
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
    <ScreenScrollView
      horizontalPadding="five"
      verticalPadding="four"
      scrollViewProps={{ contentContainerClassName: "gap-four pb-10" }}
      refreshing={devicesQuery.isRefetching}
      onRefresh={() => void devicesQuery.refetch()}
    >
      <View className="gap-one">
        <Text className="text-2xl font-bold text-foreground">
          {t("profile_menu_security")}
        </Text>
        <Text className="text-sm leading-5 text-muted-foreground">
          {t("security_description")}
        </Text>
      </View>

      <Card variant="outline" padding="four">
        <View className="gap-one">
          <Text className="text-base font-semibold text-foreground">
            {t("security_mfa_title")}
          </Text>
          <Text className="text-sm text-muted-foreground">
            {user?.is_mfa_enabled
              ? t("security_mfa_enabled")
              : t("security_mfa_disabled")}
          </Text>
        </View>
      </Card>

      <View className="gap-three">
        <Text className="text-lg font-bold text-foreground">
          {t("security_devices_title")}
        </Text>
        {devicesQuery.isPending ? (
          <LoadingState message={t("common_loading")} />
        ) : devicesQuery.isError ? (
          <EmptyState
            title={t("common_error")}
            description={t("security_devices_error")}
            actionTitle={t("common_retry")}
            onAction={() => void devicesQuery.refetch()}
          />
        ) : devices.length === 0 ? (
          <EmptyState title={t("security_no_devices")} />
        ) : (
          devices.map((device) => (
            <Card key={device.id} variant="outline" padding="four">
              <View className="gap-three">
                <View className="gap-one">
                  <Text className="text-base font-semibold text-foreground">
                    {device.device_name}
                  </Text>
                  <Text className="text-sm text-muted-foreground">
                    {[device.os, device.browser, device.device_type]
                      .filter(Boolean)
                      .join(" · ")}
                  </Text>
                  <Text className="text-xs text-muted-foreground">
                    {t("security_last_active")}: {formatDate(device.last_active_at)}
                  </Text>
                  <Text className="text-xs text-muted-foreground">
                    {device.is_trusted
                      ? t("security_device_trusted")
                      : t("security_device_untrusted")}
                  </Text>
                </View>
                <Button
                  title={t("security_revoke_device")}
                  variant="outline"
                  size="sm"
                  disabled={revokingId === device.id || revokeMutation.isPending}
                  onPress={() => confirmRevoke(device.id)}
                />
              </View>
            </Card>
          ))
        )}
      </View>
      <Text className="text-xs leading-5 text-muted-foreground">
        {t("security_mfa_management_note")}
      </Text>
    </ScreenScrollView>
  );
}
