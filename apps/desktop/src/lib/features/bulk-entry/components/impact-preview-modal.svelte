<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { formatNumber } from "$lib/core/i18n";
  import type { BulkPreviewSummary, StudentPreviewDetails, EmployeePreviewDetails, GradePreviewDetails } from "../types";

  interface Props {
    open: boolean;
    profileId: "students" | "employees" | "grades";
    summary: BulkPreviewSummary<any> | null;
    onclose: () => void;
    onconfirm: () => void;
  }

  let { open, profileId, summary, onclose, onconfirm }: Props = $props();

  const studentDetails = $derived(profileId === "students" ? (summary?.details as StudentPreviewDetails) : null);
  const employeeDetails = $derived(profileId === "employees" ? (summary?.details as EmployeePreviewDetails) : null);
  const gradeDetails = $derived(profileId === "grades" ? (summary?.details as GradePreviewDetails) : null);
</script>

{#if open && summary}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4 animate-in fade-in duration-200">
    <div class="w-full max-w-2xl rounded-2xl bg-card border border-border shadow-2xl overflow-hidden flex flex-col max-h-[85vh]">
      <!-- Header -->
      <div class="px-6 py-4 border-b border-border bg-muted/40 flex items-center justify-between">
        <div>
          <h2 class="text-lg font-bold text-foreground">{m.bulk_entry_preview_title()}</h2>
          <p class="text-xs text-muted-foreground mt-0.5">{m.bulk_entry_preview_description()}</p>
        </div>
        <button
          onclick={onclose}
          class="h-8 w-8 rounded-lg flex items-center justify-center text-muted-foreground hover:text-foreground hover:bg-muted transition-colors"
        >
          ✕
        </button>
      </div>

      <!-- Content -->
      <div class="p-6 overflow-y-auto space-y-6 text-sm">
        <!-- High-level Metric Cards -->
        <div class="grid grid-cols-3 gap-3">
          <div class="p-3.5 rounded-xl bg-primary/10 border border-primary/20 flex flex-col">
            <span class="text-xs font-medium text-primary">{m.bulk_entry_preview_valid_records()}</span>
            <span class="text-2xl font-black text-primary mt-1">{summary.validRowsCount}</span>
            <span class="text-[11px] text-primary/80 mt-0.5">{m.bulk_entry_preview_ready()}</span>
          </div>

          <div class="p-3.5 rounded-xl bg-destructive/10 border border-destructive/20 flex flex-col">
            <span class="text-xs font-medium text-destructive">{m.bulk_entry_preview_invalid_records()}</span>
            <span class="text-2xl font-black text-destructive mt-1">{summary.errorRowsCount}</span>
            <span class="text-[11px] text-destructive/80 mt-0.5">{m.bulk_entry_preview_excluded()}</span>
          </div>

          <div class="p-3.5 rounded-xl bg-muted/60 border border-border flex flex-col">
            <span class="text-xs font-medium text-muted-foreground">{m.bulk_entry_preview_warnings()}</span>
            <span class="text-2xl font-black text-foreground mt-1">{summary.warningRowsCount}</span>
            <span class="text-[11px] text-muted-foreground mt-0.5">{m.bulk_entry_preview_review_only()}</span>
          </div>
        </div>

        <!-- Student Specific Breakdown -->
        {#if studentDetails}
          <div class="space-y-4">
            <h3 class="font-bold text-foreground flex items-center gap-2">
              <span class="h-2 w-2 rounded-full bg-primary"></span>
              {m.bulk_entry_preview_students_section()}
            </h3>
            
            <div class="grid grid-cols-2 gap-3">
              <div class="p-3 rounded-lg bg-muted/40 border border-border/80">
                <span class="text-xs text-muted-foreground block">{m.bulk_entry_preview_new_student_records()}</span>
                <span class="text-base font-bold text-foreground mt-1 block">{m.bulk_entry_preview_student_count({ count: studentDetails.studentsToCreate })}</span>
              </div>

              <div class="p-3 rounded-lg bg-muted/40 border border-border/80">
                <span class="text-xs text-muted-foreground block">{m.bulk_entry_preview_linked_guardians()}</span>
                <span class="text-base font-bold text-foreground mt-1 block">{m.bulk_entry_preview_guardian_count({ count: studentDetails.guardiansToLink })}</span>
              </div>

              <div class="p-3 rounded-lg bg-muted/40 border border-border/80">
                <span class="text-xs text-muted-foreground block">{m.bulk_entry_preview_mandatory_fees()}</span>
                <span class="text-base font-bold text-foreground mt-1 block">{m.bulk_entry_preview_fee_count({ count: studentDetails.mandatoryFeesAssignedCount })}</span>
              </div>

              <div class="p-3 rounded-lg bg-muted/40 border border-border/80">
                <span class="text-xs text-muted-foreground block">{m.bulk_entry_preview_estimated_receivables()}</span>
                <span class="text-base font-bold text-primary mt-1 block">{formatNumber(studentDetails.totalEstimatedMandatoryFees)} {studentDetails.currency}</span>
              </div>
            </div>

            <!-- Class Placement Distribution -->
            {#if Object.keys(studentDetails.classPlacementsSummary).length > 0}
              <div class="p-3.5 rounded-xl bg-muted/30 border border-border">
                <span class="text-xs font-bold text-foreground block mb-2">{m.bulk_entry_preview_class_distribution()}</span>
                <div class="flex flex-wrap gap-2">
                  {#each Object.entries(studentDetails.classPlacementsSummary) as [className, count]}
                    <span class="px-2.5 py-1 rounded-md bg-card border border-border text-xs font-medium text-foreground flex items-center gap-1.5 shadow-xs">
                      <span>{className}</span>
                      <span class="px-1.5 py-0.2 rounded-full bg-primary/15 text-primary text-[10px] font-bold">{count}</span>
                    </span>
                  {/each}
                </div>
              </div>
            {/if}
          </div>
        {/if}

        <!-- Employee Specific Breakdown -->
        {#if employeeDetails}
          <div class="space-y-4">
            <h3 class="font-bold text-foreground flex items-center gap-2">
              <span class="h-2 w-2 rounded-full bg-primary"></span>
              {m.bulk_entry_preview_employees_section()}
            </h3>
            
            <div class="grid grid-cols-2 gap-3">
              <div class="p-3 rounded-lg bg-muted/40 border border-border/80">
                <span class="text-xs text-muted-foreground block">{m.bulk_entry_preview_new_employees()}</span>
                <span class="text-base font-bold text-foreground mt-1 block">{m.bulk_entry_preview_employee_count({ count: employeeDetails.employeesToCreate })}</span>
              </div>

              <div class="p-3 rounded-lg bg-muted/40 border border-border/80">
                <span class="text-xs text-muted-foreground block">{m.bulk_entry_preview_user_accounts()}</span>
                <span class="text-base font-bold text-primary mt-1 block">{m.bulk_entry_preview_account_count({ count: employeeDetails.userAccountsToProvision })}</span>
              </div>
            </div>
          </div>
        {/if}

        <!-- Grade Specific Breakdown -->
        {#if gradeDetails}
          <div class="space-y-4">
            <h3 class="font-bold text-foreground flex items-center gap-2">
              <span class="h-2 w-2 rounded-full bg-primary"></span>
              {m.bulk_entry_preview_grades_section()}
            </h3>
            
            <div class="grid grid-cols-2 gap-3">
              <div class="p-3 rounded-lg bg-muted/40 border border-border/80">
                <span class="text-xs text-muted-foreground block">{m.bulk_entry_preview_recorded_grades()}</span>
                <span class="text-base font-bold text-foreground mt-1 block">{m.bulk_entry_preview_grade_count({ count: gradeDetails.gradesToRecord })}</span>
              </div>

              <div class="p-3 rounded-lg bg-muted/40 border border-border/80">
                <span class="text-xs text-muted-foreground block">{m.bulk_entry_preview_assessment_type()}</span>
                <span class="text-base font-bold text-primary mt-1 block">{gradeDetails.assessmentType}</span>
              </div>
            </div>
          </div>
        {/if}

        <!-- Compliance & Security Notice -->
        <div class="p-3 rounded-xl bg-amber-500/10 border border-amber-500/20 text-amber-800 dark:text-amber-300 text-xs flex items-start gap-2.5">
          <span class="text-base leading-none mt-0.5">ℹ️</span>
          <span>
            {m.bulk_entry_preview_audit_notice()}
          </span>
        </div>
      </div>

      <!-- Footer -->
      <div class="px-6 py-4 border-t border-border bg-muted/30 flex items-center justify-between">
        <button
          onclick={onclose}
          class="px-4 py-2 rounded-xl border border-border text-foreground hover:bg-muted font-medium transition-colors"
        >
          {m.bulk_entry_preview_back_to_edit()}
        </button>

        <button
          onclick={onconfirm}
          disabled={summary.validRowsCount === 0}
          class="px-6 py-2 rounded-xl bg-primary text-primary-foreground font-bold hover:bg-primary/90 disabled:opacity-50 disabled:pointer-events-none transition-all shadow-md flex items-center gap-2"
        >
          <span>{m.bulk_entry_preview_confirm({ count: summary.validRowsCount })}</span>
          <span>◄</span>
        </button>
      </div>
    </div>
  </div>
{/if}
