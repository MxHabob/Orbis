<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { AdjustmentsHorizontal, ChartBar, CheckCircle, Lock, Refresh, Users, XMark } from "@orbis/icons";
import { getLocale } from "$lib/paraglide/runtime";

import { formatNumber, formatDate } from "$lib/core/i18n";

interface Props {
  selectedPeriod: any;
  isPeriodLocked: boolean;
  isGradeEntryOpen: boolean;
  isPeriodActionLoading: boolean;
  isCompileLoading: boolean;
  stats: {
    avg: number;
    max: number;
    min: number;
    passCount: number;
    passRate: number;
    total: number;
    excellent: number;
    veryGood: number;
    good: number;
    acceptable: number;
    weak: number;
    fail: number;
  } | null;
  onLockPeriod: () => void;
  onUnlockPeriod: () => void;
  onOpenGradeEntry: () => void;
  onCloseGradeEntry: () => void;
  onCompileGrades: () => void;
}

let {
  selectedPeriod,
  isPeriodLocked,
  isGradeEntryOpen,
  isPeriodActionLoading,
  isCompileLoading,
  stats,
  onLockPeriod,
  onUnlockPeriod,
  onOpenGradeEntry,
  onCloseGradeEntry,
  onCompileGrades,
}: Props = $props();
</script>

<div class="h-full flex flex-col gap-6 p-6 overflow-y-auto scrollbar-hide border-e border-border/60 bg-muted/10 select-none">
  {#if selectedPeriod}
    <!-- Period Status Card -->
    <div class="bg-background/50 border border-border rounded-lg p-4 flex flex-col gap-3">
      <h3 class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">{m.common_control_period_status()}</h3>
      <div class="flex flex-col gap-2 text-xs">
        <div class="flex items-center justify-between">
          <span class="text-muted-foreground">{m.common_control_cycle_lock()}</span>
          <span class="flex items-center gap-1 font-semibold {isPeriodLocked ? 'text-destructive' : 'text-emerald-500'}">
            {#if isPeriodLocked}
              <Lock size={12} /> {m.common_phase_locked()}
            {:else}
              <Unlock size={12} /> {m.common_control_cycle_unlocked()}
            {/if}
          </span>
        </div>
        <div class="flex items-center justify-between">
          <span class="text-muted-foreground">{m.common_control_grade_entry()}</span>
          <span class="flex items-center gap-1 font-semibold {isGradeEntryOpen ? 'text-emerald-500' : 'text-amber-500'}">
            {#if isGradeEntryOpen}
              <CheckCircle size={12} /> {m.common_exams_open_section()}
            {:else}
              <XMark size={12} /> {m.common_phase_locked()}
            {/if}
          </span>
        </div>
        {#if selectedPeriod.grade_entry_start}
          <div class="flex items-center justify-between">
            <span class="text-muted-foreground">{m.common_control_grade_entry_start()}</span>
            <span class="font-mono text-[10px]">{formatDate(selectedPeriod.grade_entry_start)}</span>
          </div>
        {/if}
        {#if selectedPeriod.grade_entry_end}
          <div class="flex items-center justify-between">
            <span class="text-muted-foreground">{m.common_control_grade_entry_end()}</span>
            <span class="font-mono text-[10px]">{formatDate(selectedPeriod.grade_entry_end)}</span>
          </div>
        {/if}
        {#if selectedPeriod.locked_by}
          <div class="flex items-center justify-between">
            <span class="text-muted-foreground">{m.common_control_locked_by()}</span>
            <span class="text-[10px]">{selectedPeriod.locked_by}</span>
          </div>
        {/if}
      </div>
    </div>

    <!-- Action Buttons Card -->
    <div class="bg-background/50 border border-border rounded-lg p-4 flex flex-col gap-3">
      <h3 class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">{m.common_je_col_actions()}</h3>
      <div class="flex flex-col gap-2">
        {#if isPeriodLocked}
          <Button
            variant="outline"
            class="w-full justify-start gap-2 text-xs h-9 cursor-pointer border-emerald-500/30 text-emerald-600 hover:bg-emerald-500/10"
            onclick={onUnlockPeriod}
            disabled={isPeriodActionLoading}
          >
            {#if isPeriodActionLoading}
              <Loader2 size={14} class="animate-spin" />
            {:else}
              <Unlock size={14} />
            {/if}
            {m.common_control_unlock_period()}
          </Button>
        {:else}
          <Button
            variant="outline"
            class="w-full justify-start gap-2 text-xs h-9 cursor-pointer border-destructive/30 text-destructive hover:bg-destructive/10"
            onclick={onLockPeriod}
            disabled={isPeriodActionLoading}
          >
            {#if isPeriodActionLoading}
              <Loader2 size={14} class="animate-spin" />
            {:else}
              <Lock size={14} />
            {/if}
            {m.common_control_lock_period()}
          </Button>

          {#if isGradeEntryOpen}
            <Button
              variant="outline"
              class="w-full justify-start gap-2 text-xs h-9 cursor-pointer border-amber-500/30 text-amber-600 hover:bg-amber-500/10"
              onclick={onCloseGradeEntry}
              disabled={isPeriodActionLoading}
            >
              {#if isPeriodActionLoading}
                <Loader2 size={14} class="animate-spin" />
              {:else}
                <CalendarOff size={14} />
              {/if}
              {m.common_control_close_entry()}
            </Button>
          {:else}
            <Button
              variant="outline"
              class="w-full justify-start gap-2 text-xs h-9 cursor-pointer border-emerald-500/30 text-emerald-600 hover:bg-emerald-500/10"
              onclick={onOpenGradeEntry}
              disabled={isPeriodActionLoading}
            >
              {#if isPeriodActionLoading}
                <Loader2 size={14} class="animate-spin" />
              {:else}
                <CalendarClock size={14} />
              {/if}
              {m.common_control_open_entry()}
            </Button>
          {/if}
        {/if}

        <div class="h-px bg-border my-1"></div>

        <Button
          variant="outline"
          class="w-full justify-start gap-2 text-xs h-9 cursor-pointer border-indigo-500/30 text-indigo-600 hover:bg-indigo-500/10"
          onclick={onCompileGrades}
          disabled={isCompileLoading}
        >
          {#if isCompileLoading}
            <Loader2 size={14} class="animate-spin" />
          {:else}
            <Refresh size={14} />
          {/if}
          {m.common_control_compile_grades()}
        </Button>
      </div>
    </div>
  {/if}

  <!-- Stats Card -->
  {#if stats}
    <div class="bg-background/50 border border-border rounded-lg p-4 flex flex-col gap-3">
      <h3 class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider flex items-center gap-1.5">
        <ChartBar size={12} />
        {m.common_control_stats()}
      </h3>
      <div class="flex flex-col gap-2 text-xs">
        <div class="flex items-center justify-between">
          <span class="text-muted-foreground flex items-center gap-1"><Users size={12} /> {m.common_word_students()}</span>
          <span class="font-bold">{formatNumber(stats.total)}</span>
        </div>
        <div class="flex items-center justify-between">
          <span class="text-muted-foreground">{m.common_detail_gpa()}</span>
          <span class="font-bold text-indigo-600">{formatNumber(Math.round(stats.avg))}%</span>
        </div>
        <div class="flex items-center justify-between">
          <span class="text-muted-foreground">{m.common_control_pass_rate()}</span>
          <span class="font-bold {stats.passRate >= 75 ? 'text-emerald-600' : 'text-amber-600'}">{formatNumber(Math.round(stats.passRate))}%</span>
        </div>
        <div class="flex items-center justify-between">
          <span class="text-muted-foreground">{m.common_control_highest()}</span>
          <span class="font-semibold">{formatNumber(stats.max)}%</span>
        </div>
        <div class="flex items-center justify-between">
          <span class="text-muted-foreground">{m.common_control_lowest()}</span>
          <span class="font-semibold">{formatNumber(stats.min)}%</span>
        </div>

        <div class="h-px bg-border my-1"></div>

        <!-- Grade breakdown distribution -->
        <div class="flex flex-col gap-1.5 mt-1">
          <span class="text-[9px] font-bold text-muted-foreground uppercase">{m.common_control_grade_distribution()}</span>
          {@render StatBar("A (>= 90%)", stats.excellent, stats.total, "bg-emerald-500")}
          {@render StatBar("B (80-89%)", stats.veryGood, stats.total, "bg-indigo-500")}
          {@render StatBar("C (70-79%)", stats.good, stats.total, "bg-blue-500")}
          {@render StatBar("D (60-69%)", stats.acceptable, stats.total, "bg-amber-500")}
          {@render StatBar("E (50-59%)", stats.weak, stats.total, "bg-orange-500")}
          {@render StatBar("F (< 50%)", stats.fail, stats.total, "bg-rose-500")}
        </div>
      </div>
    </div>
  {/if}
</div>

{#snippet StatBar(label: string, count: number, total: number, colorClass: string)}
  {@const percentage = total > 0 ? (count / total) * 100 : 0}
  <div class="flex flex-col gap-0.5">
    <div class="flex items-center justify-between text-[10px] text-muted-foreground">
      <span>{label}</span>
      <span class="font-semibold text-foreground/80">{count} ({Math.round(percentage)}%)</span>
    </div>
    <div class="w-full bg-muted/60 h-1.5 rounded-full overflow-hidden">
      <div class="h-full {colorClass} rounded-full" style="width: {percentage}%"></div>
    </div>
  </div>
{/snippet}
