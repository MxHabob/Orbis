import { Label } from "@orbis/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@orbis/ui/select";
import { useQuery } from "@tanstack/react-query";
import { Controller } from "react-hook-form";
import { useI18n } from "@/locales/client";
import { useTRPC } from "@/ipc/query-adapter";
import { type GradeImportFormData, useCsvContext } from "../shared/context";
import { FieldRow } from "../shared/field-row";

export function GradeFieldMapping() {
  const t = useI18n();
  const { setValue, control, watch } = useCsvContext<GradeImportFormData>();
  const trpc = useTRPC();

  const academicYearId = watch("academicYearId");
  const classId = watch("classId");
  const subjectId = watch("subjectId");
  const termId = watch("termId");
  const month = watch("month");

  // --- High Performance Auto-Selection logic using React Query's select/onSuccess patterns ---
  // Instead of useEffect (which causes extra renders), we use default values and state management

  const { data: years } = useQuery(
    trpc.academic.getYears.queryOptions(undefined),
  );

  const { data: classes } = useQuery(
    trpc.classes.getAll.queryOptions(
      {
        academicYearId: academicYearId ?? undefined,
      },
      {
        enabled: !!academicYearId,
      },
    ),
  );

  const { data: subjects } = useQuery(
    trpc.subjects.getAll.queryOptions(undefined),
  );

  const { data: terms } = useQuery(
    trpc.academic.getTerms.queryOptions(
      {
        yearId: academicYearId!,
      },
      {
        enabled: !!academicYearId,
      },
    ),
  );

  const { data: assessments } = useQuery(
    trpc.assessments.getByClass.queryOptions(
      {
        classId: classId!,
        subjectId: subjectId,
        termId: termId,
        month: month,
      },
      { enabled: !!classId },
    ),
  );

  return (
    <div className="mt-6">
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 border-b pb-6 mb-6">
        {/* Row 1: Year & Class */}
        <div className="space-y-2">
          <Label className="text-[#878787] text-xs font-normal">
            Academic Year
          </Label>
          <Controller
            control={control}
            name="academicYearId"
            render={({ field }) => (
              <Select
                value={field.value?.toString()}
                onValueChange={(val) => {
                  const numVal = val ? Number(val) : undefined;
                  field.onChange(numVal);
                  setValue("classId", undefined);
                  setValue("termId", undefined);
                }}
              >
                <SelectTrigger className="w-full h-9">
                  <SelectValue placeholder="Select Academic Year" />
                </SelectTrigger>
                <SelectContent>
                  {years?.map((year) => (
                    <SelectItem key={year.id} value={year.id.toString()}>
                      {year.yearName} {year.isCurrent ? "(Current)" : ""}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            )}
          />
        </div>

        <div className="space-y-2">
          <Label className="text-[#878787] text-xs font-normal">
            Class / Section
          </Label>
          <Controller
            control={control}
            name="classId"
            render={({ field }) => (
              <Select
                value={field.value?.toString()}
                onValueChange={(val) =>
                  field.onChange(val ? Number(val) : undefined)
                }
                disabled={!academicYearId}
              >
                <SelectTrigger className="w-full h-9">
                  <SelectValue placeholder="Select Class" />
                </SelectTrigger>
                <SelectContent>
                  {classes?.data?.map((cls) => (
                    <SelectItem key={cls.id} value={cls.id.toString()}>
                      {cls.name} {cls.division ? `- ${cls.division}` : ""}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            )}
          />
        </div>

        {/* Row 2: Subject & Term */}
        <div className="space-y-2">
          <Label className="text-[#878787] text-xs font-normal">Subject</Label>
          <Controller
            control={control}
            name="subjectId"
            render={({ field }) => (
              <Select
                value={field.value?.toString()}
                onValueChange={(val) =>
                  field.onChange(val ? Number(val) : undefined)
                }
              >
                <SelectTrigger className="w-full h-9">
                  <SelectValue placeholder="Select Subject" />
                </SelectTrigger>
                <SelectContent>
                  {subjects?.map((s) => (
                    <SelectItem key={s.id} value={s.id.toString()}>
                      {s.name}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            )}
          />
        </div>

        <div className="space-y-2">
          <Label className="text-[#878787] text-xs font-normal">Term</Label>
          <Controller
            control={control}
            name="termId"
            render={({ field }) => (
              <Select
                value={field.value?.toString()}
                onValueChange={(val) =>
                  field.onChange(val ? Number(val) : undefined)
                }
                disabled={!academicYearId}
              >
                <SelectTrigger className="w-full h-9">
                  <SelectValue placeholder="Select Term" />
                </SelectTrigger>
                <SelectContent>
                  {terms?.map((t) => (
                    <SelectItem key={t.id} value={t.id.toString()}>
                      {t.name}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            )}
          />
        </div>

        <div className="space-y-2 md:col-span-1">
          <Label className="text-[#878787] text-xs font-normal">
            Import Month
          </Label>
          <Controller
            control={control}
            name="month"
            render={({ field }) => (
              <Select
                value={field.value?.toString()}
                onValueChange={(val) =>
                  field.onChange(val ? Number(val) : undefined)
                }
              >
                <SelectTrigger className="w-full h-9">
                  <SelectValue placeholder="Select Month" />
                </SelectTrigger>
                <SelectContent>
                  {[
                    { value: 1, label: "محرم (1)" },
                    { value: 2, label: "صفر (2)" },
                    { value: 3, label: "ربيع أول (3)" },
                    { value: 4, label: "ربيع ثاني - نصفي (4)" },
                    { value: 5, label: "جمادى أول (5)" },
                    { value: 6, label: "جمادى ثاني (6)" },
                    { value: 7, label: "رجب (7)" },
                    { value: 8, label: "شعبان - نهائي (8)" },
                  ].map((m) => (
                    <SelectItem key={m.value} value={m.value.toString()}>
                      {m.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            )}
          />
        </div>

        <div className="space-y-2 md:col-span-1">
          <Label className="text-[#878787] text-xs font-normal">
            {t("grades.identifier_type")}
          </Label>
          <Controller
            control={control}
            name="identifierType"
            render={({ field }) => (
              <Select value={field.value} onValueChange={field.onChange}>
                <SelectTrigger className="w-full h-9">
                  <SelectValue placeholder={t("grades.identifier_type")} />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="studentNumber">
                    {t("grades.student_number_standard")}
                  </SelectItem>
                  <SelectItem value="secretNumber">
                    {t("grades.secret_number_control")}
                  </SelectItem>
                </SelectContent>
              </Select>
            )}
          />
        </div>
      </div>

      {classId && subjectId && month ? (
        <div className="grid grid-cols-2 gap-x-4 gap-y-2">
          <div className="col-span-2 mb-2">
            <div className="text-sm font-medium">{t("reports.template")}</div>
          </div>
          <FieldRow
            field="studentNumber"
            label={
              watch("identifierType") === "secretNumber"
                ? t("grades.identifier_col_label_secret")
                : t("grades.identifier_col_label_standard")
            }
            required
          />
          {assessments?.map((assessment) => (
            <FieldRow
              key={assessment.id}
              field={`mappings.id_${assessment.id}`}
              label={assessment.title}
              required={false}
            />
          ))}
        </div>
      ) : (
        <div className="text-center py-8 text-muted-foreground border-2 border-dashed rounded-lg">
          Please select a class, subject and month to proceed with grade
          mapping.
        </div>
      )}
    </div>
  );
}
