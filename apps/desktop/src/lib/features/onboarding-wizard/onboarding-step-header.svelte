<script lang="ts">
import { Badge } from "@orbis/ui/badge";
import { getLocale } from "$lib/paraglide/runtime";
import type { OnboardingStepStatus } from "./onboarding-steps";

let {
  stepNumber,
  totalSteps = 8,
  title,
  titleAr,
  description,
  descriptionAr,
  isRequired = true,
  status = "not_started" as OnboardingStepStatus,
  actionButton = undefined,
}: {
  stepNumber: number;
  totalSteps?: number;
  title: string;
  titleAr?: string;
  description: string;
  descriptionAr?: string;
  isRequired?: boolean;
  status?: OnboardingStepStatus;
  actionButton?: any;
} = $props();

const isArabic = $derived(getLocale().startsWith("ar"));
</script>

<div class="mb-8 flex flex-col gap-4 border-b border-border/40 pb-6 sm:flex-row sm:items-start sm:justify-between">
  <div class="space-y-2">
    <div class="flex flex-wrap items-center gap-2.5">
      <span class="inline-flex items-center rounded-md bg-primary/10 px-2.5 py-0.5 text-xs font-semibold tracking-wider text-primary uppercase">
        {isArabic ? `الخطوة ${stepNumber} من ${totalSteps}` : `Step ${stepNumber} of ${totalSteps}`}
      </span>

      {#if isRequired}
        <Badge variant="outline" class="border-amber-500/30 bg-amber-500/10 text-xs font-medium text-amber-600 dark:text-amber-400">
          {isArabic ? "مطلوب" : "Required"}
        </Badge>
      {:else}
        <Badge variant="secondary" class="text-xs text-muted-foreground">
          {isArabic ? "اختياري" : "Optional"}
        </Badge>
      {/if}

      {#if status === "completed"}
        <Badge variant="outline" class="border-emerald-500/30 bg-emerald-500/10 text-xs text-emerald-600 dark:text-emerald-400">
          <span class="mr-1 inline-block size-1.5 rounded-full bg-emerald-500"></span>
          {isArabic ? "مكتمل" : "Completed"}
        </Badge>
      {:else if status === "in_progress"}
        <Badge variant="outline" class="border-blue-500/30 bg-blue-500/10 text-xs text-blue-600 dark:text-blue-400">
          <span class="mr-1 inline-block size-1.5 rounded-full bg-blue-500 animate-pulse"></span>
          {isArabic ? "قيد الإعداد" : "In Progress"}
        </Badge>
      {:else if status === "blocked"}
        <Badge variant="destructive" class="text-xs">
          {isArabic ? "بانتظار متطلبات" : "Prerequisites Blocked"}
        </Badge>
      {/if}
    </div>

    <h1 class="text-2xl font-bold tracking-tight text-foreground sm:text-3xl">
      {isArabic && titleAr ? titleAr : title}
      {#if !isArabic && titleAr}
        <span class="ml-3 font-normal text-muted-foreground/60 text-lg sm:text-xl font-sans" dir="rtl">{titleAr}</span>
      {/if}
    </h1>

    <p class="max-w-2xl text-sm leading-relaxed text-muted-foreground sm:text-base">
      {isArabic && descriptionAr ? descriptionAr : description}
    </p>
  </div>

  {#if actionButton}
    <div class="shrink-0 pt-2 sm:pt-0">
      {@render actionButton()}
    </div>
  {/if}
</div>
