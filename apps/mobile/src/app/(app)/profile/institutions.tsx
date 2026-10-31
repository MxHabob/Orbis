import { useMemo, useState } from "react";
import { Text, View } from "react-native";
import { useRouter } from "expo-router";
import { useQuery } from "@tanstack/react-query";
import { BuildingOffice } from "@orbis/icons/native";

import { Header } from "@/components/header";
import { ScreenScrollView } from "@/components/layout/screen-scroll-view";
import { InstitutionCard } from "@/components/institutions/institution-card";
import { Button } from "@/components/ui/button";
import { EmptyState } from "@/components/ui/empty-state";
import { Skeleton } from "@/components/ui/skeleton";
import { useToast } from "@/components/ui/toast";
import { useAuth } from "@/lib/api";
import {
  checkInstitutionConnection,
  type DirectoryInstitution,
} from "@/lib/api/institution-discovery";
import { useI18n } from "@/lib/i18n";

export default function LinkedInstitutionsScreen() {
  const router = useRouter();
  const toast = useToast();
  const { t, isArabic } = useI18n();
  const {
    isInitializing,
    institutions,
    selectedInstitution,
    selectInstitution,
  } = useAuth();
  const [switchingId, setSwitchingId] = useState<string | null>(null);
  const selectedLocalInstitution =
    selectedInstitution?.is_local &&
    (selectedInstitution.local_source === "qr" ||
      selectedInstitution.local_source === "discovery")
      ? selectedInstitution
      : null;
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

  const linkedInstitutions = useMemo(() => {
    const byDeploymentId = new Map<string, DirectoryInstitution>();
    for (const institution of institutions) {
      if (institution.is_local) continue;
      byDeploymentId.set(institution.deployment_id, institution);
    }
    if (selectedInstitution && !selectedInstitution.is_local) {
      byDeploymentId.set(
        selectedInstitution.deployment_id,
        selectedInstitution,
      );
    }
    if (selectedLocalInstitution && selectedLocalHealthQuery.isSuccess) {
      byDeploymentId.set(
        selectedLocalInstitution.deployment_id,
        selectedLocalInstitution,
      );
    }
    return [...byDeploymentId.values()];
  }, [
    institutions,
    selectedInstitution,
    selectedLocalHealthQuery.isSuccess,
    selectedLocalInstitution,
  ]);
  const isCheckingSelectedLocal =
    selectedLocalInstitution !== null && selectedLocalHealthQuery.isPending;

  const switchInstitution = async (institution: DirectoryInstitution) => {
    setSwitchingId(institution.deployment_id);
    try {
      await checkInstitutionConnection(institution);
      await selectInstitution(institution);
      router.replace("/");
    } catch (error: unknown) {
      console.error(
        "[Orbis Mobile Institutions] Failed to switch institution.",
        error,
      );
      toast.error(t("auth_institution_connection_error"));
    } finally {
      setSwitchingId(null);
    }
  };

  return (
    <>
      <Header
        title={t("auth_institution_linked")}
        showBack
        onBack={() => router.back()}
      />
      <ScreenScrollView
        edges={["bottom", "left", "right"]}
        scrollViewProps={{
          contentContainerClassName: "gap-four px-four pb-eight",
        }}
      >
        <Text className="text-start text-sm leading-5 text-muted-foreground">
          {t("auth_institution_linked_description")}
        </Text>

        {isInitializing || isCheckingSelectedLocal ? (
          <View className="gap-three">
            <Skeleton className="h-20 w-full rounded-2xl" />
            <Skeleton className="h-20 w-full rounded-2xl" />
          </View>
        ) : linkedInstitutions.length > 0 ? (
          <View className="gap-three">
            {linkedInstitutions.map((institution) => (
              <InstitutionCard
                key={institution.deployment_id}
                institution={institution}
                name={
                  isArabic && institution.name_ar
                    ? institution.name_ar
                    : institution.name
                }
                isCurrent={
                  selectedInstitution?.deployment_id ===
                  institution.deployment_id
                }
                isBusy={switchingId === institution.deployment_id}
                disabled={switchingId !== null}
                localLabel={t("auth_institution_local_label")}
                currentLabel={t("auth_institution_current")}
                onPress={(item) => void switchInstitution(item)}
                variant="linked"
              />
            ))}
          </View>
        ) : (
          <EmptyState
            icon={BuildingOffice}
            title={t("auth_institution_linked_empty_title")}
            description={t("auth_institution_linked_empty_description")}
          />
        )}

        {!isInitializing ? (
          <Button
            title={t("auth_institution_find_more")}
            variant="outline"
            size="lg"
            fullWidth
            onPress={() => router.push("/institution-select")}
          />
        ) : null}
      </ScreenScrollView>
    </>
  );
}
