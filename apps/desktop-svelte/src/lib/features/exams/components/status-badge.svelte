<!--
@component ExamStatusBadge
Colored status badge for exam session status.
Usage: <ExamStatusBadge status="active" />
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";

let { status, size = "sm" }: { status: string; size?: "xs" | "sm" } = $props();

const config = $derived(() => {
  switch (status) {
    case "draft":
      return {
        label: m.common_status_draft(),
        color: "bg-zinc-500/10 text-zinc-500 border-zinc-500/20",
      };
    case "scheduled":
      return {
        label: m.common_exams_status_scheduled(),
        color: "bg-blue-500/10 text-blue-500 border-blue-500/20",
      };
    case "active":
      return {
        label: m.common_phase_active(),
        color: "bg-emerald-500/10 text-emerald-500 border-emerald-500/20",
      };
    case "grading":
      return {
        label: m.common_exams_status_grading(),
        color: "bg-amber-500/10 text-amber-500 border-amber-500/20",
      };
    case "completed":
      return {
        label: m.common_exams_status_completed(),
        color: "bg-purple-500/10 text-purple-500 border-purple-500/20",
      };
    case "archived":
      return {
        label: m.common_year_status_archived(),
        color: "bg-zinc-600/10 text-zinc-600 border-zinc-600/20",
      };
    default:
      return {
        label: status,
        color: "bg-muted text-muted-foreground border-border",
      };
  }
});

const sizeClass = $derived(
  size === "xs" ? "text-[8px] px-1.5 py-0.5" : "text-[9px] px-2 py-0.5",
);
</script>

<span class="inline-flex items-center gap-1 rounded-md border font-bold select-none {config().color} {sizeClass}">
	<span class="h-1.5 w-1.5 rounded-full {config().color.includes('emerald') ? 'bg-emerald-500' : config().color.includes('blue') ? 'bg-blue-500' : config().color.includes('amber') ? 'bg-amber-500' : config().color.includes('purple') ? 'bg-purple-500' : 'bg-zinc-500'} animate-pulse"></span>
	{config().label}
</span>
