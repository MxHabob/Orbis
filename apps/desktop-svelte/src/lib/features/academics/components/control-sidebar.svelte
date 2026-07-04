<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import {
  Lock,
  Unlock,
  CalendarClock,
  CalendarOff,
  CheckCircle2,
  XCircle,
  Loader2,
  RefreshCw,
  BarChart3,
  Users,
} from "lucide-svelte";
import { getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { formatNumber } from "$lib/core/i18n";

interface Props {
  selectedCycle: any;
  isCycleLocked: boolean;
  isGradeEntryOpen: boolean;
  isCycleActionLoading: boolean;
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
  onLockCycle: () => void;
  onUnlockCycle: () => void;
  onOpenGradeEntry: () => void;
  onCloseGradeEntry: () => void;
  onCompileGrades: () => void;
}

let {
  selectedCycle,
  isCycleLocked,
  isGradeEntryOpen,
  isCycleActionLoading,
  isCompileLoading,
  stats,
  onLockCycle,
  onUnlockCycle,
  onOpenGradeEntry,
  onCloseGradeEntry,
  onCompileGrades,
}: Props = $props();
</script>

<div class="h-full flex flex-col gap-6 p-6 overflow-y-auto scrollbar-hide border-e border-border/60 bg-muted/10 select-none">
  {#if selectedCycle}
    <!-- Cycle Status Card -->
    <div class="bg-background/50 border border-border rounded-lg p-4 flex flex-col gap-3">
      <h3 class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">{m.control_cycle_status()}</h3>
      <div class="flex flex-col gap-2 text-xs">
        <div class="flex items-center justify-between">
          <span class="text-muted-foreground">{m.control_cycle_lock()}</span>
          <span class="flex items-center gap-1 font-semibold {isCycleLocked ? 'text-destructive' : 'text-emerald-500'}">
            {#if isCycleLocked}
              <Lock size={12} /> {m.control_cycle_locked()}
            {:else}
              <Unlock size={12} /> {m.control_cycle_unlocked()}
            {/if}
          </span>
        </div>
        <div class="flex items-center justify-between">
          <span class="text-muted-foreground">{m.control_grade_entry()}</span>
          <span class="flex items-center gap-1 font-semibold {isGradeEntryOpen ? 'text-emerald-500' : 'text-amber-500'}">
            {#if isGradeEntryOpen}
              <CheckCircle2 size={12} /> {m.control_grade_entry_open()}
            {:else}
              <XCircle size={12} /> {m.control_grade_entry_closed()}
            {/if}
          </span>
        </div>
        {#if selectedCycle.grade_entry_start}
          <div class="flex items-center justify-between">
            <span class="text-muted-foreground">{m.control_grade_entry_start()}</span>
            <span class="font-mono text-[10px]">{new Date(selectedCycle.grade_entry_start).toLocaleDateString(getLocale())}</span>
          </div>
        {/if}
        {#if selectedCycle.grade_entry_end}
          <div class="flex items-center justify-between">
            <span class="text-muted-foreground">{m.control_grade_entry_end()}</span>
            <span class="font-mono text-[10px]">{new Date(selectedCycle.grade_entry_end).toLocaleDateString(getLocale())}</span>
          </div>
        {/if}
        {#if selectedCycle.locked_by}
          <div class="flex items-center justify-between">
            <span class="text-muted-foreground">{m.control_locked_by()}</span>
            <span class="text-[10px]">{selectedCycle.locked_by}</span>
          </div>
        {/if}
      </div>
    </div>

    <!-- Action Buttons Card -->
    <div class="bg-background/50 border border-border rounded-lg p-4 flex flex-col gap-3">
      <h3 class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">{m.control_actions()}</h3>
      <div class="flex flex-col gap-2">
        {#if isCycleLocked}
          <Button
            variant="outline"
            class="w-full justify-start gap-2 text-xs h-9 cursor-pointer border-emerald-500/30 text-emerald-600 hover:bg-emerald-500/10"
            onclick={onUnlockCycle}
            disabled={isCycleActionLoading}
          >
            {#if isCycleActionLoading}
              <Loader2 size={14} class="animate-spin" />
            {:else}
              <Unlock size={14} />
            {/if}
            {m.control_unlock_cycle()}
          </Button>
        {:else}
          <Button
            variant="outline"
            class="w-full justify-start gap-2 text-xs h-9 cursor-pointer border-destructive/30 text-destructive hover:bg-destructive/10"
            onclick={onLockCycle}
            disabled={isCycleActionLoading}
          >
            {#if isCycleActionLoading}
              <Loader2 size={14} class="animate-spin" />
            {:else}
              <Lock size={14} />
            {/if}
            {m.control_lock_cycle()}
          </Button>

          {#if isGradeEntryOpen}
            <Button
              variant="outline"
              class="w-full justify-start gap-2 text-xs h-9 cursor-pointer border-amber-500/30 text-amber-600 hover:bg-amber-500/10"
              onclick={onCloseGradeEntry}
              disabled={isCycleActionLoading}
            >
              {#if isCycleActionLoading}
                <Loader2 size={14} class="animate-spin" />
              {:else}
                <CalendarOff size={14} />
              {/if}
              {m.control_close_entry()}
            </Button>
          {:else}
            <Button
              variant="outline"
              class="w-full justify-start gap-2 text-xs h-9 cursor-pointer border-emerald-500/30 text-emerald-600 hover:bg-emerald-500/10"
              onclick={onOpenGradeEntry}
              disabled={isCycleActionLoading}
            >
              {#if isCycleActionLoading}
                <Loader2 size={14} class="animate-spin" />
              {:else}
                <CalendarClock size={14} />
              {/if}
              {m.control_open_entry()}
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
            <RefreshCw size={14} />
          {/if}
          {m.control_compile_grades()}
        </Button>
      </div>
    </div>
  {/if}

  <!-- Stats Card -->
  {#if stats}
    <div class="bg-background/50 border border-border rounded-lg p-4 flex flex-col gap-3">
      <h3 class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider flex items-center gap-1.5">
        <BarChart3 size={12} />
        {m.control_stats()}
      </h3>
      <div class="flex flex-col gap-2 text-xs">
        <div class="flex items-center justify-between">
          <span class="text-muted-foreground flex items-center gap-1"><Users size={12} /> {m.control_total_students()}</span>
          <span class="font-bold">{formatNumber(stats.total)}</span>
        </div>
        <div class="flex items-center justify-between">
          <span class="text-muted-foreground">{m.control_average()}</span>
          <span class="font-bold text-indigo-600">{formatNumber(Math.round(stats.avg))}%</span>
        </div>
        <div class="flex items-center justify-between">
          <span class="text-muted-foreground">{m.control_pass_rate()}</span>
          <span class="font-bold {stats.passRate >= 75 ? 'text-emerald-600' : 'text-amber-600'}">{formatNumber(Math.round(stats.passRate))}%</span>
        </div>
        <div class="flex items-center justify-between">
          <span class="text-muted-foreground">{m.control_highest()}</span>
          <span class="font-semibold">{formatNumber(stats.max)}%</span>
        </div>
        <div class="flex items-center justify-between">
          <span class="text-muted-foreground">{m.control_lowest()}</span>
          <span class="font-semibold">{formatNumber(stats.min)}%</span>
        </div>

        <div class="h-px bg-border my-1"></div>

        <!-- Grade breakdown distribution -->
        <div class="flex flex-col gap-1.5 mt-1">
          <span class="text-[9px] font-bold text-muted-foreground uppercase">{m.control_grade_distribution()}</span>
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
