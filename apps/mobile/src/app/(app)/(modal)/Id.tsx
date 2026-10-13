import { getPages, type TemplateDocument } from "@docforge/core";
import { XMark } from "@orbis/icons/native";
import { useRouter } from "expo-router";
import { useMemo, useState } from "react";
import { Pressable, Text, View } from "react-native";
import { DigitalIdTemplate } from "@/components/digital-id/digital-id-template";
import { ScreenScrollView } from "@/components/layout/screen-scroll-view";
import { EmptyState } from "@/components/ui/empty-state";
import { Skeleton } from "@/components/ui/skeleton";
import {
  useCurrentOrgQuery,
  useDigitalIdTemplateQuery,
  useLinkedStudentId,
  useStudentQuery,
} from "@/hooks/use-api-queries";
import { useTheme } from "@/hooks/use-theme";
import { useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";

export default function DigitalIdModalScreen() {
  const router = useRouter();
  const theme = useTheme();
  const { user, effectiveContext } = useAuth();
  const { t, isArabic } = useI18n();
  const [selectedDependentId, setSelectedDependentId] = useState<string>();
  const dependents = effectiveContext?.dependents ?? [];

  const roles = [
    ...(user?.roles ?? []),
    effectiveContext?.effective_role ?? "",
    ...(effectiveContext?.roles ?? []),
  ].map((role) => role.toLowerCase().replace(/[^a-z]/g, ""));
  const isStudent = roles.some((role) => role.includes("student"));
  const isGuardian = roles.some(
    (role) =>
      role.includes("parent") ||
      role.includes("guardian") ||
      role.includes("family"),
  );
  const selectedDependent =
    dependents.find((dep) => dep.student_id === selectedDependentId) ??
    dependents[0];
  const requestedStudentId = isStudent
    ? "me"
    : isGuardian
      ? selectedDependent?.student_id
      : undefined;
  const { studentId, isLinked, isResolving, identityError, retryIdentity } =
    useLinkedStudentId(requestedStudentId);
  const studentQuery = useStudentQuery(studentId);
  const student = studentQuery.data;
  const organizationQuery = useCurrentOrgQuery();
  const templateQuery = useDigitalIdTemplateQuery(student?.org_id);
  const template = useMemo(
    () =>
      [...(templateQuery.data ?? [])].sort(
        (left, right) =>
          Number(right.is_default) - Number(left.is_default) ||
          right.updated_at.localeCompare(left.updated_at),
      )[0],
    [templateQuery.data],
  );
  const documentResult = useMemo(() => {
    if (!template) return { document: null, error: false };
    try {
      const document = JSON.parse(template.document_json) as TemplateDocument;
      const pages = getPages(document);
      if (
        !pages.length ||
        pages.some((page) => !Array.isArray(page.children))
      ) {
        throw new Error("The ID template has no pages.");
      }
      return { document, error: false };
    } catch {
      return { document: null, error: true };
    }
  }, [template]);
  return (
    <View className="flex-1 bg-background">
      <View className="flex-row items-center justify-between px-five pb-two pt-six">
        <Text className="text-lg font-bold text-foreground">
          {t("tab_digital_id")}
        </Text>
        <Pressable
          accessibilityRole="button"
          accessibilityLabel={t("digital_id_close")}
          onPress={() => router.back()}
          hitSlop={8}
          className="h-11 w-11 items-center justify-center rounded-full bg-secondary active:opacity-70"
        >
          <XMark size={20} color={theme.foreground} />
        </Pressable>
      </View>

      <ScreenScrollView
        edges={["bottom", "left", "right"]}
        scrollViewProps={{
          contentContainerClassName:
            "items-center gap-five px-five pb-ten pt-two",
        }}
      >
        {isGuardian && dependents.length > 1 ? (
          <View className="w-full flex-row flex-wrap gap-two">
            {dependents.map((dependent) => {
              const selected =
                dependent.student_id === selectedDependent?.student_id;
              const name = isArabic
                ? dependent.full_name_ar || dependent.full_name
                : dependent.full_name;
              return (
                <Pressable
                  key={dependent.student_id}
                  accessibilityRole="button"
                  accessibilityState={{ selected }}
                  onPress={() => setSelectedDependentId(dependent.student_id)}
                  className={`min-h-11 justify-center rounded-full px-four ${
                    selected ? "bg-primary" : "border border-border bg-card"
                  }`}
                >
                  <Text
                    className={`text-sm font-semibold ${
                      selected ? "text-primary-foreground" : "text-foreground"
                    }`}
                  >
                    {name}
                  </Text>
                </Pressable>
              );
            })}
          </View>
        ) : null}

        {isResolving || (isLinked && studentQuery.isPending) ? (
          <Skeleton
            className="w-full max-w-[420px] rounded-xl"
            style={{ aspectRatio: 85.6 / 53.98 }}
          />
        ) : identityError ? (
          <EmptyState
            title={t("digital_id_student_error")}
            actionTitle={t("home_retry")}
            onAction={() => void retryIdentity()}
            fullScreen
          />
        ) : isLinked && studentQuery.isError ? (
          <EmptyState
            title={t("digital_id_student_error")}
            actionTitle={t("home_retry")}
            onAction={() => void studentQuery.refetch()}
            fullScreen
          />
        ) : !studentId || !student ? (
          <EmptyState title={t("digital_id_context_required")} fullScreen />
        ) : templateQuery.isPending ? (
          <Skeleton
            className="w-full max-w-[420px] rounded-xl"
            style={{ aspectRatio: 85.6 / 53.98 }}
          />
        ) : templateQuery.isError ? (
          <EmptyState
            title={t("digital_id_template_error")}
            actionTitle={t("home_retry")}
            onAction={() => void templateQuery.refetch()}
            fullScreen
          />
        ) : !template ? (
          <EmptyState title={t("digital_id_template_missing")} fullScreen />
        ) : documentResult.error || !documentResult.document ? (
          <EmptyState title={t("digital_id_template_invalid")} fullScreen />
        ) : (
          <DigitalIdTemplate
            template={template}
            document={documentResult.document}
            student={student}
            organization={organizationQuery.data}
            isArabic={isArabic}
          />
        )}
      </ScreenScrollView>
    </View>
  );
}
