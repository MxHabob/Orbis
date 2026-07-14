<!--
@component PeriodStatusBar
Shows the current academic period status with gate windows.
Place at the top of any page that operates within a period context.

Usage:
  <PeriodStatusBar periodId={activePeriod.id} />
-->
<script lang="ts">
import { commands } from "$lib/core/ipc";
import { createQuery } from "@tanstack/svelte-query";
import { Badge } from "@orbis/ui-svelte/badge";
import { CalendarClock, BookOpen, ClipboardCheck, Lock, Unlock } from "lucide-svelte";
import type { GateVerdict } from "@orbis/sdk-core";

interface Props {
  periodId: string;
  showGates?: ("enrollment" | "grade_entry" | "attendance" | "exam_entry")[];
}

let { periodId, showGates = ["enrollment", "grade_entry"] }: Props = $props();

// Query all relevant gates in parallel
const gateQueries = $derived(
  showGates.map((gate) =>
    createQuery({
      queryKey: ["period-gate", gate, periodId],
      queryFn: () => commands.academic.checkPeriodGate(gate, periodId),
      staleTime: 30_000,
      enabled: !!periodId,
    }),
  ),
);

const gateLabels: Record<string, { label: string; icon: typeof BookOpen }> = {
  enrollment: { label: "التسجيل", icon: BookOpen },
  grade_entry: { label: "الدرجات", icon: ClipboardCheck },
  attendance: { label: "الحضور", icon: CalendarClock },
  exam_entry: { label: "الاختبارات", icon: CalendarClock },
};

const statusColors: Record<string, string> = {
  allowed: "bg-emerald-500/15 text-emerald-600 border-emerald-500/30",
  late: "bg-amber-500/15 text-amber-600 border-amber-500/30",
  requires_override: "bg-orange-500/15 text-orange-600 border-orange-500/30",
  blocked: "bg-red-500/15 text-red-600 border-red-500/30",
};

const statusIcons: Record<string, typeof Lock> = {
  allowed: Unlock,
  late: CalendarClock,
  requires_override: CalendarClock,
  blocked: Lock,
};

const statusLabels: Record<string, string> = {
  allowed: "مفتوح",
  late: "متأخر",
  requires_override: "يتطلب تجاوز",
  blocked: "مغلق",
};

function formatDate(d?: string | null): string {
  if (!d) return "";
  try {
    return new Date(d).toLocaleDateString("ar-SA", {
      month: "short",
      day: "numeric",
    });
  } catch {
    return d;
  }
}
</script>

<div
  class="flex items-center gap-3 rounded-lg border border-border/50 bg-card/50 px-4 py-2 text-sm backdrop-blur-sm"
>
  {#each showGates as gate, i}
    {@const query = gateQueries[i]}
    {@const verdict = $query?.data as GateVerdict | undefined}
    {@const info = gateLabels[gate]}

    {#if verdict}
      {@const StatusIcon = statusIcons[verdict.decision] ?? Lock}
      <div class="flex items-center gap-1.5">
        <svelte:component this={info.icon} class="h-3.5 w-3.5 opacity-60" />
        <span class="font-medium opacity-70">{info.label}:</span>
        <Badge variant="outline" class={statusColors[verdict.decision] ?? ""}>
          <svelte:component this={StatusIcon} class="mr-1 h-3 w-3" />
          {statusLabels[verdict.decision] ?? verdict.decision}
        </Badge>
        {#if verdict.context.window_end}
          <span class="text-xs opacity-50">
            حتى {formatDate(verdict.context.window_end)}
          </span>
        {/if}
      </div>

      {#if i < showGates.length - 1}
        <div class="h-4 w-px bg-border/50"></div>
      {/if}
    {:else if $query?.isLoading}
      <div class="flex items-center gap-1.5 animate-pulse">
        <span class="opacity-50">{info.label}: ...</span>
      </div>
    {/if}
  {/each}
</div>
