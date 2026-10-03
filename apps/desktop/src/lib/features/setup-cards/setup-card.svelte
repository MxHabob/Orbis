<!--
  Setup Card — individual action card for progressive onboarding item.
  Matches DesktopBanner design language with dot-grid aesthetic, clean borders, and smooth transitions.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { goto } from "$app/navigation";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import type { SetupItem } from "$lib/api/types";
import { ArrowLeft, ArrowRight, BookOpen, Calendar, CheckCircle, ClockRotateRight, DocumentChartBar, Home, Keyframes, Sparkles } from "@orbis/icons";

let {
  item,
  orgId,
  href,
}: {
  item: SetupItem;
  orgId: string;
  href: string;
} = $props();

const isArabic = $derived(getLocale().startsWith("ar"));
const title = $derived(isArabic && item.label_ar ? item.label_ar : item.label);

// Icon mapping per setup item
const iconMap: Record<string, any> = {
  academic_year: Calendar,
  classes: BookOpen,
  subjects: BookOpen,
  rooms: Home,
  time_slots: ClockRotateRight,
  fee_templates: DocumentChartBar,
  org_units: Keyframes,
};

const IconComponent = $derived(iconMap[item.key] ?? Sparkles);
</script>

<div
  class="group/item relative flex items-center justify-between p-3 rounded-lg border transition-all duration-200 {item.is_completed
    ? 'bg-emerald-500/5 border-emerald-500/20 text-muted-foreground'
    : 'border-border bg-card hover:bg-card/90 hover:border-primary/40 shadow-xs'}"
>
  <div class="flex items-center gap-3 min-w-0">
    <div
      class="shrink-0 flex items-center justify-center size-8 rounded-lg border transition-colors {item.is_completed
        ? 'border-emerald-500/30 bg-emerald-500/10 text-emerald-500'
        : 'border-border bg-background text-muted-foreground group-hover/item:text-foreground group-hover/item:border-primary/50'}"
    >
      <svelte:component this={IconComponent} class="size-4" />
    </div>

    <div class="flex flex-col min-w-0">
      <div class="flex items-center gap-2">
        <span class="font-medium text-xs text-foreground truncate {item.is_completed ? 'line-through opacity-70' : ''}">
          {title}
        </span>
        {#if item.is_required && !item.is_completed}
          <span class="text-[9px] font-semibold px-1.5 py-0.5 rounded bg-amber-500/10 text-amber-600 dark:text-amber-400 border border-amber-500/20">
            {m.ui_setup_card_required()}
          </span>
        {/if}
      </div>
      <p class="text-[10px] text-muted-foreground truncate">
        {#if item.is_completed}
          {m.ui_setup_card_completed_and_configured()}
        {:else}
          {m.ui_setup_card_pending_configuration()}
        {/if}
      </p>
    </div>
  </div>

  <div class="shrink-0 flex items-center gap-2">
    {#if item.is_completed}
      <div class="flex items-center gap-1 text-[11px] font-medium text-emerald-500">
        <CheckCircle class="size-3.5" />
      </div>
    {:else}
      <button
        type="button"
        onclick={() => goto(localizeHref(`/${orgId}${href}`))}
        class="inline-flex items-center gap-1 text-[11px] font-medium text-primary hover:text-primary/80 transition-colors px-2.5 py-1 rounded-md border border-primary/20 bg-primary/5 hover:bg-primary/10"
      >
        <span>{m.ui_setup_card_configure()}</span>
        {#if isArabic}
          <ArrowLeft class="size-3" />
        {:else}
          <ArrowRight class="size-3" />
        {/if}
      </button>
    {/if}
  </div>
</div>
