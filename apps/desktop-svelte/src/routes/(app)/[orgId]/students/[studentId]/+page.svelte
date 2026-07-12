<script lang="ts">
import { page } from "$app/state";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { Switch } from "@orbis/ui-svelte/switch";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { Badge } from "@orbis/ui-svelte/badge";
import AttendanceTrendsCard from "$lib/features/metrics/cards/attendance-trends-card.svelte";
import GradeDistributionCard from "$lib/features/metrics/cards/grade-distribution-card.svelte";
import { BrainCircuit } from "lucide-svelte";
import { onMount } from "svelte";
import { toast } from "svelte-sonner";
import { fade } from "svelte/transition";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
import { createEnrollmentsQuery } from "$lib/features/students";
import { createClassesQuery } from "$lib/features/academics";
import { commands, extractErrorMessage } from "$lib/core/ipc";
import { queryClient } from "$lib/core/query/client";
import { academicContext } from "$lib/stores";

const orgId = $derived(page.params.orgId || "");
const studentId = $derived(page.params.studentId || ""); // Enrollment ID

// TanStack Queries
const enrollmentsQuery = createEnrollmentsQuery(orgId);
const classesQuery = createClassesQuery(orgId);

const enrollments = $derived($enrollmentsQuery.data ?? []);
const classes = $derived($classesQuery.data ?? []);
const student = $derived(enrollments.find((s) => s.id === studentId));
const isLoading = $derived(
  $enrollmentsQuery.isLoading || $classesQuery.isLoading,
);

let isSaving = $state(false);

// Form fields
let nameAr = $state("");
let nameEn = $state("");
let birthDate = $state("");
let sex = $state("male");
let classId = $state("");
let studentNumber = $state("");
let email = $state("");
let phone = $state("");
let address = $state("");
let isActiveState = $state(true);

const selectedClassName = $derived(
  classes.find((c) => String(c.id) === classId)?.name ||
    classes.find((c) => String(c.id) === classId)?.name_ar ||
    m.form_select_class(),
);

// Date picker options and reactive values
const currentYear = new Date().getFullYear();
const yearsList = Array.from({ length: 47 }, (_, i) => currentYear - i);
const daysList = Array.from({ length: 31 }, (_, i) => String(i + 1));
const monthsList = $derived([
  { value: "1", label: m.form_month_1() },
  { value: "2", label: m.form_month_2() },
  { value: "3", label: m.form_month_3() },
  { value: "4", label: m.form_month_4() },
  { value: "5", label: m.form_month_5() },
  { value: "6", label: m.form_month_6() },
  { value: "7", label: m.form_month_7() },
  { value: "8", label: m.form_month_8() },
  { value: "9", label: m.form_month_9() },
  { value: "10", label: m.form_month_10() },
  { value: "11", label: m.form_month_11() },
  { value: "12", label: m.form_month_12() },
]);

let birthDay = $state("");
let birthMonth = $state("");
let birthYear = $state("");

$effect(() => {
  if (birthDay && birthMonth && birthYear) {
    birthDate = `${birthYear}-${birthMonth.padStart(2, "0")}-${birthDay.padStart(2, "0")}`;
  } else {
    birthDate = "";
  }
});

let initialized = $state(false);
$effect(() => {
  if (student && !initialized) {
    nameAr = String(student.full_name || student.name_ar || "");
    nameEn = String(student.name_en || "");
    birthDate = student.birth_date
      ? String(student.birth_date).slice(0, 10)
      : "";

    if (birthDate) {
      const parts = birthDate.split("-");
      if (parts.length === 3) {
        birthYear = parts[0];
        birthMonth = String(parseInt(parts[1], 10));
        birthDay = String(parseInt(parts[2], 10));
      }
    }

    sex = String(student.sex || "male");
    classId = String(student.class_id || "");
    studentNumber = String(student.student_number || "");
    email = String(student.email || "");
    phone = String(student.phone_number || student.phone || "");
    address = String(student.address || "");
    isActiveState = student.status === "active";
    initialized = true;
  }
});

// Evaluation State & Loading
let evaluationLoading = $state(true);
let gradesReport = $state<any>(null);
let attendanceReport = $state<any>(null);
let behaviorRecords = $state<any[]>([]);

onMount(async () => {
  evaluationLoading = true;
  try {
    const periodId = academicContext.effectivePeriodId || student?.period_id || "";
    if (periodId && studentId) {
      const [gRes, aRes, bRes] = await Promise.all([
        commands.assessment.getReportCard(studentId, periodId).catch(() => null),
        commands.assessment.getAttendanceReport(periodId, studentId).catch(() => null),
        commands.assessment.listBehavior(orgId, studentId).catch(() => []),
      ]);
      gradesReport = gRes;
      attendanceReport = aRes;
      behaviorRecords = bRes;
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
  let points = 100; // start with baseline 100 behavior score
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
      badge: m.student_evaluation_decision_title(),
      badgeVariant: "outline" as const,
      rec: m.student_evaluation_no_data(),
      colorClass: "text-muted-foreground bg-muted/25 border-border/60",
    };
  }

  if (acadAvg >= 85 && attRate >= 95 && negBeh === 0) {
    return {
      status: "excellent",
      badge: m.student_evaluation_status_excellent(),
      badgeVariant: "lime" as const,
      rec: m.student_evaluation_rec_excellent(),
      colorClass: "text-emerald-500 bg-emerald-500/10 border-emerald-500/20",
    };
  } else if (acadAvg <= 45 || attRate <= 70 || negBeh >= 4) {
    return {
      status: "critical",
      badge: m.student_evaluation_status_critical(),
      badgeVariant: "destructive" as const,
      rec: m.student_evaluation_rec_critical(),
      colorClass: "text-destructive bg-destructive/10 border-destructive/20",
    };
  } else if (acadAvg < 60 || attRate < 85 || negBeh >= 2) {
    return {
      status: "warning",
      badge: m.student_evaluation_status_warning(),
      badgeVariant: "secondary" as const,
      rec: m.student_evaluation_rec_warning(),
      colorClass: "text-amber-500 bg-amber-500/10 border-amber-500/20",
    };
  } else {
    return {
      status: "good",
      badge: m.student_evaluation_status_good(),
      badgeVariant: "default" as const,
      rec: m.student_evaluation_rec_good(),
      colorClass: "text-primary bg-primary/10 border-primary/20",
    };
  }
});

async function saveStudentDetails(e: Event) {
  e.preventDefault();
  if (!student || !student.student_id) return;
  if (!nameAr.trim()) {
    toast.error(m.form_error_name_required());
    return;
  }
  if (!classId) {
    toast.error(m.form_error_class_required());
    return;
  }
  if (!birthDate) {
    toast.error(m.form_error_birth_required());
    return;
  }

  isSaving = true;
  try {
    // 1. Verify parent control account '110301' exists in chart of accounts
    const accounts = await commands.finance.listAccounts(orgId);
    const hasControlAccount = accounts.some((acc: any) => acc.code === "110301");
    if (!hasControlAccount) {
      toast.error(m.form_error_control_account_missing());
      isSaving = false;
      return;
    }

    const studentUserData = {
      full_name: nameAr.trim(),
      name_ar: nameAr.trim(),
      name_en: nameEn.trim() || null,
      email: email.trim() || null,
      phone: phone.trim() || null,
      sex: sex,
      date_of_birth: birthDate,
      address: address.trim() || null,
      status: isActiveState ? "active" : "inactive",
    };

    // Update student user details
    await commands.iam.updateUser(
      String(student.student_id),
      studentUserData,
    );

    // If class has changed, invoke transfer command
    if (student.class_id !== classId) {
      await commands.academic.transferEnrollment(orgId, studentId, classId);
    }

    // Transition enrollment status if it was draft/pending_review and active switch is checked
    if (
      (student.status === "draft" || student.status === "pending_review") &&
      isActiveState
    ) {
      await commands.platform.transitionEnrollmentStatus(orgId, studentId, "active");
    }

    toast.success(m.form_success_update());

    // Invalidate query data to trigger reactive updates
    queryClient.invalidateQueries({ queryKey: ["enrollments", orgId] });
    initialized = false; // re-populate form with updated data
  } catch (e: any) {
    console.error("Save error:", e);
    toast.error(extractErrorMessage(e));
  } finally {
    isSaving = false;
  }
}
</script>

<div class="h-full w-full overflow-y-auto scrollbar-hide text-start pb-10" dir={getLocale() === "ar" ? "rtl" : "ltr"}>
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
              <Skeleton class="lg:col-span-2 h-[260px] rounded-xl" />
              <Skeleton class="h-[260px] rounded-xl" />
            </div>
            <Skeleton class="h-16 w-full rounded-xl" />
          </div>
        {:else}
          <div class="flex flex-col gap-6">
            <!-- Charts Section (Same split layout as class details page) -->
            {#if classId}
              <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                <!-- Left: Attendance Trends Card (2/3 width) -->
                <div class="lg:col-span-2">
                  <AttendanceTrendsCard orgId={orgId} selectedClass={classId} />
                </div>

                <!-- Right: Grade Distribution Card (1/3 width) -->
                <div>
                  <GradeDistributionCard orgId={orgId} selectedClass={classId} />
                </div>
              </div>
            {/if}

            <!-- Decision Helper Banner -->
            <div class="p-4 rounded-xl border flex items-center justify-between gap-4 shadow-sm {decisionHelper.colorClass}">
              <div class="flex items-center gap-3">
                <BrainCircuit size={18} class="shrink-0" />
                <div class="flex flex-col gap-0.5 text-start">
                  <span class="text-xs font-bold text-foreground">{m.student_evaluation_recommendation_title()}</span>
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

      <!-- Profile Edit Form -->
      <form onsubmit={saveStudentDetails} in:fade={{ duration: 100 }} class="flex flex-col gap-6">
      
      <!-- Organization Settings style Card -->
      <div class="border border-border rounded-xl overflow-hidden flex flex-col">
        
        <!-- Card Header -->
        <div class="bg-muted/30 px-6 py-4 border-b border-border flex items-center justify-between text-xs font-bold text-foreground text-start select-none">
          <span>{m.form_student_info()}</span>
        </div>

        <!-- Card Body with Clear Vertical Rows (Layout identical to settings_+page.svelte) -->
        <div class="flex flex-col">
          
          <!-- Row 1: Student Active Status -->
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
            <div class="flex flex-col gap-1 text-start">
              <span class="text-xs font-bold text-foreground">{m.student_detail_row_status_title()}</span>
              <span class="text-[10px] text-muted-foreground">{m.student_detail_row_status_desc()}</span>
            </div>
            <div class="w-full md:w-80 shrink-0 flex items-center gap-2">
              <Switch
                id="student_status_switch"
                checked={isActiveState}
                onCheckedChange={(checked) => isActiveState = checked}
              />
              <Label for="student_status_switch" class="text-xs text-muted-foreground select-none cursor-pointer">
                {isActiveState ? m.form_active_status() : m.student_detail_row_status_inactive()}
              </Label>
            </div>
          </div>

          <!-- Row 2: Full Name (Arabic) -->
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
            <div class="flex flex-col gap-1 text-start">
              <span class="text-xs font-bold text-foreground">{m.form_full_name_ar()} *</span>
              <span class="text-[10px] text-muted-foreground">{m.student_detail_row_name_ar_desc()}</span>
            </div>
            <div class="w-full md:w-80 shrink-0">
              <Input type="text" class="text-xs font-semibold h-9" bind:value={nameAr} required />
            </div>
          </div>

          <!-- Row 3: Full Name (English) -->
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
            <div class="flex flex-col gap-1 text-start">
              <span class="text-xs font-bold text-foreground">{m.form_full_name_en()}</span>
              <span class="text-[10px] text-muted-foreground">{m.student_detail_row_name_en_desc()}</span>
            </div>
            <div class="w-full md:w-80 shrink-0">
              <Input type="text" class="text-xs font-semibold h-9 font-mono text-left" dir="ltr" bind:value={nameEn} />
            </div>
          </div>

          <!-- Row 4: Birth Date -->
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
            <div class="flex flex-col gap-1 text-start">
              <span class="text-xs font-bold text-foreground">{m.form_birth_date()} *</span>
              <span class="text-[10px] text-muted-foreground">{m.student_detail_row_birth_date_desc()}</span>
            </div>
            <div class="w-full md:w-80 shrink-0 grid grid-cols-3 gap-2" dir={getLocale() === "ar" ? "rtl" : "ltr"}>
              
              <!-- اليوم -->
              <Select bind:value={birthDay} type="single">
                <SelectTrigger class="w-full h-9 text-xs text-start flex items-center justify-between border border-input px-2">
                  <span class="truncate">{birthDay || m.form_day()}</span>
                </SelectTrigger>
                <SelectContent class="text-start max-h-48 overflow-y-auto" align="end">
                  {#each daysList as dy}
                    <SelectItem value={dy} class="text-xs text-start justify-start cursor-pointer">
                      {dy}
                    </SelectItem>
                  {/each}
                </SelectContent>
              </Select>

              <!-- الشهر -->
              <Select bind:value={birthMonth} type="single">
                <SelectTrigger class="w-full h-9 text-xs text-start flex items-center justify-between border border-input px-2">
                  <span class="truncate">{monthsList.find(m => m.value === birthMonth)?.label || m.form_month()}</span>
                </SelectTrigger>
                <SelectContent class="text-start max-h-48 overflow-y-auto" align="end">
                  {#each monthsList as mo}
                    <SelectItem value={mo.value} class="text-xs text-start justify-start cursor-pointer">
                      {mo.label}
                    </SelectItem>
                  {/each}
                </SelectContent>
              </Select>

              <!-- السنة -->
              <Select bind:value={birthYear} type="single">
                <SelectTrigger class="w-full h-9 text-xs text-start flex items-center justify-between border border-input px-2">
                  <span class="truncate">{birthYear || m.form_year()}</span>
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
              <span class="text-xs font-bold text-foreground">{m.form_sex()} *</span>
              <span class="text-[10px] text-muted-foreground">{m.student_detail_row_sex_desc()}</span>
            </div>
            <div class="w-full md:w-80 shrink-0">
              <Select type="single" value={sex} onValueChange={(v) => sex = v ?? "male"}>
                <SelectTrigger class="h-9 w-full text-xs">
                  {sex === "male" ? m.form_male() : m.form_female()}
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="male" class="text-xs text-start cursor-pointer justify-start">{m.form_male()}</SelectItem>
                  <SelectItem value="female" class="text-xs text-start cursor-pointer justify-start">{m.form_female()}</SelectItem>
                </SelectContent>
              </Select>
            </div>
          </div>

          <!-- Row 6: Class/Grade -->
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
            <div class="flex flex-col gap-1 text-start">
              <span class="text-xs font-bold text-foreground">{m.form_target_class()} *</span>
              <span class="text-[10px] text-muted-foreground">{m.student_detail_row_class_desc()}</span>
            </div>
            <div class="w-full md:w-80 shrink-0">
              <Select bind:value={classId} type="single">
                <SelectTrigger class="w-full h-9 text-xs text-start flex items-center justify-between border border-input">
                  <span>{selectedClassName}</span>
                </SelectTrigger>
                <SelectContent class="text-start max-h-48 overflow-y-auto" align="end">
                  {#each classes as cls (cls.id)}
                    <SelectItem value={String(cls.id)} class="text-xs text-start justify-start cursor-pointer">
                      {cls.name || cls.name_ar}
                    </SelectItem>
                  {/each}
                </SelectContent>
              </Select>
            </div>
          </div>

          <!-- Row 7: Academic ID -->
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
            <div class="flex flex-col gap-1 text-start">
              <span class="text-xs font-bold text-foreground">{m.form_student_number()}</span>
              <span class="text-[10px] text-muted-foreground">{m.student_detail_row_student_number_desc()}</span>
            </div>
            <div class="w-full md:w-80 shrink-0">
              <Input type="text" class="text-xs h-9 bg-muted/30" bind:value={studentNumber} disabled />
            </div>
          </div>

          <!-- Row 8: Contact Phone -->
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
            <div class="flex flex-col gap-1 text-start">
              <span class="text-xs font-bold text-foreground">{m.form_student_phone()}</span>
              <span class="text-[10px] text-muted-foreground">{m.student_detail_row_phone_desc()}</span>
            </div>
            <div class="w-full md:w-80 shrink-0">
              <Input type="text" class="text-xs h-9 font-mono text-left" dir="ltr" bind:value={phone} />
            </div>
          </div>

          <!-- Row 9: Email Address -->
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
            <div class="flex flex-col gap-1 text-start">
              <span class="text-xs font-bold text-foreground">{m.form_student_email()}</span>
              <span class="text-[10px] text-muted-foreground">{m.student_detail_row_email_desc()}</span>
            </div>
            <div class="w-full md:w-80 shrink-0">
              <Input type="email" class="text-xs h-9 font-mono text-left" dir="ltr" bind:value={email} />
            </div>
          </div>

          <!-- Row 10: Address -->
          <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
            <div class="flex flex-col gap-1 text-start">
              <span class="text-xs font-bold text-foreground">{m.form_address()}</span>
              <span class="text-[10px] text-muted-foreground">{m.student_detail_row_address_desc()}</span>
            </div>
            <div class="w-full md:w-80 shrink-0">
              <Input type="text" class="text-xs h-9" bind:value={address} />
            </div>
          </div>

        </div>

        <!-- Card Footer Actions (Identical to settings page) -->
        <div class="flex justify-end gap-3 px-6 py-4 bg-muted/20 border-t border-border">
          <Button type="button" variant="ghost" size="sm" onclick={() => { initialized = false; }}>
            {m.settings_user_detail_btn_reset()}
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
        </div>

      </div>

    </form>
    </div>
  {/if}
</div>
