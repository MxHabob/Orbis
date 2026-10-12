import { StudentRecordLayout } from "@/components/student-record-layout";
import { Card } from "@/components/ui/card";
import { EmptyState } from "@/components/ui/empty-state";
import {
  useLinkedStudentId,
  useStudentBehaviorQuery,
} from "@/hooks/use-api-queries";
import { useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";
import { formatNumber } from "@/lib/i18n";
import type { TranslationKey } from "@/lib/i18n/types";
import { useState } from "react";
import { Pressable, Text, View } from "react-native";

const behaviorTypeKeys: Record<string, TranslationKey> = {
  merit: "behavior_record_merit",
  violation: "behavior_record_violation",
};

export default function BehaviorTabScreen() {
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
  const behaviorQuery = useStudentBehaviorQuery(studentId);
  const records = [...(behaviorQuery.data ?? [])].sort((a, b) =>
    b.created_at.localeCompare(a.created_at),
  );
  const netPoints = records.reduce((total, record) => total + record.points, 0);
  const hasStudentContext = isStudent || isGuardian;
  const hasSelectedStudent = isStudent || Boolean(selectedDependent);

  return (
    <StudentRecordLayout
      title={t("behavior_title")}
      isPending={hasSelectedStudent && (isResolving || (isLinked && behaviorQuery.isPending))}
      isError={
        !hasStudentContext ||
        (hasSelectedStudent && !isResolving && !isLinked) ||
        behaviorQuery.isError
      }
      canRetry={hasStudentContext}
      errorDescription={
        hasStudentContext
          ? isLinked
            ? undefined
            : t("student_access_denied")
          : t("behavior_student_context_required")
      }
      onRetry={() =>
        void (isLinked ? behaviorQuery.refetch() : retryIdentity())
      }
    >
      {isGuardian && dependents.length > 1 ? (
        <View className="gap-two">
          <Text className="text-sm font-semibold text-foreground">
            {t("home_dependents")}
          </Text>
          <View className="gap-two">
            {dependents.map((dependent) => {
              const selected = dependent.student_id === selectedDependent?.student_id;
              return (
                <Pressable
                  key={dependent.student_id}
                  accessibilityRole="button"
                  accessibilityState={{ selected }}
                  onPress={() => setSelectedDependentId(dependent.student_id)}
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
        </View>
      ) : null}

      {isGuardian && dependents.length === 0 ? (
        <EmptyState title={t("home_no_dependents")} />
      ) : records.length === 0 ? (
        <EmptyState
          title={t("behavior_empty_title")}
          description={t("behavior_empty_desc")}
        />
      ) : (
        <View className="gap-three">
          <Card variant="outline" padding="four">
            <View className="gap-one">
              <Text className="text-sm text-muted-foreground">
                {t("behavior_score_net")}
              </Text>
              <Text className="text-2xl font-bold text-foreground">
                {t("behavior_score_unit").replace(
                  "{count}",
                  formatNumber(netPoints),
                )}
              </Text>
            </View>
          </Card>
          {records.map((record) => {
            const recordType = record.record_type.toLowerCase();
            const typeKey = behaviorTypeKeys[recordType];
            return (
              <Card key={record.id} variant="outline" padding="four">
                <View className="gap-two">
                  <View className="flex-row items-center justify-between gap-three">
                    <Text className="text-base font-semibold text-foreground">
                      {typeKey ? t(typeKey) : t("behavior_record_other")}
                    </Text>
                    <Text
                      className={`text-sm font-semibold ${
                        record.points >= 0
                          ? "text-primary"
                          : "text-destructive"
                      }`}
                    >
                      {record.points > 0 ? "+" : ""}
                      {formatNumber(record.points)}
                    </Text>
                  </View>
                  <Text className="text-sm leading-5 text-foreground">
                    {record.description}
                  </Text>
                  <Text className="text-xs text-muted-foreground">
                    {formatDate(record.created_at)}
                  </Text>
                </View>
              </Card>
            );
          })}
        </View>
      )}
    </StudentRecordLayout>
  );
}
