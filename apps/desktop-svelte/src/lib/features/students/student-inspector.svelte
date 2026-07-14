<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { authStore } from "$lib/stores";
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Avatar, AvatarFallback, AvatarImage } from "@orbis/ui-svelte";
import { Badge } from "@orbis/ui-svelte/badge";
import { X, ExternalLink, CreditCard, Award } from "lucide-svelte";

import { localizeHref } from "$lib/paraglide/runtime";
import { formatCurrency, formatDate, formatPercent } from "$lib/core/i18n";
import { academicContext } from "$lib/stores";
import { fade } from "svelte/transition";
import type { JsonRecord } from "@orbis/sdk-core";

let { student, onClose }: { student: any; onClose: () => void } = $props();

const studentId = $derived(student?.id || "");
const orgId = $derived(student?.org_id || authStore.orgId || "");

let balanceDue = $state<string>(m.common_loading());
let gradeSummary = $state<string>(m.common_loading());
let guardiansList = $state<any[]>([]);
let isLoading = $state(true);

async function fetchStats() {
  if (!orgId || !studentId) return;
  balanceDue = m.common_loading();
  gradeSummary = m.common_loading();
  guardiansList = [];
  isLoading = true;
  try {
    const periodId =
      academicContext.effectivePeriodId || student.period_id || "";

    // Fetch finances
    const [fees, payments] = await Promise.all([
      commands.finance.listStudentFees(studentId).catch(() => []),
      commands.finance
        .listPayments(orgId)
        .then((list) => list.filter((p: any) => p.enrollment_id === studentId))
        .catch(() => []),
    ]);

    const totalFees = fees.reduce((sum, f) => sum + Number(f.amount || 0), 0);
    const totalPaid = payments.reduce(
      (sum, p) => sum + Number(p.amount || 0),
      0,
    );
    const diff = totalFees - totalPaid;
    balanceDue = diff <= 0 ? m.common_loading_no_dues() : formatCurrency(diff);

    // Fetch term_grades report card summary
    if (periodId) {
      const report = await commands.assessment
        .getReportCard(studentId, periodId)
        .catch(() => null);
      if (report && Array.isArray(report.term_grades)) {
        const avg =
          report.term_grades.reduce(
            (sum: number, g: any) => sum + Number(g.score || 0),
            0,
          ) / (report.term_grades.length || 1);
        gradeSummary = formatPercent(avg);
      } else {
        gradeSummary = m.common_loading_no_data();
      }
    } else {
      gradeSummary = m.common_loading_no_data();
    }

    // Fetch guardians list
    const links = await commands.academic.listGuardiansByStudent(student.student_id || student.id,
    );
    if (links) {
      const resolved = [];
      for (const link of links) {
        try {
          const user = await commands.iam.getUser(String(link.guardian_id));
          if (user) {
            resolved.push({
              name: user.full_name,
              phone: user.phone || "",
              relationship:
                link.relationship || link.relationship || "ولي أمر",
              is_primary: link.is_primary === 1,
            });
          }
        } catch (err) {
          console.error(err);
        }
      }
      guardiansList = resolved;
    }
  } catch (e) {
    balanceDue = m.common_loading_error();
    gradeSummary = m.common_loading_no_data();
  } finally {
    isLoading = false;
  }
}

$effect(() => {
  if (studentId && orgId) {
    fetchStats();
  }
});
</script>

{#if student}
  <div class="h-full flex flex-col overflow-hidden text-xs text-foreground bg-card">
    <!-- Header -->
    <div class="flex items-center justify-between p-4 border-b border-border/40 shrink-0">
      <div class="flex items-center gap-2">
        <Button
          href={localizeHref(`/${orgId}/students/${studentId}`)}
          variant="ghost"
          class="text-muted-foreground hover:text-foreground h-8 px-2 cursor-pointer"
          title={m.students_inspector_view_full()}
        >
          <ExternalLink size={14} />
        </Button>
      </div>
      
      <Button
        variant="ghost"
        class="text-muted-foreground hover:text-foreground h-8 px-2 cursor-pointer"
        onclick={onClose}
      >
        <X size={14} />
      </Button>
    </div>

    <!-- Main Profile Section -->
    <div class="flex-1 overflow-y-auto flex flex-col gap-6 scrollbar-hide p-4 select-none">
      <div class="flex flex-col items-center text-center gap-3">
        <Avatar class="w-[85%] max-w-[180px] min-w-[120px] aspect-square h-auto rounded-xl animate-fade-in">
          <AvatarImage src={student.avatar ? String(student.avatar) : undefined} alt={student.full_name ? String(student.full_name) : ''} />
          <AvatarFallback class="text-5xl font-bold rounded-xl flex items-center justify-center bg-muted text-muted-foreground">
            {String(student.full_name || student.name_ar || "S").slice(0, 1)}
          </AvatarFallback>
        </Avatar>
        <div class="space-y-1">
          <h3 class="text-sm font-bold text-foreground">{student.full_name || student.name_ar || ''}</h3>
          <div class="flex justify-center">
            <Badge variant={student.status === 'active' ? 'lime' : student.status === 'draft' ? 'outline' : 'secondary'} class="text-[9px] scale-95 font-semibold px-2 py-0.5">
              {student.status === 'active' ? m.common_phase_active() : 
               student.status === 'draft' ? m.common_status_draft() :
               student.status === 'pending_review' ? m.common_form_status_pending_review() :
               student.status === 'withdrawn' ? m.common_status_withdrawn() :
               student.status === 'graduated' ? m.common_form_status_graduated() :
               student.status}
            </Badge>
          </div>
        </div>
      </div>

      <div class="grid grid-cols-2 gap-2 text-center">
        <div class="flex flex-col items-center justify-center p-2 rounded-xl bg-muted/20 border border-border/40 hover:bg-muted/30 transition-colors">
          <Award size={14} class="text-primary mb-1" />
          <span class="text-[9px] text-muted-foreground">{m.students_inspector_grade_avg()}</span>
          <span class="font-bold text-foreground mt-0.5">{gradeSummary}</span>
        </div>
        <div class="flex flex-col items-center justify-center p-2 rounded-xl bg-muted/20 border border-border/40 hover:bg-muted/30 transition-colors">
          <CreditCard size={14} class="text-emerald-500 mb-1" />
          <span class="text-[9px] text-muted-foreground">{m.students_inspector_balance_due()}</span>
          <span class="font-bold text-foreground mt-0.5">{balanceDue}</span>
        </div>
      </div>

      <!-- Student Metadata List -->
      <div class="flex flex-col gap-4 border-t border-border/40 pt-4 w-full text-start">
        {#if student.student_number || student.id}
          <div class="flex flex-col gap-0.5">
            <span class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider">
              {m.common_word_academic_number()}
            </span>
            <span class="text-xs font-mono font-bold text-foreground">
              {student.student_number || student.id}
            </span>
          </div>
        {/if}

        {#if student.phone || student.phone_number}
          <div class="flex flex-col gap-0.5">
            <span class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider">
              {m.common_word_phone()}
            </span>
            <span class="text-xs text-foreground font-mono">
              {student.phone || student.phone_number}
            </span>
          </div>
        {/if}

        {#if student.email}
          <div class="flex flex-col gap-0.5">
            <span class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider">
              {m.common_word_email()}
            </span>
            <span class="text-xs text-foreground font-mono truncate max-w-full" title={String(student.email)}>
              {student.email}
            </span>
          </div>
        {/if}

        {#if student.sex}
          <div class="flex flex-col gap-0.5">
            <span class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider">
              {m.common_form_sex()}
            </span>
            <span class="text-xs text-foreground font-medium">
              {student.sex === 'male' ? m.common_form_male() : m.common_form_female()}
            </span>
          </div>
        {/if}

        {#if student.birth_date}
          <div class="flex flex-col gap-0.5">
            <span class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider">
              {m.common_word_dob()}
            </span>
            <span class="text-xs text-foreground font-mono">
              {student.birth_date}
            </span>
          </div>
        {/if}
      </div>

      <!-- Linked Guardians Section -->
      {#if guardiansList.length > 0}
        <div class="flex flex-col gap-2 border-t border-border/40 pt-4 w-full text-start">
          <span class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider">
            {m.students_inspector_guardians_title()}
          </span>
          <div class="flex flex-col gap-2 pt-1">
            {#each guardiansList as g}
              <div class="flex items-center justify-between p-2.5 border border-border/60 rounded-xl bg-muted/20">
                <div class="flex flex-col gap-0.5">
                  <span class="font-bold text-foreground text-xs">{g.name} ({g.relationship})</span>
                  <span class="text-[10px] text-muted-foreground font-mono">{g.phone || m.common_form_guardian_no_phone()}</span>
                </div>
                {#if g.is_primary}
                  <Badge variant="lime" class="text-[8px] scale-90 px-1.5 py-0.5 shrink-0 font-semibold uppercase">{m.common_form_guardian_role_primary()}</Badge>
                {/if}
              </div>
            {/each}
          </div>
        </div>
      {/if}
    </div>
  </div>
{/if}
