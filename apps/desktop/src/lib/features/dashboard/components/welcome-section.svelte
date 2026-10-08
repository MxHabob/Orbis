<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { quadOut } from "svelte/easing";
  import { fly } from "svelte/transition";
  import { localizeHref } from "$lib/paraglide/runtime";
  import type { DashboardCountsDto } from "$lib/api/types";
  import DashboardGreeting from "./dashboard-greeting.svelte";

  let {
    type = "both",
    stats,
    orgId,
  }: {
    type?: "greeting" | "summary" | "both";
    stats?: DashboardCountsDto | null;
    orgId: string;
  } = $props();

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

    const items: Insight[] = [];

    if (stats.ai_insights && Array.isArray(stats.ai_insights)) {
      for (const ai of stats.ai_insights) {
        if (ai.key === "critical") {
          const link = m.ai_insight_critical_link({ count: ai.count });
          const parts = m.ai_insight_critical({
            link: "||LINK||",
          }).split("||LINK||");
          items.push({
            key: "critical",
            before: parts[0] ?? "",
            link,
            after: parts[1] ?? "",
            href: ai.href ?? `/${orgId}/settings`,
          });
        } else if (ai.key === "warning") {
          const link = m.ai_insight_warning_link({ count: ai.count });
          const parts = m.ai_insight_warning({
            link: "||LINK||",
          }).split("||LINK||");
          items.push({
            key: "warning",
            before: parts[0] ?? "",
            link,
            after: parts[1] ?? "",
            href: ai.href ?? `/${orgId}/settings`,
          });
        } else if (ai.key === "healthy") {
          const link = m.ai_insight_healthy_link();
          const parts = m.ai_insight_healthy({
            link: "||LINK||",
          }).split("||LINK||");
          items.push({
            key: "healthy",
            before: parts[0] ?? "",
            link,
            after: parts[1] ?? "",
            href: ai.href ?? `/${orgId}/settings`,
          });
        }
      }
    }

    if (items.length === 0) {
      const studentLink = m.common_insight_students_link({
        count: stats.total_students ?? 0,
      });
      const studentParts = m.common_insight_students({
        link: "||LINK||",
      }).split("||LINK||");
      items.push({
        key: "students",
        before: studentParts[0] ?? "",
        link: studentLink,
        after: studentParts[1] ?? "",
        href: `/${orgId}/students`,
      });

      const classLink = m.common_insight_classes_link({
        count: stats.active_classes ?? 0,
      });
      const classParts = m.common_insight_classes({
        link: "||LINK||",
      }).split("||LINK||");
      items.push({
        key: "classes",
        before: classParts[0] ?? "",
        link: classLink,
        after: classParts[1] ?? "",
        href: `/${orgId}/academics`,
      });

      const employeeLink = m.common_insight_staff_link({
        count: stats.active_employees ?? 0,
      });
      const employeeParts = m.common_insight_staff({
        link: "||LINK||",
      }).split("||LINK||");
      items.push({
        key: "employees",
        before: employeeParts[0] ?? "",
        link: employeeLink,
        after: employeeParts[1] ?? "",
        href: `/${orgId}/employees`,
      });
    }

    return items;
  });

  function tick() {
    if (!isHovered && insights.length > 1) {
      const now = Date.now();
      const elapsed = now - start;
      progress = Math.min(elapsed / TICK_DURATION, 1);

      if (elapsed >= TICK_DURATION) {
        index = (index + 1) % insights.length;
        start = Date.now();
        elapsedOnPause = 0;
        progress = 0;
      }
    }
    rafId = requestAnimationFrame(tick);
  }

  function goTo(i: number) {
    index = i;
    start = Date.now();
    elapsedOnPause = 0;
    progress = 0;
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

{#if type === "greeting" || type === "both"}
  <DashboardGreeting showSubtitle={type === "greeting" || insights.length === 0} stats={stats} />
{/if}

{#if type === "summary" || type === "both"}
  <div
    class="flex flex-col items-center max-w-lg w-full gap-3 mt-1 justify-center text-center mx-auto"
    role="region"
    aria-label="Insights summary slider"
    onmouseenter={handleMouseEnter}
    onmouseleave={handleMouseLeave}
  >
    <div
      class="relative h-10 overflow-hidden w-full flex items-center justify-center"
    >
      {#if insights.length > 0}
        {@const current = insights[index % insights.length]}
        {#key index}
          <div
            in:fly={{ y: 8, opacity: 0, duration: 300, easing: quadOut }}
            out:fly={{ y: -8, opacity: 0, duration: 300, easing: quadOut }}
            class="absolute inset-0 flex items-center justify-center text-center px-4"
          >
            <span
              class="text-muted-foreground text-sm leading-relaxed text-center"
            >
              {current.before}
              {#if current.link}
                <a
                  href={localizeHref(current.href)}
                  class="border-b border-dashed border-[#878787]/30 hover:text-foreground transition-colors font-medium"
                >
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
        {#each insights as insight, i (insight.key)}
          <button
            type="button"
            class="group/bar relative cursor-pointer py-2 -my-2 focus:outline-none"
            aria-label={m.common_insight_show_number({ number: i + 1 })}
            onclick={() => goTo(i)}
            onmouseenter={() => goTo(i)}
          >
            <div
              class="relative h-[2px] w-4 rounded-full bg-primary/10 overflow-hidden"
            >
              <div
                class="absolute inset-0 bg-primary/40 rounded-full origin-left rtl:origin-right"
                style="transform: scaleX({i === index ? progress : 0});"
              ></div>
            </div>
          </button>
        {/each}
      </div>
    {/if}
  </div>
{/if}
