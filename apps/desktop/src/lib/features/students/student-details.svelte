<!--
  @component StudentDetails
  Student profile, evaluation analytics dashboard, and details form.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Switch } from "@orbis/ui/switch";
import { Skeleton } from "@orbis/ui/skeleton";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { Badge } from "@orbis/ui/badge";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardAction,
  CardContent,
  CardFooter,
} from "@orbis/ui/card";
import AttendanceTrendsCard from "$lib/features/metrics/cards/attendance-trends-card.svelte";
import GradeDistributionCard from "$lib/features/metrics/cards/grade-distribution-card.svelte";
import { AcademicCap } from "@orbis/icons";
import { onMount } from "svelte";
import { toast } from "@orbis/ui";
import { fade } from "svelte/transition";

import { l } from "$lib/core/i18n";
import { createStudentQuery } from "$lib/features/students";
import { createClassesQuery } from "$lib/features/academics";
import NationalitySelect from "$lib/shared/components/nationality-select.svelte";
import {
  api,
  extractErrorMessage,
  queryKeys,
  type CalculationResultDto,
  type AttendanceDto,
  type BehaviorRecordDto,
  type ClassDto,
  type StudentDetailDto,
} from "$lib/api";
import { queryClient } from "$lib/api";
import { academicContext } from "$lib/stores";
import AuditTimeline from "$lib/shared/components/audit-timeline.svelte";

let { orgId, studentId }: { orgId: string; studentId: string } = $props();

// TanStack Queries
const studentQuery = createStudentQuery(() => studentId);
const classesQuery = createClassesQuery(() => orgId);

const studentDetail = $derived(studentQuery.data);
const student = $derived(studentDetail?.student);
const person = $derived(studentDetail?.person);
const enrollment = $derived(studentDetail?.enrollment);

const classes = $derived(classesQuery.data ?? []);
const isLoading = $derived(studentQuery.isLoading || classesQuery.isLoading);

let isSaving = $state(false);

// Form fields
let nameAr = $state("");
let nameEn = $state("");
let sex = $state("male");
let classId = $state("");
let studentNumber = $state("");
let email = $state("");
let phone = $state("");
let address = $state("");
let isActiveState = $state(true);
let nationalId = $state("");
let passportNumber = $state("");
let nationality = $state("");
let placeOfBirth = $state("");
let bloodType = $state("");
let medicalNotes = $state("");
let specialNeedsNotes = $state("");

let birthDay = $state("");
let birthMonth = $state("");
let birthYear = $state("");

const birthDate = $derived(
  birthDay && birthMonth && birthYear
    ? `${birthYear}-${birthMonth.padStart(2, "0")}-${birthDay.padStart(2, "0")}`
    : ""
);

const selectedClassName = $derived(
  (() => {
    const cls = classes.find((c: ClassDto) => String(c.id) === classId);
    return cls
      ? cls.name || String(cls.id)
      : m.common_form_select_class();
  })(),
);

// Date picker options and reactive values
const currentYear = new Date().getFullYear();
const yearsList = Array.from({ length: 47 }, (_, i) => currentYear - i);
const daysList = Array.from({ length: 31 }, (_, i) => String(i + 1));
const monthsList = $derived([
  { value: "1", label: m.common_form_month_1() },
  { value: "2", label: m.common_form_month_2() },
  { value: "3", label: m.common_form_month_3() },
  { value: "4", label: m.common_form_month_4() },
  { value: "5", label: m.common_form_month_5() },
  { value: "6", label: m.common_form_month_6() },
  { value: "7", label: m.common_form_month_7() },
  { value: "8", label: m.common_form_month_8() },
  { value: "9", label: m.common_form_month_9() },
  { value: "10", label: m.common_form_month_10() },
  { value: "11", label: m.common_form_month_11() },
  { value: "12", label: m.common_form_month_12() },
]);

// Evaluation State & Loading
let evaluationLoading = $state(true);
let gradesReport = $state<CalculationResultDto | null>(null);
let attendanceReport = $state<AttendanceDto | null>(null);
let behaviorRecords = $state<BehaviorRecordDto[]>([]);

function populateForm(detail: StudentDetailDto) {
  const p = detail.person;
  const s = detail.student;
  const e = detail.enrollment;

  nameAr = (p?.first_name && p?.last_name ? `${p.first_name} ${p.last_name}` : "") || p?.first_name_ar || p?.display_name || p?.full_name || "";
  nameEn = p?.full_name || "";
  const rawBirth = p?.birth_date ? String(p.birth_date).slice(0, 10) : "";
  if (rawBirth) {
    const parts = rawBirth.split("-");
    if (parts.length === 3) {
      birthYear = parts[0];
      birthMonth = String(parseInt(parts[1], 10));
      birthDay = String(parseInt(parts[2], 10));
    }
  }

  sex = p?.gender || "male";
  classId = String(e?.class_id || "");
  studentNumber = String(s?.student_number || "");
  email = String(p?.email || "");
  phone = String(p?.phone || "");
  address = String(p?.address || "");
  isActiveState = s?.status === "active";
  nationalId = p?.national_id || "";
  passportNumber = p?.passport_number || "";
  nationality = p?.nationality || "Saudi";
  placeOfBirth = p?.place_of_birth || "";
  bloodType = p?.blood_type || "";
  medicalNotes = p?.medical_notes || "";
  specialNeedsNotes = p?.special_needs_notes || "";
}

onMount(async () => {
  evaluationLoading = true;
  try {
    const detail = await api.getStudent(studentId);
    if (detail) {
      populateForm(detail);
    }

    const periodId =
      academicContext.effectivePeriodId || detail?.enrollment?.period_id || "";
    if (periodId && studentId) {
      const gRes = await api.calculateGpa({ student_id: studentId, period_id: periodId }).catch(() => null);
      gradesReport = gRes;
      attendanceReport = null;
      behaviorRecords = [];
    }
  } catch (e) {
    console.error("Failed to load evaluation reports:", e);
  } finally {
    evaluationLoading = false;
  }
});

// Derived performance statistics
const academicStats = $derived.by(() => {
  const list = gradesReport?.grades || [];
  if (list.length === 0)
    return { avg: 0, passed: 0, failed: 0, gradeLetter: "—" };
  let totalPct = 0;
  let passed = 0;
  let failed = 0;
  for (const g of list) {
    const max = Number(g.max_score || 100);
    const score = Number(g.score || 0);
    const pctVal = max > 0 ? (score / max) * 100 : score;
    totalPct += pctVal;
    if (pctVal >= 50) {
      passed++;
    } else {
      failed++;
    }
  }
  const avg = Math.round(totalPct / list.length);
  let letter = "F";
  if (avg >= 90) letter = "A";
  else if (avg >= 85) letter = "B+";
  else if (avg >= 80) letter = "B";
  else if (avg >= 75) letter = "C+";
  else if (avg >= 70) letter = "C";
  else if (avg >= 60) letter = "D";
  else if (avg >= 50) letter = "E";
  return { avg, passed, failed, gradeLetter: letter };
});

const behaviorStats = $derived.by(() => {
  let positive = 0;
  let negative = 0;
  let points = 100;
  for (const rec of behaviorRecords) {
    const type = String(rec.behavior_type || "").toLowerCase();
    const pts = Number(rec.points || 0);
    if (type === "positive") {
      positive++;
      points += pts;
    } else {
      negative++;
      points -= pts;
    }
  }
  return { positive, negative, points };
});

const decisionHelper = $derived.by(() => {
  const acadAvg = academicStats.avg;
  const attRate = attendanceReport ? Number(attendanceReport.rate ?? 100) : 100;
  const negBeh = behaviorStats.negative;

  const hasNoGrades = (gradesReport?.grades || []).length === 0;
  const hasNoAttendance =
    !attendanceReport ||
    ((attendanceReport.present_count ?? 0) === 0 &&
      (attendanceReport.absent_count ?? 0) === 0 &&
      (attendanceReport.late_count ?? 0) === 0);
  const hasNoBehavior = behaviorRecords.length === 0;
  const hasNoData = hasNoGrades && hasNoAttendance && hasNoBehavior;

  if (hasNoData) {
    return {
      status: "no_data",
      badge: m.academics_student_evaluation_decision_title(),
      badgeVariant: "outline" as const,
      rec: m.academics_student_evaluation_no_data(),
      colorClass: "text-muted-foreground bg-muted/25 border-border/60",
    };
  }

  if (acadAvg >= 85 && attRate >= 95 && negBeh === 0) {
    return {
      status: "excellent",
      badge: m.common_control_grade_excellent(),
      badgeVariant: "lime" as const,
      rec: m.academics_student_evaluation_rec_excellent(),
      colorClass: "text-emerald-500 bg-emerald-500/10 border-emerald-500/20",
    };
  } else if (acadAvg <= 45 || attRate <= 70 || negBeh >= 4) {
    return {
      status: "critical",
      badge: m.academics_student_evaluation_status_critical(),
      badgeVariant: "destructive" as const,
      rec: m.academics_student_evaluation_rec_critical(),
      colorClass: "text-destructive bg-destructive/10 border-destructive/20",
    };
  } else if (acadAvg < 60 || attRate < 85 || negBeh >= 2) {
    return {
      status: "warning",
      badge: m.academics_student_evaluation_status_warning(),
      badgeVariant: "secondary" as const,
      rec: m.academics_student_evaluation_rec_warning(),
      colorClass: "text-amber-500 bg-amber-500/10 border-amber-500/20",
    };
  } else {
    return {
      status: "good",
      badge: m.academics_student_evaluation_status_good(),
      badgeVariant: "default" as const,
      rec: m.academics_student_evaluation_rec_good(),
      colorClass: "text-primary bg-primary/10 border-primary/20",
    };
  }
});

async function saveStudentDetails(e: Event) {
  e.preventDefault();
  if (!studentDetail) return;
  if (!nameAr.trim()) {
    toast.error(m.common_form_error_name_required());
    return;
  }
  if (!classId) {
    toast.error(m.common_form_error_class_required());
    return;
  }
  if (!birthDate) {
    toast.error(m.common_form_error_birth_required());
    return;
  }

  isSaving = true;
  try {
    const arParts = nameAr.trim().split(/\s+/);
    const enParts = nameEn.trim().split(/\s+/);

    const firstName = enParts[0] || arParts[0] || "طالب";
    const lastName = enParts.length > 1 ? enParts[enParts.length - 1] : (arParts.length > 1 ? arParts[arParts.length - 1] : "جديد");
    const middleName = enParts.length > 2 ? enParts.slice(1, -1).join(" ") : undefined;

    const firstNameAr = arParts[0] || undefined;
    const lastNameAr = arParts.length > 1 ? arParts[arParts.length - 1] : undefined;
    const middleNameAr = arParts.length > 2 ? arParts.slice(1, -1).join(" ") : undefined;

    const targetStudentId = student?.id || studentId;

    await api.updateStudentProfile({
      student_id: targetStudentId,
      first_name: firstName,
      middle_name: middleName,
      last_name: lastName,
      first_name_ar: firstNameAr,
      middle_name_ar: middleNameAr,
      last_name_ar: lastNameAr,
      gender: sex || undefined,
      birth_date: birthDate || undefined,
      national_id: nationalId.trim() || undefined,
      passport_number: passportNumber.trim() || undefined,
      nationality: nationality.trim() || undefined,
      place_of_birth: placeOfBirth.trim() || undefined,
      blood_type: bloodType.trim() || undefined,
      medical_notes: medicalNotes.trim() || undefined,
      special_needs_notes: specialNeedsNotes.trim() || undefined,
      email: email.trim() || undefined,
      phone: phone.trim() || undefined,
      address: address.trim() || undefined,
    });

    const currentStatus = student?.status === "active";
    if (isActiveState !== currentStatus) {
      await api.updateStudentStatus({
        student_id: targetStudentId,
        status: isActiveState ? "active" : "withdrawn",
      });
    }

    toast.success(m.common_form_success_update());

    queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
    queryClient.invalidateQueries({ queryKey: queryKeys.student.detail(studentId) });
  } catch (e: unknown) {
    console.error("Save error:", e);
    toast.error(extractErrorMessage(e));
  } finally {
    isSaving = false;
  }
}
</script>

<div class="h-full w-full overflow-y-auto scrollbar-hide text-start pb-10">
  {#if isLoading}
    <div class="flex flex-col gap-6">
      <Skeleton class="h-32 w-full rounded-xl" />
      <Skeleton class="h-48 w-full rounded-xl" />
    </div>
  {:else}
    <div class="flex flex-col gap-8 w-full max-w-6xl mx-auto p-1">
      
      <!-- Evaluation Dashboard Section -->
      <div class="flex flex-col gap-4">

        {#if evaluationLoading}
          <div class="flex flex-col gap-6">
            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
              <Skeleton class="lg:col-span-2 h-65 rounded-xl" />
              <Skeleton class="h-65 rounded-xl" />
            </div>
            <Skeleton class="h-16 w-full rounded-xl" />
          </div>
        {:else}
          <div class="flex flex-col gap-6">
            {#if classId}
              <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                <!-- Left: Attendance Trends Card (2/3 width) -->
                <div class="lg:col-span-2">
                  <AttendanceTrendsCard {orgId} selectedClass={classId} />
                </div>

                <!-- Right: Grade Distribution Card (1/3 width) -->
                <div>
                  <GradeDistributionCard {orgId} selectedClass={classId} />
                </div>
              </div>
            {/if}

            <!-- Decision Helper Banner -->
            <div class="p-4 rounded-xl border flex items-center justify-between gap-4 shadow-sm {decisionHelper.colorClass}">
              <div class="flex items-center gap-3">
                <AcademicCap size={18} class="shrink-0" />
                <div class="flex flex-col gap-0.5 text-start">
                  <span class="text-xs font-bold text-foreground">{m.academics_student_evaluation_recommendation_title()}</span>
                  <p class="text-[10px] leading-relaxed font-semibold">{decisionHelper.rec}</p>
                </div>
              </div>
              <Badge variant={decisionHelper.badgeVariant} class="text-[9px] font-semibold py-0.5 px-2 rounded shrink-0">
                {decisionHelper.badge}
              </Badge>
            </div>
          </div>
        {/if}
      </div>

      <!-- Profile Form -->
      <form onsubmit={saveStudentDetails} in:fade={{ duration: 100 }} class="flex flex-col gap-6">
      
      <Card class="w-full">
        <!-- Card Header -->
        <CardHeader class="py-3 px-6">
          <CardTitle class="text-[11px] font-bold text-muted-foreground uppercase tracking-wider text-start select-none">
            {m.common_form_student_info()}
          </CardTitle>
        </CardHeader>

        <!-- Card Body -->
        <CardContent class="p-0">
          <!-- Row 1: Student Active Status -->
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
            <div class="flex flex-col gap-1 text-start">
              <span class="text-xs font-bold text-foreground">{m.students_student_detail_row_status_title()}</span>
              <span class="text-[10px] text-muted-foreground">{m.students_student_detail_row_status_desc()}</span>
            </div>
            <div class="w-full md:w-80 shrink-0 flex items-center gap-2">
              <Switch
                id="student_status_switch"
                checked={isActiveState}
                onCheckedChange={(checked) => isActiveState = checked}
              />
              <Label for="student_status_switch" class="text-xs text-muted-foreground select-none cursor-pointer">
                {isActiveState ? m.common_phase_active() : m.students_student_detail_row_status_inactive()}
              </Label>
            </div>
          </div>

          <!-- Row 2: Full Name (Arabic) -->
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
            <div class="flex flex-col gap-1 text-start">
              <span class="text-xs font-bold text-foreground">{m.common_form_full_name_ar()} *</span>
              <span class="text-[10px] text-muted-foreground">{m.students_student_detail_row_name_ar_desc()}</span>
            </div>
            <div class="w-full md:w-80 shrink-0">
              <Input type="text" class="text-xs font-semibold h-9" bind:value={nameAr} required />
            </div>
          </div>

          <!-- Row 3: Full Name (English) -->
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
            <div class="flex flex-col gap-1 text-start">
              <span class="text-xs font-bold text-foreground">{m.common_form_full_name_en()}</span>
              <span class="text-[10px] text-muted-foreground">{m.students_student_detail_row_name_en_desc()}</span>
            </div>
            <div class="w-full md:w-80 shrink-0">
              <Input type="text" class="text-xs font-semibold h-9 font-mono" bind:value={nameEn} />
            </div>
          </div>

          <!-- Row 4: Birth Date -->
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
            <div class="flex flex-col gap-1 text-start">
              <span class="text-xs font-bold text-foreground">{m.common_bulk_col_dob()} *</span>
              <span class="text-[10px] text-muted-foreground">{m.students_student_detail_row_birth_date_desc()}</span>
            </div>
            <div class="w-full md:w-80 shrink-0 grid grid-cols-3 gap-2">
              <Select bind:value={birthDay} type="single">
                <SelectTrigger class="w-full h-9 text-xs text-start flex items-center justify-between border border-input px-2">
                  <span class="truncate">{birthDay || m.common_form_day()}</span>
                </SelectTrigger>
                <SelectContent class="text-start max-h-48 overflow-y-auto" align="end">
                  {#each daysList as dy}
                    <SelectItem value={dy} class="text-xs text-start justify-start cursor-pointer">
                      {dy}
                    </SelectItem>
                  {/each}
                </SelectContent>
              </Select>

              <Select bind:value={birthMonth} type="single">
                <SelectTrigger class="w-full h-9 text-xs text-start flex items-center justify-between border border-input px-2">
                  <span class="truncate">{monthsList.find(m => m.value === birthMonth)?.label || m.common_form_month()}</span>
                </SelectTrigger>
                <SelectContent class="text-start max-h-48 overflow-y-auto" align="end">
                  {#each monthsList as mo}
                    <SelectItem value={mo.value} class="text-xs text-start justify-start cursor-pointer">
                      {mo.label}
                    </SelectItem>
                  {/each}
                </SelectContent>
              </Select>

              <Select bind:value={birthYear} type="single">
                <SelectTrigger class="w-full h-9 text-xs text-start flex items-center justify-between border border-input px-2">
                  <span class="truncate">{birthYear || m.common_form_year()}</span>
                </SelectTrigger>
                <SelectContent class="text-start max-h-48 overflow-y-auto" align="end">
                  {#each yearsList as yr}
                    <SelectItem value={String(yr)} class="text-xs text-start justify-start cursor-pointer">
                      {yr}
                    </SelectItem>
                  {/each}
                </SelectContent>
              </Select>
            </div>
          </div>

          <!-- Row 5: Gender -->
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
            <div class="flex flex-col gap-1 text-start">
              <span class="text-xs font-bold text-foreground">{m.common_form_sex()} *</span>
              <span class="text-[10px] text-muted-foreground">{m.students_student_detail_row_sex_desc()}</span>
            </div>
            <div class="w-full md:w-80 shrink-0">
              <Select type="single" value={sex} onValueChange={(v) => sex = v ?? "male"}>
                <SelectTrigger class="h-9 w-full text-xs">
                  {sex === "male" ? m.common_form_male() : m.common_form_female()}
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="male" class="text-xs text-start cursor-pointer justify-start">{m.common_form_male()}</SelectItem>
                  <SelectItem value="female" class="text-xs text-start cursor-pointer justify-start">{m.common_form_female()}</SelectItem>
                </SelectContent>
              </Select>
            </div>
          </div>

          <!-- Row 6: National ID & Passport Number -->
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
            <div class="flex flex-col gap-1 text-start">
              <span class="text-xs font-bold text-foreground">{m.common_form_doc_national_id()} / {m.ui_copy_passport_33cc7232()}</span>
              <span class="text-[10px] text-muted-foreground">{m.ui_copy_national_id_or_passport_number_fca40628()}</span>
            </div>
            <div class="w-full md:w-80 shrink-0 grid grid-cols-2 gap-2">
              <Input type="text" class="text-xs h-9 font-mono" placeholder={m.common_form_national_id_placeholder()} bind:value={nationalId} />
              <Input type="text" class="text-xs h-9 font-mono" placeholder={m.ui_copy_passport_no_ad264b9a()} bind:value={passportNumber} />
            </div>
          </div>

          <!-- Row 7: Nationality & Place of Birth -->
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
            <div class="flex flex-col gap-1 text-start">
              <span class="text-xs font-bold text-foreground">{m.common_form_nationality()} / {m.common_form_place_of_birth()}</span>
              <span class="text-[10px] text-muted-foreground">{m.ui_copy_student_nationality_and_official_place_of__07b08712()}</span>
            </div>
            <div class="w-full md:w-80 shrink-0 grid grid-cols-2 gap-2">
              <NationalitySelect bind:value={nationality} />
              <Input type="text" class="text-xs h-9" placeholder={m.common_form_place_of_birth()} bind:value={placeOfBirth} />
            </div>
          </div>

          <!-- Row 8: Blood Type -->
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
            <div class="flex flex-col gap-1 text-start">
              <span class="text-xs font-bold text-foreground">{m.employees_blood_type()}</span>
              <span class="text-[10px] text-muted-foreground">{m.ui_copy_blood_type_for_medical_emergencies_0765ea0d()}</span>
            </div>
            <div class="w-full md:w-80 shrink-0">
              <Input type="text" class="text-xs h-9 font-mono" placeholder={m.operations_ui_blood_type_example()} bind:value={bloodType} />
            </div>
          </div>

          <!-- Row 9: Medical & Special Needs Notes -->
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
            <div class="flex flex-col gap-1 text-start">
              <span class="text-xs font-bold text-foreground">{m.ui_copy_medical_special_needs_c30793bd()}</span>
              <span class="text-[10px] text-muted-foreground">{m.ui_copy_chronic_conditions_allergies_or_special_ca_8c96894a()}</span>
            </div>
            <div class="w-full md:w-80 shrink-0 flex flex-col gap-2">
              <Input type="text" class="text-xs h-9" placeholder={m.ui_copy_medical_notes_allergies_7ed23c0e()} bind:value={medicalNotes} />
              <Input type="text" class="text-xs h-9" placeholder={m.ui_copy_special_needs_accommodations_95953f5b()} bind:value={specialNeedsNotes} />
            </div>
          </div>

          <!-- Row 10: Class/Grade -->
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
            <div class="flex flex-col gap-1 text-start">
              <span class="text-xs font-bold text-foreground">{m.common_form_target_class()} *</span>
              <span class="text-[10px] text-muted-foreground">{m.students_student_detail_row_class_desc()}</span>
            </div>
            <div class="w-full md:w-80 shrink-0">
              <Select bind:value={classId} type="single">
                <SelectTrigger class="w-full h-9 text-xs text-start flex items-center justify-between border border-input">
                  <span>{selectedClassName}</span>
                </SelectTrigger>
                <SelectContent class="text-start max-h-48 overflow-y-auto" align="end">
                    {#each classes as cls (cls.id)}
                      <SelectItem value={String(cls.id)} class="text-xs text-start justify-start cursor-pointer">
                        {l(cls.name_ar, cls.name)}
                      </SelectItem>
                    {/each}
                </SelectContent>
              </Select>
            </div>
          </div>

          <!-- Row 11: Academic ID -->
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
            <div class="flex flex-col gap-1 text-start">
              <span class="text-xs font-bold text-foreground">{m.common_form_student_number()}</span>
              <span class="text-[10px] text-muted-foreground">{m.students_student_detail_row_student_number_desc()}</span>
            </div>
            <div class="w-full md:w-80 shrink-0">
              <Input type="text" class="text-xs h-9 bg-muted/30" bind:value={studentNumber} disabled />
            </div>
          </div>

          <!-- Row 12: Contact Phone -->
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
            <div class="flex flex-col gap-1 text-start">
              <span class="text-xs font-bold text-foreground">{m.common_form_student_phone()}</span>
              <span class="text-[10px] text-muted-foreground">{m.students_student_detail_row_phone_desc()}</span>
            </div>
            <div class="w-full md:w-80 shrink-0">
              <Input type="tel" class="text-xs h-9 font-mono" bind:value={phone} />
            </div>
          </div>

          <!-- Row 13: Email Address -->
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
            <div class="flex flex-col gap-1 text-start">
              <span class="text-xs font-bold text-foreground">{m.common_form_student_email()}</span>
              <span class="text-[10px] text-muted-foreground">{m.students_student_detail_row_email_desc()}</span>
            </div>
            <div class="w-full md:w-80 shrink-0">
              <Input type="email" class="text-xs h-9 font-mono" bind:value={email} />
            </div>
          </div>

          <!-- Row 14: Address -->
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
            <div class="flex flex-col gap-1 text-start">
              <span class="text-xs font-bold text-foreground">{m.common_form_address()}</span>
              <span class="text-[10px] text-muted-foreground">{m.students_student_detail_row_address_desc()}</span>
            </div>
            <div class="w-full md:w-80 shrink-0">
              <Input type="text" class="text-xs h-9" bind:value={address} />
            </div>
          </div>
        </CardContent>

        <!-- Card Footer Actions -->
        <CardFooter class="flex justify-end gap-3 px-6 py-4 bg-muted/20 border-t border-border">
          <Button type="button" variant="ghost" size="sm" onclick={() => { if (studentDetail) populateForm(studentDetail); }}>
            {m.common_switcher_reset()}
          </Button>
          <Button
            type="submit"
            variant="lime"
            size="sm"
            class="font-bold text-xs px-6"
            disabled={isSaving}
          >
            {isSaving ? "..." : m.settings_user_detail_btn_save_details()}
          </Button>
        </CardFooter>
      </Card>

    </form>

    <!-- Audit Timeline History -->
    <div class="mt-4">
      <AuditTimeline entityType="student" entityId={studentId} />
    </div>
    </div>
  {/if}
</div>
