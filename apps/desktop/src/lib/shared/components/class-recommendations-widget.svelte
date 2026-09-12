<script lang="ts">
import { createQuery } from "@tanstack/svelte-query";
import { api } from "$lib/api";
import { Skeleton } from "@orbis/ui/skeleton";
import { Sparkles, AlertTriangle, CheckCircle2, Info } from "@lucide/svelte";
import { getLocale } from "$lib/paraglide/runtime";
import WorkspaceCard from "./workspace-card.svelte";

let { classId, periodId }: { classId: string; periodId: string } = $props();

const isArabic = $derived(getLocale() === "ar");

// Fetch metrics to generate contextual recommendations
const classMetricsQuery = createQuery(() => ({
  queryKey: ["class-metrics", classId, periodId],
  queryFn: () => api.listGrades().catch(() => []),
  enabled: !!classId && !!periodId,
  staleTime: 5 * 60 * 1000,
}));

const metrics = $derived((classMetricsQuery.data || []) as any[]);
const isLoading = $derived(classMetricsQuery.isLoading);

const attendanceRate = $derived.by(() => {
  if (metrics.length === 0) return null;
  const m = metrics[0];
  if (!m || !m.total_attendance_records || m.total_attendance_records === 0)
    return null;
  return Math.round((m.present_records / m.total_attendance_records) * 100);
});

const recommendations = $derived.by(() => {
  if (isLoading || attendanceRate === null) return [];

  const recs = [];

  // Attendance based recommendation
  if (attendanceRate < 90) {
    recs.push({
      id: "attendance-low",
      title: "Review Attendance Records",
      desc: `Overall attendance is ${attendanceRate}%. Action required.`,
      icon: AlertTriangle,
      colorClass: "bg-rose-100 text-rose-600",
    });
  } else {
    recs.push({
      id: "attendance-good",
      title: "Attendance on Track",
      desc: `Overall attendance is healthy at ${attendanceRate}%.`,
      icon: CheckCircle2,
      colorClass: "bg-green-100 text-green-600",
    });
  }

  return recs;
});
</script>

<WorkspaceCard title={isArabic ? "توصيات الذكاء الاصطناعي" : "AI Recommendations"}>
  {#snippet actions()}
    <Sparkles size={14} class="text-primary animate-pulse" />
  {/snippet}

  {#if isLoading}
    <div class="flex flex-col gap-3 h-full">
      {#each Array(3) as _}
        <Skeleton class="h-14 w-full rounded-lg" />
      {/each}
    </div>
  {:else}
    <div class="flex flex-col gap-3 h-full overflow-y-auto scrollbar-hide">
      {#each recommendations as rec}
        <div class="flex items-center gap-3 p-3 rounded-md bg-muted/10 hover:bg-muted/30 transition-colors cursor-pointer group">
          <div class={`flex items-center justify-center w-8 h-8 rounded-md shrink-0 ${rec.colorClass}`}>
            <rec.icon size={14} />
          </div>
          <div class="flex flex-col min-w-0">
            <span class="font-bold text-xs text-foreground group-hover:text-primary transition-colors">{rec.title}</span>
            <span class="text-[9px] text-muted-foreground truncate">{rec.desc}</span>
          </div>
          <Info size={12} class="text-muted-foreground opacity-50 ml-auto shrink-0 group-hover:opacity-100 transition-opacity" />
        </div>
      {/each}
    </div>
  {/if}
</WorkspaceCard>
