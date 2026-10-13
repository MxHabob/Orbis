import { Card } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { EmptyState } from "@/components/ui/empty-state";
import { StudentRecordLayout } from "@/components/student-record-layout";
import {
  useBatchMarkAttendanceMutation,
  useClassAttendanceQuery,
  useTeacherClassDataQuery,
} from "@/hooks/use-api-queries";
import { useEffectiveContext } from "@/providers/effective-context";
import { useI18n } from "@/lib/i18n";
import type { TranslationKey } from "@/lib/i18n/types";
import { useToast } from "@/components/ui/toast";
import { useLocalSearchParams } from "expo-router";
import { useMemo, useState } from "react";
import { Pressable, Text, View } from "react-native";

const attendanceOptions: {
  value: string;
  key: TranslationKey;
}[] = [
  { value: "present", key: "student_attendance_present" },
  { value: "absent", key: "student_attendance_absent" },
  { value: "late", key: "student_attendance_late" },
  { value: "excused", key: "student_attendance_excused" },
];

function getLocalDate(date: Date) {
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, "0");
  const day = String(date.getDate()).padStart(2, "0");
  return `${year}-${month}-${day}`;
}

function shiftDate(value: string, amount: number) {
  const [year, month, day] = value.split("-").map(Number);
  const date = new Date(year, month - 1, day);
  date.setDate(date.getDate() + amount);
  return getLocalDate(date);
}

export default function ClassAttendanceScreen() {
  const { id } = useLocalSearchParams<{ id: string }>();
  const classId = typeof id === "string" ? id : undefined;
  const [date, setDate] = useState(() => getLocalDate(new Date()));
  const [selectedStatuses, setSelectedStatuses] = useState<
    Record<string, string>
  >({});
  const classQuery = useTeacherClassDataQuery(classId);
  const attendanceQuery = useClassAttendanceQuery(classId, date);
  const mutation = useBatchMarkAttendanceMutation();
  const { hasPermission } = useEffectiveContext();
  const { t, isArabic, formatDate } = useI18n();
  const toast = useToast();
  const canRecord = hasPermission("attendance:write");
  const recordedByStudent = useMemo(
    () =>
      new Map(
        (attendanceQuery.data ?? []).map((record) => [
          record.student_id,
          record,
        ]),
      ),
    [attendanceQuery.data],
  );
  const roster = classQuery.data?.roster ?? [];
  const markedCount = roster.filter(
    ({ student }) =>
      !recordedByStudent.has(student.id) && selectedStatuses[student.id],
  ).length;
  const isPending = classQuery.isPending || attendanceQuery.isPending;
  const isError = classQuery.isError || attendanceQuery.isError;

  const saveAttendance = async () => {
    if (!classId || !canRecord) return;
    const items = roster.flatMap(({ student }) => {
      const status = selectedStatuses[student.id];
      if (!status || recordedByStudent.has(student.id)) return [];
      return [{ student_id: student.id, status }];
    });
    if (items.length === 0) {
      toast.warning(t("teacher_no_records_to_save"));
      return;
    }
    try {
      await mutation.mutateAsync({ class_id: classId, date, items });
      setSelectedStatuses({});
      toast.success(t("teacher_save_success"));
    } catch (error) {
      toast.error(error instanceof Error ? error.message : t("common_error"));
    }
  };

  return (
    <StudentRecordLayout
      title={t("teacher_mark_attendance")}
      isPending={isPending}
      isError={!classId || isError}
      onRetry={() => {
        void classQuery.refetch();
        void attendanceQuery.refetch();
      }}
    >
      <View className="gap-three">
        <View className="flex-row items-center justify-between gap-three">
          <Pressable
            accessibilityRole="button"
            accessibilityLabel={t("teacher_previous_day")}
            onPress={() => {
              setDate((current) => shiftDate(current, -1));
              setSelectedStatuses({});
            }}
            className="rounded-md border border-border bg-card px-three py-two"
          >
            <Text className="text-sm font-semibold text-foreground">‹</Text>
          </Pressable>
          <View className="items-center gap-one">
            <Text className="text-sm font-semibold text-foreground">
              {formatDate(new Date(`${date}T12:00:00`))}
            </Text>
            {date === getLocalDate(new Date()) ? (
              <Text className="text-xs text-muted-foreground">
                {t("teacher_today")}
              </Text>
            ) : null}
          </View>
          <Pressable
            accessibilityRole="button"
            accessibilityLabel={t("teacher_next_day")}
            onPress={() => {
              setDate((current) => shiftDate(current, 1));
              setSelectedStatuses({});
            }}
            className="rounded-md border border-border bg-card px-three py-two"
          >
            <Text className="text-sm font-semibold text-foreground">›</Text>
          </Pressable>
        </View>
        {!canRecord ? (
          <EmptyState title={t("teacher_no_permission")} />
        ) : roster.length === 0 ? (
          <EmptyState title={t("teacher_no_students")} />
        ) : (
          <>
            {roster.map(({ student, enrollment }) => {
              const record = recordedByStudent.get(student.id);
              const studentName = isArabic
                ? student.name_ar || student.full_name
                : student.display_name || student.full_name;
              return (
                <Card key={student.id} variant="outline" padding="three">
                  <View className="gap-three">
                    <View className="flex-row items-center justify-between gap-two">
                      <Text className="flex-1 text-sm font-semibold text-foreground">
                        {studentName}
                      </Text>
                      <Text className="text-xs text-muted-foreground">
                        {enrollment.student_number}
                      </Text>
                    </View>
                    {record ? (
                      <Text className="text-xs font-medium text-success">
                        {t("teacher_already_recorded")}:{" "}
                        {attendanceOptions.find(
                          (option) => option.value === record.status.toLowerCase(),
                        )
                          ? t(
                              attendanceOptions.find(
                                (option) =>
                                  option.value === record.status.toLowerCase(),
                              )!.key,
                            )
                          : record.status}
                      </Text>
                    ) : (
                      <View className="gap-two">
                        <Text className="text-xs text-muted-foreground">
                          {t("teacher_select_status")}
                        </Text>
                        <View className="flex-row flex-wrap gap-two">
                          {attendanceOptions.map((option) => {
                            const isSelected =
                              selectedStatuses[student.id] === option.value;
                            return (
                              <Pressable
                                key={option.value}
                                accessibilityRole="button"
                                accessibilityState={{ selected: isSelected }}
                                onPress={() =>
                                  setSelectedStatuses((current) => ({
                                    ...current,
                                    [student.id]: option.value,
                                  }))
                                }
                                className={`min-h-10 items-center justify-center rounded-md border px-three py-two ${
                                  isSelected
                                    ? "border-primary bg-primary/10"
                                    : "border-border bg-card"
                                }`}
                              >
                                <Text
                                  className={`text-xs font-semibold ${
                                    isSelected
                                      ? "text-primary"
                                      : "text-foreground"
                                  }`}
                                >
                                  {t(option.key)}
                                </Text>
                              </Pressable>
                            );
                          })}
                        </View>
                      </View>
                    )}
                  </View>
                </Card>
              );
            })}
            <Button
              title={t("teacher_save_attendance")}
              onPress={() => void saveAttendance()}
              loading={mutation.isPending}
              disabled={markedCount === 0 || attendanceQuery.isPending}
              fullWidth
            />
          </>
        )}
      </View>
    </StudentRecordLayout>
  );
}
