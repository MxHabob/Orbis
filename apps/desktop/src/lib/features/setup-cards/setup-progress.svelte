<!--
  Setup Progress Toast Banner — progressive configuration widget.
  Styled exactly matching DesktopBanner (fixed bottom toast, w-[300px], dot-grid header, platform/step capsules, progress line, and clean typography).
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import { systemStore } from "$lib/stores/system.svelte";
import { ArrowLeft, ArrowRight, BookOpen, Calendar, Check, ClockRotateRight, DocumentChartBar, Home, Keyframes, Sparkles, XMark } from "@orbis/icons";
import { onMount } from "svelte";
import { fly } from "svelte/transition";

let { orgId }: { orgId: string } = $props();

const isArabic = $derived(getLocale().startsWith("ar"));
let dismissed = $state(true);
let mounted = $state(false);

onMount(async () => {
  const saved = localStorage.getItem("orbis_setup_progress_dismissed");
  dismissed = saved === "true";
  mounted = true;
  await systemStore.loadOnboardingState(orgId);
});

function handleDismiss(e: MouseEvent) {
  e.preventDefault();
  e.stopPropagation();
  dismissed = true;
  localStorage.setItem("orbis_setup_progress_dismissed", "true");
}

const onboardingState = $derived(systemStore.onboardingState);
const tasks = $derived(systemStore.onboardingTasks);
const pendingTasks = $derived(systemStore.pendingOnboardingTasks);
const completedTasks = $derived(systemStore.completedOnboardingTasks);
const percentage = $derived(systemStore.onboardingProgressPercentage);
const isReady = $derived(systemStore.isInstitutionReady);

// First available actionable task (not blocked, not completed)
const nextTask = $derived(
  pendingTasks.find((t) => t.status !== "blocked") ?? pendingTasks[0]
);

const nextHref = $derived(
  nextTask ? (nextTask.route.startsWith("/") ? nextTask.route : `/${nextTask.route}`) : "/onboarding"
);

const nextTitle = $derived(
  nextTask
    ? isArabic ? nextTask.title_ar : nextTask.title
    : isArabic ? "مركز التهيئة" : "Onboarding Hub"
);

// Show only if institution is not fully ready and user has not dismissed it
const shouldShow = $derived(mounted && !dismissed && tasks.length > 0 && !isReady);

// Display first 4 tasks in banner
const displaySteps = $derived(tasks.slice(0, 4));
</script>

{#if shouldShow}
  <div
    in:fly={{ y: 40, opacity: 0, duration: 500, delay: 500 }}
    out:fly={{ y: 20, opacity: 0, duration: 300 }}
    class="fixed bottom-2.5 {isArabic ? 'left-4' : 'right-4'} z-50 group"
  >
    <div class="relative border border-border bg-background shadow-lg w-[300px] overflow-hidden rounded-xl">
      <!-- Dismiss button matching DesktopBanner -->
      <button
        type="button"
        onclick={handleDismiss}
        class="absolute top-2.5 {isArabic ? 'left-2.5' : 'right-2.5'} z-20 text-muted-foreground hover:text-foreground p-1 opacity-0 group-hover:opacity-100 transition-opacity duration-200 focus:outline-none rounded-lg hover:bg-muted/50"
        title={isArabic ? "إخفاء" : "Dismiss"}
      >
        <XMark size={14} />
      </button>

      <!-- Dot-grid visual header area matching DesktopBanner -->
      <div class="relative h-[100px] flex items-center justify-center border-b border-border bg-muted/10">
        <!-- Light mode dot pattern grid -->
        <div
          class="absolute inset-0 dark:hidden pointer-events-none"
          style="background-image: radial-gradient(circle, rgba(0,0,0,0.08) 1px, transparent 1px); background-size: 12px 12px;"
        ></div>
        <!-- Dark mode dot pattern grid -->
        <div
          class="absolute inset-0 hidden dark:block pointer-events-none"
          style="background-image: radial-gradient(circle, rgba(255,255,255,0.07) 1px, transparent 1px); background-size: 12px 12px;"
        ></div>

        <!-- Setup steps icons row -->
        <div class="relative z-10 flex items-center justify-center gap-2.5 w-full px-4">
          {#each displaySteps as task (task.id)}
            {@const isTaskDone = task.status === 'completed'}
            {@const isTaskBlocked = task.status === 'blocked'}
            {@const taskTitle = isArabic ? task.title_ar : task.title}
            <button
              type="button"
              onclick={() => goto(localizeHref(`/${orgId}${task.route.startsWith('/') ? task.route : '/' + task.route}`))}
              class="relative flex items-center justify-center size-9 rounded-lg border transition-all duration-300 shadow-xs focus:outline-none {isTaskDone
                ? 'border-emerald-500/30 bg-emerald-500/10 text-emerald-500'
                : isTaskBlocked
                  ? 'border-border bg-muted/20 text-muted-foreground opacity-60 cursor-not-allowed'
                  : 'border-border bg-card text-muted-foreground hover:text-foreground hover:border-primary hover:scale-105'}"
              title="{taskTitle} ({isTaskDone ? (isArabic ? 'مكتمل' : 'Completed') : (isArabic ? 'معلق' : 'Pending')})"
            >
              <Sparkles class="size-4" />
              {#if isTaskDone}
                <span class="absolute -top-1 -right-1 flex items-center justify-center size-3.5 rounded-full bg-emerald-500 text-white text-[8px] font-bold shadow-xs">
                  <Check class="size-2.5" />
                </span>
              {/if}
            </button>
          {/each}
        </div>
      </div>

      <!-- Linear Progress Line -->
      <div class="w-full bg-muted/40 h-1 overflow-hidden">
        <div
          class="bg-gradient-to-r from-primary to-emerald-500 h-full transition-all duration-500 ease-out"
          style="width: {percentage}%"
        ></div>
      </div>

      <!-- Content matching DesktopBanner typography & styling -->
      <div class="block p-4 select-none">
        <div class="flex items-center justify-between mb-1">
          <a
            href={localizeHref(`/${orgId}/onboarding`)}
            class="text-xs font-semibold tracking-wide text-foreground hover:text-primary transition-colors flex items-center gap-1"
          >
            <span>{isArabic ? "تهيئة وإعداد النظام" : "System Setup Progress"}</span>
          </a>
          <span class="text-[10px] font-bold text-primary px-1.5 py-0.5 rounded bg-primary/10 border border-primary/20">
            {percentage}%
          </span>
        </div>

        <p class="text-[10px] text-muted-foreground leading-normal mb-3">
          {isArabic
            ? `تم إنجاز ${completedTasks.length} من ${tasks.length} متطلبات أساسية.`
            : `${completedTasks.length} of ${tasks.length} essential requirements configured.`}
        </p>

        <!-- CTA Action Button to next pending task -->
        {#if nextTask}
          <button
            type="button"
            onclick={() => goto(localizeHref(`/${orgId}${nextHref}`))}
            class="w-full flex items-center justify-center gap-2 py-2 px-3 rounded-lg bg-primary text-primary-foreground text-xs font-medium hover:bg-primary/90 transition-all duration-200 shadow-2xs group/btn cursor-pointer"
          >
            <span class="truncate">{isArabic ? `إعداد: ${nextTitle}` : `Configure: ${nextTitle}`}</span>
            {#if isArabic}
              <ArrowLeft class="size-3.5 shrink-0 transition-transform group-hover/btn:-translate-x-0.5" />
            {:else}
              <ArrowRight class="size-3.5 shrink-0 transition-transform group-hover/btn:translate-x-0.5" />
            {/if}
          </button>
        {/if}
      </div>
    </div>
  </div>
{/if}
