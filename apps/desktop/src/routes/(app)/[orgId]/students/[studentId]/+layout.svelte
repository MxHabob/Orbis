<!--
@component StudentDetailsLayout
Streamlined layout for deep-dive student pages.
Uses shared DetailPageHeader, SecondaryMenu, and MainViewport.
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { page } from "$app/state";
import { api, extractErrorMessage, queryKeys } from "$lib/api";
import {
  createClassesQuery,
  createPeriodsQuery,
} from "$lib/features/academics";
import {
  createEnrollmentsQuery,
  createStudentQuery,
} from "$lib/features/students";
import * as m from "$lib/paraglide/messages";
import PdfReportViewer from "$lib/shared/components/pdf-report-viewer.svelte";
import RelatedActions, {
  type ActionGroup,
  type FooterAction,
} from "$lib/shared/components/related-actions.svelte";
import DetailPageHeader from "$lib/shell/detail-page-header.svelte";
import MainViewport from "$lib/shell/main-viewport.svelte";
import SecondaryMenu from "$lib/shell/secondary-menu.svelte";

import { useStudentFolder } from "$lib/features/documents/queries";
import AvatarUpload from "$lib/shared/components/avatar-upload.svelte";
import { academicContext, confirmStore } from "$lib/stores";
import {
  AcademicCap,
  ArrowsRightLeft,
  Banknotes,
  BookOpen,
  Document,
  DocumentCheck,
  DocumentPage,
  Phone,
  Printer,
  Sparkles,
  User,
  UserBadgeCheck,
  Users,
} from "@orbis/icons";
import { toast } from "@orbis/ui";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@orbis/ui/dialog";
import { Skeleton } from "@orbis/ui/skeleton";
import { useQueryClient } from "@tanstack/svelte-query";
import type { Snippet } from "svelte";

import SplitActionMenu from "$lib/shared/components/split-action-menu.svelte";

let { children }: { children: Snippet } = $props();
const queryClient = useQueryClient();

const orgId = $derived(page.params.orgId || "");
const studentId = $derived(page.params.studentId || "");

// TanStack Queries
const studentQuery = createStudentQuery(() => studentId);
const classesQuery = createClassesQuery(() => orgId);
const periodsQuery = createPeriodsQuery(() => orgId);

const studentDetail = $derived(studentQuery.data);
const student = $derived(studentDetail?.student);
const person = $derived(studentDetail?.person);
const enrollment = $derived(studentDetail?.enrollment);

const studentFullName = $derived(
  (person?.first_name && person?.last_name
    ? `${person.first_name} ${person.last_name}`
    : "") ||
    person?.name_ar ||
    person?.name_en ||
    person?.full_name ||
    "",
);

function handleAskOrbis() {
  const newUrl = new URL(page.url.href);
  newUrl.searchParams.set(
    "ask_orbis",
    m.academics_student_evaluation_ai_ask_orbis_query({
      name: studentFullName,
    }),
  );
  goto(newUrl.pathname + newUrl.search, {
    replaceState: true,
    keepFocus: true,
    noScroll: true,
  });
}

const studentFolderQuery = useStudentFolder(
  () => student?.id || studentId,
  () => studentFullName,
);

async function handleStudentAvatarUpload(url: string) {
  try {
    await api.updateStudentProfile({
      student_id: student?.id || studentId,
      avatar_url: url,
    });
    await queryClient.invalidateQueries({
      queryKey: queryKeys.student.detail(student?.id || studentId),
    });
    toast.success(m.students_avatar_updated());
  } catch (error) {
    toast.error(extractErrorMessage(error));
  }
}

const currentClass = $derived(
  enrollment?.class_id
    ? (classesQuery.data ?? []).find(
        (c: any) => String(c.id) === String(enrollment.class_id),
      )
    : null,
);

// Enrollment history for this student
const actualStudentId = $derived(student?.id || studentId);
const enrollmentsQuery = createEnrollmentsQuery(
  () => orgId,
  () => ({ studentId: actualStudentId }),
);
const enrollmentHistory = $derived(
  (enrollmentsQuery.data ?? []).filter(
    (e: any) =>
      String(e.student_id || e.student?.id) === String(actualStudentId) ||
      String(e.id) === String(studentId),
  ),
);

const basePath = $derived(`/${orgId}/students/${studentId}`);

let isWithdrawing = $state(false);

async function withdrawStudent() {
  const confirmed = await confirmStore.prompt({
    title: m.shared_ui_withdraw_student_title(),
    description: m.shared_ui_withdraw_student_confirmation(),
    confirmText: m.shared_ui_confirm_withdrawal(),
    variant: "destructive",
  });
  if (!confirmed) return;
  isWithdrawing = true;
  try {
    await api.updateStudentStatus({
      student_id: studentId,
      status: "withdrawn",
    });
    toast.success(m.shared_ui_student_status_updated());
    queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
    queryClient.invalidateQueries({
      queryKey: queryKeys.student.detail(studentId),
    });
  } catch (e: any) {
    console.error("Withdraw error:", e);
    toast.error(e?.message || String(e));
  } finally {
    isWithdrawing = false;
  }
}

let showTransferDialog = $state(false);
let showPrintCertificate = $state(false);
let showPrintAcademicCertificate = $state(false);
let transferTargetClassId = $state("");
let isTransferring = $state(false);

async function transferClass() {
  if (!transferTargetClassId) {
    toast.error(m.common_form_error_class_required());
    return;
  }
  isTransferring = true;
  try {
    await api.transferClass({
      enrollmentId: enrollment?.id || studentId,
      targetClassId: transferTargetClassId,
    });
    toast.success(m.common_form_success_update());
    showTransferDialog = false;
    transferTargetClassId = "";
    queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
    queryClient.invalidateQueries({
      queryKey: queryKeys.student.detail(studentId),
    });
  } catch (e: any) {
    console.error("Transfer error:", e);
    toast.error(e?.message || String(e));
  } finally {
    isTransferring = false;
  }
}

function getClassName(clsId: string) {
  const cls = (classesQuery.data ?? []).find(
    (c: any) => String(c.id) === String(clsId),
  );
  return cls ? String(cls.name || clsId) : clsId;
}

function getPeriodName(prdId: string) {
  const prd = (periodsQuery.data ?? []).find(
    (p: any) => String(p.id) === String(prdId),
  );
  return prd ? String(prd.name || prdId) : prdId;
}

const menuItems = $derived([
  {
    path: "",
    label: m.common_breadcrumb_student_profile(),
    icon: User,
  },
  {
    path: "grades",
    label: m.students_detail_tab_grades(),
    icon: BookOpen,
  },
  {
    path: "fees",
    label: m.students_detail_tab_fees(),
    icon: Document,
  },
  {
    path: "attendance",
    label: m.common_promo_attendance(),
    icon: UserBadgeCheck,
  },
  {
    path: "behavior",
    label: m.common_detail_tab_behavior(),
    icon: UserBadgeCheck,
  },
  {
    path: "guardians",
    label: m.common_guardians_tab(),
    icon: Users,
  },
  {
    path: "documents",
    label: m.students_detail_tab_documents(),
    icon: DocumentPage,
  },
]);

const relatedActionGroups = $derived<ActionGroup[]>(
  studentDetail
    ? [
        {
          id: "academic_actions",
          title: m.ui_copy_academic_operations_42dbfc55(),
          icon: AcademicCap,
          defaultExpanded: true,
          subLinks: [
            {
              id: "print_cert",
              label: m.ui_copy_print_enrollment_certificate_d360ef39(),
              icon: Printer,
              onclick: () => (showPrintCertificate = true),
            },
            {
              id: "print_academic_cert",
              label: m.student_layout_print_official_academic_certificate(),
              icon: DocumentCheck,
              onclick: () => (showPrintAcademicCertificate = true),
            },
            {
              id: "transfer_class",
              label: m.common_student_layout_transfer_class(),
              icon: ArrowsRightLeft,
              onclick: () => (showTransferDialog = true),
            },
            ...(currentClass?.id
              ? [
                  {
                    id: "class_curriculum",
                    label: m.ui_copy_class_curriculum_8b6731cd(),
                    icon: BookOpen,
                    href: `/${orgId}/academics/sections/${currentClass.id}/curriculum`,
                  },
                ]
              : []),
          ],
        },
        {
          id: "financial_actions",
          title: m.ui_copy_financial_operations_c98b7d37(),
          icon: Banknotes,
          defaultExpanded: true,
          subLinks: [
            {
              id: "record_payment",
              label: m.ui_copy_record_payment_2d05fe8e(),
              icon: Banknotes,
              href: `/${orgId}/finance/payments?studentId=${actualStudentId}`,
            },
            {
              id: "account_statement",
              label: m.ui_copy_account_statement_6b810b21(),
              icon: Document,
              href: `/${orgId}/finance/statement?studentId=${actualStudentId}`,
            },
            {
              id: "student_fees",
              label: m.students_detail_tab_fees(),
              icon: DocumentPage,
              href: `/${orgId}/students/${studentId}/fees`,
            },
          ],
        },
      ]
    : [],
);

const relatedFooterActions = $derived<FooterAction[]>(
  studentDetail
    ? [
        {
          id: "withdraw_action",
          label: m.common_inspector_withdraw(),
          icon: UserBadgeCheck,
          onclick: withdrawStudent,
          disabled: isWithdrawing,
        },
      ]
    : [],
);
</script>

<div class="h-full w-full flex flex-col overflow-hidden bg-background">
  <!-- Header using shared DetailPageHeader -->
  <DetailPageHeader
    backHref={`/${orgId}/students`}
    backLabel={m.students_detail_back_to_directory()}
  >
    {#if studentDetail}
      <h1
        class="text-lg font-bold tracking-tight text-foreground truncate shrink min-w-0"
      >
        {studentFullName || m.shared_ui_student_fallback_name()}
      </h1>
      <Badge
        variant="secondary"
        class="text-[9px] py-0 px-1.5 h-4.5 rounded shrink-0"
      >
        {currentClass
          ? String(currentClass.name || "")
          : m.common_status_not_enrolled()}
      </Badge>
    {:else}
      <Skeleton class="h-6 w-32" />
      <Skeleton class="h-4.5 w-16" />
    {/if}

    {#snippet actions()}
      {#if studentDetail}
        <SplitActionMenu
          label={m.academics_student_evaluation_ask_orbis_btn_short()}
          menuLabel={m.operations_ui_actions()}
          icon={Sparkles}
          variant="outline"
          onprimary={handleAskOrbis}
          actions={[
            {
              id: "call-guardian",
              label: m.students_detail_call_guardian(),
              icon: Phone,
              onSelect: () =>
                toast.info(
                  m.students_detail_call_guardian_toast({
                    phone:
                      person?.phone ||
                      m.students_detail_phone_unavailable(),
                  }),
                ),
            },
            {
              id: "print-certificate",
              label: m.shared_ui_print_enrollment_certificate(),
              icon: Printer,
              onSelect: () => (showPrintCertificate = true),
            },
            {
              id: "record-payment",
              label: m.shared_ui_record_payment(),
              icon: Banknotes,
              onSelect: () =>
                goto(`/${orgId}/finance/payments?studentId=${studentId}`),
            },
            {
              id: "transfer-class",
              label: m.common_student_layout_transfer_class(),
              icon: ArrowsRightLeft,
              onSelect: () => (showTransferDialog = true),
            },
            {
              id: "withdraw",
              label: m.common_inspector_withdraw(),
              icon: UserBadgeCheck,
              destructive: true,
              disabled: isWithdrawing,
              separatorBefore: true,
              onSelect: withdrawStudent,
            },
          ]}
        />
      {/if}
    {/snippet}
  </DetailPageHeader>

  <!-- Resizable split view below the header -->
  <div class="flex-1 w-full overflow-hidden">
    <MainViewport
      name="student-detail-viewport"
      leftWidth={{ default: 250, min: 210 }}
    >
      <!-- Left Sidebar (Avatar, basic student info, enrollment history) -->
      {#snippet left()}
        <div
          class="h-full p-4 flex flex-col gap-5 overflow-y-auto scrollbar-hide text-start bg-transparent"
        >
          {#if studentDetail}
            <!-- Squared Avatar Upload -->
            <div class="w-full flex justify-center">
              <AvatarUpload
                avatarUrl={person?.avatar_url || null}
                entityType="student"
                entityId={student?.id || studentId}
                folderId={studentFolderQuery.data?.id}
                isLoading={!studentFolderQuery.data}
                fallbackText={studentFullName || "S"}
                size={180}
                class="rounded-xl shadow-xs border border-border/50"
                onupload={handleStudentAvatarUpload}
              />
            </div>
            {#if studentFolderQuery.error}
              <p class="text-[10px] text-destructive text-center">
                {m.students_archive_folder_failed({
                  error: extractErrorMessage(studentFolderQuery.error),
                })}
              </p>
            {/if}

            <!-- Student Metadata List -->
            <div
              class="flex flex-col gap-3.5 border-t border-border/40 pt-4 w-full text-start"
            >
              {#if student?.student_number || student?.id}
                <div class="flex flex-col gap-0.5">
                  <span
                    class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider"
                  >
                    {m.common_word_academic_number()}
                  </span>
                  <span class="text-xs font-mono font-bold text-foreground">
                    {student?.student_number || student?.id}
                  </span>
                </div>
              {/if}

              {#if person?.phone}
                <div class="flex flex-col gap-0.5">
                  <span
                    class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider"
                    >{m.common_word_phone()}</span
                  >
                  <span class="text-xs text-foreground font-mono">
                    {person.phone}
                  </span>
                </div>
              {/if}

              {#if person?.email}
                <div class="flex flex-col gap-0.5">
                  <span
                    class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider"
                    >{m.common_word_email()}</span
                  >
                  <span
                    class="text-xs text-foreground font-mono truncate max-w-full"
                    title={String(person.email)}
                  >
                    {person.email}
                  </span>
                </div>
              {/if}

              {#if person?.gender}
                <div class="flex flex-col gap-0.5">
                  <span
                    class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider"
                    >{m.common_form_sex()}</span
                  >
                  <span class="text-xs text-foreground font-medium">
                    {person.gender === "male"
                      ? m.common_form_male()
                      : m.common_form_female()}
                  </span>
                </div>
              {/if}

              {#if person?.birth_date}
                <div class="flex flex-col gap-0.5">
                  <span
                    class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider"
                    >{m.common_word_dob()}</span
                  >
                  <span class="text-xs text-foreground font-mono">
                    {person.birth_date}
                  </span>
                </div>
              {/if}

              {#if student?.status}
                <div class="flex flex-col gap-0.5">
                  <span
                    class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider"
                    >{m.common_word_status()}</span
                  >
                  <span class="text-xs text-foreground font-medium capitalize">
                    {student.status}
                  </span>
                </div>
              {/if}
            </div>

            <!-- Enrollment History Section -->
            <div class="flex flex-col gap-2.5 border-t border-border/40 pt-4 w-full text-start">
              <div class="flex items-center justify-between">
                <span class="text-[10px] text-foreground font-bold tracking-wide flex items-center gap-1.5">
                  <AcademicCap size={13} class="text-primary" />
                  {m.shared_ui_enrollment_history()}
                </span>
                <span class="text-[9px] text-muted-foreground font-mono">
                  {enrollmentHistory.length}
                </span>
              </div>

              {#if enrollmentHistory.length > 0}
                <div class="flex flex-col gap-1.5">
                  {#each enrollmentHistory as enr (enr.id)}
                    <div
                      class="flex flex-col gap-1 p-2 rounded-lg border text-start transition-colors {String(enr.id) === String(studentId)
                        ? 'border-primary/40 bg-primary/5'
                        : 'border-border/50 bg-muted/20'}"
                    >
                      <div class="flex items-center justify-between gap-1">
                        <span class="text-[11px] font-semibold text-foreground truncate">
                          {getClassName(enr.section_id)}
                        </span>
                        <Badge
                          variant={enr.status === "enrolled" || enr.status === "active" ? "default" : "secondary"}
                          class="text-[8px] py-0 px-1 h-3.5"
                        >
                          {enr.status || "enrolled"}
                        </Badge>
                      </div>
                      {#if enr.period_id}
                        <span class="text-[9px] text-muted-foreground">
                          {getPeriodName(enr.period_id)}
                        </span>
                      {/if}
                    </div>
                  {/each}
                </div>
              {:else}
                <div class="text-[10px] text-muted-foreground py-1 text-center bg-muted/10 rounded-md border border-dashed border-border/60">
                  {m.shared_ui_no_previous_enrollments()}
                </div>
              {/if}
            </div>
          {:else}
            <!-- Skeleton for sidebar -->
            <div class="w-full flex justify-start">
              <Skeleton class="h-28 w-28 rounded-xl" />
            </div>
            <div class="space-y-4 pt-4 border-t border-border/40">
              <div class="space-y-1">
                <Skeleton class="h-3 w-16" />
                <Skeleton class="h-4 w-24" />
              </div>
              <div class="space-y-1">
                <Skeleton class="h-3 w-16" />
                <Skeleton class="h-4 w-28" />
              </div>
            </div>
          {/if}
        </div>
      {/snippet}

      <!-- Right/Middle Panel (SecondaryMenu & sub-pages content) -->
      {#snippet middle()}
        <div
          class="h-full w-full overflow-y-auto p-6 scrollbar-hide text-start"
        >
          <div class="flex flex-col gap-6 w-full min-h-full">
            <!-- Tabs using shared SecondaryMenu -->
            <SecondaryMenu items={menuItems} {basePath} />

            <!-- Children Sub-Route Viewport -->
            <div class="w-full flex-1">
              {@render children()}
            </div>

            <!-- Contextual Related Actions (Footer across page width) -->
            {#if student}
              <RelatedActions
                title={m.ui_copy_related_support_bf36e16f()}
                groups={relatedActionGroups}
                footerActions={relatedFooterActions}
              />
            {/if}
          </div>
        </div>
      {/snippet}
    </MainViewport>
  </div>
</div>

<!-- Class Transfer Dialog using canonical UI Dialog component -->
<Dialog bind:open={showTransferDialog}>
  <DialogContent class="sm:max-w-[420px]">
    <DialogHeader>
      <DialogTitle class="text-sm font-bold flex items-center gap-2">
        <ArrowsRightLeft size={16} class="text-primary" />
        {m.common_student_layout_transfer_class()}
      </DialogTitle>
      <DialogDescription class="text-xs text-muted-foreground">
        {m.shared_ui_transfer_student_description()}
      </DialogDescription>
    </DialogHeader>

    <div class="py-3 flex flex-col gap-2">
      <label for="target-class-select" class="text-[11px] font-semibold text-foreground">
        {m.shared_ui_target_academic_class()}
      </label>
      <select
        id="target-class-select"
        class="w-full border border-border rounded-md px-3 py-2 text-xs bg-background focus:outline-hidden focus:ring-1 focus:ring-ring"
        bind:value={transferTargetClassId}
      >
        <option value="">{m.shared_ui_select_new_class()}</option>
        {#each (classesQuery.data ?? []) as cls (cls.id)}
          <option value={String(cls.id)}>{cls.name}</option>
        {/each}
      </select>
    </div>

    <DialogFooter class="gap-2 sm:gap-0">
      <Button
        variant="outline"
        size="sm"
        class="text-xs h-8"
        onclick={() => (showTransferDialog = false)}
      >
        {m.common_action_cancel()}
      </Button>
      <Button
        size="sm"
        class="text-xs h-8 gap-1.5"
        onclick={transferClass}
        disabled={isTransferring || !transferTargetClassId}
      >
        <ArrowsRightLeft size={12} />
        {m.common_student_layout_transfer_class()}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>

<!-- Print Certificate Dialog -->
<PdfReportViewer
  bind:open={showPrintCertificate}
  templateId="administration.enrollment-certificate"
  parameters={{
    student_id: actualStudentId,
    enrollment_id: studentId,
    period_id: academicContext.effectivePeriodId,
  }}
  reportTitle={m.shared_ui_student_enrollment_certificate_title()}
/>

<!-- Print Official Academic Certificate Dialog -->
<PdfReportViewer
  bind:open={showPrintAcademicCertificate}
  templateId="academic.student-certificate-yemeni"
  parameters={{
    student_id: actualStudentId,
    calendar_id: academicContext.currentYear,
    period_id: academicContext.effectivePeriodId,
  }}
  reportTitle={m.student_grades_certificate_print_title()}
/>
