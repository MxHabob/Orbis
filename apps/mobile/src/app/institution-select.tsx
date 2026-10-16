import {
  BuildingOffice,
  CheckCircle,
  Language,
  Logo,
  Pin,
} from "@orbis/icons/native";
import { useQuery } from "@tanstack/react-query";
import { Image } from "expo-image";
import { LinearGradient } from "expo-linear-gradient";
import * as Location from "expo-location";
import { useRouter } from "expo-router";
import { memo, useCallback, useMemo, useState } from "react";
import {
  FlatList,
  ImageBackground,
  type ListRenderItemInfo,
  Pressable,
  StyleSheet,
  Text,
  useWindowDimensions,
  View,
} from "react-native";
import { useSafeAreaInsets } from "react-native-safe-area-context";
import { Button } from "@/components/ui/button";
import { EmptyState } from "@/components/ui/empty-state";
import { LoadingIndicator } from "@/components/ui/loading-indicator";
import { SearchInput } from "@/components/ui/search-input";
import { Skeleton } from "@/components/ui/skeleton";
import { ThemedStatusBar } from "@/components/ui/themed-status-bar";
import { useToast } from "@/components/ui/toast";
import { useTheme } from "@/hooks/use-theme";
import { useAuth } from "@/lib/api";
import {
  checkInstitutionConnection,
  type DirectoryInstitution,
  searchInstitutionDirectory,
} from "@/lib/api/institution-discovery";
import { useI18n } from "@/lib/i18n";

interface InstitutionTileProps {
  institution: DirectoryInstitution;
  name: string;
  isCurrent: boolean;
  isConnecting: boolean;
  disabled: boolean;
  currentLabel: string;
  primaryColor: string;
  onSelect: (institution: DirectoryInstitution) => void;
  width: number;
}

const InstitutionTile = memo(function InstitutionTile({
  institution,
  name,
  isCurrent,
  isConnecting,
  disabled,
  currentLabel,
  primaryColor,
  onSelect,
  width,
}: InstitutionTileProps) {
  const [failedLogoUrl, setFailedLogoUrl] = useState<string | null>(null);
  const logoUrl = institution.logo_url;
  const initials = Array.from(name.trim())[0]?.toLocaleUpperCase() ?? "?";
  const handlePress = useCallback(
    () => onSelect(institution),
    [institution, onSelect],
  );

  return (
    <Pressable
      accessibilityRole="button"
      accessibilityLabel={`${name}, ${institution.deployment_name}`}
      accessibilityState={{
        disabled,
        selected: isCurrent,
        busy: isConnecting,
      }}
      disabled={disabled}
      onPress={handlePress}
      className={`min-h-[164px] items-start rounded-2xl border p-3 ${
        isCurrent ? "border-primary/50 bg-primary/5" : "border-border bg-card"
      }`}
      style={({ pressed }) => [
        { width },
        pressed && !disabled ? { opacity: 0.82 } : undefined,
      ]}
    >
      <View className="mb-3 h-14 w-14 items-center justify-center overflow-hidden rounded-2xl border border-border bg-background">
        {logoUrl && failedLogoUrl !== logoUrl ? (
          <Image
            source={{ uri: logoUrl }}
            contentFit="contain"
            cachePolicy="memory-disk"
            transition={120}
            onError={() => setFailedLogoUrl(logoUrl)}
            style={styles.logoImage}
          />
        ) : (
          <Text className="text-xl font-bold text-primary">{initials}</Text>
        )}
      </View>

      <View className="w-full flex-1">
        <Text
          numberOfLines={2}
          className="text-start text-sm leading-5 font-semibold text-foreground"
        >
          {name}
        </Text>
        <Text
          numberOfLines={1}
          className="mt-1 text-start text-xs leading-4 text-muted-foreground"
        >
          {institution.deployment_name}
        </Text>
      </View>

      <View className="absolute end-2 top-2 h-8 w-8 items-center justify-center">
        {isConnecting ? (
          <LoadingIndicator size="sm" />
        ) : isCurrent ? (
          <CheckCircle size={20} color={primaryColor} />
        ) : null}
      </View>

      {isCurrent ? (
        <Text className="mt-2 text-xs font-medium text-primary">
          {currentLabel}
        </Text>
      ) : null}
    </Pressable>
  );
});

function InstitutionTileSkeleton({ width }: { width: number }) {
  return (
    <View
      accessibilityLabel="Loading institution"
      accessibilityRole="progressbar"
      className="min-h-[164px] rounded-2xl border border-border bg-card p-3"
      style={{ width }}
    >
      <Skeleton width={56} height={56} borderRadius={16} />
      <View className="mt-3 gap-2">
        <Skeleton width="76%" height={14} borderRadius={6} />
        <Skeleton width="54%" height={11} borderRadius={6} />
      </View>
    </View>
  );
}

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
  const tileWidth = Math.max((windowWidth - 44) / 2, 132);
  const theme = useTheme();
  const { t, locale, setLocale, isArabic } = useI18n();
  const toast = useToast();
  const {
    isInitializing,
    institutions: savedInstitutions,
    selectedInstitution,
    selectInstitution,
  } = useAuth();
  const [search, setSearch] = useState("");
  const [connectingId, setConnectingId] = useState<string | null>(null);
  const [connectionError, setConnectionError] = useState<string | null>(null);
  const [userLocation, setUserLocation] = useState<{
    latitude: number;
    longitude: number;
  } | null>(null);

  const directoryQuery = useQuery({
    queryKey: ["institution-directory"],
    queryFn: ({ signal }) => searchInstitutionDirectory(undefined, signal),
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

  const connectInstitution = useCallback(
    async (institution: DirectoryInstitution) => {
      setConnectingId(institution.deployment_id);
      setConnectionError(null);
      try {
        await checkInstitutionConnection(institution);
        await selectInstitution(institution);
        toast.success(t("auth_institution_connected"));
        router.replace("/(auth)/sign-in");
      } catch (caught: unknown) {
        console.error("[Institution Select] Connection failed.", caught);
        const message = t("auth_institution_connection_error");
        setConnectionError(message);
        toast.error(message);
      } finally {
        setConnectingId(null);
      }
    },
    [router, selectInstitution, t, toast],
  );

  const renderInstitution = useCallback(
    ({ item }: ListRenderItemInfo<DirectoryInstitution>) => (
      <InstitutionTile
        institution={item}
        name={isArabic && item.name_ar ? item.name_ar : item.name}
        isCurrent={selectedInstitution?.deployment_id === item.deployment_id}
        isConnecting={connectingId === item.deployment_id}
        disabled={connectingId !== null}
        currentLabel={t("auth_institution_current")}
        primaryColor={theme.primary}
        onSelect={connectInstitution}
        width={tileWidth}
      />
    ),
    [
      connectInstitution,
      connectingId,
      isArabic,
      selectedInstitution?.deployment_id,
      t,
      theme.primary,
      tileWidth,
    ],
  );

  const requestNearbyLocation = useCallback(async () => {
    if (userLocation) {
      setUserLocation(null);
      return;
    }

    try {
      const { status } = await Location.requestForegroundPermissionsAsync();
      if (status !== "granted") {
        toast.error(t("auth_institution_location_error"));
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
      toast.error(t("auth_institution_location_error"));
    }
  }, [t, toast, userLocation]);

  const toggleLanguage = useCallback(() => {
    void setLocale(locale === "ar" ? "en" : "ar");
  }, [locale, setLocale]);

  const isLoading = isInitializing || directoryQuery.isPending;
  const directoryError = directoryQuery.isError
    ? t("auth_institution_search_error")
    : null;

  const renderEmpty = useCallback(() => {
    if (isLoading) {
      return (
        <View className="flex-row gap-3">
          <InstitutionTileSkeleton width={tileWidth} />
          <InstitutionTileSkeleton width={tileWidth} />
        </View>
      );
    }

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
        style={styles.emptyState}
      />
    );
  }, [directoryError, directoryQuery, isLoading, t, tileWidth]);

  return (
    <ImageBackground
      source={require("../../assets/Dreamy Arrival at the Historic Campus.png")}
      resizeMode="cover"
      style={styles.background}
    >
      <LinearGradient
        pointerEvents="none"
        colors={[
          "rgba(21, 16, 39, 0.24)",
          "rgba(21, 16, 39, 0.1)",
          "rgba(21, 16, 39, 0.42)",
          "rgba(21, 16, 39, 0.56)",
        ]}
        locations={[0, 0.28, 0.58, 1]}
        style={StyleSheet.absoluteFill}
      />
      <ThemedStatusBar backgroundColor="transparent" style="light" />
      <View className="flex-1" style={{ paddingTop: insets.top + 8 }}>
        <View className="mb-5 h-11 flex-row items-center justify-between px-5">
          <View className="flex-row items-center gap-2">
            <Logo size={25} color="#FFFFFF" dotColor="#A821D9" />
            <Text className="text-lg font-bold tracking-tight text-white">
              Orbis
            </Text>
          </View>
          <Pressable
            accessibilityRole="button"
            accessibilityLabel={
              locale === "ar" ? "Switch to English" : "التبديل إلى العربية"
            }
            onPress={toggleLanguage}
            hitSlop={8}
            className="min-h-11 flex-row items-center gap-2 rounded-full border border-white/30 bg-black/10 px-3"
          >
            <Language size={15} color="#FFFFFF" />
            <Text className="text-sm font-medium text-white">
              {locale === "ar" ? "English" : "العربية"}
            </Text>
          </Pressable>
        </View>

        <View className="flex-1 overflow-hidden rounded-t-[28px] bg-background">
          <FlatList
            data={visibleInstitutions}
            keyExtractor={(institution) => institution.deployment_id}
            renderItem={renderInstitution}
            numColumns={2}
            columnWrapperStyle={styles.tileRow}
            ListHeaderComponent={
              <View className="gap-4 pb-5">
                <View className="gap-1">
                  <Text className="text-start text-2xl leading-8 font-bold text-foreground">
                    {t("auth_institution_search_title")}
                  </Text>
                  <Text className="text-start text-sm leading-5 text-muted-foreground">
                    {t("auth_institution_search_hint")}
                  </Text>
                </View>

                <SearchInput
                  accessibilityLabel={t("auth_institution_filter_label")}
                  placeholder={t("auth_institution_search_placeholder")}
                  value={search}
                  onChangeText={setSearch}
                />

                <View className="flex-row items-center justify-between gap-3">
                  <Text className="flex-1 text-start text-sm font-semibold text-foreground">
                    {t("auth_institution_results")}
                  </Text>
                  <Pressable
                    accessibilityRole="button"
                    accessibilityLabel={t("auth_institution_nearby")}
                    accessibilityState={{ selected: userLocation !== null }}
                    onPress={() => void requestNearbyLocation()}
                    className={`min-h-11 flex-row items-center gap-2 rounded-full border px-3 ${
                      userLocation
                        ? "border-primary/40 bg-primary/10"
                        : "border-border bg-card"
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
                </View>
              </View>
            }
            ListEmptyComponent={renderEmpty}
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
            showsVerticalScrollIndicator={false}
            refreshing={directoryQuery.isRefetching}
            onRefresh={() => void directoryQuery.refetch()}
            initialNumToRender={8}
            maxToRenderPerBatch={8}
            windowSize={7}
          />

          <View
            className="border-t border-border bg-background px-4 pt-3"
            style={{ paddingBottom: Math.max(insets.bottom, 16) }}
          >
            {connectionError ? (
              <Text
                accessibilityRole="alert"
                className="mb-2 text-center text-xs leading-4 text-destructive"
              >
                {connectionError}
              </Text>
            ) : null}
            {selectedInstitution ? (
              <View className="mb-3 flex-row items-center gap-3 rounded-xl bg-primary/5 px-3 py-2">
                <CheckCircle size={17} color={theme.primary} />
                <View className="min-w-0 flex-1">
                  <Text className="text-[11px] leading-4 text-muted-foreground">
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
              </View>
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
    </ImageBackground>
  );
}

const styles = StyleSheet.create({
  background: {
    flex: 1,
    backgroundColor: "#081521",
  },
  emptyState: {
    minHeight: 200,
    justifyContent: "center",
  },
  listContent: {
    flexGrow: 1,
    paddingHorizontal: 16,
    paddingTop: 24,
    gap: 12,
  },
  tileRow: {
    justifyContent: "space-between",
    gap: 12,
  },
  logoImage: {
    width: 46,
    height: 46,
  },
});
