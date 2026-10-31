import { BuildingOffice, Refresh } from "@orbis/icons/native";
import Constants, { ExecutionEnvironment } from "expo-constants";
import { useIsFocused } from "expo-router";
import { useCallback, useEffect, useMemo, useState } from "react";
import { ActivityIndicator, Pressable, Text, View } from "react-native";
import Zeroconf, { useZeroconf } from "react-native-zeroconf";
import { Section } from "@/components/layout/section";
import { InstitutionCard } from "@/components/institutions/institution-card";
import { useTheme } from "@/hooks/use-theme";
import {
  createDiscoveredLocalInstitution,
  type DirectoryInstitution,
} from "@/lib/api/institution-discovery";
import { useI18n } from "@/lib/i18n";

type LocalNetworkAccess = "granted" | "denied" | "unknown";

export const isExpoGo =
  Constants.executionEnvironment === ExecutionEnvironment.StoreClient;

interface LocalInstitutionDiscoveryProps {
  connectingId: string | null;
  selectedInstitution: DirectoryInstitution | null;
  onConnect: (institution: DirectoryInstitution) => void;
}

export function LocalInstitutionDiscovery({
  connectingId,
  selectedInstitution,
  onConnect,
}: LocalInstitutionDiscoveryProps) {
  const isFocused = useIsFocused();
  const theme = useTheme();
  const { t, isArabic } = useI18n();
  const [networkAccess, setNetworkAccess] = useState<
    LocalNetworkAccess | "checking" | "unavailable"
  >("checking");
  const [permissionAttempt, setPermissionAttempt] = useState(0);

  useEffect(() => {
    if (!isFocused) return;

    let active = true;
    let zeroconf: Zeroconf | undefined;
    void Promise.resolve()
      .then(() => {
        if (!active) return undefined;
        zeroconf = new Zeroconf();
        return zeroconf.checkLocalNetworkAccess({
          type: "orbis-api",
          protocol: "tcp",
        });
      })
      .then((access) => {
        if (active && access) setNetworkAccess(access);
      })
      .catch((error: unknown) => {
        console.warn(
          "[Institution Discovery] Local network permission check failed.",
          error,
        );
        if (active) setNetworkAccess("unavailable");
      })
      .finally(() => {
        zeroconf?.removeDeviceListeners();
      });

    return () => {
      active = false;
      zeroconf?.removeDeviceListeners();
    };
  }, [isFocused, permissionAttempt]);

  const {
    services,
    isScanning,
    error: scanError,
    restart,
  } = useZeroconf({
    type: "orbis-api",
    protocol: "tcp",
    resolveTimeout: 4,
    enabled: isFocused && networkAccess === "granted",
  });

  const institutions = useMemo(() => {
    const uniqueInstitutions = new Map<string, DirectoryInstitution>();
    for (const service of services) {
      const institution = createDiscoveredLocalInstitution(service);
      if (institution) {
        uniqueInstitutions.set(institution.deployment_id, institution);
      }
    }
    return [...uniqueInstitutions.values()];
  }, [services]);

  const retryDiscovery = useCallback(() => {
    if (networkAccess === "granted") {
      restart();
      return;
    }
    setNetworkAccess("checking");
    setPermissionAttempt((attempt) => attempt + 1);
  }, [networkAccess, restart]);

  return (
    <Section
      title={t("auth_institution_lan_title")}
      subtitle={t("auth_institution_lan_description")}
      spacing="four"
      action={
        <Pressable
          accessibilityRole="button"
          accessibilityLabel={t("auth_institution_lan_retry")}
          onPress={retryDiscovery}
          hitSlop={8}
          className="min-h-11 min-w-11 items-center justify-center rounded-full bg-background-element"
        >
          <Refresh size={18} color={theme.textMuted} />
        </Pressable>
      }
    >
      {institutions.length > 0 ? (
        <View className="gap-2">
          {institutions.map((institution) => (
            <InstitutionCard
              key={institution.deployment_id}
              institution={institution}
              name={
                isArabic && institution.name_ar
                  ? institution.name_ar
                  : institution.name
              }
              isCurrent={
                selectedInstitution?.deployment_id === institution.deployment_id
              }
              isBusy={connectingId === institution.deployment_id}
              disabled={connectingId !== null}
              localLabel={t("auth_institution_local_label")}
              onPress={onConnect}
            />
          ))}
        </View>
      ) : (
        <View className="min-h-20 flex-row items-center gap-3 rounded-2xl border border-border bg-card px-4 py-3">
          {networkAccess === "checking" || isScanning ? (
            <ActivityIndicator size="small" color={theme.primary} />
          ) : (
            <BuildingOffice size={20} color={theme.textMuted} />
          )}
          <Text
            accessibilityRole={
              networkAccess === "denied" || networkAccess === "unavailable"
                ? "alert"
                : undefined
            }
            className="flex-1 text-start text-sm leading-5 text-muted-foreground"
          >
            {networkAccess === "checking"
              ? t("auth_institution_lan_checking")
              : networkAccess === "denied"
                ? t("auth_institution_lan_permission_error")
                : networkAccess === "unavailable" || scanError
                  ? t("auth_institution_lan_error")
                  : t("auth_institution_lan_empty")}
          </Text>
        </View>
      )}
    </Section>
  );
}
