import { Search, CheckCircle } from "@orbis/icons/native";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import {
  FlatList,
  RefreshControl,
  Text,
  View,
  type ListRenderItemInfo,
} from "react-native";
import { useRouter } from "expo-router";
import { Screen } from "@/components/layout/screen";
import { Button } from "@/components/ui/button";
import { EmptyState } from "@/components/ui/empty-state";
import { Input } from "@/components/ui/input";
import { ListItem } from "@/components/ui/list-item";
import { LoadingIndicator } from "@/components/ui/loading-indicator";
import { LoadingState } from "@/components/ui/loading-state";
import { useTheme } from "@/hooks/use-theme";
import { useAuth } from "@/lib/api";
import {
  checkInstitutionConnection,
  searchInstitutionDirectory,
  type DirectoryInstitution,
} from "@/lib/api/institution-discovery";
import { useI18n } from "@/lib/i18n";
import { useToast } from "@/components/ui/toast";

const InstitutionSeparator = () => (
  <View className="mx-four h-px bg-border" />
);

export default function InstitutionSelectScreen() {
  const router = useRouter();
  const theme = useTheme();
  const { t, isArabic } = useI18n();
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
      const isCurrent =
        selectedInstitution?.deployment_id === item.deployment_id;
      const isConnecting = connectingId === item.deployment_id;
      const subtitle = isCurrent
        ? `${item.deployment_name} · ${t("auth_institution_current")}`
        : item.deployment_name;

      return (
        <ListItem
          title={name}
          subtitle={subtitle}
          disabled={connectingId !== null}
          onPress={() => void connectInstitution(item)}
          chevron={!isConnecting}
          right={
            isConnecting ? (
              <LoadingIndicator size="sm" />
            ) : isCurrent ? (
              <CheckCircle size={18} color={theme.primary} />
            ) : null
          }
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
    if (isLoadingDirectory) {
      return (
        <LoadingState
          message={t("auth_institution_loading")}
          style={{ flex: 1 }}
        />
      );
    }

    if (error) {
      return (
        <View className="flex-1 items-center justify-center gap-four px-four py-eight">
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
        title={t("auth_institution_no_matches")}
        style={{ flex: 1 }}
      />
    );
  }, [error, isLoadingDirectory, loadInstitutions, t]);

  if (isInitializing) {
    return (
      <Screen>
        <LoadingState message={t("common_loading")} fullscreen />
      </Screen>
    );
  }

  return (
    <Screen keyboardAvoiding>
      <FlatList
        className="flex-1"
        data={visibleInstitutions}
        keyExtractor={(institution) => institution.deployment_id}
        renderItem={renderInstitution}
        ItemSeparatorComponent={InstitutionSeparator}
        ListHeaderComponent={
          <View className="gap-four px-four pb-four pt-six">
            <View className="gap-two">
              <Text className="font-sans text-2xl font-bold text-foreground">
                {t("auth_institution_search_title")}
              </Text>
              <Text className="font-sans text-sm leading-5 text-muted-foreground">
                {t("auth_institution_search_hint")}
              </Text>
            </View>

            <Input
              label={t("auth_institution_filter_label")}
              placeholder={t("auth_institution_search_placeholder")}
              value={search}
              onChangeText={(value) => {
                setSearch(value);
                setError(null);
              }}
              icon={<Search size={18} color={theme.textMuted} />}
              autoCapitalize="none"
              autoCorrect={false}
              returnKeyType="search"
            />

            {error && visibleInstitutions.length > 0 ? (
              <View className="flex-row items-center justify-between gap-three rounded-md border border-destructive/30 bg-destructive/10 px-three py-two">
                <Text
                  accessibilityRole="alert"
                  className="flex-1 text-start text-xs leading-4 text-destructive"
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

            <Text className="font-sans text-sm font-semibold text-foreground">
              {t("auth_institution_results")}
            </Text>
          </View>
        }
        ListEmptyComponent={renderEmpty}
        contentContainerStyle={{ flexGrow: 1, paddingBottom: 16 }}
        keyboardShouldPersistTaps="handled"
        keyboardDismissMode="on-drag"
        alwaysBounceVertical
        overScrollMode="always"
        showsVerticalScrollIndicator={false}
        initialNumToRender={10}
        maxToRenderPerBatch={10}
        windowSize={7}
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

      <View className="border-t border-border bg-background px-four pb-two pt-three">
        <Button
          title={t("auth_sign_in")}
          variant="outline"
          size="lg"
          fullWidth
          onPress={() => router.replace("/(auth)/sign-in")}
          disabled={!selectedInstitution || connectingId !== null}
        />
      </View>
    </Screen>
  );
}
