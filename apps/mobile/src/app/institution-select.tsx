import {
  BuildingOffice,
  CheckCircle,
  Language,
  Logo,
  MapPin,
  Search,
} from "@orbis/icons/native";
import { Image } from "expo-image";
import { LinearGradient } from "expo-linear-gradient";
import * as Location from "expo-location";
import { useRouter } from "expo-router";
import { memo, useCallback, useEffect, useMemo, useRef, useState } from "react";
import {
  FlatList,
  ImageBackground,
  KeyboardAvoidingView,
  type ListRenderItemInfo,
  Platform,
  Pressable,
  RefreshControl,
  ScrollView,
  StyleSheet,
  Text,
  useWindowDimensions,
  View,
} from "react-native";
import { useSafeAreaInsets } from "react-native-safe-area-context";
import { Button } from "@/components/ui/button";
import {
  Drawer,
  DrawerContent,
  DrawerDescription,
  DrawerFooter,
  DrawerHeader,
  DrawerTitle,
} from "@/components/ui/drawer";
import { EmptyState } from "@/components/ui/empty-state";
import { Input } from "@/components/ui/input";
import { LoadingIndicator } from "@/components/ui/loading-indicator";
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

const INSTITUTION_DRAWER_SNAP_POINTS = [0.52, 0.88] as const;

interface InstitutionCardProps {
  institution: DirectoryInstitution;
  name: string;
  cardWidth: number;
  isCurrent: boolean;
  isConnecting: boolean;
  disabled: boolean;
  currentLabel: string;
  primaryColor: string;
  onSelect: (institution: DirectoryInstitution) => void;
}

const InstitutionCard = memo(function InstitutionCard({
  institution,
  name,
  cardWidth,
  isCurrent,
  isConnecting,
  disabled,
  currentLabel,
  primaryColor,
  onSelect,
}: InstitutionCardProps) {
  const handlePress = useCallback(
    () => onSelect(institution),
    [institution, onSelect],
  );
  const [failedLogoUrl, setFailedLogoUrl] = useState<string | null>(null);
  const logoUrl = institution.logo_url;
  const initials = Array.from(name.trim())[0]?.toLocaleUpperCase() ?? "?";

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
      className={`mx-1.5 h-24 flex-row items-center gap-3 rounded-2xl border px-3 ${
        isCurrent
          ? "border-primary/50 bg-primary/5"
          : "border-border bg-background"
      }`}
      style={({ pressed }) => [
        { width: cardWidth },
        pressed && !disabled ? { opacity: 0.82 } : undefined,
      ]}
    >
      <View className="h-14 w-14 shrink-0 items-center justify-center overflow-hidden rounded-xl border border-border bg-card">
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

      <View className="min-w-0 flex-1 gap-1">
        <Text
          numberOfLines={2}
          className="text-start text-[15px] leading-5 font-semibold text-foreground"
        >
          {name}
        </Text>
        <Text
          numberOfLines={1}
          className="text-start text-xs leading-4 text-muted-foreground"
        >
          {isCurrent
            ? `${institution.deployment_name} · ${currentLabel}`
            : institution.deployment_name}
        </Text>
      </View>

      <View className="h-11 w-11 shrink-0 items-center justify-center">
        {isConnecting ? (
          <LoadingIndicator size="sm" />
        ) : isCurrent ? (
          <CheckCircle size={21} color={primaryColor} />
        ) : (
          <View className="h-2 w-2 rounded-full bg-border" />
        )}
      </View>
    </Pressable>
  );
});

function InstitutionCardSkeleton({ cardWidth }: { cardWidth: number }) {
  return (
    <View
      className="mx-1.5 h-24 flex-row items-center gap-3 rounded-2xl border border-border bg-background px-3"
      style={{ width: cardWidth }}
      accessibilityLabel="Loading institutions"
      accessibilityRole="progressbar"
    >
      <Skeleton width={56} height={56} borderRadius={12} />
      <View className="flex-1 gap-2">
        <Skeleton width="72%" height={15} borderRadius={6} />
        <Skeleton width="48%" height={11} borderRadius={6} />
      </View>
    </View>
  );
}

export default function InstitutionSelectScreen() {
  const router = useRouter();
  const insets = useSafeAreaInsets();
  const { width: windowWidth } = useWindowDimensions();
  const institutionCardWidth = Math.min(Math.max(windowWidth - 56, 240), 320);
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
  const [directoryInstitutions, setDirectoryInstitutions] = useState<
    DirectoryInstitution[]
  >([]);
  const [isLoadingDirectory, setIsLoadingDirectory] = useState(true);
  const [isRefreshing, setIsRefreshing] = useState(false);
  const [connectingId, setConnectingId] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [drawerVisible, setDrawerVisible] = useState(true);
  const directoryRequest = useRef<AbortController | null>(null);

  const loadInstitutions = useCallback(
    async (refresh = false) => {
      directoryRequest.current?.abort();
      const controller = new AbortController();
      directoryRequest.current = controller;
      setError(null);
      if (refresh) {
        setIsLoadingDirectory(false);
        setIsRefreshing(true);
      } else {
        setIsLoadingDirectory(true);
      }

      try {
        const institutions = await searchInstitutionDirectory(
          undefined,
          controller.signal,
        );
        if (!controller.signal.aborted) {
          setDirectoryInstitutions(institutions);
        }
      } catch (caught: unknown) {
        if (controller.signal.aborted) return;
        console.error("[Institution Select] Directory load failed.", caught);
        setError(t("auth_institution_search_error"));
      } finally {
        if (directoryRequest.current === controller) {
          directoryRequest.current = null;
          setIsLoadingDirectory(false);
          setIsRefreshing(false);
        }
      }
    },
    [t],
  );

  useEffect(() => {
    if (!isInitializing) {
      void loadInstitutions();
    }
    return () => {
      directoryRequest.current?.abort();
      directoryRequest.current = null;
    };
  }, [isInitializing, loadInstitutions]);

  const visibleInstitutions = useMemo(() => {
    const institutionsByDeploymentId = new Map<string, DirectoryInstitution>();
    for (const institution of directoryInstitutions) {
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
  }, [directoryInstitutions, savedInstitutions, search]);

  const connectInstitution = useCallback(
    async (institution: DirectoryInstitution) => {
      setConnectingId(institution.deployment_id);
      setError(null);
      try {
        await checkInstitutionConnection(institution);
        await selectInstitution(institution);
        toast.success(t("auth_institution_connected"));
        router.replace("/(auth)/sign-in");
      } catch (caught: unknown) {
        console.error("[Institution Select] Connection failed.", caught);
        const message = t("auth_institution_connection_error");
        setError(message);
        toast.error(message);
      } finally {
        setConnectingId(null);
      }
    },
    [router, selectInstitution, t, toast],
  );

  const renderInstitution = useCallback(
    ({ item }: ListRenderItemInfo<DirectoryInstitution>) => {
      const name = isArabic && item.name_ar ? item.name_ar : item.name;
      return (
        <InstitutionCard
          institution={item}
          name={name}
          cardWidth={institutionCardWidth}
          isCurrent={selectedInstitution?.deployment_id === item.deployment_id}
          isConnecting={connectingId === item.deployment_id}
          disabled={connectingId !== null}
          currentLabel={t("auth_institution_current")}
          primaryColor={theme.primary}
          onSelect={connectInstitution}
        />
      );
    },
    [
      connectInstitution,
      connectingId,
      institutionCardWidth,
      isArabic,
      selectedInstitution?.deployment_id,
      t,
      theme.primary,
    ],
  );

  const renderEmpty = useCallback(() => {
    if (isInitializing || isLoadingDirectory) {
      return <InstitutionCardSkeleton cardWidth={institutionCardWidth} />;
    }

    if (error) {
      return (
        <View
          className="items-center justify-center gap-4 px-5"
          style={{ ...styles.emptyState, width: institutionCardWidth }}
        >
          <Text
            accessibilityRole="alert"
            className="text-center text-sm leading-5 text-destructive"
          >
            {error}
          </Text>
          <Button
            title={t("common_retry")}
            variant="outline"
            onPress={() => void loadInstitutions()}
          />
        </View>
      );
    }

    return (
      <EmptyState
        icon={BuildingOffice}
        title={t("auth_institution_no_matches")}
        style={{ ...styles.emptyState, width: institutionCardWidth }}
      />
    );
  }, [
    error,
    institutionCardWidth,
    isInitializing,
    isLoadingDirectory,
    loadInstitutions,
    t,
  ]);

  const toggleLanguage = useCallback(() => {
    void setLocale(locale === "ar" ? "en" : "ar");
  }, [locale, setLocale]);

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
      <View style={[styles.brandRow, { paddingTop: insets.top + 8 }]}>
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
          className="absolute end-5 min-h-11 flex-row items-center gap-2 rounded-full border border-white/30 bg-black/10 px-3"
        >
          <Language size={15} color="#FFFFFF" />
          <Text className="text-sm font-medium text-white">
            {locale === "ar" ? "English" : "العربية"}
          </Text>
        </Pressable>
      </View>
      {!drawerVisible ? (
        <Button
          title={t("auth_institution_search_title")}
          fullWidth
          onPress={() => setDrawerVisible(true)}
          style={styles.reopenDrawer}
        />
      ) : null}
      <Drawer
        visible={drawerVisible}
        onClose={() => setDrawerVisible(false)}
        snapPoints={INSTITUTION_DRAWER_SNAP_POINTS}
      >
        <DrawerContent>
          <KeyboardAvoidingView
            behavior={Platform.OS === "ios" ? "padding" : undefined}
            style={styles.drawerBody}
          >
            <DrawerHeader>
              <DrawerTitle>{t("auth_institution_search_title")}</DrawerTitle>
              <DrawerDescription>
                {t("auth_institution_search_hint")}
              </DrawerDescription>
            </DrawerHeader>
            <ScrollView
              style={styles.drawerScroll}
              contentContainerStyle={styles.drawerContent}
              keyboardShouldPersistTaps="handled"
              keyboardDismissMode="on-drag"
              showsVerticalScrollIndicator={false}
              refreshControl={
                <RefreshControl
                  refreshing={isRefreshing}
                  onRefresh={() => void loadInstitutions(true)}
                  tintColor={theme.primary}
                  colors={[theme.primary]}
                  progressBackgroundColor={theme.background}
                />
              }
            >
              <Input
                accessibilityLabel={t("auth_institution_filter_label")}
                placeholder={t("auth_institution_search_placeholder")}
                value={search}
                onChangeText={setSearch}
                icon={<Search size={18} color={theme.textMuted} />}
                autoCapitalize="none"
                autoCorrect={false}
                returnKeyType="search"
              />

              {error && visibleInstitutions.length > 0 ? (
                <View className="flex-row items-center justify-between gap-3 rounded-xl border border-destructive/30 bg-destructive/10 px-3 py-2">
                  <Text
                    accessibilityRole="alert"
                    className="flex-1 text-start text-xs leading-5 text-destructive"
                  >
                    {error}
                  </Text>
                  <Button
                    title={t("common_retry")}
                    variant="outline"
                    size="sm"
                    onPress={() => void loadInstitutions()}
                  />
                </View>
              ) : null}

              <FlatList
                horizontal
                style={styles.institutionCarousel}
                data={visibleInstitutions}
                keyExtractor={(institution) => institution.deployment_id}
                renderItem={renderInstitution}
                ListEmptyComponent={renderEmpty}
                contentContainerStyle={styles.carouselContent}
                keyboardShouldPersistTaps="handled"
                showsHorizontalScrollIndicator={false}
                initialNumToRender={3}
                maxToRenderPerBatch={4}
                updateCellsBatchingPeriod={50}
                windowSize={5}
              />
            </ScrollView>
            <DrawerFooter>
              <View className="gap-2">
                {selectedInstitution ? (
                  <View className="flex-row items-center gap-3 rounded-xl bg-primary/5 px-3 py-2">
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
            </DrawerFooter>
          </KeyboardAvoidingView>
        </DrawerContent>
      </Drawer>
    </ImageBackground>
  );
}

const styles = StyleSheet.create({
  background: {
    flex: 1,
    backgroundColor: "#081521",
  },
  emptyState: {
    height: 136,
    justifyContent: "center",
  },
  brandRow: {
    position: "absolute",
    top: 0,
    left: 0,
    right: 0,
    height: 64,
    alignItems: "center",
    justifyContent: "center",
  },
  reopenDrawer: {
    position: "absolute",
    left: 16,
    right: 16,
    bottom: 24,
  },
  drawerBody: {
    flex: 1,
  },
  drawerScroll: {
    flex: 1,
    flexShrink: 1,
  },
  drawerContent: {
    gap: 12,
    paddingHorizontal: 16,
    paddingTop: 12,
    paddingBottom: 8,
  },
  institutionCarousel: {
    height: 104,
    flexGrow: 0,
  },
  carouselContent: {
    alignItems: "center",
    paddingHorizontal: 8,
  },
  logoImage: {
    width: 46,
    height: 46,
  },
});
