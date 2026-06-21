<!--
@component AcademicsLayout
Wraps all Academics routes inside a GitButler-style resizable double-sidebar layout
using `<MainViewport>`. Renders vertical sidebar links with live counts.
-->
<script lang="ts">
import { page } from "$app/state";
import { onMount } from "svelte";
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { deLocalizeUrl, localizeHref, getLocale } from "$lib/paraglide/runtime";
import { GraduationCap, BookOpen, Calendar, Award, Compass, ChevronRight } from "lucide-svelte";
import MainViewport from "$lib/components/layout/MainViewport.svelte";
import type { Snippet } from "svelte";

let { data, children }: { data: { orgId: string }; children: Snippet } = $props();

const isAr = $derived(getLocale() === "ar");

// Sidebar live statistics
let classesCount = $state(0);
let subjectsCount = $state(0);

async function loadSidebarStats() {
  try {
    const [classList, subjectList] = await Promise.all([
      commands.classes.getAll(data.orgId),
      commands.subjects.getAll(data.orgId),
    ]);
    classesCount = classList.length;
    subjectsCount = subjectList.length;
  } catch (e) {
    console.error("Error loading sidebar stats:", e);
  }
}

// Reactively reload stats on route pathname change
$effect(() => {
  const path = page.url.pathname;
  if (data.orgId) {
    loadSidebarStats();
  }
});

// Active path helper
function isActive(pathSegment: string): boolean {
  const unlocalizedPathname = deLocalizeUrl(page.url).pathname;
  const baseUrl = `/${data.orgId}/academics`;
  if (pathSegment === "") {
    return unlocalizedPathname === baseUrl;
  }
  return (
    unlocalizedPathname === `${baseUrl}/${pathSegment}` ||
    unlocalizedPathname.startsWith(`${baseUrl}/${pathSegment}/`)
  );
}

const navItems = $derived([
  {
    path: "",
    label: isAr ? "نظرة عامة" : "Overview",
    icon: Compass,
  },
  {
    path: "classes",
    label: isAr ? "الفصول الدراسية" : "Classes",
    icon: GraduationCap,
    badge: classesCount > 0 ? classesCount : null,
  },
  {
    path: "subjects",
    label: isAr ? "المواد الدراسية" : "Subjects",
    icon: BookOpen,
    badge: subjectsCount > 0 ? subjectsCount : null,
  },
  {
    path: "schedule",
    label: isAr ? "الجدول الدراسي" : "Schedule",
    icon: Calendar,
  },
  {
    path: "assessments",
    label: isAr ? "الامتحانات والدرجات" : "Assessments",
    icon: Award,
  },
]);
</script>

<div class="h-full w-full bg-background overflow-hidden">
	<MainViewport
		name="academics-workspace"
		leftWidth={{ default: 240, min: 200 }}
	>
		<!-- Left Resizable Panel: Vertical Secondary Sidebar -->
		{#snippet left()}
			<div class="h-full border-r border-border bg-card/20 p-4 flex flex-col gap-4 overflow-y-auto scrollbar-hide select-none">
				<div class="flex flex-col gap-0.5 text-start px-2 py-1.5">
					<span class="font-bold text-[10px] text-muted-foreground uppercase tracking-wider">
						{isAr ? "الشؤون الأكاديمية" : "Academics Panel"}
					</span>
					<span class="text-[9px] text-muted-foreground line-clamp-1">
						{isAr ? "إدارة وتخطيط المدرسة" : "Curriculums & Timetables"}
					</span>
				</div>

				<!-- Separator -->
				<div class="h-px bg-border/60 w-full"></div>

				<!-- Vertical Links List -->
				<div class="flex flex-col gap-1 w-full text-xs">
					{#each navItems as item (item.path)}
						<Button
							href={localizeHref(`/${data.orgId}/academics/${item.path}`)}
							variant="ghost"
							class="w-full flex items-center justify-between px-3 py-2 rounded-lg text-start transition-all duration-150 cursor-pointer h-9
								{isActive(item.path) ? 'bg-primary/10 text-primary font-semibold hover:bg-primary/15' : 'text-muted-foreground hover:bg-accent hover:text-foreground'}"
						>
							<div class="flex items-center gap-2">
								<item.icon size={14} class={isActive(item.path) ? 'text-primary' : 'text-muted-foreground'} />
								<span>{item.label}</span>
							</div>

							{#if item.badge !== undefined && item.badge !== null}
								<span class="text-[9px] px-1.5 py-0.5 rounded-md font-mono bg-muted/80 text-muted-foreground border border-border/40 font-bold">
									{item.badge}
								</span>
							{:else if isActive(item.path)}
								<ChevronRight size={10} class="text-primary" />
							{/if}
						</Button>
					{/each}
				</div>
			</div>
		{/snippet}

		<!-- Middle Panel: Display Child Sub-routes -->
		{#snippet middle()}
			<div class="h-full w-full overflow-hidden">
				{@render children()}
			</div>
		{/snippet}
	</MainViewport>
</div>
