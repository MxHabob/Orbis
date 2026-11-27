import { Button } from "@/components/ui/button";
import { EmptyState } from "@/components/ui/empty-state";
import { Input } from "@/components/ui/input";
import { StudentRecordLayout } from "@/components/student-record-layout";
import {
  useBatchRecordGradesMutation,
  useClassGradesQuery,
  useTeacherClassDataQuery,
} from "@/hooks/use-api-queries";
import { api } from "@/lib/api/client";
import { queryKeys } from "@/lib/api/query-keys";
import { useEffectiveContext } from "@/providers/effective-context";
import { useI18n } from "@/lib/i18n";
import { useToast } from "@/components/ui/toast";
import type { GradeDto, SectionSubjectDto, SubjectDto } from "@/lib/api/types";
import type { TeacherRosterEntry } from "@/hooks/use-api-queries";
import { useLocalSearchParams } from "expo-router";
import { useQuery } from "@tanstack/react-query";
import { useMemo, useState } from "react";
import { Pressable, ScrollView, View } from "react-native";
import { Text } from "@/components/ui/text";

type MatrixComponent = {
  code: string;
  name: string;
  name_ar?: string | null;
  max_score: number;
  sort_order: number;
};

function extractSchemeComponents(
  scheme: Record<string, unknown> | null | undefined,
): MatrixComponent[] {
  if (!scheme) return [];
  const raw: unknown[] =
    Array.isArray(scheme.components) && scheme.components.length > 0
      ? (scheme.components as unknown[])
      : Array.isArray(
            (scheme.formula as Record<string, unknown> | null)?.components,
          )
        ? (((scheme.formula as Record<string, unknown>)
            .components as unknown[]) ?? [])
        : [];
  if (raw.length === 0) return [];
  return raw
    .map((c, i) => {
      const comp = (typeof c === "object" && c !== null ? c : {}) as Record<
        string,
        unknown
      >;
      return {
        code: String(comp.code ?? comp.id ?? `comp-${i}`),
        name: String(comp.name ?? ""),
        name_ar:
          typeof comp.name_ar === "string"
            ? comp.name_ar
            : typeof comp.nameAr === "string"
              ? comp.nameAr
              : null,
        max_score: Number(comp.maxScore ?? comp.max_score ?? 100) || 100,
        sort_order: Number(comp.sortOrder ?? comp.sort_order ?? i + 1) || i + 1,
      } satisfies MatrixComponent;
    })
    .sort((a, b) => a.sort_order - b.sort_order);
}

function findSchemeForSectionSubject(
  schemes: Record<string, unknown>[],
  sectionSubject: SectionSubjectDto | undefined,
): Record<string, unknown> | null {
  if (!schemes.length) return null;
  const schemeId = (sectionSubject as any)?.scheme_id;
  if (schemeId) {
    const byId = schemes.find((s) => String(s.id) === String(schemeId));
    if (byId) return byId;
  }
  return (
    schemes.find(
      (s) =>
        String(s.level) === "monthly_period" || String(s.level) === "monthly",
    ) ??
    schemes.find((s) => s.is_default === true) ??
    schemes[0] ??
    null
  );
}

function gradeComponentKey(grade: GradeDto): string {
  const anyGrade = grade as any;
  return String(anyGrade.component_code ?? anyGrade.component_id ?? "__score");
}

export default function ClassGradesScreen() {
  const { id } = useLocalSearchParams<{ id: string }>();
  const classId = typeof id === "string" ? id : undefined;
  const [selectedSubjectId, setSelectedSubjectId] = useState("");
  const [scores, setScores] = useState<Record<string, string>>({});
  const [saveError, setSaveError] = useState<string | null>(null);
  const classQuery = useTeacherClassDataQuery(classId);
  const classSubjects = useMemo(
    () => classQuery.data?.SectionSubject ?? [],
    [classQuery.data?.SectionSubject],
  );

  const resolvedSelectedSubjectId = useMemo(() => {
    if (classSubjects.length === 0) return "";
    return classSubjects.some(
      (item: SectionSubjectDto) => item.id === selectedSubjectId,
    )
      ? selectedSubjectId
      : classSubjects[0].id;
  }, [classSubjects, selectedSubjectId]);

  const selectedClassSubject =
    classSubjects.find(
      (item: SectionSubjectDto) => item.id === resolvedSelectedSubjectId,
    ) ?? classSubjects[0];
  const gradesQuery = useClassGradesQuery(
    selectedClassSubject?.id,
    classQuery.isInScope,
  );
  const schemesQuery = useQuery({
    queryKey: queryKeys.assessment.schemes(),
    queryFn: async () => {
      const response = await (api as any).listSchemes({ throwOnError: true });
      let data: unknown = response;
      while (
        typeof data === "object" &&
        data !== null &&
        "data" in (data as any)
      ) {
        data = (data as any).data;
      }
      return Array.isArray(data) ? (data as Record<string, unknown>[]) : [];
    },
    enabled: classQuery.isInScope && Boolean(selectedClassSubject),
    staleTime: 1000 * 60 * 5,
  });
  const mutation = useBatchRecordGradesMutation();
  const { hasPermission } = useEffectiveContext();
  const { t, isArabic } = useI18n();
  const toast = useToast();
  const canRecord = hasPermission("grade:write");
  const classItem = classQuery.data?.classItem;

  const matrixComponents = useMemo<MatrixComponent[]>(() => {
    const scheme = findSchemeForSectionSubject(
      schemesQuery.data ?? [],
      selectedClassSubject,
    );
    const comps = extractSchemeComponents(scheme);
    if (comps.length > 0) return comps;
    return [
      {
        code: "__score",
        name: t("teacher_score"),
        name_ar: t("teacher_score"),
        max_score: 100,
        sort_order: 1,
      },
    ];
  }, [schemesQuery.data, selectedClassSubject, t]);

  const existingGradesByStudent = useMemo(() => {
    const map = new Map<string, Map<string, GradeDto>>();
    for (const grade of gradesQuery.data ?? []) {
      if (grade.period_id !== classItem?.period_id) continue;
      const studentId = String(grade.student_id);
      const compKey = gradeComponentKey(grade);
      if (!map.has(studentId)) map.set(studentId, new Map());
      const byComp = map.get(studentId)!;
      // prefer exact component match; for single-score fallback keep first
      if (!byComp.has(compKey)) byComp.set(compKey, grade);
      if (compKey === "__score" && !byComp.has("__score"))
        byComp.set("__score", grade);
    }
    return map;
  }, [gradesQuery.data, classItem]);

  const getCellKey = (studentId: string, componentCode: string) =>
    `${studentId}::${componentCode}`;
  const getInputValue = (studentId: string, componentCode: string) =>
    scores[getCellKey(studentId, componentCode)] ?? "";

  const isPending =
    classQuery.isInScope &&
    (classQuery.isPending ||
      (Boolean(selectedClassSubject) && gradesQuery.isPending));
  const isError =
    !classQuery.isInScope ||
    classQuery.isError ||
    (Boolean(selectedClassSubject) && gradesQuery.isError);

  const saveGrades = async () => {
    if (!selectedClassSubject || !classItem || !canRecord) return;
    setSaveError(null);
    const invalidStudentIds: string[] = [];
    const items = (classQuery.data?.roster ?? []).flatMap(
      ({ student, enrollment }: TeacherRosterEntry) => {
        const existingByComp =
          existingGradesByStudent.get(String(student.id)) ??
          new Map<string, GradeDto>();
        return matrixComponents.flatMap((comp) => {
          const raw = getInputValue(String(student.id), comp.code).trim();
          if (!raw) return [];
          if (
            existingByComp.has(comp.code) ||
            (comp.code === "__score" &&
              existingByComp.size > 0 &&
              existingByComp.has("__score"))
          ) {
            return [];
          }
          // if fallback single-score mode and there is any existing grade for student in period, treat as recorded
          if (
            comp.code === "__score" &&
            matrixComponents.length === 1 &&
            existingByComp.size > 0
          )
            return [];
          const numericScore = Number(raw);
          if (
            !Number.isFinite(numericScore) ||
            numericScore < 0 ||
            numericScore > comp.max_score
          ) {
            invalidStudentIds.push(String(student.id));
            return [];
          }
          if (!classItem.period_id) {
            invalidStudentIds.push(String(student.id));
            return [];
          }
          return [
            {
              period_id: classItem.period_id,
              student_id: String(student.id),
              enrollment_id: enrollment.id,
              section_subject_id: selectedClassSubject.id,
              component_code: comp.code === "__score" ? undefined : comp.code,
              score: String(numericScore),
              max_score: String(comp.max_score),
            },
          ];
        });
      },
    );
    if (invalidStudentIds.length > 0) {
      setSaveError(t("teacher_invalid_scores"));
      return;
    }
    if (items.length === 0) {
      setSaveError(t("teacher_no_records_to_save"));
      return;
    }
    try {
      await mutation.mutateAsync({ items: items as any });
      setScores({});
      toast.success(t("teacher_save_success"));
    } catch (error) {
      setSaveError(error instanceof Error ? error.message : t("common_error"));
    }
  };

  const studentColWidth = 180;
  const scoreColWidth = 112;

  return (
    <StudentRecordLayout
      title={t("teacher_record_grades")}
      isPending={isPending}
      isError={!classId || isError}
      canRetry={classQuery.isInScope}
      errorDescription={
        classQuery.isInScope ? undefined : t("class_access_denied")
      }
      onRetry={() => {
        void classQuery.refetch();
        void gradesQuery.refetch();
        void schemesQuery.refetch();
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
          <ScrollView
            horizontal
            showsHorizontalScrollIndicator={false}
            contentContainerClassName="gap-two pe-three"
          >
            {classSubjects.map((classSubject: SectionSubjectDto) => {
              const subject = classQuery.data?.subjects.find(
                (item: SubjectDto) => item.id === classSubject.subject_id,
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
                    setSaveError(null);
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
          </ScrollView>
          {classQuery.data?.roster.length === 0 ? (
            <EmptyState title={t("teacher_no_students")} />
          ) : (
            <>
              <ScrollView horizontal showsHorizontalScrollIndicator>
                <View className="overflow-hidden rounded-xl border border-border bg-card">
                  <View className="flex-row items-center border-b border-border bg-muted/40 px-three py-two">
                    <Text className="w-8 text-xs font-semibold text-muted-foreground">
                      #
                    </Text>
                    <View style={{ width: studentColWidth }}>
                      <Text className="text-xs font-semibold text-muted-foreground">
                        {isArabic ? "الطالب" : "Student"}
                      </Text>
                    </View>
                    {matrixComponents.map((comp) => (
                      <View
                        key={comp.code}
                        style={{ width: scoreColWidth }}
                        className="items-end"
                      >
                        <Text
                          className="text-xs font-semibold text-muted-foreground"
                          numberOfLines={1}
                        >
                          {(isArabic ? comp.name_ar || comp.name : comp.name) ||
                            comp.code}
                        </Text>
                        <Text className="text-[11px] text-muted-foreground">
                          {comp.max_score}
                        </Text>
                      </View>
                    ))}
                  </View>
                  {classQuery.data?.roster.map(
                    (
                      { student, enrollment }: TeacherRosterEntry,
                      index: number,
                    ) => {
                      const studentId = String(student.id);
                      const existingByComp =
                        existingGradesByStudent.get(studentId) ??
                        new Map<string, GradeDto>();
                      const studentName = isArabic
                        ? student.name_ar || student.full_name
                        : student.display_name || student.full_name;
                      const isLastRow =
                        index === (classQuery.data?.roster.length ?? 0) - 1;
                      return (
                        <View
                          key={student.id}
                          className={`flex-row items-center gap-two px-three py-two ${
                            isLastRow ? "" : "border-b border-border"
                          }`}
                        >
                          <Text className="w-8 text-xs text-muted-foreground">
                            {index + 1}
                          </Text>
                          <View style={{ width: studentColWidth }}>
                            <Text
                              className="text-sm font-medium text-foreground"
                              numberOfLines={1}
                            >
                              {studentName}
                            </Text>
                            <Text
                              className="text-xs text-muted-foreground"
                              numberOfLines={1}
                            >
                              {enrollment.student_number}
                            </Text>
                          </View>
                          {matrixComponents.map((comp) => {
                            const existing =
                              existingByComp.get(comp.code) ??
                              (comp.code === "__score"
                                ? Array.from(existingByComp.values())[0]
                                : undefined);
                            return (
                              <View
                                key={comp.code}
                                style={{ width: scoreColWidth }}
                                className="items-end"
                              >
                                {existing ? (
                                  <View className="items-end">
                                    <Text className="text-sm font-semibold text-primary">
                                      {existing.score} / {existing.max_score}
                                    </Text>
                                  </View>
                                ) : (
                                  <Input
                                    value={getInputValue(studentId, comp.code)}
                                    onChangeText={(value) => {
                                      setSaveError(null);
                                      setScores((current) => ({
                                        ...current,
                                        [getCellKey(studentId, comp.code)]:
                                          value,
                                      }));
                                    }}
                                    keyboardType="decimal-pad"
                                    inputMode="decimal"
                                    maxLength={6}
                                    placeholder={`0-${comp.max_score}`}
                                    containerStyle={{
                                      width: scoreColWidth - 8,
                                    }}
                                    accessibilityLabel={`${studentName}, ${isArabic ? comp.name_ar || comp.name : comp.name || t("teacher_score")}`}
                                  />
                                )}
                              </View>
                            );
                          })}
                        </View>
                      );
                    },
                  )}
                </View>
              </ScrollView>
              {classQuery.data?.roster.every(
                ({ student }: TeacherRosterEntry) => {
                  const byComp = existingGradesByStudent.get(
                    String(student.id),
                  );
                  if (!byComp || byComp.size === 0) return false;
                  if (
                    matrixComponents.length === 1 &&
                    matrixComponents[0].code === "__score"
                  )
                    return byComp.size > 0;
                  return matrixComponents.every((c) => byComp.has(c.code));
                },
              ) ? (
                <Text className="text-sm text-muted-foreground">
                  {t("teacher_no_grade_entries")}
                </Text>
              ) : null}
              {saveError ? (
                <Text
                  accessibilityRole="alert"
                  className="text-start text-xs leading-4 font-medium text-destructive"
                >
                  {saveError}
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
