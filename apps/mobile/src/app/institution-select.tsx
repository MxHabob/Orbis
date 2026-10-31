import { BuildingOffice, CheckCircle, Search } from "@orbis/icons/native";
import { useQuery } from "@tanstack/react-query";
import { useLocalSearchParams, useRouter } from "expo-router";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { FlatList, Platform, StyleSheet, Text, View } from "react-native";
import type { ListRenderItemInfo } from "react-native";
import { useSafeAreaInsets } from "react-native-safe-area-context";
import { Screen } from "@/components/layout/screen";
import { InstitutionCard } from "@/components/institutions/institution-card";
import { InstitutionPickerHeader } from "@/components/institutions/institution-picker-header";
import {
  isLocalDiscoveryAvailable,
  LocalInstitutionDiscovery,
} from "@/components/institutions/local-institution-discovery";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { EmptyState } from "@/components/ui/empty-state";
import { Input } from "@/components/ui/input";
import { Skeleton } from "@/components/ui/skeleton";
import { useTheme } from "@/hooks/use-theme";
import { useAuth } from "@/lib/api";
import {
  checkInstitutionConnection,
  parseLocalInstitutionQr,
  type DirectoryInstitution,
  searchInstitutionDirectory,
} from "@/lib/api/institution-discovery";
import { useI18n } from "@/lib/i18n";

export default function InstitutionSelectScreen() {
  const router = useRouter();
  const { institutionQr } = useLocalSearchParams<{
    institutionQr?: string;
  }>();
  const insets = useSafeAreaInsets();
  const topInset =
    Platform.OS === "ios" ||
    (Platform.OS === "android" && Number(Platform.Version) >= 35)
      ? insets.top
      : 0;
  const theme = useTheme();
  const { t, locale, setLocale, isArabic } = useI18n();
  const {
    isInitializing,
    institutions: savedInstitutions,
    selectedInstitution,
    selectInstitution,
  } = useAuth();
  const [search, setSearch] = useState("");
  const [connectingId, setConnectingId] = useState<string | null>(null);
  const [screenError, setScreenError] = useState<string | null>(null);
  const processedQrRef = useRef<string | null>(null);
  const qrValue = typeof institutionQr === "string" ? institutionQr : null;

  const directoryQuery = useQuery({
    queryKey: ["institution-directory"],
    queryFn: ({ signal }: { signal: AbortSignal }) =>
      searchInstitutionDirectory(undefined, signal),
    enabled: !isInitializing,
    staleTime: 5 * 60 * 1000,
  });

  const visibleInstitutions = useMemo(() => {
    const institutionsByDeploymentId = new Map<string, DirectoryInstitution>();
    for (const institution of directoryQuery.data ?? []) {
      if (institution.is_local) continue;
      institutionsByDeploymentId.set(institution.deployment_id, institution);
    }
    for (const institution of savedInstitutions) {
      if (institution.is_local) continue;
      institutionsByDeploymentId.set(institution.deployment_id, institution);
    }

    const query = search.trim().toLocaleLowerCase();
    return [...institutionsByDeploymentId.values()].filter(
      (institution) =>
        !query ||
        [
          institution.name,
          institution.name_ar ?? "",
          institution.slug,
          institution.deployment_name,
        ].some((value) => value.toLocaleLowerCase().includes(query)),
    );
  }, [directoryQuery.data, savedInstitutions, search]);
  const connectInstitution = useCallback(
    async (institution: DirectoryInstitution) => {
      setConnectingId(institution.deployment_id);
      setScreenError(null);
      try {
        await checkInstitutionConnection(institution);
        await selectInstitution(institution);
        router.replace("/");
      } catch (caught: unknown) {
        console.error("[Institution Select] Connection failed.", caught);
        const message = t("auth_institution_connection_error");
        setScreenError(message);
      } finally {
        setConnectingId(null);
      }
    },
    [router, selectInstitution, t],
  );

  useEffect(() => {
    if (!qrValue || isInitializing || processedQrRef.current === qrValue) {
      return;
    }

    processedQrRef.current = qrValue;
    router.setParams({ institutionQr: undefined });
    void Promise.resolve()
      .then(() => connectInstitution(parseLocalInstitutionQr(qrValue)))
      .catch((caught: unknown) => {
        console.error("[Institution Select] QR code is invalid.", caught);
        setScreenError(t("auth_institution_local_qr_error"));
      });
  }, [connectInstitution, isInitializing, qrValue, router, t]);

  const selectedLocalInstitution = selectedInstitution?.is_local
    ? selectedInstitution.local_source === "qr" ||
      selectedInstitution.local_source === "discovery"
      ? selectedInstitution
      : null
    : null;
  const selectedLocalNeedsReconnect =
    selectedInstitution?.is_local === true && !selectedLocalInstitution;
  const selectedLocalHealthQuery = useQuery({
    queryKey: [
      "institution-local-health",
      selectedLocalInstitution?.deployment_id,
    ],
    queryFn: () => {
      if (!selectedLocalInstitution) {
        throw new Error("No local institution is selected.");
      }
      return checkInstitutionConnection(selectedLocalInstitution);
    },
    enabled: !isInitializing && selectedLocalInstitution !== null,
    retry: false,
    staleTime: 0,
  });

  const renderInstitution = useCallback(
    ({ item }: ListRenderItemInfo<DirectoryInstitution>) => (
      <InstitutionCard
        institution={item}
        name={isArabic && item.name_ar ? item.name_ar : item.name}
        isCurrent={selectedInstitution?.deployment_id === item.deployment_id}
        isBusy={connectingId === item.deployment_id}
        disabled={connectingId !== null}
        localLabel={t("auth_institution_local_label")}
        onPress={connectInstitution}
      />
    ),
    [
      connectInstitution,
      connectingId,
      isArabic,
      selectedInstitution?.deployment_id,
      t,
    ],
  );

  const toggleLanguage = useCallback(() => {
    void setLocale(locale === "ar" ? "en" : "ar");
  }, [locale, setLocale]);

  const isLoading = isInitializing || directoryQuery.isPending;
  const directoryError = directoryQuery.isError
    ? t("auth_institution_search_error")
    : null;

  const renderEmpty = useCallback(() => {
    if (directoryError) {
      return (
        <View className="items-center gap-4 px-5 py-8">
          <Text
            accessibilityRole="alert"
            className="text-center text-sm leading-5 text-destructive"
          >
            {directoryError}
          </Text>
          <Button
            title={t("common_retry")}
            variant="outline"
            onPress={() => void directoryQuery.refetch()}
          />
        </View>
      );
    }

    return (
      <EmptyState
        icon={BuildingOffice}
        title={t("auth_institution_no_matches")}
        actionTitle={
          search ? t("auth_institution_empty_search_action") : undefined
        }
        onAction={search ? () => setSearch("") : undefined}
      />
    );
  }, [directoryError, directoryQuery, search, t]);

  return (
    <Screen edges={[]} statusBarStyle="auto">
      <View
        className="flex-1"
        style={{ paddingTop: topInset }}
      >
        <InstitutionPickerHeader
          languageLabel={locale === "ar" ? "English" : "العربية"}
          accessibilityLabel={
            locale === "ar" ? "Switch to English" : "التبديل إلى العربية"
          }
          onToggleLanguage={toggleLanguage}
        />

        <View className="flex-1 overflow-hidden bg-background">
          <FlatList
            data={visibleInstitutions}
            keyExtractor={(institution) => institution.deployment_id}
            renderItem={renderInstitution}
            ListHeaderComponent={
              <View className="gap-3 pb-2">
                <Text className="text-start text-2xl leading-8 font-bold text-foreground">
                  {t("auth_institution_search_title")}
                </Text>

                <Input
                  accessibilityLabel={t("auth_institution_filter_label")}
                  placeholder={t("auth_institution_search_placeholder")}
                  value={search}
                  onChangeText={setSearch}
                  icon={<Search size={18} color={theme.textMuted} />}
                  returnKeyType="search"
                  autoCapitalize="none"
                  autoCorrect={false}
                  clearButtonMode="while-editing"
                />

                {Platform.OS !== "web" ? (
                  <Button
                    title={t("auth_institution_local_scan")}
                    variant="outline"
                    size="md"
                    fullWidth
                    onPress={() => router.push("/institution-scan")}
                  />
                ) : null}

                {Platform.OS !== "web" && !isLocalDiscoveryAvailable ? (
                  <Text className="text-start text-xs leading-4 text-muted-foreground">
                    {t("auth_institution_lan_build_required")}
                  </Text>
                ) : isLocalDiscoveryAvailable ? (
                  <LocalInstitutionDiscovery
                    connectingId={connectingId}
                    selectedInstitution={selectedInstitution}
                    onConnect={connectInstitution}
                  />
                ) : null}

                {isLoading ? (
                  <Card
                    variant="flat"
                    padding="three"
                    className="min-h-[84px] flex-row items-center rounded-2xl bg-card"
                  >
                    <Skeleton width={56} height={56} borderRadius={16} />
                    <View className="ms-3 flex-1 gap-2">
                      <Skeleton width="72%" height={16} borderRadius={6} />
                      <Skeleton width="48%" height={12} borderRadius={6} />
                    </View>
                  </Card>
                ) : null}
              </View>
            }
            ListEmptyComponent={
              visibleInstitutions.length === 0 && !isLoading
                ? renderEmpty
                : undefined
            }
            ListFooterComponent={
              directoryError && visibleInstitutions.length > 0 ? (
                <View className="mt-4 flex-row items-center justify-between gap-3 rounded-xl border border-destructive/30 bg-destructive/10 px-3 py-2">
                  <Text
                    accessibilityRole="alert"
                    className="flex-1 text-start text-xs leading-5 text-destructive"
                  >
                    {directoryError}
                  </Text>
                  <Button
                    title={t("common_retry")}
                    variant="outline"
                    size="sm"
                    onPress={() => void directoryQuery.refetch()}
                  />
                </View>
              ) : undefined
            }
            contentContainerStyle={[
              styles.listContent,
              { paddingBottom: Math.max(insets.bottom, 16) + 12 },
            ]}
            keyboardShouldPersistTaps="handled"
            keyboardDismissMode="on-drag"
            automaticallyAdjustKeyboardInsets
            showsVerticalScrollIndicator={false}
            refreshing={directoryQuery.isRefetching}
            onRefresh={() => void directoryQuery.refetch()}
            initialNumToRender={8}
            maxToRenderPerBatch={8}
            windowSize={7}
          />

          <View
            className="bg-background px-4 pt-3"
            style={{ paddingBottom: Math.max(insets.bottom, 16) }}
          >
            {screenError ? (
              <Text
                accessibilityRole="alert"
                className="mb-2 text-center text-xs leading-4 text-destructive"
              >
                {screenError}
              </Text>
            ) : null}
            {selectedLocalInstitution &&
            !selectedLocalHealthQuery.isSuccess ? (
              <Text
                accessibilityRole={
                  selectedLocalHealthQuery.isError ? "alert" : undefined
                }
                className="mb-3 text-center text-xs leading-4 text-muted-foreground"
              >
                {selectedLocalHealthQuery.isError
                  ? t("auth_institution_connection_error")
                  : t("auth_institution_lan_checking")}
              </Text>
            ) : selectedLocalNeedsReconnect ? (
              <Text
                accessibilityRole="alert"
                className="mb-3 text-center text-xs leading-4 text-muted-foreground"
              >
                {t("auth_institution_local_reconnect_required")}
              </Text>
            ) : selectedInstitution ? (
              <Card
                variant="flat"
                padding="three"
                className="mb-3 flex-row items-center gap-3 rounded-xl bg-primary/5"
              >
                <CheckCircle size={17} color={theme.primary} />
                <View className="min-w-0 flex-1">
                  <Text className="text-xs leading-4 text-muted-foreground">
                    {t("auth_institution_current")}
                  </Text>
                  <Text
                    numberOfLines={1}
                    className="text-sm font-semibold text-foreground"
                  >
                    {isArabic && selectedInstitution.name_ar
                      ? selectedInstitution.name_ar
                      : selectedInstitution.name}
                  </Text>
                </View>
              </Card>
            ) : null}
            <Button
              title={t("auth_sign_in")}
              size="lg"
              fullWidth
              onPress={() => router.replace("/(auth)/sign-in")}
              disabled={
                !selectedInstitution ||
                connectingId !== null ||
                selectedLocalNeedsReconnect ||
                (selectedLocalInstitution !== null &&
                  !selectedLocalHealthQuery.isSuccess)
              }
            />
          </View>
        </View>
      </View>
    </Screen>
  );
}

const styles = StyleSheet.create({
  listContent: {
    flexGrow: 1,
    paddingHorizontal: 16,
    paddingTop: 8,
    gap: 8,
  },
});
