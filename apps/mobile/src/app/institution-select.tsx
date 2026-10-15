import { useEffect, useMemo, useState } from "react";
import { Pressable, Text, View } from "react-native";
import { useRouter } from "expo-router";
import { useSafeAreaInsets } from "react-native-safe-area-context";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { KeyboardAwareView } from "@/components/ui/keyboard-aware-view";
import { LoadingIndicator } from "@/components/ui/loading-indicator";
import { useToast } from "@/components/ui/toast";
import { useAuth } from "@/lib/api";
import {
  checkInstitutionConnection,
  searchInstitutionDirectory,
  type DirectoryInstitution,
} from "@/lib/api/institution-discovery";
import { useI18n } from "@/lib/i18n";

export default function InstitutionSelectScreen() {
  const router = useRouter();
  const insets = useSafeAreaInsets();
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
  const [connectingId, setConnectingId] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  if (isInitializing) {
    return (
      <View className="flex-1 items-center justify-center bg-background">
        <LoadingIndicator size="lg" />
      </View>
    );
  }

  const loadInstitutions = async (signal?: AbortSignal) => {
    setError(null);
    setIsLoadingDirectory(true);
    try {
      const institutions = await searchInstitutionDirectory(undefined, signal);
      if (!signal?.aborted) setDirectoryInstitutions(institutions);
    } catch (caught: unknown) {
      if (signal?.aborted) return;
      console.error("[Institution Select] Directory load failed.", caught);
      const message = t("auth_institution_search_error");
      setError(message);
      toast.error(message);
    } finally {
      if (!signal?.aborted) setIsLoadingDirectory(false);
    }
  };

  useEffect(() => {
    const controller = new AbortController();
    void loadInstitutions(controller.signal);
    return () => controller.abort();
  }, []);

  const visibleInstitutions = useMemo(() => {
    const institutionsByDeploymentId = new Map<string, DirectoryInstitution>();
    for (const institution of [...directoryInstitutions, ...savedInstitutions]) {
      institutionsByDeploymentId.set(institution.deployment_id, institution);
    }
    const query = search.trim().toLocaleLowerCase();
    return [...institutionsByDeploymentId.values()].filter((institution) =>
      !query ||
      [
        institution.name,
        institution.name_ar ?? "",
        institution.slug,
        institution.deployment_name,
      ].some((value) => value.toLocaleLowerCase().includes(query)),
    );
  }, [directoryInstitutions, savedInstitutions, search]);

  const connectInstitution = async (institution: DirectoryInstitution) => {
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
  };

  const renderInstitution = (institution: DirectoryInstitution) => {
    const name =
      isArabic && institution.name_ar
        ? institution.name_ar
        : institution.name;
    const isCurrent =
      selectedInstitution?.deployment_id === institution.deployment_id;

    return (
      <Pressable
        key={institution.deployment_id}
        accessibilityRole="button"
        accessibilityState={{
          busy: connectingId === institution.deployment_id,
          disabled: connectingId !== null,
        }}
        disabled={connectingId !== null}
        onPress={() => void connectInstitution(institution)}
        className="min-h-16 flex-row items-center justify-between gap-3 rounded-xl border border-border bg-card px-4 py-3"
      >
        <View className="min-w-0 flex-1 gap-1">
          <Text className="font-sans text-sm font-semibold text-foreground">
            {name}
          </Text>
          <Text className="font-sans text-xs text-muted-foreground">
            {institution.deployment_name}
          </Text>
          {isCurrent ? (
            <Text className="font-sans text-xs font-medium text-primary">
              {t("auth_institution_current")}
            </Text>
          ) : null}
        </View>
        {connectingId === institution.deployment_id ? (
          <LoadingIndicator size="sm" />
        ) : null}
      </Pressable>
    );
  };

  return (
    <View className="flex-1 bg-background">
      <KeyboardAwareView
        style={{ flex: 1 }}
        contentContainerStyle={{
          flexGrow: 1,
          paddingHorizontal: 24,
          paddingTop: insets.top + 24,
          paddingBottom: Math.max(insets.bottom, 20) + 16,
        }}
      >
        <View className="flex-1 gap-6">
          <View className="gap-2">
            <Text className="font-sans text-[28px] font-extrabold tracking-[-0.5px] text-foreground">
              {t("auth_institution_search_title")}
            </Text>
            <Text className="font-sans text-sm leading-5 text-muted-foreground">
              {t("auth_institution_search_hint")}
            </Text>
          </View>

          <View className="gap-3">
            <Input
              label={t("auth_institution_filter_label")}
              placeholder={t("auth_institution_search_placeholder")}
              value={search}
              onChangeText={(value) => {
                setSearch(value);
                setError(null);
              }}
              autoCapitalize="none"
              autoCorrect={false}
              returnKeyType="search"
            />
          </View>

          {error ? (
            <Text accessibilityRole="alert" className="text-sm text-destructive">
              {error}
            </Text>
          ) : null}

          {isLoadingDirectory ? (
            <View className="items-center gap-3 py-8">
              <LoadingIndicator size="md" />
              <Text className="text-sm text-muted-foreground">
                {t("auth_institution_loading")}
              </Text>
            </View>
          ) : null}

          {visibleInstitutions.length > 0 ? (
            <View className="gap-3">
              <Text className="font-sans text-base font-semibold text-foreground">
                {t("auth_institution_results")}
              </Text>
              {visibleInstitutions.map(renderInstitution)}
            </View>
          ) : !isLoadingDirectory && !error ? (
            <Text className="text-sm text-muted-foreground">
              {t("auth_institution_no_matches")}
            </Text>
          ) : null}
        </View>
        <View className="pt-6">
          <Button
            title={t("auth_sign_in")}
            variant="outline"
            fullWidth
            onPress={() => router.replace("/(auth)/sign-in")}
            disabled={!selectedInstitution || connectingId !== null}
          />
          {error ? (
            <Button
              title={t("common_retry")}
              variant="outline"
              fullWidth
              onPress={() => void loadInstitutions()}
            />
          ) : null}
        </View>
      </KeyboardAwareView>
    </View>
  );
}
