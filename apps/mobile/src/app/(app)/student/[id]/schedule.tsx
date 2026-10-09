import { StudentRecordLayout } from "@/components/student-record-layout";
import { Card } from "@/components/ui/card";
import { EmptyState } from "@/components/ui/empty-state";
import {
  useLinkedStudentId,
  useStudentScheduleQuery,
} from "@/hooks/use-api-queries";
import { useI18n } from "@/lib/i18n";
import type { TranslationKey } from "@/lib/i18n/types";
import { useLocalSearchParams } from "expo-router";
import { Text, View } from "react-native";

const weekdayKeys: TranslationKey[] = [
  "student_schedule_day_1",
  "student_schedule_day_2",
  "student_schedule_day_3",
  "student_schedule_day_4",
  "student_schedule_day_5",
  "student_schedule_day_6",
  "student_schedule_day_7",
];

export default function StudentScheduleScreen() {
  const { id } = useLocalSearchParams<{ id: string }>();
  const requestedStudentId = typeof id === "string" ? id : undefined;
  const {
    studentId,
    isLinked,
    isResolving,
    identityError,
    retryIdentity,
  } =
    useLinkedStudentId(requestedStudentId);
  const scheduleQuery = useStudentScheduleQuery(studentId);
  const { t, isArabic } = useI18n();
  const schedule = scheduleQuery.data ?? [];

  return (
    <StudentRecordLayout
      title={t("student_schedule")}
      isPending={isResolving || (isLinked && scheduleQuery.isPending)}
      isError={!isResolving && (!isLinked || scheduleQuery.isError)}
      canRetry
      errorDescription={
        isLinked || isResolving || identityError
          ? undefined
          : t("student_access_denied")
      }
      onRetry={() =>
        void (isLinked ? scheduleQuery.refetch() : retryIdentity())
      }
    >
      {schedule.length === 0 ? (
        <EmptyState title={t("student_no_schedule")} />
      ) : (
        <View className="gap-four">
          {weekdayKeys.map((dayKey, index) => {
            const dayOfWeek = index + 1;
            const lessons = schedule.filter(
              (item) => item.timeSlot.day_of_week === dayOfWeek,
            );
            if (lessons.length === 0) return null;

            return (
              <View key={dayKey} className="gap-two">
                <Text className="text-base font-semibold text-foreground">
                  {t(dayKey)}
                </Text>
                {lessons.map(({ lesson, timeSlot, subject }) => (
                  <Card key={lesson.id} variant="outline" padding="three">
                    <View className="flex-row items-center justify-between gap-three">
                      <View className="flex-1 gap-one">
                        <Text className="text-sm font-semibold text-foreground">
                          {(isArabic ? subject?.name_ar : subject?.name) ||
                            subject?.name ||
                            subject?.name_ar ||
                            t("domain_schedule")}
                        </Text>
                        {timeSlot.name ? (
                          <Text className="text-xs text-muted-foreground">
                            {isArabic
                              ? timeSlot.name_ar || timeSlot.name
                              : timeSlot.name}
                          </Text>
                        ) : null}
                      </View>
                      <View className="items-end gap-one">
                        <Text className="text-xs text-muted-foreground">
                          {t("student_schedule_time")}
                        </Text>
                        <Text className="text-sm font-medium text-foreground">
                          {timeSlot.start_time} – {timeSlot.end_time}
                        </Text>
                      </View>
                    </View>
                  </Card>
                ))}
              </View>
            );
          })}
        </View>
      )}
    </StudentRecordLayout>
  );
}
