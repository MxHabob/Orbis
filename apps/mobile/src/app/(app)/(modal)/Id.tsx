import { StudentRecordLayout } from "@/components/student-record-layout";
import { Card } from "@/components/ui/card";
import { EmptyState } from "@/components/ui/empty-state";
import {
  useLinkedStudentId,
  useStudentQuery,
} from "@/hooks/use-api-queries";
import { useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";
import type { TranslationKey } from "@/lib/i18n/types";
import { useRouter } from "expo-router";
import { Pressable, Text, View } from "react-native";
import { useState } from "react";

const studentStatusKeys: Partial<Record<string, TranslationKey>> = {
  active: "student_status_active",
  inactive: "student_status_inactive",
  on_leave: "student_status_on_leave",
  suspended: "student_status_suspended",
  withdrawn: "student_status_withdrawn",
  transferred_out: "student_status_transferred_out",
  graduated: "student_status_graduated",
  expelled: "student_status_expelled",
};

export default function DigitalIdModalScreen() {
  const router = useRouter();
  const { user, effectiveContext } = useAuth();
  const { t, isArabic, formatDate } = useI18n();
  const [selectedDependentId, setSelectedDependentId] = useState<string>();
  const dependents = effectiveContext?.dependents ?? [];
  const roles = [
    ...(user?.roles ?? []),
    effectiveContext?.effective_role ?? "",
    ...(effectiveContext?.roles ?? []),
  ].map((role) => role.toLowerCase().replace(/[^a-z]/g, ""));
  const isStudent = roles.some((role) => role.includes("student"));
  const isGuardian = roles.some(
    (role) => role.includes("parent") || role.includes("guardian"),
  );
  const selectedDependent =
    dependents.find((dependent) => dependent.student_id === selectedDependentId) ??
    dependents[0];
  const requestedStudentId = isStudent
    ? "me"
    : isGuardian
      ? selectedDependent?.student_id
      : undefined;
  const {
    studentId,
    isLinked,
    isResolving,
    retryIdentity,
  } = useLinkedStudentId(requestedStudentId);
  const studentQuery = useStudentQuery(studentId);
  const student = studentQuery.data;
  const statusKey = student
    ? studentStatusKeys[student.status.toLowerCase()]
    : undefined;
  const hasStudentContext = isStudent || isGuardian;
  const hasSelectedStudent = isStudent || Boolean(selectedDependent);
  const studentName = student
    ? isArabic
      ? student.name_ar || student.full_name
      : student.display_name || student.full_name
    : "";

  return (
    <StudentRecordLayout
      title={t("digital_id_title")}
      isPending={
        hasSelectedStudent &&
        (isResolving ||
          (isLinked && studentQuery.isPending))
      }
      isError={
        !hasStudentContext ||
        (hasSelectedStudent && !isResolving && !isLinked) ||
        studentQuery.isError
      }
      canRetry={hasStudentContext}
      errorDescription={
        hasStudentContext
          ? isLinked
            ? undefined
            : t("student_access_denied")
          : t("digital_id_context_required")
      }
      onRetry={() =>
        void (isLinked ? studentQuery.refetch() : retryIdentity())
      }
    >
      {isGuardian && dependents.length === 0 ? (
        <EmptyState title={t("home_no_dependents")} />
      ) : student ? (
        <View className="gap-four">
          {isGuardian && dependents.length > 1 ? (
            <View className="gap-two">
              <Text className="text-sm font-semibold text-foreground">
                {t("home_dependents")}
              </Text>
              {dependents.map((dependent) => {
                const selected =
                  dependent.student_id === selectedDependent?.student_id;
                return (
                  <Pressable
                    key={dependent.student_id}
                    accessibilityRole="button"
                    accessibilityState={{ selected }}
                    onPress={() =>
                      setSelectedDependentId(dependent.student_id)
                    }
                    className={`rounded-lg border px-four py-three ${
                      selected
                        ? "border-primary bg-primary/10"
                        : "border-border bg-card"
                    }`}
                  >
                    <Text className="font-semibold text-foreground">
                      {isArabic
                        ? dependent.full_name_ar || dependent.full_name
                        : dependent.full_name}
                    </Text>
                  </Pressable>
                );
              })}
            </View>
          ) : null}

          <Card variant="elevated" padding="six">
            <View className="items-center gap-three">
              <Text className="text-sm font-semibold text-primary">
                {t("nav_app_name")}
              </Text>
              <Text className="text-center text-2xl font-bold text-foreground">
                {studentName}
              </Text>
              <View className="w-full items-center gap-one rounded-lg bg-background-element px-four py-three">
                <Text className="text-xs text-muted-foreground">
                  {t("student_number")}
                </Text>
                <Text
                  selectable
                  className="text-xl font-bold tracking-widest text-foreground"
                >
                  {student.student_number}
                </Text>
              </View>
              <View className="w-full flex-row items-center justify-between gap-three">
                <Text className="text-sm text-muted-foreground">
                  {t("student_status")}
                </Text>
                <Text className="text-sm font-medium text-foreground">
                  {statusKey ? t(statusKey) : student.status}
                </Text>
              </View>
              {student.date_of_birth ? (
                <View className="w-full flex-row items-center justify-between gap-three">
                  <Text className="text-sm text-muted-foreground">
                    {t("student_birth_date")}
                  </Text>
                  <Text className="text-sm font-medium text-foreground">
                    {formatDate(student.date_of_birth)}
                  </Text>
                </View>
              ) : null}
            </View>
          </Card>

          <Text className="text-center text-xs leading-5 text-muted-foreground">
            {t("digital_id_privacy_notice")}
          </Text>
        </View>
      ) : (
        <EmptyState title={t("digital_id_context_required")} />
      )}
    </StudentRecordLayout>
  );
}
