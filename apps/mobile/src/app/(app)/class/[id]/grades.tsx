import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { EmptyState } from "@/components/ui/empty-state";
import { Input } from "@/components/ui/input";
import { StudentRecordLayout } from "@/components/student-record-layout";
import {
  useBatchRecordGradesMutation,
  useClassGradesQuery,
  useTeacherClassDataQuery,
} from "@/hooks/use-api-queries";
import { useEffectiveContext } from "@/providers/effective-context";
import { useI18n } from "@/lib/i18n";
import { useToast } from "@/components/ui/toast";
import { useLocalSearchParams } from "expo-router";
import { useMemo, useState } from "react";
import { Pressable, Text, View } from "react-native";

export default function ClassGradesScreen() {
  const { id } = useLocalSearchParams<{ id: string }>();
  const classId = typeof id === "string" ? id : undefined;
  const [selectedSubjectId, setSelectedSubjectId] = useState("");
  const [scores, setScores] = useState<Record<string, string>>({});
  const classQuery = useTeacherClassDataQuery(classId);
  const classSubjects = classQuery.data?.SectionSubject ?? [];
  const selectedClassSubject =
    classSubjects.find((item) => item.id === selectedSubjectId) ??
    classSubjects[0];
  const gradesQuery = useClassGradesQuery(selectedClassSubject?.id);
  const mutation = useBatchRecordGradesMutation();
  const { hasPermission } = useEffectiveContext();
  const { t, isArabic, formatDate } = useI18n();
  const toast = useToast();
  const canRecord = hasPermission("grade:write");
  const existingGrades = useMemo(
    () =>
      new Map(
        (gradesQuery.data ?? [])
          .filter(
            (grade) =>
              grade.period_id === classQuery.data?.classItem.period_id,
          )
          .map((grade) => [grade.student_id, grade]),
      ),
    [gradesQuery.data, classQuery.data?.classItem.period_id],
  );


  const isPending =
    classQuery.isPending ||
    Boolean(selectedClassSubject) && gradesQuery.isPending;
  const isError =
    classQuery.isError ||
    Boolean(selectedClassSubject) && gradesQuery.isError;

  const saveGrades = async () => {
    const classItem = classQuery.data?.classItem;
    if (!selectedClassSubject || !classItem || !canRecord) return;
    const invalidStudentIds: string[] = [];
    const items = (classQuery.data?.roster ?? []).flatMap(
      ({ student, enrollment }) => {
      const score = scores[student.id]?.trim();
      if (!score || existingGrades.has(student.id)) return [];
      const numericScore = Number(score);
      if (!Number.isFinite(numericScore) || numericScore < 0 || numericScore > 100) {
          invalidStudentIds.push(student.id);
          return [];
      }
      return [
          {
            period_id: classItem.period_id,
            student_id: student.id,
            enrollment_id: enrollment.id,
            section_subject_id: selectedClassSubject.id,
            score: String(numericScore),
            max_score: "100",
          },
      ];
      },
    );
    if (invalidStudentIds.length > 0) {
      toast.error(t("teacher_invalid_scores"));
      return;
    }
    if (items.length === 0) {
      toast.warning(t("teacher_no_records_to_save"));
      return;
    }
    try {
      await mutation.mutateAsync({ items });
      setScores({});
      toast.success(t("teacher_save_success"));
    } catch (error) {
      toast.error(error instanceof Error ? error.message : t("common_error"));
    }
  };

  return (
    <StudentRecordLayout
      title={t("teacher_record_grades")}
      isPending={isPending}
      isError={!classId || isError}
      onRetry={() => {
        void classQuery.refetch();
        void gradesQuery.refetch();
      }}
    >
      {!canRecord ? (
        <EmptyState title={t("teacher_no_permission")} />
      ) : classSubjects.length === 0 ? (
        <EmptyState title={t("teacher_no_subjects")} />
      ) : (
        <View className="gap-three">
          <Text className="text-sm font-semibold text-foreground">
            {t("teacher_select_subject")}
          </Text>
          <View className="flex-row flex-wrap gap-two">
            {classSubjects.map((classSubject) => {
              const subject = classQuery.data?.subjects.find(
                (item) => item.id === classSubject.subject_id,
              );
              const label = subject
                ? isArabic
                  ? subject.name_ar || subject.name
                  : subject.name
                : t("teacher_select_subject");
              const isSelected = classSubject.id === selectedClassSubject?.id;
              return (
                <Pressable
                  key={classSubject.id}
                  accessibilityRole="button"
                  accessibilityState={{ selected: isSelected }}
                  onPress={() => {
                    setSelectedSubjectId(classSubject.id);
                    setScores({});
                  }}
                  className={`min-h-10 items-center justify-center rounded-md border px-three py-two ${
                    isSelected
                      ? "border-primary bg-primary/10"
                      : "border-border bg-card"
                  }`}
                >
                  <Text
                    className={`text-sm font-semibold ${
                      isSelected ? "text-primary" : "text-foreground"
                    }`}
                  >
                    {label}
                  </Text>
                </Pressable>
              );
            })}
          </View>
          {classQuery.data?.roster.length === 0 ? (
            <EmptyState title={t("teacher_no_students")} />
          ) : (
            <>
              {classQuery.data?.roster.map(({ student, enrollment }) => {
                const grade = existingGrades.get(student.id);
                const studentName = isArabic
                  ? student.name_ar || student.full_name
                  : student.display_name || student.full_name;
                return (
                  <Card key={student.id} variant="outline" padding="three">
                    <View className="gap-two">
                      <View className="flex-row items-center justify-between gap-two">
                        <Text className="flex-1 text-sm font-semibold text-foreground">
                          {studentName}
                        </Text>
                        <Text className="text-xs text-muted-foreground">
                          {enrollment.student_number}
                        </Text>
                      </View>
                      {grade ? (
                        <View className="flex-row items-center justify-between gap-two">
                          <Text className="text-xs text-muted-foreground">
                            {t("teacher_grade_already_recorded")}
                          </Text>
                          <Text className="text-sm font-semibold text-primary">
                            {grade.score} / {grade.max_score}
                            {grade.created_at
                              ? ` · ${formatDate(grade.created_at)}`
                              : ""}
                          </Text>
                        </View>
                      ) : (
                        <Input
                          label={t("teacher_score")}
                          value={scores[student.id] ?? ""}
                          onChangeText={(value) =>
                            setScores((current) => ({
                              ...current,
                              [student.id]: value,
                            }))
                          }
                          keyboardType="decimal-pad"
                          inputMode="decimal"
                          maxLength={6}
                          placeholder="0–100"
                          accessibilityLabel={`${studentName}, ${t("teacher_score")}`}
                        />
                      )}
                    </View>
                  </Card>
                );
              })}
              {classQuery.data?.roster.every(({ student }) =>
                existingGrades.has(student.id),
              ) ? (
                <Text className="text-sm text-muted-foreground">
                  {t("teacher_no_grade_entries")}
                </Text>
              ) : null}
              <Button
                title={t("teacher_save_grades")}
                onPress={() => void saveGrades()}
                loading={mutation.isPending}
                disabled={
                  mutation.isPending ||
                  !Object.values(scores).some((score) => score.trim() !== "")
                }
                fullWidth
              />
            </>
          )}
        </View>
      )}
    </StudentRecordLayout>
  );
}
