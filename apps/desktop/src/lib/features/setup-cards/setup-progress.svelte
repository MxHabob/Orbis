<!--
  Setup Progress Toast Banner — progressive configuration widget.
  Styled exactly matching DesktopBanner (fixed bottom toast, w-[300px], dot-grid header, platform/step capsules, progress line, and clean typography).
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import { systemStore } from "$lib/stores/system.svelte";
import {
  X,
  Calendar,
  GraduationCap,
  BookOpen,
  Building2,
  Clock,
  Coins,
  Layers,
  Check,
  ArrowRight,
  ArrowLeft,
  Sparkles,
} from "@lucide/svelte";
import { onMount } from "svelte";
import { fly } from "svelte/transition";

let { orgId }: { orgId: string } = $props();

const isArabic = $derived(getLocale().startsWith("ar"));
let dismissed = $state(true); // Default to true to prevent hydration mismatch/flash
let mounted = $state(false);

onMount(() => {
  const saved = localStorage.getItem("orbis_setup_progress_dismissed");
  dismissed = saved === "true";
  mounted = true;
});

function handleDismiss(e: MouseEvent) {
  e.preventDefault();
  e.stopPropagation();
  dismissed = true;
  localStorage.setItem("orbis_setup_progress_dismissed", "true");
}

// Map setup item keys to application routes
const routeMap: Record<string, string> = {
  academic_year: "/settings/academic-years",
  classes: "/classes",
  subjects: "/subjects",
  rooms: "/settings/rooms",
  time_slots: "/scheduling/slots",
  fee_templates: "/finance/templates",
  org_units: "/settings/units",
};

const iconMap: Record<string, any> = {
  academic_year: Calendar,
  classes: GraduationCap,
  subjects: BookOpen,
  rooms: Building2,
  time_slots: Clock,
  fee_templates: Coins,
  org_units: Layers,
};

const items = $derived(systemStore.orgReadiness?.pending_setup_items ?? []);
const pendingItems = $derived(systemStore.pendingSetupItems);
const pendingCount = $derived(pendingItems.length);
const totalCount = $derived(items.length);
const percentage = $derived(systemStore.setupProgressPercentage);

// First pending item to configure
const nextItem = $derived(pendingItems[0] ?? items[0]);
const nextHref = $derived(nextItem ? (routeMap[nextItem.key] ?? "/settings") : "/settings");
const nextTitle = $derived(
  nextItem
    ? isArabic && nextItem.label_ar
      ? nextItem.label_ar
      : nextItem.label
    : isArabic
      ? "إعدادات النظام"
      : "System Settings"
);

// Show only if there are pending items and user has not dismissed it
const shouldShow = $derived(mounted && !dismissed && totalCount > 0 && pendingCount > 0);

// Core display items for the top dot-grid capsule row (show first 4-5 items)
const displaySteps = $derived(items.slice(0, 4));
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
        <X size={14} />
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
          {#each displaySteps as item (item.key)}
            {@const IconComponent = iconMap[item.key] ?? Sparkles}
            {@const itemTitle = isArabic && item.label_ar ? item.label_ar : item.label}
            <button
              type="button"
              onclick={() => goto(localizeHref(`/${orgId}${routeMap[item.key] ?? "/settings"}`))}
              class="relative flex items-center justify-center size-9 rounded-lg border transition-all duration-300 shadow-xs focus:outline-none {item.is_completed
                ? 'border-emerald-500/30 bg-emerald-500/10 text-emerald-500'
                : 'border-border bg-card text-muted-foreground hover:text-foreground hover:border-primary hover:scale-105'}"
              title="{itemTitle} ({item.is_completed ? (isArabic ? 'مكتمل' : 'Completed') : (isArabic ? 'معلق' : 'Pending')})"
            >
              <svelte:component this={IconComponent} class="size-4" />
              {#if item.is_completed}
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
          <h3 class="text-xs font-semibold tracking-wide text-foreground">
            {isArabic ? "تهيئة وإعداد النظام" : "System Setup Progress"}
          </h3>
          <span class="text-[10px] font-bold text-primary px-1.5 py-0.5 rounded bg-primary/10 border border-primary/20">
            {percentage}%
          </span>
        </div>

        <p class="text-[10px] text-muted-foreground leading-normal mb-3">
          {isArabic
            ? `تم إنجاز ${totalCount - pendingCount} من ${totalCount} متطلبات أساسية.`
            : `${totalCount - pendingCount} of ${totalCount} essential requirements configured.`}
        </p>

        <!-- CTA Action Button to next pending task -->
        {#if nextItem}
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
