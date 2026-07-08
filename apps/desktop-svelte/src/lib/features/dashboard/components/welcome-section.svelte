<script lang="ts">
import { authStore } from "$lib/stores";
import { onMount, onDestroy } from "svelte";
import { fly } from "svelte/transition";
import { quadOut } from "svelte/easing";
import * as m from "$lib/paraglide/messages";
import { localizeHref } from "$lib/paraglide/runtime";
import { Avatar, AvatarController } from "@orbis/avatar-engine";

let {
  type,
  stats,
  orgId,
}: { type?: "greeting" | "summary" | "both"; stats?: any; orgId: string } =
  $props();

const avatarController = new AvatarController();

// Greeting Logic
let hourState = $state(new Date().getHours());
let greetingInterval: any;

onMount(() => {
  greetingInterval = setInterval(() => {
    hourState = new Date().getHours();
  }, 60000);
});

onDestroy(() => {
  if (greetingInterval) clearInterval(greetingInterval);
  avatarController.destroy();
});

const greeting = $derived.by(() => {
  if (hourState >= 5 && hourState < 12) return m.greeting_morning();
  if (hourState >= 12 && hourState < 17) return m.greeting_afternoon();
  return m.greeting_evening();
});

const firstName = $derived(authStore.user?.full_name?.split(" ")[0]);

let manualHat = $state<"none" | "crown" | "graduation" | "firefighter" | null>(null);

const userHat = $derived.by(() => {
  if (manualHat !== null) return manualHat;
  const rawRole = authStore.user?.role || authStore.user?.primary_role || "viewer";
  const role = String(rawRole).toLowerCase();
  if (role === "admin" || role === "super_admin") return "crown";
  if (role === "teacher") return "graduation";
  if (role === "registrar") return "firefighter";
  return "none";
});

function cycleHat() {
  const hatList = ["none", "crown", "graduation", "firefighter"] as const;
  const currentIndex = hatList.indexOf(userHat);
  manualHat = hatList[(currentIndex + 1) % hatList.length];
}

// Ticker / Insights Logic
let index = $state(0);
let progress = $state(0);
let isHovered = $state(false);

const TICK_DURATION = 6000;
let start = $state(Date.now());
let elapsedOnPause = $state(0);
let rafId: number;

type Insight = {
  key: string;
  before: string;
  link: string;
  after: string;
  href: string;
};

const insights = $derived.by<Insight[]>(() => {
  if (!stats) return [];

  const studentLink = m.insight_students_link({ count: stats.total_students });
  const studentParts = m
    .insight_students({ link: "||LINK||" })
    .split("||LINK||");

  const attendanceLink = m.insight_attendance_link({
    rate: stats.attendance_overview?.present_rate ?? 0,
  });
  const attendanceParts = m
    .insight_attendance({
      link: "||LINK||",
      lateRate: stats.attendance_overview?.late_rate ?? 0,
    })
    .split("||LINK||");

  const classLink = m.insight_classes_link({ count: stats.total_classes });
  const classParts = m
    .insight_classes({
      link: "||LINK||",
      teacherCount: stats.total_teachers,
    })
    .split("||LINK||");

  const behaviorLink = m.insight_behavior_link({
    count: stats.behavior_summary?.total_records ?? 0,
  });
  const behaviorParts = m
    .insight_behavior({
      link: "||LINK||",
      criticalCount: stats.behavior_summary?.critical_count ?? 0,
    })
    .split("||LINK||");

  const staffLink = m.insight_staff_link({ count: stats.total_staff });
  const staffParts = m.insight_staff({ link: "||LINK||" }).split("||LINK||");

  return [
    {
      key: "students",
      before: studentParts[0] ?? "",
      link: studentLink,
      after: studentParts[1] ?? "",
      href: `/${orgId}/students`,
    },
    {
      key: "attendance",
      before: attendanceParts[0] ?? "",
      link: attendanceLink,
      after: attendanceParts[1] ?? "",
      href: `/${orgId}/reports`,
    },
    {
      key: "classes",
      before: classParts[0] ?? "",
      link: classLink,
      after: classParts[1] ?? "",
      href: `/${orgId}/academics`,
    },
    {
      key: "behavior",
      before: behaviorParts[0] ?? "",
      link: behaviorLink,
      after: behaviorParts[1] ?? "",
      href: `/${orgId}/students`,
    },
    {
      key: "staff",
      before: staffParts[0] ?? "",
      link: staffLink,
      after: staffParts[1] ?? "",
      href: `/${orgId}/settings`,
    },
  ];
});

function goTo(i: number) {
  if (insights.length <= 1) return;
  const next = i % insights.length;
  if (next === index) return;
  index = next;
  progress = 0;
  start = Date.now();
  elapsedOnPause = 0;
}

function tick() {
  if (isHovered) {
    rafId = requestAnimationFrame(tick);
    return;
  }

  const elapsed = elapsedOnPause + (Date.now() - start);
  const pct = Math.min(elapsed / TICK_DURATION, 1);
  progress = pct;

  if (pct >= 1) {
    index = (index + 1) % insights.length;
    progress = 0;
    start = Date.now();
    elapsedOnPause = 0;
  }

  rafId = requestAnimationFrame(tick);
}

$effect(() => {
  if (insights.length > 1 && !rafId) {
    rafId = requestAnimationFrame(tick);
  }
  return () => {
    if (rafId) cancelAnimationFrame(rafId);
  };
});

function handleMouseEnter() {
  isHovered = true;
  elapsedOnPause += Date.now() - start;
}

function handleMouseLeave() {
  isHovered = false;
  start = Date.now();
}
</script>

{#if type === "greeting" || type === "both" || !type}
	<div class="flex flex-col sm:flex-row items-center justify-center gap-1 sm:gap-2 mb-0 text-center sm:text-start">
		<button
			type="button"
			class="cursor-pointer focus:outline-none transition-all duration-150 ease-out -translate-y-1 hover:-translate-y-2 active:translate-y-0 shrink-0"
			onclick={() => {
				cycleHat();
				const rand = Math.random();
				if (rand < 0.33) {
					avatarController.happy();
				} else if (rand < 0.66) {
					avatarController.think();
				} else {
					avatarController.blink();
				}
			}}
			aria-label="Interactive Avatar"
		>
			<Avatar controller={avatarController} hat={userHat} width="80px" height="80px" />
		</button>
		<h1 class="text-[38px] font-serif leading-tight">
			{greeting}{#if firstName}, <span class="text-muted-foreground">{firstName}</span>{/if}
		</h1>
	</div>
{/if}

{#if type === "summary" || type === "both" || !type}
	<div
		class="flex flex-col items-center max-w-lg w-full gap-3 mt-1 justify-center text-center mx-auto"
		role="region"
		aria-label="Insights summary slider"
		onmouseenter={handleMouseEnter}
		onmouseleave={handleMouseLeave}
	>
		<div class="relative h-10 overflow-hidden w-full flex items-center justify-center">
			{#if insights.length > 0}
				{@const current = insights[index % insights.length]}
				{#key index}
					<div
						in:fly={{ y: 8, opacity: 0, duration: 300, easing: quadOut }}
						out:fly={{ y: -8, opacity: 0, duration: 300, easing: quadOut }}
						class="absolute inset-0 flex items-center justify-center text-center px-4"
					>
						<span class="text-muted-foreground text-sm leading-relaxed text-center">
							{current.before}
							{#if current.link}
								<a href={localizeHref(current.href)} class="border-b border-dashed border-[#878787]/30 hover:text-foreground transition-colors font-medium">
									{current.link}
								</a>
							{/if}
							{current.after}
						</span>
					</div>
				{/key}
			{/if}
		</div>

		{#if insights.length > 1}
			<div class="flex items-center gap-1.5 justify-center">
				{#each insights as _, i}
					<button
						type="button"
						class="group/bar relative cursor-pointer py-2 -my-2 focus:outline-none"
						aria-label={m.insight_show_number({ number: i + 1 })}
						onclick={() => goTo(i)}
						onmouseenter={() => goTo(i)}
					>
						<div class="relative h-[2px] w-4 rounded-full bg-primary/10 overflow-hidden">
							<div
								class="absolute inset-0 bg-primary/40 rounded-full origin-left"
								style="transform: scaleX({i === index ? progress : 0});"
							></div>
						</div>
					</button>
				{/each}
			</div>
		{/if}
	</div>
{/if}

