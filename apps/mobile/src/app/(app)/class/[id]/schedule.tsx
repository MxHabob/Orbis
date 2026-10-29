import { Card } from "@/components/ui/card";
import { EmptyState } from "@/components/ui/empty-state";
import { StudentRecordLayout } from "@/components/student-record-layout";
import { useClassScheduleQuery } from "@/hooks/use-api-queries";
import type { StudentScheduleItem } from "@/hooks/use-api-queries";
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

export default function ClassScheduleScreen() {
  const { id } = useLocalSearchParams<{ id: string }>();
  const classId = typeof id === "string" ? id : undefined;
  const scheduleQuery = useClassScheduleQuery(classId);
  const { t, isArabic } = useI18n();
  const schedule = scheduleQuery.data ?? [];

  return (
    <StudentRecordLayout
      title={t("teacher_class_schedule")}
      isPending={Boolean(classId) && scheduleQuery.isPending}
      isError={!classId || scheduleQuery.isError}
      onRetry={() => void scheduleQuery.refetch()}
    >
      {schedule.length === 0 ? (
        <EmptyState title={t("student_no_schedule")} />
      ) : (
        <View className="gap-four">
          {weekdayKeys.map((dayKey, index) => {
            const lessons = schedule.filter(
              (item: StudentScheduleItem) => item.timeSlot.day_of_week === index + 1,
            );
            if (lessons.length === 0) return null;
            return (
              <View key={dayKey} className="gap-two">
                <Text className="text-base font-semibold text-foreground">
                  {t(dayKey)}
                </Text>
                {lessons.map(({ lesson, timeSlot, subject }: StudentScheduleItem) => (
                  <Card key={lesson.id} variant="outline" padding="three">
                    <View className="flex-row items-center justify-between gap-three">
                      <View className="flex-1 gap-one">
                        <Text className="text-sm font-semibold text-foreground">
                          {(isArabic ? subject?.name_ar : subject?.name) ||
                            subject?.name ||
                            subject?.name_ar ||
                            t("domain_schedule")}
                        </Text>
                        <Text className="text-xs text-muted-foreground">
                          {(isArabic ? timeSlot.name_ar : timeSlot.name) ||
                            timeSlot.name ||
                            ""}
                        </Text>
                      </View>
                      <Text className="text-sm font-medium text-foreground">
                        {timeSlot.start_time} – {timeSlot.end_time}
                      </Text>
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
