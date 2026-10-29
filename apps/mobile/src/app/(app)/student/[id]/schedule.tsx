/**
 * @module @orbis/mobile/app/(app)/student/[id]/schedule
 *
 * Weekly Academic Timetable & Class Schedule for Students.
 * Features weekday tab switching, period time blocks, subject badges, and room details.
 */

import React, { useState } from "react";
import { View, Text, Pressable } from "react-native";
import { useLocalSearchParams } from "expo-router";
import { StudentRecordLayout } from "@/components/student-record-layout";
import { Card } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { EmptyState } from "@/components/ui/empty-state";
import {
  useLinkedStudentId,
  useStudentScheduleQuery,
} from "@/hooks/use-api-queries";
import type { StudentScheduleItem } from "@/hooks/use-api-queries";
import { useI18n } from "@/lib/i18n";
import { useTheme } from "@/hooks/use-theme";
import type { TranslationKey } from "@/lib/i18n/types";
import { Calendar, Clock, Sparkles } from "@orbis/icons/native";

const weekdayKeys: TranslationKey[] = [
  "student_schedule_day_1",
  "student_schedule_day_2",
  "student_schedule_day_3",
  "student_schedule_day_4",
  "student_schedule_day_5",
];

export default function StudentScheduleScreen() {
  const { id } = useLocalSearchParams<{ id: string }>();
  const theme = useTheme();
  const { t, isArabic } = useI18n();

  const requestedStudentId = typeof id === "string" ? id : undefined;
  const { studentId, isLinked, isResolving, identityMissing, retryIdentity } =
    useLinkedStudentId(requestedStudentId);

  const scheduleQuery = useStudentScheduleQuery(studentId);
  const schedule = scheduleQuery.data ?? [];

  // Default to day 1 (Sunday) or current day
  const [selectedDayIndex, setSelectedDayIndex] = useState(0);
  const activeDayOfWeek = selectedDayIndex + 1;

  const activeDayLessons = schedule.filter(
    (item: StudentScheduleItem) =>
      item.timeSlot.day_of_week === activeDayOfWeek,
  );

  return (
    <StudentRecordLayout
      title={t("student_schedule") || "Schedule"}
      showBack={true}
      isPending={isResolving || (isLinked && scheduleQuery.isPending)}
      isError={!isResolving && (!isLinked || scheduleQuery.isError)}
      canRetry={isLinked}
      errorTitle={identityMissing ? t("student_profile_missing") : undefined}
      errorDescription={
        isLinked || isResolving || identityMissing
          ? undefined
          : t("student_access_denied")
      }
      onRetry={() =>
        void (isLinked ? scheduleQuery.refetch() : retryIdentity())
      }
    >
      {/* 1. Day of Week Switcher Tabs */}
      <View className="flex-row justify-between gap-one pb-one">
        {weekdayKeys.map((dayKey, index) => {
          const isSelected = index === selectedDayIndex;
          return (
            <Pressable
              key={dayKey}
              accessibilityRole="tab"
              accessibilityState={{ selected: isSelected }}
              onPress={() => setSelectedDayIndex(index)}
              className={`flex-1 items-center justify-center rounded-md py-two ${
                isSelected
                  ? "bg-primary shadow-xs"
                  : "border border-border/80 bg-card"
              }`}
            >
              <Text
                className={`text-xs font-bold ${
                  isSelected
                    ? "text-primary-foreground"
                    : "text-muted-foreground"
                }`}
                numberOfLines={1}
              >
                {t(dayKey)}
              </Text>
            </Pressable>
          );
        })}
      </View>

      {/* 2. Schedule Timeline for Selected Day */}
      {activeDayLessons.length === 0 ? (
        <View className="py-ten">
          <EmptyState
            title={t("student_no_schedule") || "No Classes Scheduled"}
            description={
              t("schedule_day_off_hint") ||
              "No lessons are assigned for this day."
            }
            icon={<Calendar size={32} className="text-muted-foreground" />}
          />
        </View>
      ) : (
        <View className="gap-three">
          {activeDayLessons.map(
            ({ lesson, timeSlot, subject }: StudentScheduleItem) => {
              const subjectName =
                (isArabic ? subject?.name_ar : subject?.name) ||
                subject?.name ||
                t("domain_schedule");

              return (
                <Card key={lesson.id} variant="outline" padding="four">
                  <View className="flex-row items-center justify-between pb-two">
                    <View className="gap-half flex-1">
                      <Text className="text-base font-bold text-foreground">
                        {subjectName}
                      </Text>
                      {timeSlot.name ? (
                        <Text className="text-xs text-muted-foreground">
                          {isArabic
                            ? timeSlot.name_ar || timeSlot.name
                            : timeSlot.name}
                        </Text>
                      ) : null}
                    </View>

                    <Badge
                      variant="secondary"
                      className="flex-row items-center gap-one px-two py-half"
                    >
                      <Clock size={12} color={theme.primary} />
                      <Text className="text-xs font-bold text-primary">
                        {timeSlot.start_time} – {timeSlot.end_time}
                      </Text>
                    </Badge>
                  </View>

                  {lesson.room_id ? (
                    <View className="border-t border-border/40 pt-two">
                      <Text className="text-xs text-muted-foreground">
                        {t("classroom_label") || "Classroom"}: {lesson.room_id}
                      </Text>
                    </View>
                  ) : null}
                </Card>
              );
            },
          )}
        </View>
      )}
    </StudentRecordLayout>
  );
}
