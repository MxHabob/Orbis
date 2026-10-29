import { Card } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { EmptyState } from "@/components/ui/empty-state";
import { StudentRecordLayout } from "@/components/student-record-layout";
import {
  useBatchMarkAttendanceMutation,
  useClassAttendanceQuery,
  useClassScheduleQuery,
  useTeacherClassDataQuery,
  type StudentScheduleItem,
  type TeacherRosterEntry,
} from "@/hooks/use-api-queries";
import { useEffectiveContext } from "@/providers/effective-context";
import { useI18n } from "@/lib/i18n";
import type { TranslationKey } from "@/lib/i18n/types";
import type { AttendanceDto } from "@/lib/api/types";
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

function isValidDate(value: string) {
  if (!/^\d{4}-\d{2}-\d{2}$/.test(value)) return false;
  const date = new Date(`${value}T00:00:00Z`);
  return (
    !Number.isNaN(date.getTime()) && date.toISOString().slice(0, 10) === value
  );
}

function shiftDate(value: string, amount: number) {
  const [year, month, day] = value.split("-").map(Number);
  const date = new Date(year, month - 1, day);
  date.setDate(date.getDate() + amount);
  return getLocalDate(date);
}

export default function ClassAttendanceScreen() {
  const {
    id,
    lessonId,
    date: requestedDate,
  } = useLocalSearchParams<{
    id: string;
    lessonId?: string;
    date?: string;
  }>();
  const classId = typeof id === "string" ? id : undefined;
  const initialDate =
    typeof requestedDate === "string" && isValidDate(requestedDate)
      ? requestedDate
      : getLocalDate(new Date());
  const [date, setDate] = useState(initialDate);
  const [selectedLessonId, setSelectedLessonId] = useState(
    typeof lessonId === "string" ? lessonId : "",
  );
  const [selectedStatuses, setSelectedStatuses] = useState<
    Record<string, string>
  >({});
  const classQuery = useTeacherClassDataQuery(classId);
  const scheduleQuery = useClassScheduleQuery(classId);
  const attendanceQuery = useClassAttendanceQuery(
    classId,
    date,
    classQuery.isInScope,
  );
  const mutation = useBatchMarkAttendanceMutation();
  const { hasPermission } = useEffectiveContext();
  const { t, isArabic, formatDate } = useI18n();
  const toast = useToast();
  const canRecord = hasPermission("attendance:write");
  const weekday = new Date(`${date}T12:00:00`).getDay() + 1;
  const lessonsToday = useMemo(
    () =>
      (scheduleQuery.data ?? []).filter(
        (item: StudentScheduleItem) =>
          item.timeSlot.day_of_week === weekday && !item.timeSlot.is_break,
      ),
    [scheduleQuery.data, weekday],
  );
  const selectedLesson =
    lessonsToday.find(({ lesson }) => lesson.id === selectedLessonId) ??
    (lessonsToday.length === 1 ? lessonsToday[0] : undefined);
  const activeLessonId = selectedLesson?.lesson.id;
  const recordedByStudent = useMemo(
    () =>
      new Map(
        (attendanceQuery.data ?? [])
          .filter(
            (record: AttendanceDto) =>
              record.scheduled_lesson_id === activeLessonId &&
              record.section_subject_id ===
                selectedLesson?.lesson.section_subject_id,
          )
          .map((record: AttendanceDto) => [record.student_id, record]),
      ),
    [activeLessonId, attendanceQuery.data, selectedLesson],
  );
  const roster = classQuery.data?.roster ?? [];
  const markedCount = roster.filter(({ student }: TeacherRosterEntry) => {
    const status = selectedStatuses[student.id];
    const existingStatus = recordedByStudent
      .get(student.id)
      ?.status.toLowerCase();
    return status && status !== existingStatus;
  }).length;
  const isPending =
    classQuery.isInScope &&
    (classQuery.isPending ||
      scheduleQuery.isPending ||
      attendanceQuery.isPending);
  const isError =
    !classQuery.isInScope ||
    classQuery.isError ||
    scheduleQuery.isError ||
    attendanceQuery.isError;

  const saveAttendance = async () => {
    if (!classId || !canRecord || !selectedLesson) return;
    const items = roster.flatMap(({ student }: TeacherRosterEntry) => {
      const status = selectedStatuses[student.id];
      const existingStatus = recordedByStudent
        .get(student.id)
        ?.status.toLowerCase();
      if (!status || status === existingStatus) return [];
      return [
        {
          student_id: student.id,
          status,
          section_subject_id: selectedLesson.lesson.section_subject_id,
          scheduled_lesson_id: selectedLesson.lesson.id,
        },
      ];
    });
    if (items.length === 0) return;
    try {
      await mutation.mutateAsync({ section_id: classId, date, items });
      setSelectedStatuses({});
      await attendanceQuery.refetch();
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
      canRetry={classQuery.isInScope}
      errorDescription={
        classQuery.isInScope ? undefined : t("class_access_denied")
      }
      onRetry={() => {
        void classQuery.refetch();
        void scheduleQuery.refetch();
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
              setSelectedLessonId("");
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
              setSelectedLessonId("");
              setSelectedStatuses({});
            }}
            className="rounded-md border border-border bg-card px-three py-two"
          >
            <Text className="text-sm font-semibold text-foreground">›</Text>
          </Pressable>
        </View>
        {!canRecord ? (
          <EmptyState title={t("teacher_no_permission")} />
        ) : scheduleQuery.data && lessonsToday.length === 0 ? (
          <EmptyState
            title={t("teacher_no_lesson_today")}
            description={t("teacher_no_lesson_today_hint")}
          />
        ) : lessonsToday.length > 1 && !selectedLesson ? (
          <View className="gap-three">
            <Text className="text-sm font-semibold text-foreground">
              {t("teacher_select_lesson")}
            </Text>
            {lessonsToday.map(({ lesson, timeSlot, subject }) => {
              const isSelected = selectedLessonId === lesson.id;
              const subjectName = isArabic
                ? subject?.name_ar || subject?.name
                : subject?.name || subject?.name_ar;
              return (
                <Pressable
                  key={lesson.id}
                  accessibilityRole="button"
                  accessibilityState={{ selected: isSelected }}
                  onPress={() => {
                    setSelectedLessonId(lesson.id);
                    setSelectedStatuses({});
                  }}
                  className={`min-h-12 rounded-md border px-four py-three ${
                    isSelected
                      ? "border-primary bg-primary/10"
                      : "border-border bg-card"
                  }`}
                >
                  <Text className="text-sm font-semibold text-foreground">
                    {subjectName || t("domain_schedule")}
                  </Text>
                  <Text className="mt-one text-xs text-muted-foreground">
                    {timeSlot.start_time} – {timeSlot.end_time}
                  </Text>
                </Pressable>
              );
            })}
          </View>
        ) : !selectedLesson ? (
          <EmptyState
            title={t("teacher_select_lesson")}
            description={t("teacher_select_lesson_hint")}
          />
        ) : roster.length === 0 ? (
          <EmptyState title={t("teacher_no_students")} />
        ) : (
          <>
            <View className="gap-one">
              <Text className="text-base font-semibold text-foreground">
                {isArabic
                  ? selectedLesson.subject?.name_ar ||
                    selectedLesson.subject?.name
                  : selectedLesson.subject?.name ||
                    selectedLesson.subject?.name_ar}
              </Text>
              <Text className="text-xs text-muted-foreground">
                {selectedLesson.timeSlot.start_time} –{" "}
                {selectedLesson.timeSlot.end_time}
              </Text>
            </View>
            {roster.map(({ student, enrollment }: TeacherRosterEntry) => {
              const record = recordedByStudent.get(student.id);
              const selectedStatus =
                selectedStatuses[student.id] ?? record?.status.toLowerCase();
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
                        {t("teacher_already_recorded")}
                      </Text>
                    ) : null}
                    <View className="gap-two">
                      <Text className="text-xs text-muted-foreground">
                        {t("teacher_select_status")}
                      </Text>
                      <View className="flex-row flex-wrap gap-two">
                        {attendanceOptions.map((option) => {
                          const isSelected = selectedStatus === option.value;
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
                              className={`min-h-11 items-center justify-center rounded-md border px-three py-two ${
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
                  </View>
                </Card>
              );
            })}
            <Button
              title={t("teacher_save_attendance")}
              onPress={() => void saveAttendance()}
              loading={mutation.isPending}
              disabled={
                markedCount === 0 ||
                attendanceQuery.isPending ||
                !selectedLesson
              }
              fullWidth
            />
          </>
        )}
      </View>
    </StudentRecordLayout>
  );
}
