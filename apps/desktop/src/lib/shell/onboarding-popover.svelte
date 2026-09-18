<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  Popover,
  PopoverTrigger,
  PopoverContent,
  Button,
  Tooltip,
  TooltipTrigger,
  TooltipContent,
} from "@orbis/ui";
import { Check, ChevronRight, Sparkles } from "@lucide/svelte";
import { systemStore } from "$lib/stores";
import { goto } from "$app/navigation";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import { page } from "$app/state";

const isArabic = $derived(getLocale().startsWith("ar"));
const orgId = $derived(page.params.orgId || "");

const routeMap: Record<string, string> = {
  academic_year: "/settings/academic-years",
  classes: "/classes",
  subjects: "/subjects",
  rooms: "/settings/rooms",
  time_slots: "/scheduling/slots",
  fee_templates: "/finance/templates",
  org_units: "/settings/units",
};

const items = $derived(systemStore.orgReadiness?.pending_setup_items ?? []);
const totalTasks = $derived(items.length || 1);
const completedTasks = $derived(systemStore.completedSetupItems.length);
const remainingTasks = $derived(systemStore.pendingSetupItems.length);
const progressPercent = $derived(systemStore.setupProgressPercentage);

function handleItemClick(itemKey: string) {
  if (orgId) {
    const route = routeMap[itemKey] ?? "/settings";
    goto(localizeHref(`/${orgId}${route}`));
  }
}

function handleGoToOnboarding() {
  goto(localizeHref("/onboarding"));
}
</script>

<Popover>
  <PopoverTrigger>
    {#snippet child({ props: popoverProps }: { props: any })}
      <Tooltip>
        <TooltipTrigger>
          {#snippet child({ props: tooltipProps }: { props: any })}
            <span {...tooltipProps} class="inline-block">
              <Button
                {...popoverProps}
                variant="outline"
                size="icon-sm"
                class="relative cursor-pointer rounded-lg onboarding-popover-trigger"
              >
                <div class="relative size-5 flex items-center justify-center">
                  <!-- Progress Circular Ring SVG -->
                  <svg class="absolute inset-0 size-5 select-none" viewBox="0 0 24 24" width="20" height="20">
                    <circle
                      cx="12"
                      cy="12"
                      r="9"
                      class="stroke-muted/40 fill-none"
                      stroke-width="2.5"
                    />
                    {#if progressPercent > 0}
                      <circle
                        cx="12"
                        cy="12"
                        r="9"
                        class="stroke-primary fill-none transition-all duration-300 origin-center -rotate-90"
                        stroke-width="2.5"
                        stroke-dasharray="56.5"
                        stroke-dashoffset={56.5 - (56.5 * progressPercent) / 100}
                        stroke-linecap="round"
                      />
                    {/if}
                  </svg>
                  
                  {#if remainingTasks === 0}
                    <Check class="size-3 text-primary stroke-[3px]" />
                  {:else}
                    <span class="text-[8.5px] font-extrabold text-foreground leading-none">
                      {remainingTasks}
                    </span>
                    <span class="absolute -top-0.5 -right-0.5 w-1.5 h-1.5 rounded-full bg-primary border border-background"></span>
                  {/if}
                </div>
              </Button>
            </span>
          {/snippet}
        </TooltipTrigger>
        <TooltipContent side="right" sideOffset={12}>
          {m.common_onboarding_progress_label() || 'Getting Started'}
        </TooltipContent>
      </Tooltip>
    {/snippet}
  </PopoverTrigger>
  
  <PopoverContent
    side="right"
    align="end"
    sideOffset={12}
    class="w-84 bg-card border border-border shadow-xl rounded-xl p-0 overflow-hidden z-50 flex flex-col font-sans text-start onboarding-popover-panel relative text-card-foreground"
  >
    <!-- Pointer Bezel/Tail -->
    <div class="absolute bottom-5 -inset-s-1.5 w-3 h-3 bg-card border-l border-b border-border rotate-45 z-20 rtl:inset-s-auto rtl:-inset-e-1.5 rtl:border-l-0 rtl:border-b-0 rtl:border-r rtl:border-t"></div>

    <!-- Header Section -->
    <div class="flex items-center justify-between p-3.5 border-b border-border bg-card z-10 select-none">
      <div class="flex items-center gap-3">
        <svg class="size-7 transform -rotate-90 text-primary shrink-0" viewBox="0 0 24 24">
          <circle cx="12" cy="12" r="9" class="stroke-muted/40 fill-none" stroke-width="2.5" />
          <circle
            cx="12"
            cy="12"
            r="9"
            class="stroke-primary fill-none transition-all duration-300"
            stroke-width="2.5"
            stroke-dasharray="56.5"
            stroke-dashoffset={56.5 - (56.5 * progressPercent) / 100}
            stroke-linecap="round"
          />
        </svg>

        <div class="flex flex-col">
          <h4 class="font-bold text-xs text-foreground leading-tight">
            {m.common_onboarding_progress_label() || 'خطوات التهيئة'}
          </h4>
          <p class="text-[10.5px] text-muted-foreground font-medium mt-0.5">
            {isArabic ? `تم إنجاز ${completedTasks} من ${totalTasks}` : `${completedTasks} of ${totalTasks} completed`}
          </p>
        </div>
      </div>
    </div>

    <!-- Scrollable Checklist Area -->
    <div class="onboarding-list flex-1 overflow-y-auto max-h-85 p-1.5 flex flex-col gap-0.5 bg-card z-10">
      
      <!-- Initial Setup Link -->
      <button
        onclick={handleGoToOnboarding}
        class="flex items-center justify-between p-2.5 rounded-lg text-start w-full transition-all cursor-pointer border border-transparent hover:bg-muted/60 text-foreground bg-transparent group"
      >
        <div class="flex items-center gap-2.5 flex-1 min-w-0">
          <div class="shrink-0 size-4 rounded-full bg-primary/10 text-primary flex items-center justify-center">
            <Sparkles class="size-2.5" />
          </div>
          <div class="flex flex-col gap-0.5 flex-1 min-w-0">
            <span class="text-[11.5px] leading-tight font-bold text-foreground">
              {isArabic ? "معالج التهيئة الأكاديمية" : "Academic Setup Wizard"}
            </span>
            <span class="text-[10px] leading-relaxed text-muted-foreground">
              {isArabic ? "تعديل التقويم والفصول والمواد" : "Edit calendar, classes & subjects"}
            </span>
          </div>
        </div>
        <ChevronRight size={12} class="text-muted-foreground group-hover:text-foreground rtl:rotate-180 transition-colors" />
      </button>

      <div class="h-px bg-border/40 my-1 mx-2"></div>

      <!-- Setup Tasks Items -->
      {#each items as item, idx}
        {@const isCompleted = item.is_completed}
        
        <button
          onclick={() => handleItemClick(item.key)}
          class="flex items-start justify-between p-2.5 rounded-lg text-start w-full transition-all cursor-pointer font-sans border border-transparent hover:bg-muted/60 text-foreground bg-transparent"
        >
          <div class="flex items-start gap-2.5 flex-1 min-w-0">
            {#if isCompleted}
              <div class="mt-0.5 shrink-0 size-4 rounded-full text-emerald-500 flex items-center justify-center">
                <Check class="size-3 stroke-[3px]" />
              </div>
            {:else}
              <div class="mt-0.5 shrink-0 size-4 rounded-full border border-muted-foreground/30 flex items-center justify-center text-[9px] font-bold text-muted-foreground">
                {idx + 1}
              </div>
            {/if}
            
            <div class="flex flex-col gap-0.5 flex-1 min-w-0">
              <span class="text-[11.5px] leading-tight font-sans {isCompleted ? 'text-muted-foreground line-through decoration-muted-foreground/45 font-normal' : 'text-foreground font-semibold'}">
                {isArabic && item.label_ar ? item.label_ar : item.label}
              </span>
            </div>
          </div>
          
          {#if !isCompleted}
            <div class="mt-1 shrink-0 text-muted-foreground ml-2 rtl:rotate-180">
              <ChevronRight size={12} strokeWidth={2} />
            </div>
          {/if}
        </button>
      {/each}
    </div>
  </PopoverContent>
</Popover>

<style>
  :global(.onboarding-list) {
    scrollbar-width: thin;
    scrollbar-color: var(--border) transparent;
  }
  :global(.onboarding-list)::-webkit-scrollbar {
    width: 3px !important;
    display: block !important;
  }
  :global(.onboarding-list)::-webkit-scrollbar-track {
    background: transparent !important;
  }
  :global(.onboarding-list)::-webkit-scrollbar-thumb {
    background-color: var(--border) !important;
    border-radius: 9999px !important;
  }
  :global(.onboarding-list)::-webkit-scrollbar-thumb:hover {
    background-color: rgba(var(--foreground), 0.25) !important;
  }
</style>
