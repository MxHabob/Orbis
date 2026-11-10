import { Refresh } from "@orbis/icons/native";
import { useIsFocused } from "expo-router";
import { useCallback, useEffect, useMemo, useState } from "react";
import { Platform, Pressable, TurboModuleRegistry, View, type TurboModule } from "react-native";
import Zeroconf, { useZeroconf } from "react-native-zeroconf";
import {
  InstitutionCard,
  type InstitutionCardProps,
} from "@/components/institutions/institution-card";
import { useTheme } from "@/hooks/use-theme";
import {
  createDiscoveredLocalInstitution,
  type DirectoryInstitution,
} from "@/lib/api/institution-discovery";
import { useI18n } from "@/lib/i18n";
import { Text } from "@/components/ui/text";

type LocalNetworkAccess = "granted" | "denied" | "unknown";

export const isLocalDiscoveryAvailable =
  Platform.OS !== "web" &&
  TurboModuleRegistry.get<TurboModule>("Zeroconf") !== null;

interface LocalInstitutionDiscoveryProps {
  title: string;
  connectingId: string | null;
  selectedInstitution: DirectoryInstitution | null;
  onConnect: (institution: DirectoryInstitution) => void;
  institutionCardVariant?: InstitutionCardProps["variant"];
}

export function LocalInstitutionDiscovery({
  title,
  connectingId,
  selectedInstitution,
  onConnect,
  institutionCardVariant,
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
    <View className="gap-2">
      <View className="min-h-11 flex-row items-center justify-between">
        <Text className="text-start text-xs font-medium tracking-[1px] text-muted-foreground">
          {title.toLocaleUpperCase(isArabic ? "ar" : "en")}
        </Text>
        <Pressable
          accessibilityRole="button"
          accessibilityLabel={t("auth_institution_lan_retry")}
          accessibilityState={{
            disabled:
              networkAccess === "checking" ||
              (isScanning && institutions.length === 0),
          }}
          disabled={
            networkAccess === "checking" ||
            (isScanning && institutions.length === 0)
          }
          onPress={retryDiscovery}
          hitSlop={8}
          className="min-h-11 min-w-11 items-center justify-center rounded-full"
        >
          <Refresh size={17} color={theme.info} />
        </Pressable>
      </View>
      {institutions.length > 0 ? (
        institutions.map((institution) => (
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
            variant={institutionCardVariant}
            onPress={onConnect}
          />
        ))
      ) : (
        <View className="min-h-[68px] flex-row items-center gap-3 rounded-2xl bg-card px-4 py-3">
          <View className="h-2.5 w-2.5 rounded-full bg-muted-foreground/50" />
          <Text
            accessibilityRole={
              networkAccess === "denied" || networkAccess === "unavailable"
                ? "alert"
                : undefined
            }
            numberOfLines={2}
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
    </View>
  );
}
