import {
  BuildingOffice,
  CheckCircle,
  Language,
  Logo,
  Search,
} from "@orbis/icons/native";
import { Image } from "expo-image";
import { LinearGradient } from "expo-linear-gradient";
import { useRouter } from "expo-router";
import React, {
  memo,
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState,
} from "react";
import {
  FlatList,
  ImageBackground,
  KeyboardAvoidingView,
  Platform,
  Pressable,
  RefreshControl,
  StyleSheet,
  Text,
  View,
  type ListRenderItemInfo,
} from "react-native";
import { useSafeAreaInsets } from "react-native-safe-area-context";
import { Button } from "@/components/ui/button";
import { EmptyState } from "@/components/ui/empty-state";
import { Input } from "@/components/ui/input";
import { LoadingIndicator } from "@/components/ui/loading-indicator";
import { LoadingState } from "@/components/ui/loading-state";
import { ThemedStatusBar } from "@/components/ui/themed-status-bar";
import { useTheme } from "@/hooks/use-theme";
import { useAuth } from "@/lib/api";
import {
  checkInstitutionConnection,
  searchInstitutionDirectory,
  type DirectoryInstitution,
} from "@/lib/api/institution-discovery";
import { useI18n } from "@/lib/i18n";
import { useToast } from "@/components/ui/toast";

const CARD_HEIGHT = 78;
const ITEM_SPACING = 8;
const ITEM_LAYOUT_SIZE = CARD_HEIGHT + ITEM_SPACING;

interface InstitutionCardProps {
  institution: DirectoryInstitution;
  name: string;
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
      className={`mx-5 h-[78px] flex-row items-center gap-2.5 rounded-2xl border px-3 ${
        isCurrent
          ? "border-primary/50 bg-primary/5"
          : "border-border bg-background"
      }`}
      style={({ pressed }) => [
        pressed && !disabled ? { opacity: 0.82 } : undefined,
      ]}
    >
      <View className="h-12 w-12 shrink-0 items-center justify-center overflow-hidden rounded-xl border border-border bg-card">
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

      <View className="h-9 w-9 shrink-0 items-center justify-center">
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

const InstitutionSeparator = memo(function InstitutionSeparator() {
  return <View style={styles.separator} />;
});

export default function InstitutionSelectScreen() {
  const router = useRouter();
  const insets = useSafeAreaInsets();
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
      isArabic,
      selectedInstitution?.deployment_id,
      t,
      theme.primary,
    ],
  );

  const renderEmpty = useCallback(() => {
    if (isInitializing || isLoadingDirectory) {
      return (
        <LoadingState
          message={
            isInitializing ? t("common_loading") : t("auth_institution_loading")
          }
          style={styles.emptyState}
        />
      );
    }

    if (error) {
      return (
        <View className="flex-1 items-center justify-center gap-4 px-5 py-8">
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
        style={styles.emptyState}
      />
    );
  }, [error, isInitializing, isLoadingDirectory, loadInstitutions, t]);

  const toggleLanguage = useCallback(() => {
    void setLocale(locale === "ar" ? "en" : "ar");
  }, [locale, setLocale]);

  return (
    <View className="flex-1 bg-primary-dark">
      <ImageBackground
        source={require("../../assets/Dreamy Arrival at the Historic Campus.png")}
        resizeMode="cover"
        className="flex-1"
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
        <KeyboardAvoidingView
          className="flex-1"
          behavior={Platform.OS === "ios" ? "padding" : undefined}
        >
          <View
            className="flex-1"
            style={{
              paddingTop: insets.top + 8,
            }}
          >
            <View className="mb-4 h-11 flex-row items-center justify-center px-5">
              <View className="flex-row items-center gap-2">
                <Logo size={27} color="#FFFFFF" dotColor="#A821D9" />
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
                <Language size={16} color="#FFFFFF" />
                <Text className="text-sm font-medium text-white">
                  {locale === "ar" ? "English" : "العربية"}
                </Text>
              </Pressable>
            </View>

            <View className="px-5 pb-5">
              <Text className="text-[30px] leading-9 font-semibold tracking-tight text-white">
                {t("auth_institution_search_title")}
              </Text>
              <Text className="mt-2 max-w-[360px] text-sm leading-5 text-white/90">
                {t("auth_institution_search_hint")}
              </Text>
            </View>

            <View className="flex-1 overflow-hidden rounded-t-[28px] border-t border-white/30 bg-card">
              <View className="gap-3 px-5 pb-2 pt-4">
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
              </View>

              <FlatList
                className="flex-1"
                data={visibleInstitutions}
                keyExtractor={(institution) => institution.deployment_id}
                renderItem={renderInstitution}
                ItemSeparatorComponent={InstitutionSeparator}
                ListEmptyComponent={renderEmpty}
                contentContainerStyle={styles.listContent}
                keyboardShouldPersistTaps="handled"
                keyboardDismissMode="on-drag"
                showsVerticalScrollIndicator={false}
                initialNumToRender={8}
                maxToRenderPerBatch={8}
                updateCellsBatchingPeriod={50}
                windowSize={5}
                getItemLayout={(_, index) => ({
                  length: ITEM_LAYOUT_SIZE,
                  offset: ITEM_LAYOUT_SIZE * index,
                  index,
                })}
                refreshControl={
                  <RefreshControl
                    refreshing={isRefreshing}
                    onRefresh={() => void loadInstitutions(true)}
                    tintColor={theme.primary}
                    colors={[theme.primary]}
                    progressBackgroundColor={theme.background}
                  />
                }
              />

              <View
                className="gap-3 border-t border-border bg-card px-5 pt-3"
                style={{ paddingBottom: Math.max(insets.bottom, 10) }}
              >
                {selectedInstitution ? (
                  <View className="flex-row items-center gap-3">
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
        </KeyboardAvoidingView>
      </ImageBackground>
    </View>
  );
}

const styles = StyleSheet.create({
  emptyState: {
    flex: 1,
    minHeight: 200,
  },
  listContent: {
    flexGrow: 1,
    paddingTop: 12,
    paddingBottom: 16,
  },
  logoImage: {
    width: 38,
    height: 38,
  },
  separator: {
    height: ITEM_SPACING,
  },
});
