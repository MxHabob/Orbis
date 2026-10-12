/**
 * @module @orbis/mobile/app/(app)/student/[id]/grades
 *
 * Academic Grades & Assessment Performance for Students.
 * Shows overall score average, letter grade badges, progress indicators,
 * and individual assessment breakdowns.
 */

import React from "react";
import { View, Text } from "react-native";
import { useLocalSearchParams } from "expo-router";
import { StudentRecordLayout } from "@/components/student-record-layout";
import { Card } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { EmptyState } from "@/components/ui/empty-state";
import {
  useGradesQuery,
  useLinkedStudentId,
} from "@/hooks/use-api-queries";
import { useI18n, formatNumber } from "@/lib/i18n";
import { useTheme } from "@/hooks/use-theme";
import {
  ChartBar,
  AcademicCap,
  Sparkles,
} from "@orbis/icons/native";

export default function StudentGradesScreen() {
  const { id } = useLocalSearchParams<{ id: string }>();
  const theme = useTheme();
  const { t, formatDate } = useI18n();

  const requestedStudentId = typeof id === "string" ? id : undefined;
  const {
    studentId,
    isLinked,
    isResolving,
    identityError,
    retryIdentity,
  } = useLinkedStudentId(requestedStudentId);

  const gradesQuery = useGradesQuery({ studentId });
  const rawGrades = gradesQuery.data ?? [];
  const grades = [...rawGrades].sort((a, b) =>
    b.created_at.localeCompare(a.created_at),
  );

  // Compute average percentage
  let validPercentSum = 0;
  let validPercentCount = 0;
  grades.forEach((g) => {
    const s = Number(g.score);
    const m = Number(g.max_score);
    if (Number.isFinite(s) && Number.isFinite(m) && m > 0) {
      validPercentSum += (s / m) * 100;
      validPercentCount += 1;
    }
  });

  const overallAverage =
    validPercentCount > 0 ? Math.round(validPercentSum / validPercentCount) : null;

  return (
    <StudentRecordLayout
      title={t("student_grades") || "Grades"}
      showBack={true}
      isPending={isResolving || (isLinked && gradesQuery.isPending)}
      isError={!isResolving && (!isLinked || gradesQuery.isError)}
      canRetry
      errorDescription={
        isLinked || isResolving || identityError
          ? undefined
          : t("student_access_denied")
      }
      onRetry={() =>
        void (isLinked ? gradesQuery.refetch() : retryIdentity())
      }
    >
      {/* 1. Academic Performance Summary Widget */}
      {overallAverage !== null ? (
        <Card
          variant="default"
          padding="four"
          className="bg-card border border-border/80 shadow-sm"
        >
          <View className="flex-row items-center justify-between">
            <View className="gap-half">
              <Text className="text-xs font-semibold text-muted-foreground">
                {t("academic_average_label") || "Academic Average"}
              </Text>
              <View className="flex-row items-baseline gap-two">
                <Text className="text-3xl font-extrabold text-primary">
                  {formatNumber(overallAverage)}%
                </Text>
                <Text className="text-xs font-medium text-muted-foreground">
                  ({grades.length} {t("assessments_count") || "Assessments"})
                </Text>
              </View>
            </View>

            <View className="h-12 w-12 items-center justify-center rounded-2xl bg-primary/10">
              <AcademicCap size={28} color={theme.primary} />
            </View>
          </View>
        </Card>
      ) : null}

      {/* 2. Grades List */}
      {grades.length === 0 ? (
        <View className="py-ten">
          <EmptyState
            title={t("student_no_grades")}
            icon={<ChartBar size={32} className="text-muted-foreground" />}
          />
        </View>
      ) : (
        <View className="gap-three">
          {grades.map((grade) => {
            const score = Number(grade.score);
            const maxScore = Number(grade.max_score);
            const scoreText =
              Number.isFinite(score) && Number.isFinite(maxScore)
                ? `${formatNumber(score)} / ${formatNumber(maxScore)}`
                : grade.score;

            const percent =
              Number.isFinite(score) && Number.isFinite(maxScore) && maxScore > 0
                ? Math.round((score / maxScore) * 100)
                : null;

            const letter = grade.letter_grade || (percent !== null ? (percent >= 90 ? "A" : percent >= 80 ? "B" : percent >= 70 ? "C" : "D") : "—");

            return (
              <Card
                key={grade.id}
                variant="outline"
                padding="four"
                className="bg-card border-border/80 shadow-2xs gap-two"
              >
                <View className="flex-row items-center justify-between">
                  <View className="flex-row items-center gap-two">
                    <Badge variant="secondary" className="px-two py-half">
                      <Text className="text-xs font-bold text-primary">
                        {letter}
                      </Text>
                    </Badge>
                    <Text className="text-sm font-bold text-foreground">
                      {grade.letter_grade || t("student_grade_score")}
                    </Text>
                  </View>

                  {percent !== null ? (
                    <Text className="text-sm font-extrabold text-primary">
                      {formatNumber(percent)}%
                    </Text>
                  ) : null}
                </View>

                {/* Score Ratio Progress Bar */}
                {percent !== null ? (
                  <View className="h-1.5 w-full overflow-hidden rounded-full bg-secondary my-one">
                    <View
                      style={{ width: `${Math.min(percent, 100)}%` }}
                      className="h-full rounded-full bg-primary"
                    />
                  </View>
                ) : null}

                <View className="flex-row items-center justify-between border-t border-border/40 pt-two">
                  <Text className="text-xs font-medium text-muted-foreground">
                    {scoreText}
                  </Text>
                  <Text className="text-xs text-muted-foreground">
                    {formatDate(grade.created_at)}
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
