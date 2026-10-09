import { StudentRecordLayout } from "@/components/student-record-layout";
import { Card } from "@/components/ui/card";
import { EmptyState } from "@/components/ui/empty-state";
import {
  useLinkedStudentId,
  useStudentAttendanceQuery,
} from "@/hooks/use-api-queries";
import { useI18n } from "@/lib/i18n";
import type { TranslationKey } from "@/lib/i18n/types";
import { useLocalSearchParams } from "expo-router";
import { Text, View } from "react-native";

const attendanceStatusKeys: Record<string, TranslationKey> = {
  present: "student_attendance_present",
  absent: "student_attendance_absent",
  late: "student_attendance_late",
  excused: "student_attendance_excused",
};

export default function StudentAttendanceScreen() {
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
  const attendanceQuery = useStudentAttendanceQuery(studentId);
  const { t, formatDate } = useI18n();
  const records = [...(attendanceQuery.data ?? [])].sort((a, b) =>
    b.date.localeCompare(a.date),
  );

  return (
    <StudentRecordLayout
      title={t("student_attendance")}
      isPending={isResolving || (isLinked && attendanceQuery.isPending)}
      isError={!isResolving && (!isLinked || attendanceQuery.isError)}
      canRetry
      errorDescription={
        isLinked || isResolving || identityError
          ? undefined
          : t("student_access_denied")
      }
      onRetry={() =>
        void (isLinked ? attendanceQuery.refetch() : retryIdentity())
      }
    >
      {records.length === 0 ? (
        <EmptyState title={t("student_no_attendance")} />
      ) : (
        <View className="gap-three">
          {records.map((record) => {
            const status = record.status.toLowerCase();
            const statusKey = attendanceStatusKeys[status];
            return (
              <Card key={record.id} variant="outline" padding="four">
                <View className="flex-row items-center justify-between gap-three">
                  <View className="flex-1 gap-one">
                    <Text className="text-sm text-muted-foreground">
                      {t("student_attendance_date")}
                    </Text>
                    <Text className="text-sm font-medium text-foreground">
                      {formatDate(record.date)}
                    </Text>
                  </View>
                  <View className="items-end gap-one">
                    <Text className="text-sm text-muted-foreground">
                      {t("student_attendance_status")}
                    </Text>
                    <Text
                      className={`text-sm font-semibold ${status === "absent" ? "text-destructive" : status === "late" ? "text-warning" : "text-success"}`}
                    >
                      {statusKey ? t(statusKey) : record.status}
                    </Text>
                  </View>
                </View>
                {record.notes ? (
                  <Text className="mt-three text-sm leading-5 text-muted-foreground">
                    {record.notes}
                  </Text>
                ) : null}
              </Card>
            );
          })}
        </View>
      )}
    </StudentRecordLayout>
  );
}
