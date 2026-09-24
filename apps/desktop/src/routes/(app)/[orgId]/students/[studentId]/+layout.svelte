<!--
@component StudentDetailsLayout
Streamlined layout for deep-dive student pages.
Uses shared DetailPageHeader, SecondaryMenu, and MainViewport.
-->
<script lang="ts">
  import { page } from "$app/state";
  import { goto } from "$app/navigation";
  import { api } from "$lib/api";
  import { createClassesQuery, createPeriodsQuery } from "$lib/features/academics";
  import { createEnrollmentQuery, createEnrollmentsQuery } from "$lib/features/students";
  import * as m from "$lib/paraglide/messages";
  import DetailPageHeader from "$lib/shell/detail-page-header.svelte";
  import MainViewport from "$lib/shell/main-viewport.svelte";
  import SecondaryMenu from "$lib/shell/secondary-menu.svelte";
  import PdfReportViewer from "$lib/shared/components/pdf-report-viewer.svelte";
  import {
    AcademicCap,
    ArrowsRightLeft,
    Banknotes,
    BookOpen,
    Document,
    DocumentPage,
    EllipsisHorizontal,
    Phone,
    Printer,
    User,
    UserBadgeCheck,
    Users,
  } from "@orbis/icons";
  import { toast } from "@orbis/ui";
  import { academicContext, confirmStore } from "$lib/stores";
  import { Avatar, AvatarFallback, AvatarImage } from "@orbis/ui/avatar";
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
  import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
  } from "@orbis/ui/dropdown-menu";
  import { Skeleton } from "@orbis/ui/skeleton";
  import { useQueryClient } from "@tanstack/svelte-query";
  import type { Snippet } from "svelte";

  import AskOrbisButton from "$lib/shared/components/ask-orbis-button.svelte";

  let { children }: { children: Snippet } = $props();
  const queryClient = useQueryClient();

  const orgId = $derived(page.params.orgId || "");
  const studentId = $derived(page.params.studentId || "");

  // TanStack Queries
  const studentQuery = createEnrollmentQuery(
    () => orgId,
    () => studentId,
  );
  const classesQuery = createClassesQuery(() => orgId);
  const periodsQuery = createPeriodsQuery(() => orgId);

  const student = $derived(studentQuery.data);
  const currentClass = $derived(
    student
      ? (classesQuery.data ?? []).find(
          (c: any) => String(c.id) === String(student.class_id),
        )
      : null,
  );

  // Enrollment history for this student
  const actualStudentId = $derived(
    student?.student_id || studentId,
  );
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
      title: m.common_action_delete ? `${m.common_action_delete()} (انسحاب الطالب)` : "انسحاب الطالب",
      description: "هل أنت متأكد من تسجيل انسحاب هذا الطالب؟ سيتم تحديث حالة القيد الأكاديمي إلى منسحب.",
      confirmText: "تأكيد الانسحاب",
      variant: "destructive",
    });
    if (!confirmed) return;
    isWithdrawing = true;
    try {
      await api.updateStudentStatus({ student_id: studentId, status: "withdrawn" });
      toast.success("تم تحديث حالة الطالب بنجاح");
      queryClient.invalidateQueries({ queryKey: ["enrollments", orgId] });
      queryClient.invalidateQueries({ queryKey: ["student", studentId] });
    } catch (e: any) {
      console.error("Withdraw error:", e);
      toast.error(e?.message || String(e));
    } finally {
      isWithdrawing = false;
    }
  }

  let showTransferDialog = $state(false);
  let showPrintCertificate = $state(false);
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
        enrollmentId: studentId,
        targetClassId: transferTargetClassId,
      });
      toast.success(m.common_form_success_update());
      showTransferDialog = false;
      transferTargetClassId = "";
      queryClient.invalidateQueries({ queryKey: ["enrollments", orgId] });
      queryClient.invalidateQueries({ queryKey: ["student", studentId] });
    } catch (e: any) {
      console.error("Transfer error:", e);
      toast.error(e?.message || String(e));
    } finally {
      isTransferring = false;
    }
  }

  function getClassName(clsId: string) {
    const cls = (classesQuery.data ?? []).find((c: any) => String(c.id) === String(clsId));
    return cls ? String(cls.name || cls.name_ar || clsId) : clsId;
  }

  function getPeriodName(prdId: string) {
    const prd = (periodsQuery.data ?? []).find((p: any) => String(p.id) === String(prdId));
    return prd ? String(prd.name || prd.name_ar || prdId) : prdId;
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
</script>

<div class="h-full w-full flex flex-col overflow-hidden bg-background">
  <!-- Header using shared DetailPageHeader -->
  <DetailPageHeader
    backHref={`/${orgId}/students`}
    backLabel={m.students_detail_back_to_directory()}
  >
    {#if student}
      <h1
        class="text-lg font-bold tracking-tight text-foreground truncate shrink min-w-0"
      >
        {student.full_name || student.name_ar || ""}
      </h1>
      <Badge
        variant="secondary"
        class="text-[9px] py-0 px-1.5 h-4.5 rounded shrink-0"
      >
        {currentClass
          ? String(currentClass.name || currentClass.name_ar || "")
          : m.common_status_not_enrolled()}
      </Badge>
    {:else}
      <Skeleton class="h-6 w-32" />
      <Skeleton class="h-4.5 w-16" />
    {/if}

    {#snippet actions()}
      {#if student}
        <AskOrbisButton
          query={m.academics_student_evaluation_ai_ask_orbis_query({
            name: student.full_name || student.name_ar || "",
          })}
          label={m.academics_student_evaluation_ask_orbis_btn({
            name: student.full_name || student.name_ar || "",
          })}
          variant="outline"
          size="sm"
          class="h-8 text-[11px]"
        />

        <Button
          variant="outline"
          size="sm"
          class="h-8 text-[11px] gap-1.5 font-semibold"
          onclick={() =>
            toast.info(
              m.students_detail_call_guardian_toast({
                phone:
                  student.phone_number ||
                  student.phone ||
                  m.students_detail_phone_unavailable(),
              }),
            )}
        >
          <Phone size={12} />
          {m.students_detail_call_guardian()}
        </Button>

        <DropdownMenu>
          <DropdownMenuTrigger>
            {#snippet child({ props })}
              <Button
                variant="outline"
                size="sm"
                class="h-8 w-8 p-0 cursor-pointer"
                {...props}
              >
                <EllipsisHorizontal size={14} />
              </Button>
            {/snippet}
          </DropdownMenuTrigger>
          <DropdownMenuContent
            align="end"
            class="w-52 p-1 z-50 bg-popover border border-border shadow-md rounded-md select-none"
          >
            <DropdownMenuItem
              class="flex items-center gap-2 px-2.5 py-2 text-[11px] font-semibold cursor-pointer text-foreground hover:bg-accent rounded-sm text-start"
              onclick={() => (showPrintCertificate = true)}
            >
              <Printer size={12} class="text-muted-foreground shrink-0" />
              <span>طباعة شهادة قيد / تسجيل</span>
            </DropdownMenuItem>

            <DropdownMenuItem
              class="flex items-center gap-2 px-2.5 py-2 text-[11px] font-semibold cursor-pointer text-foreground hover:bg-accent rounded-sm text-start"
              onclick={() => goto(`/${orgId}/finance/payments?studentId=${studentId}`)}
            >
              <Banknotes size={12} class="text-muted-foreground shrink-0" />
              <span>تسجيل دفعة مالية</span>
            </DropdownMenuItem>

            <DropdownMenuItem
              class="flex items-center gap-2 px-2.5 py-2 text-[11px] font-semibold cursor-pointer text-foreground hover:bg-accent rounded-sm text-start"
              onclick={() => (showTransferDialog = true)}
            >
              <ArrowsRightLeft size={12} class="text-muted-foreground shrink-0" />
              <span>{m.common_student_layout_transfer_class()}</span>
            </DropdownMenuItem>

            <DropdownMenuSeparator class="bg-border/60 my-1" />

            <DropdownMenuItem
              class="flex items-center gap-2 px-2.5 py-2 text-[11px] font-semibold cursor-pointer text-destructive focus:text-destructive focus:bg-destructive/5 hover:bg-destructive/5 rounded-sm text-start"
              onclick={withdrawStudent}
              disabled={isWithdrawing}
            >
              <UserBadgeCheck size={12} class="text-destructive shrink-0" />
              <span>{m.common_inspector_withdraw()}</span>
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
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
          {#if student}
            <!-- Squared Avatar Display -->
            <div class="w-full flex justify-center">
              <Avatar
                class="w-[85%] max-w-[180px] min-w-[120px] aspect-square h-auto rounded-xl shadow-xs border border-border/50"
              >
                <AvatarImage
                  src={student.avatar_url || student.avatar || student.a}
                  alt={student.full_name ? String(student.full_name) : ""}
                />
                <AvatarFallback
                  class="text-5xl font-bold rounded-xl flex items-center justify-center bg-muted text-muted-foreground"
                >
                  {String(student.full_name || "S").slice(0, 1)}
                </AvatarFallback>
              </Avatar>
            </div>

            <!-- Student Metadata List -->
            <div
              class="flex flex-col gap-3.5 border-t border-border/40 pt-4 w-full text-start"
            >
              {#if student.student_number || student.id}
                <div class="flex flex-col gap-0.5">
                  <span
                    class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider"
                  >
                    {m.common_word_academic_number()}
                  </span>
                  <span class="text-xs font-mono font-bold text-foreground">
                    {student.student_number || student.id}
                  </span>
                </div>
              {/if}

              {#if student.phone}
                <div class="flex flex-col gap-0.5">
                  <span
                    class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider"
                    >{m.common_word_phone()}</span
                  >
                  <span class="text-xs text-foreground font-mono">
                    {student.phone}
                  </span>
                </div>
              {/if}

              {#if student.email}
                <div class="flex flex-col gap-0.5">
                  <span
                    class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider"
                    >{m.common_word_email()}</span
                  >
                  <span
                    class="text-xs text-foreground font-mono truncate max-w-full"
                    title={String(student.email)}
                  >
                    {student.email}
                  </span>
                </div>
              {/if}

              {#if student.sex}
                <div class="flex flex-col gap-0.5">
                  <span
                    class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider"
                    >{m.common_form_sex()}</span
                  >
                  <span class="text-xs text-foreground font-medium">
                    {student.sex === "male"
                      ? m.common_form_male()
                      : m.common_form_female()}
                  </span>
                </div>
              {/if}

              {#if student.birth_date}
                <div class="flex flex-col gap-0.5">
                  <span
                    class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider"
                    >{m.common_word_dob()}</span
                  >
                  <span class="text-xs text-foreground font-mono">
                    {student.birth_date}
                  </span>
                </div>
              {/if}

              {#if student.guardian_name}
                <div class="flex flex-col gap-0.5">
                  <span
                    class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider"
                    >{m.common_users_filter_guardian()}</span
                  >
                  <span class="text-xs text-foreground font-medium">
                    {student.guardian_name}
                  </span>
                </div>
              {/if}

              {#if student.guardian_phone}
                <div class="flex flex-col gap-0.5">
                  <span
                    class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider"
                    >{m.common_bulk_col_guardian_phone()}</span
                  >
                  <span class="text-xs text-foreground font-mono">
                    {student.guardian_phone}
                  </span>
                </div>
              {/if}
            </div>

            <!-- Enrollment History Section -->
            <div class="flex flex-col gap-2.5 border-t border-border/40 pt-4 w-full text-start">
              <div class="flex items-center justify-between">
                <span class="text-[10px] text-foreground font-bold tracking-wide flex items-center gap-1.5">
                  <AcademicCap size={13} class="text-primary" />
                  سجل القيود والفصول
                </span>
                <span class="text-[9px] text-muted-foreground font-mono">
                  {enrollmentHistory.length}
                </span>
              </div>

              {#if enrollmentHistory.length > 0}
                <div class="flex flex-col gap-1.5">
                  {#each enrollmentHistory as enr}
                    <div
                      class="flex flex-col gap-1 p-2 rounded-lg border text-start transition-colors {String(enr.id) === String(studentId)
                        ? 'border-primary/40 bg-primary/5'
                        : 'border-border/50 bg-muted/20'}"
                    >
                      <div class="flex items-center justify-between gap-1">
                        <span class="text-[11px] font-semibold text-foreground truncate">
                          {getClassName(enr.class_id)}
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
                  لا توجد قيود سابقة
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
          <div class="flex flex-col gap-6 w-full h-full">
            <!-- Tabs using shared SecondaryMenu -->
            <SecondaryMenu items={menuItems} {basePath} />

            <!-- Children Sub-Route Viewport -->
            <div class="w-full flex-1">
              {@render children()}
            </div>
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
        اختر الشعبة أو الفصل الجديد لنقل الطالب وتحديث القيد الأكاديمي.
      </DialogDescription>
    </DialogHeader>

    <div class="py-3 flex flex-col gap-2">
      <label for="target-class-select" class="text-[11px] font-semibold text-foreground">
        الفصل الدراسي المستهدف
      </label>
      <select
        id="target-class-select"
        class="w-full border border-border rounded-md px-3 py-2 text-xs bg-background focus:outline-hidden focus:ring-1 focus:ring-ring"
        bind:value={transferTargetClassId}
      >
        <option value="">— اختر الفصل الجديد —</option>
        {#each (classesQuery.data ?? []) as cls}
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
  templateId="student.enrollment_certificate"
  parameters={{
    student_id: actualStudentId,
    enrollment_id: studentId,
    period_id: academicContext.effectivePeriodId,
  }}
  reportTitle="طباعة شهادة قيد طالب"
/>
