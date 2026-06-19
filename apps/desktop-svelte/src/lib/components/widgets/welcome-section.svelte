<script lang="ts">
import { authStore } from "$lib/stores";
import { onMount, onDestroy } from "svelte";
import { fly } from "svelte/transition";
import { quadOut } from "svelte/easing";

let {
  type,
  stats,
  orgId,
}: { type?: "greeting" | "summary" | "both"; stats?: any; orgId: string } =
  $props();

// Greeting Logic
function getTimeBasedGreeting(): string {
  const hour = new Date().getHours();
  if (hour >= 5 && hour < 12) return "Good morning";
  if (hour >= 12 && hour < 17) return "Good afternoon";
  return "Good evening";
}

let greeting = $state(getTimeBasedGreeting());
let greetingInterval: any;

onMount(() => {
  greetingInterval = setInterval(() => {
    greeting = getTimeBasedGreeting();
  }, 60000);
});

onDestroy(() => {
  if (greetingInterval) clearInterval(greetingInterval);
});

const firstName = $derived(authStore.user?.full_name?.split(" ")[0]);

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
  return [
    {
      key: "students",
      before: "You have ",
      link: `${stats.total_students} students`,
      after: ` currently enrolled in the system.`,
      href: `/${orgId}/students`,
    },
    {
      key: "attendance",
      before: "Today's average attendance rate sits at ",
      link: `${stats.attendance_overview?.present_rate ?? 0}%`,
      after: `, with ${stats.attendance_overview?.late_rate ?? 0}% marked late.`,
      href: `/${orgId}/reports`,
    },
    {
      key: "classes",
      before: "There are ",
      link: `${stats.total_classes} active classes`,
      after: ` being managed by ${stats.total_teachers} teachers.`,
      href: `/${orgId}/academics`,
    },
    {
      key: "behavior",
      before: "There are ",
      link: `${stats.behavior_summary?.total_records ?? 0} behavior records`,
      after: `, with ${stats.behavior_summary?.critical_count ?? 0} critical incidents.`,
      href: `/${orgId}/students`,
    },
    {
      key: "staff",
      before: "A total of ",
      link: `${stats.total_staff} staff members`,
      after: " supporting organizational operations.",
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
	<h1 class="text-[38px] font-serif leading-tight text-center">
		{greeting}
		{#if firstName}
			, <span class="text-muted-foreground">{firstName}</span>
		{/if}
	</h1>
{/if}

{#if type === "summary" || type === "both" || !type}
	<div
		class="flex flex-col items-center max-w-lg w-full gap-3 mt-3 justify-center text-center mx-auto"
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
								<a href={current.href} class="border-b border-dashed border-[#878787]/30 hover:text-foreground transition-colors font-medium">
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
						aria-label="Show insight {i + 1}"
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
