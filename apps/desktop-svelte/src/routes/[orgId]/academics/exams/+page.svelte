<!--
@component ExamsOverview
Dashboard overview page for Examination Management.
Shows stats cards and navigation cards.
-->
<script lang="ts">
import { onMount } from "svelte";
import { Button } from "@orbis/ui-svelte/button";
import {
  ClipboardList,
  ListOrdered,
  Calendar,
  FileText,
  ArrowRight,
  BarChart3,
  Users,
  Building2,
  UserCheck,
} from "lucide-svelte";
import { localizeHref } from "$lib/paraglide/runtime";
import { fade } from "svelte/transition";
import * as m from "$lib/paraglide/messages";

let { data }: { data: { orgId: string } } = $props();

let isLoading = $state(true);
let sessionsCount = $state(0);
let studentsCount = $state(0);
let hallsCount = $state(0);
let proctorsCount = $state(0);

async function loadOverview() {
  isLoading = true;
  try {
    // TODO: Wire to IPC commands when available
    sessionsCount = 0;
    studentsCount = 0;
    hallsCount = 0;
    proctorsCount = 0;
  } catch (e) {
    console.error("Error loading exams overview:", e);
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (data.orgId) {
    loadOverview();
  }
});

const stats = $derived([
  {
    label: m.exams_stat_sessions(),
    value: sessionsCount,
    icon: ClipboardList,
    color: "text-blue-500 bg-blue-500/10",
  },
  {
    label: m.exams_stat_students(),
    value: studentsCount,
    icon: Users,
    color: "text-emerald-500 bg-emerald-500/10",
  },
  {
    label: m.exams_stat_halls(),
    value: hallsCount,
    icon: Building2,
    color: "text-purple-500 bg-purple-500/10",
  },
  {
    label: m.exams_stat_proctors(),
    value: proctorsCount,
    icon: UserCheck,
    color: "text-amber-500 bg-amber-500/10",
  },
]);

const cards = $derived([
  {
    path: "sessions",
    title: m.exams_card_sessions_title(),
    desc: m.exams_card_sessions_desc(),
    icon: ClipboardList,
    color: "bg-blue-500/10 text-blue-500 border-blue-500/20",
  },
  {
    path: "ordering",
    title: m.exams_card_ordering_title(),
    desc: m.exams_card_ordering_desc(),
    icon: ListOrdered,
    color: "bg-emerald-500/10 text-emerald-500 border-emerald-500/20",
  },
  {
    path: "sessions",
    title: m.exams_card_schedule_title(),
    desc: m.exams_card_schedule_desc(),
    icon: Calendar,
    color: "bg-purple-500/10 text-purple-500 border-purple-500/20",
  },
  {
    path: "reports",
    title: m.exams_card_reports_title(),
    desc: m.exams_card_reports_desc(),
    icon: FileText,
    color: "bg-amber-500/10 text-amber-500 border-amber-500/20",
  },
]);
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 select-none">
	<!-- Header Banner -->
	<div
		class="card-tactile bg-gradient-to-br from-card to-card/50 p-6 flex flex-col md:flex-row md:items-center md:justify-between gap-4"
		in:fade={{ duration: 150 }}
	>
		<div class="flex items-center gap-4 text-start">
			<div class="h-12 w-12 rounded-xl bg-primary/10 text-primary flex items-center justify-center shrink-0">
				<BarChart3 size={24} />
			</div>
			<div class="flex flex-col">
				<h2 class="text-sm font-bold text-foreground uppercase tracking-wider">
					{m.exams_overview_title()}
				</h2>
				<p class="text-[10px] text-muted-foreground mt-0.5 max-w-lg">
					{m.exams_overview_subtitle()}
				</p>
			</div>
		</div>
	</div>

	<!-- Stats Row -->
	<div class="grid grid-cols-2 sm:grid-cols-4 gap-3" in:fade={{ delay: 50, duration: 200 }}>
		{#each stats as stat (stat.label)}
			<div class="card-tactile bg-card p-4 flex items-center gap-3">
				<div class="h-9 w-9 rounded-lg {stat.color} flex items-center justify-center shrink-0">
					<stat.icon size={16} />
				</div>
				<div class="flex flex-col">
					<span class="text-lg font-bold text-foreground font-mono">{stat.value}</span>
					<span class="text-[9px] text-muted-foreground uppercase tracking-wide">{stat.label}</span>
				</div>
			</div>
		{/each}
	</div>

	<!-- Navigation Cards -->
	<div class="grid grid-cols-1 sm:grid-cols-2 gap-4" in:fade={{ delay: 100, duration: 200 }}>
		{#each cards as card (card.title)}
			<div class="card-tactile bg-card flex flex-col justify-between p-6 h-44 text-start transition-all duration-200 hover:-translate-y-1 relative group">
				<div class="flex items-start justify-between">
					<div class="h-10 w-10 rounded-lg border flex items-center justify-center shrink-0 transition-transform duration-200 group-hover:scale-105 {card.color}">
						<card.icon size={20} />
					</div>
				</div>

				<div class="flex flex-col gap-1.5 my-3">
					<h3 class="text-xs font-bold text-foreground group-hover:text-primary transition-colors">
						{card.title}
					</h3>
					<p class="text-[10px] text-muted-foreground leading-relaxed line-clamp-2">
						{card.desc}
					</p>
				</div>

				<div class="border-t border-border/60 pt-3 flex items-center justify-end">
					<Button
						href={localizeHref(`/${data.orgId}/academics/exams/${card.path}`)}
						variant="ghost"
						class="text-[10px] font-bold text-primary group-hover:underline h-7.5 px-3 flex items-center gap-1 cursor-pointer"
					>
						<span>{m.exams_open_section()}</span>
						<ArrowRight size={11} class="transition-transform group-hover:translate-x-1" />
					</Button>
				</div>
			</div>
		{/each}
	</div>
</div>
