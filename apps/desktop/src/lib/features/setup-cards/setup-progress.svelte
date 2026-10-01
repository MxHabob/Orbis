<!--
  @component SetupProgressBanner
  Minimalist, elegant floating setup progress widget for ongoing institutional onboarding.
  Uses UI design tokens and clean typography.
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import { systemStore } from "$lib/stores/system.svelte";
import { onMount } from "svelte";
import { fly } from "svelte/transition";
import { Button } from "@orbis/ui/button";
import { Progress } from "@orbis/ui/progress";

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

const tasks = $derived(systemStore.onboardingTasks);
const pendingTasks = $derived(systemStore.pendingOnboardingTasks);
const completedTasks = $derived(systemStore.completedOnboardingTasks);
const percentage = $derived(systemStore.onboardingProgressPercentage);
const isReady = $derived(systemStore.isInstitutionReady);

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

const shouldShow = $derived(mounted && !dismissed && tasks.length > 0 && !isReady);
</script>

{#if shouldShow}
  <div
    in:fly={{ y: 20, opacity: 0, duration: 300, delay: 300 }}
    out:fly={{ y: 20, opacity: 0, duration: 200 }}
    class="fixed bottom-3 {isArabic ? 'left-4' : 'right-4'} z-40 group"
  >
    <div class="relative border border-border/80 bg-background/95 backdrop-blur-xs shadow-md w-72 rounded-xl p-3.5 space-y-2.5">
      <!-- Dismiss Button -->
      <button
        type="button"
        onclick={handleDismiss}
        class="absolute top-2.5 {isArabic ? 'left-2.5' : 'right-2.5'} text-muted-foreground hover:text-foreground text-xs leading-none p-1 rounded transition-colors"
        title={isArabic ? "إخفاء" : "Dismiss"}
      >
        ✕
      </button>

      <!-- Header & Percentage -->
      <div class="space-y-1">
        <div class="flex items-center justify-between text-xs pe-4">
          <a
            href={localizeHref(`/${orgId}/onboarding`)}
            class="font-semibold text-foreground hover:text-primary transition-colors truncate"
          >
            {isArabic ? "تهيئة المؤسسة" : "Setup Progress"}
          </a>
          <span class="font-mono text-[11px] text-muted-foreground font-medium">
            {percentage}%
          </span>
        </div>

        <p class="text-[11px] text-muted-foreground">
          {isArabic
            ? `${completedTasks.length} من ${tasks.length} مهام مكتملة`
            : `${completedTasks.length} of ${tasks.length} tasks completed`}
        </p>
      </div>

      <!-- Linear Progress Line -->
      <Progress value={percentage} max={100} class="h-1" />

      <!-- Next Action CTA -->
      {#if nextTask}
        <Button
          variant="default"
          size="sm"
          onclick={() => goto(localizeHref(`/${orgId}${nextHref}`))}
          class="w-full h-7 text-xs font-normal"
        >
          <span class="truncate">{isArabic ? `إعداد: ${nextTitle}` : `Configure: ${nextTitle}`}</span>
        </Button>
      {/if}
    </div>
  </div>
{/if}
