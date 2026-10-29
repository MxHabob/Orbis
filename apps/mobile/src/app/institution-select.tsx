import {
  BuildingOffice,
  Camera,
  CheckCircle,
  Pin,
  Search,
} from "@orbis/icons/native";
import { useQuery } from "@tanstack/react-query";
import * as Location from "expo-location";
import { useLocalSearchParams, useRouter } from "expo-router";
import { useCallback, useEffect, useMemo, useState } from "react";
import {
  FlatList,
  Modal,
  Pressable,
  StyleSheet,
  Text,
  useWindowDimensions,
  View,
} from "react-native";
import type { ListRenderItemInfo } from "@react-native/virtualized-lists";
import { useSafeAreaInsets } from "react-native-safe-area-context";
import { Screen } from "@/components/layout/screen";
import { Section } from "@/components/layout/section";
import { InstitutionCard } from "@/components/institutions/institution-card";
import { InstitutionPickerHeader } from "@/components/institutions/institution-picker-header";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import {
  Drawer,
  DrawerContent,
  DrawerDescription,
  DrawerFooter,
  DrawerHeader,
  DrawerScrollView,
  DrawerTitle,
} from "@/components/ui/drawer";
import {
  DrawerFormField,
  DrawerSubmitButton,
} from "@/components/ui/drawer-form-primitives";
import { EmptyState } from "@/components/ui/empty-state";
import { Input } from "@/components/ui/input";
import { Skeleton } from "@/components/ui/skeleton";
import { useTheme } from "@/hooks/use-theme";
import { useAuth } from "@/lib/api";
import {
  checkInstitutionConnection,
  createManualLocalInstitution,
  type DirectoryInstitution,
  parseLocalInstitutionQr,
  searchInstitutionDirectory,
} from "@/lib/api/institution-discovery";
import { useI18n } from "@/lib/i18n";

function haversineKm(
  lat1: number,
  lon1: number,
  lat2: number,
  lon2: number,
): number {
  const toRadians = (degrees: number) => (degrees * Math.PI) / 180;
  const earthRadiusKm = 6371;
  const dLat = toRadians(lat2 - lat1);
  const dLon = toRadians(lon2 - lon1);
  const a =
    Math.sin(dLat / 2) * Math.sin(dLat / 2) +
    Math.cos(toRadians(lat1)) *
      Math.cos(toRadians(lat2)) *
      Math.sin(dLon / 2) *
      Math.sin(dLon / 2);
  const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
  return earthRadiusKm * c;
}

export default function InstitutionSelectScreen() {
  const router = useRouter();
  const insets = useSafeAreaInsets();
  const { width: windowWidth } = useWindowDimensions();
  const { institutionQr } = useLocalSearchParams<{ institutionQr?: string }>();
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
  const [manualEntryError, setManualEntryError] = useState<string | null>(null);
  const [manualEntryVisible, setManualEntryVisible] = useState(false);
  const [localCandidate, setLocalCandidate] =
    useState<DirectoryInstitution | null>(null);
  const [localName, setLocalName] = useState("");
  const [localEndpoint, setLocalEndpoint] = useState("");
  const [userLocation, setUserLocation] = useState<{
    latitude: number;
    longitude: number;
  } | null>(null);

  const directoryQuery = useQuery({
    queryKey: ["institution-directory"],
    queryFn: ({ signal }: { signal: AbortSignal }) => searchInstitutionDirectory(undefined, signal),
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
    const filtered = [...institutionsByDeploymentId.values()].filter(
      (institution) =>
        !query ||
        [
          institution.name,
          institution.name_ar ?? "",
          institution.slug,
          institution.deployment_name,
        ].some((value) => value.toLocaleLowerCase().includes(query)),
    );

    if (!userLocation) return filtered;

    return filtered
      .map((institution) => {
        if (institution.latitude == null || institution.longitude == null) {
          return { institution, distanceKm: Number.POSITIVE_INFINITY };
        }

        return {
          institution,
          distanceKm: haversineKm(
            userLocation.latitude,
            userLocation.longitude,
            institution.latitude,
            institution.longitude,
          ),
        };
      })
      .sort((a, b) => a.distanceKm - b.distanceKm)
      .map(({ institution }) => institution);
  }, [directoryQuery.data, savedInstitutions, search, userLocation]);
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

  const requestNearbyLocation = useCallback(async () => {
    setScreenError(null);
    if (userLocation) {
      setUserLocation(null);
      return;
    }

    try {
      const { status } = await Location.requestForegroundPermissionsAsync();
      if (status !== "granted") {
        setScreenError(t("auth_institution_location_error"));
        return;
      }

      const currentLocation = await Location.getCurrentPositionAsync({
        accuracy: Location.Accuracy.Balanced,
      });
      setUserLocation({
        latitude: currentLocation.coords.latitude,
        longitude: currentLocation.coords.longitude,
      });
    } catch (caught: unknown) {
      console.warn(
        "[Institution Select] Unable to read device location.",
        caught,
      );
      setScreenError(t("auth_institution_location_error"));
    }
  }, [t, userLocation]);

  const toggleLanguage = useCallback(() => {
    void setLocale(locale === "ar" ? "en" : "ar");
  }, [locale, setLocale]);

  useEffect(() => {
    if (!institutionQr) return;
    try {
      setLocalCandidate(parseLocalInstitutionQr(institutionQr));
    } catch (caught: unknown) {
      console.warn(
        "[Institution Select] Rejected an unsupported QR code.",
        caught,
      );
      setScreenError(t("auth_institution_local_qr_error"));
    }
    router.setParams({ institutionQr: undefined });
  }, [institutionQr, router, t]);

  const submitManualInstitution = useCallback(() => {
    try {
      setLocalCandidate(createManualLocalInstitution(localName, localEndpoint));
      setManualEntryError(null);
      setManualEntryVisible(false);
    } catch {
      setManualEntryError(t("auth_institution_local_details_error"));
    }
  }, [localEndpoint, localName, t]);

  const confirmLocalInstitution = useCallback(() => {
    if (!localCandidate) return;
    const candidate = localCandidate;
    setLocalCandidate(null);
    void connectInstitution(candidate);
  }, [connectInstitution, localCandidate]);

  const isLoading = isInitializing || directoryQuery.isPending;
  const directoryError = directoryQuery.isError
    ? t("auth_institution_search_error")
    : null;

  const renderEmpty = useCallback(() => {
    if (isLoading) return null;

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
          search
            ? t("auth_institution_empty_search_action")
            : t("auth_institution_empty_directory_action")
        }
        onAction={() => {
          if (search) {
            setSearch("");
          } else {
            setManualEntryError(null);
            setManualEntryVisible(true);
          }
        }}
      />
    );
  }, [directoryError, directoryQuery, isLoading, search, t]);

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
                    title={
                      userLocation
                        ? t("auth_institution_featured_nearby")
                        : t("auth_institution_featured_recommended")
                    }
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

                <View className="flex-row gap-2">
                  <Button
                    title={t("auth_institution_local_scan")}
                    variant="outline"
                    size="md"
                    className="flex-1"
                    icon={<Camera size={16} color={theme.primary} />}
                    onPress={() => router.push("/institution-scan")}
                  />
                  <Button
                    title={t("auth_institution_local_manual")}
                    variant="ghost"
                    size="md"
                    className="flex-1 border border-border bg-card"
                    onPress={() => {
                      setManualEntryError(null);
                      setManualEntryVisible(true);
                    }}
                  />
                </View>

                <Section
                  title={t("auth_institution_results")}
                  spacing="one"
                  headerSpacing={0}
                  action={
                    <Pressable
                      accessibilityRole="button"
                      accessibilityLabel={t("auth_institution_nearby")}
                      accessibilityState={{ selected: userLocation !== null }}
                      onPress={() => void requestNearbyLocation()}
                      className={`min-h-11 flex-row items-center gap-2 rounded-full px-3 ${
                        userLocation ? "bg-primary/10" : "bg-background-element"
                      }`}
                    >
                      <Pin
                        size={16}
                        color={userLocation ? theme.primary : theme.textMuted}
                      />
                      <Text
                        className={`text-xs font-medium ${
                          userLocation ? "text-primary" : "text-foreground"
                        }`}
                      >
                        {t("auth_institution_nearby")}
                      </Text>
                    </Pressable>
                  }
                />
              </View>
            }
            ListEmptyComponent={
              visibleInstitutions.length === 0 ? renderEmpty : null
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
              ) : null
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

      <Drawer
        visible={manualEntryVisible}
        onClose={() => {
          setManualEntryVisible(false);
          setManualEntryError(null);
        }}
        snapPoints={[0.72]}
      >
        <DrawerContent>
          <DrawerHeader>
            <DrawerTitle>{t("auth_institution_local_title")}</DrawerTitle>
            <DrawerDescription>
              {t("auth_institution_local_hint")}
            </DrawerDescription>
          </DrawerHeader>
          {manualEntryError ? (
            <Text
              accessibilityRole="alert"
              className="px-five text-start text-xs leading-4 font-medium text-destructive"
            >
              {manualEntryError}
            </Text>
          ) : null}
          <DrawerScrollView
            style={{ flex: 1 }}
            contentContainerStyle={{ gap: 16, paddingVertical: 16 }}
          >
            <DrawerFormField
              label={t("auth_institution_local_name")}
              placeholder={t("auth_institution_local_enter_name")}
              value={localName}
              onChangeText={(value) => {
                setLocalName(value);
                setManualEntryError(null);
              }}
              autoCapitalize="words"
              maxLength={100}
              returnKeyType="next"
            />
            <DrawerFormField
              label={t("auth_institution_local_server")}
              placeholder={t("auth_institution_local_server_hint")}
              value={localEndpoint}
              onChangeText={(value) => {
                setLocalEndpoint(value);
                setManualEntryError(null);
              }}
              autoCapitalize="none"
              autoCorrect={false}
              keyboardType="url"
              maxLength={2048}
              returnKeyType="done"
            />
            <Text className="px-5 text-start text-xs leading-5 text-muted-foreground">
              {t("auth_institution_local_secure_hint")}
            </Text>
          </DrawerScrollView>
          <DrawerFooter>
            <DrawerSubmitButton
              label={t("auth_institution_local_confirm")}
              onPress={submitManualInstitution}
            />
          </DrawerFooter>
        </DrawerContent>
      </Drawer>

      <Modal
        visible={localCandidate !== null}
        transparent
        animationType="fade"
        onRequestClose={() => setLocalCandidate(null)}
      >
        <View className="flex-1 items-center justify-center bg-black/55 px-5">
          <View className="w-full max-w-md gap-4 rounded-3xl bg-background p-5">
            <View className="gap-1">
              <Text className="text-start text-xl font-bold text-foreground">
                {t("auth_institution_connection_title")}
              </Text>
              <Text className="text-start text-sm leading-5 text-muted-foreground">
                {t("auth_institution_connection_hint")}
              </Text>
            </View>
            {localCandidate ? (
              <View className="gap-2 rounded-xl border border-border bg-card p-4">
                <Text className="text-start text-xs font-medium text-primary">
                  {t("auth_institution_local_label")}
                </Text>
                <Text className="text-start text-base font-semibold text-foreground">
                  {isArabic && localCandidate.name_ar
                    ? localCandidate.name_ar
                    : localCandidate.name}
                </Text>
                <Text
                  numberOfLines={2}
                  className="text-start text-xs leading-5 text-muted-foreground"
                >
                  {localCandidate.endpoint_url}
                </Text>
              </View>
            ) : null}
            <View className="flex-row gap-3">
              <Button
                title={t("common_cancel")}
                variant="outline"
                size="lg"
                onPress={() => setLocalCandidate(null)}
                className="flex-1"
              />
              <Button
                title={t("auth_institution_local_confirm")}
                size="lg"
                onPress={confirmLocalInstitution}
                className="flex-1"
              />
            </View>
          </View>
        </View>
      </Modal>
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
