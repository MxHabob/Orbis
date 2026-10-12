import { Card } from "@/components/ui/card";
import { EmptyState } from "@/components/ui/empty-state";
import { StudentRecordLayout } from "@/components/student-record-layout";
import {
  useLinkedStudentId,
  useStudentExamsQuery,
} from "@/hooks/use-api-queries";
import { useI18n } from "@/lib/i18n";
import { useLocalSearchParams } from "expo-router";
import { Text, View } from "react-native";

export default function StudentExamsScreen() {
  const { id } = useLocalSearchParams<{ id: string }>();
  const requestedStudentId = typeof id === "string" ? id : undefined;
  const {
    studentId,
    isLinked,
    isResolving,
    retryIdentity,
  } = useLinkedStudentId(requestedStudentId);
  const examsQuery = useStudentExamsQuery(studentId);
  const { t, isArabic, formatDate } = useI18n();

  const exams = examsQuery.data ?? [];

  return (
    <StudentRecordLayout
      title={t("student_exams")}
      isPending={isResolving || (isLinked && examsQuery.isPending)}
      isError={!isResolving && (!isLinked || examsQuery.isError)}
      errorDescription={isLinked ? undefined : t("student_access_denied")}
      onRetry={() =>
        void (isLinked ? examsQuery.refetch() : retryIdentity())
      }
    >
      {exams.length === 0 ? (
        <EmptyState title={t("student_no_exams")} />
      ) : (
        <View className="gap-three">
          {exams.map(({ schedule, session, subject }) => (
            <Card key={schedule.id} variant="outline" padding="four">
              <View className="gap-two">
                <Text className="text-base font-semibold text-foreground">
                  {subject
                    ? isArabic
                      ? subject.name_ar || subject.name
                      : subject.name
                    : t("student_exam")}
                </Text>
                <Text className="text-sm text-muted-foreground">
                  {session.name}
                </Text>
                <View className="flex-row items-center justify-between gap-three">
                  <Text className="text-sm text-muted-foreground">
                    {t("student_exam_date")}
                  </Text>
                  <Text className="text-sm font-medium text-foreground">
                    {formatDate(schedule.exam_date)}
                  </Text>
                </View>
                <View className="flex-row items-center justify-between gap-three">
                  <Text className="text-sm text-muted-foreground">
                    {t("student_exam_time")}
                  </Text>
                  <Text className="text-sm font-medium text-foreground">
                    {schedule.start_time} - {schedule.end_time}
                  </Text>
                </View>
              </View>
            </Card>
          ))}
        </View>
      )}
    </StudentRecordLayout>
  );
}
