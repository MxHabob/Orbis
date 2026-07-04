<!--
@component AcademicsMainPage
Landing / dashboard overview page for Academics.
Provides cards linking to sub-sections and brief stats.
-->
<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import {
  GraduationCap,
  BookOpen,
  Calendar,
  Award,
  ArrowRight,
  ShieldCheck,
  Milestone,
  Compass,
} from "lucide-svelte";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";
import { fade, scale } from "svelte/transition";
import * as m from "$lib/paraglide/messages";

let { data }: { data: { orgId: string } } = $props();

// State
let classesCount = $state(0);
let subjectsCount = $state(0);
let currentCycleName = $state("");
let isLoading = $state(true);

async function loadOverview() {
  isLoading = true;
  try {
    const [classList, subjectList, cycleList] = await Promise.all([
      commands.classes.getAll(data.orgId),
      commands.subjects.getAll(data.orgId),
      commands.academics.getCycles(data.orgId),
    ]);
    classesCount = classList.length;
    subjectsCount = subjectList.length;

    const active =
      cycleList.find((c) => c.is_current === 1 || c.is_current === true) ||
      cycleList[0];
    if (active) {
      currentCycleName = String(active.name_ar || active.name || "");
    }
  } catch (e) {
    console.error("Error loading academics overview:", e);
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (data.orgId) {
    loadOverview();
  }
});

const cards = $derived([
  {
    path: "classes",
    title: m.academics_overview_card_classes_title(),
    desc: m.academics_overview_card_classes_desc(),
    icon: GraduationCap,
    color: "bg-blue-500/10 text-blue-500 border-blue-500/20",
    badge: m.academics_overview_card_classes_badge({ count: classesCount }),
  },
  {
    path: "subjects",
    title: m.academics_overview_card_subjects_title(),
    desc: m.academics_overview_card_subjects_desc(),
    icon: BookOpen,
    color: "bg-emerald-500/10 text-emerald-500 border-emerald-500/20",
    badge: m.academics_overview_card_subjects_badge({ count: subjectsCount }),
  },
  {
    path: "schedule",
    title: m.academics_overview_card_schedule_title(),
    desc: m.academics_overview_card_schedule_desc(),
    icon: Calendar,
    color: "bg-purple-500/10 text-purple-500 border-purple-500/20",
    badge: m.academics_overview_card_schedule_badge(),
  },
  {
    path: "assessments",
    title: m.academics_overview_card_assessments_title(),
    desc: m.academics_overview_card_assessments_desc(),
    icon: Award,
    color: "bg-amber-500/10 text-amber-500 border-amber-500/20",
    badge: m.academics_overview_card_assessments_badge(),
  },
]);
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 select-none">
	<!-- Welcome Banner & Info -->
	<div 
		class="card-tactile bg-gradient-to-br from-card to-card/50 p-6 flex flex-col md:flex-row md:items-center md:justify-between gap-4"
		in:fade={{ duration: 150 }}
	>
		<div class="flex items-center gap-4 text-start">
			<div class="h-12 w-12 rounded-xl bg-primary/10 text-primary flex items-center justify-center shrink-0">
				<Compass size={24} />
			</div>
			<div class="flex flex-col">
				<h2 class="text-sm font-bold text-foreground uppercase tracking-wider">
					{m.academics_overview_title()}
				</h2>
				<p class="text-[10px] text-muted-foreground mt-0.5 max-w-lg">
					{m.academics_overview_subtitle()}
				</p>
			</div>
		</div>

		<!-- Academic Year Info -->
		{#if currentCycleName}
			<div class="shrink-0 flex items-center gap-2 bg-background/50 border border-border px-3 py-1.5 rounded-lg text-[10px]">
				<Milestone size={13} class="text-primary" />
				<span class="font-bold text-muted-foreground">{m.academics_overview_active_cycle()}</span>
				<span class="font-bold text-foreground">{currentCycleName}</span>
			</div>
		{/if}
	</div>

	<!-- Navigation Cards Grid -->
	<div class="grid grid-cols-1 sm:grid-cols-2 gap-4" in:fade={{ delay: 50, duration: 200 }}>
		{#each cards as card (card.path)}
			<div class="card-tactile bg-card flex flex-col justify-between p-6 h-52 text-start transition-all duration-200 hover:-translate-y-1 relative group">
				<div class="flex items-start justify-between">
					<!-- Colored themed icon -->
					<div class="h-10 w-10 rounded-lg border flex items-center justify-center shrink-0 transition-transform duration-200 group-hover:scale-105 {card.color}">
						<card.icon size={20} />
					</div>
					<!-- Badge indicator -->
					<span class="px-2 py-0.5 rounded border border-border bg-background/40 font-mono text-[9px] text-muted-foreground">
						{card.badge}
					</span>
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
						href={localizeHref(`/${data.orgId}/academics/${card.path}`)}
						variant="ghost"
						class="text-[10px] font-bold text-primary group-hover:underline h-7.5 px-3 flex items-center gap-1 cursor-pointer"
					>
						<span>{m.academics_overview_open_section()}</span>
						<ArrowRight size={11} class="transition-transform group-hover:translate-x-1" />
					</Button>
				</div>
			</div>
		{/each}
	</div>
</div>
