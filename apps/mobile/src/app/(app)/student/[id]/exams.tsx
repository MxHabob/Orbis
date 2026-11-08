/**
 * @module @orbis/mobile/app/(app)/student/[id]/exams
 *
 * Examination Schedules & Testing Timetable for Students.
 * Lists upcoming assessments, session details, and exam hall timings.
 */

import React from "react";
import { View, Text } from "react-native";
import { useLocalSearchParams } from "expo-router";
import { StudentRecordLayout } from "@/components/student-record-layout";
import { Card } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { EmptyState } from "@/components/ui/empty-state";
import {
  useLinkedStudentId,
  useStudentExamsQuery,
} from "@/hooks/use-api-queries";
import type { StudentExamItem } from "@/hooks/use-api-queries";
import { useI18n, formatNumber } from "@/lib/i18n";
import { useTheme } from "@/hooks/use-theme";
import { AcademicCap, Clock } from "@orbis/icons/native";

export default function StudentExamsScreen() {
  const { id } = useLocalSearchParams<{ id: string }>();
  const theme = useTheme();
  const { t, isArabic, formatDate } = useI18n();

  const requestedStudentId = typeof id === "string" ? id : undefined;
  const { studentId, isLinked, isResolving, identityMissing, retryIdentity } =
    useLinkedStudentId(requestedStudentId);

  const examsQuery = useStudentExamsQuery(studentId);
  const exams = examsQuery.data ?? [];

  return (
    <StudentRecordLayout
      title={t("student_exams") || "Exams"}
      showBack={true}
      isPending={isResolving || (isLinked && examsQuery.isPending)}
      isError={!isResolving && (!isLinked || examsQuery.isError)}
      canRetry={isLinked}
      errorTitle={identityMissing ? t("student_profile_missing") : undefined}
      errorDescription={
        isLinked || isResolving || identityMissing
          ? undefined
          : t("student_access_denied")
      }
      onRetry={() => void (isLinked ? examsQuery.refetch() : retryIdentity())}
    >
      {/* 1. Exam Count Banner */}
      {exams.length > 0 ? (
        <Card variant="default" padding="four">
          <View className="flex-row items-center justify-between">
            <View className="gap-half">
              <Text className="text-xs font-semibold text-muted-foreground">
                {t("upcoming_exams_label") || "Upcoming Exams"}
              </Text>
              <Text className="text-2xl font-extrabold text-foreground">
                {formatNumber(exams.length)}{" "}
                {t("exams_scheduled") || "Scheduled"}
              </Text>
            </View>

            <View className="h-11 w-11 items-center justify-center rounded-md bg-primary/10">
              <AcademicCap size={24} color={theme.primary} />
            </View>
          </View>
        </Card>
      ) : null}

      {/* 2. Exams List */}
      {exams.length === 0 ? (
        <View className="py-ten">
          <EmptyState
            title={t("student_no_exams")}
            description={
              t("student_no_exams_description") ||
              "No exams are currently scheduled for your courses."
            }
            icon={<AcademicCap size={32} color={theme.mutedForeground} />}
          />
        </View>
      ) : (
        <View className="gap-three">
          {exams.map(({ schedule, session, subject }: StudentExamItem) => {
            const subjectName = subject
              ? isArabic
                ? subject.name_ar || subject.name
                : subject.name
              : t("student_exam");

            return (
              <Card
                key={schedule.id}
                variant="outline"
                padding="four"
                className="gap-two"
              >
                <View className="flex-row items-center justify-between pb-one">
                  <View className="gap-half flex-1">
                    <Text className="text-base font-bold text-foreground">
                      {subjectName}
                    </Text>
                    <Text className="text-xs text-muted-foreground">
                      {session.name}
                    </Text>
                  </View>

                  <Badge variant="secondary" className="px-two py-half">
                    <Text className="text-xs font-bold text-primary">
                      {formatDate(schedule.exam_date)}
                    </Text>
                  </Badge>
                </View>

                <View className="flex-row items-center justify-between border-t border-border/40 pt-two">
                  <View className="flex-row items-center gap-one">
                    <Clock size={14} color={theme.mutedForeground} />
                    <Text className="text-xs font-medium text-muted-foreground">
                      {schedule.start_time} – {schedule.end_time}
                    </Text>
                  </View>
                </View>
              </Card>
            );
          })}
        </View>
      )}
    </StudentRecordLayout>
  );
}
