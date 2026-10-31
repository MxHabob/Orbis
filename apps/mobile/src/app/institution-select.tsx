import { BuildingOffice, CheckCircle, Search } from "@orbis/icons/native";
import { useQuery } from "@tanstack/react-query";
import { useRouter } from "expo-router";
import { useCallback, useMemo, useState } from "react";
import {
  FlatList,
  Platform,
  StyleSheet,
  Text,
  useWindowDimensions,
  View,
} from "react-native";
import type { ListRenderItemInfo } from "react-native";
import { useSafeAreaInsets } from "react-native-safe-area-context";
import { Screen } from "@/components/layout/screen";
import { Section } from "@/components/layout/section";
import { InstitutionCard } from "@/components/institutions/institution-card";
import { InstitutionPickerHeader } from "@/components/institutions/institution-picker-header";
import {
  isExpoGo,
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
  type DirectoryInstitution,
  searchInstitutionDirectory,
} from "@/lib/api/institution-discovery";
import { useI18n } from "@/lib/i18n";

export default function InstitutionSelectScreen() {
  const router = useRouter();
  const insets = useSafeAreaInsets();
  const { width: windowWidth } = useWindowDimensions();
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
      institutionsByDeploymentId.set(institution.deployment_id, institution);
    }
    for (const institution of savedInstitutions) {
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
  const featuredInstitution = visibleInstitutions[0];
  const otherInstitutions = visibleInstitutions.slice(1);

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
      <View className="flex-1" style={{ paddingTop: insets.top + 8 }}>
        <InstitutionPickerHeader
          languageLabel={locale === "ar" ? "English" : "العربية"}
          accessibilityLabel={
            locale === "ar" ? "Switch to English" : "التبديل إلى العربية"
          }
          onToggleLanguage={toggleLanguage}
        />

        <View className="flex-1 overflow-hidden bg-background">
          <FlatList
            data={otherInstitutions}
            keyExtractor={(institution) => institution.deployment_id}
            renderItem={renderInstitution}
            ListHeaderComponent={
              <View className="gap-4 pb-4">
                {Platform.OS !== "web" && isExpoGo ? (
                  <Section
                    title={t("auth_institution_lan_title")}
                    subtitle={t("auth_institution_lan_description")}
                    spacing="one"
                  >
                    <View className="rounded-2xl border border-border bg-card px-4 py-3">
                      <Text className="text-start text-sm leading-5 text-muted-foreground">
                        {t("auth_institution_lan_expo_go")}
                      </Text>
                    </View>
                  </Section>
                ) : Platform.OS !== "web" ? (
                  <LocalInstitutionDiscovery
                    connectingId={connectingId}
                    selectedInstitution={selectedInstitution}
                    onConnect={connectInstitution}
                  />
                ) : null}

                <View>
                  <Text className="text-start text-2xl leading-8 font-bold text-foreground">
                    {t("auth_institution_search_title")}
                  </Text>
                  <Text className="mt-1 text-start text-sm leading-5 text-muted-foreground">
                    {t("auth_institution_search_description")}
                  </Text>
                </View>

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

                {featuredInstitution ? (
                  <Section
                    title={t("auth_institution_featured_recommended")}
                    spacing="one"
                    headerSpacing="two"
                  >
                    <InstitutionCard
                      institution={featuredInstitution}
                      name={
                        isArabic && featuredInstitution.name_ar
                          ? featuredInstitution.name_ar
                          : featuredInstitution.name
                      }
                      isCurrent={
                        selectedInstitution?.deployment_id ===
                        featuredInstitution.deployment_id
                      }
                      isBusy={
                        connectingId === featuredInstitution.deployment_id
                      }
                      disabled={connectingId !== null}
                      localLabel={t("auth_institution_local_label")}
                      onPress={connectInstitution}
                      width={windowWidth - 32}
                      variant="featured"
                    />
                  </Section>
                ) : isLoading ? (
                  <Section
                    title={t("auth_institution_featured_recommended")}
                    spacing="one"
                    headerSpacing="two"
                  >
                    <Card
                      variant="flat"
                      padding="five"
                      className="min-h-[136px] flex-row items-center rounded-2xl bg-card shadow-sm"
                      style={{ width: windowWidth - 32 }}
                    >
                      <Skeleton width={72} height={72} borderRadius={16} />
                      <View className="ms-4 flex-1 gap-2">
                        <Skeleton width="76%" height={18} borderRadius={6} />
                        <Skeleton width="54%" height={12} borderRadius={6} />
                      </View>
                    </Card>
                  </Section>
                ) : null}

                <Section
                  title={t("auth_institution_results")}
                  spacing="one"
                  headerSpacing={0}
                />
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
            {selectedInstitution ? (
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
              disabled={!selectedInstitution || connectingId !== null}
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
    paddingTop: 24,
    gap: 12,
  },
});
