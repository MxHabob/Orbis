<!--
  Setup Card — individual action card for progressive onboarding item.
  Shows title, description, status badge, and action button.
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import type { SetupItem } from "$lib/api/types";
import { CheckCircle2, Circle, ArrowRight, ArrowLeft } from "@lucide/svelte";

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
</script>

<div
  class="group relative flex items-center justify-between p-4 rounded-xl border transition-all duration-200 {item.is_completed
    ? 'bg-emerald-500/5 border-emerald-500/20 text-muted-foreground'
    : 'bg-card/80 hover:bg-card border-border hover:border-primary/40 shadow-sm hover:shadow-md'}"
>
  <div class="flex items-center gap-3.5 min-w-0">
    <div
      class="shrink-0 flex items-center justify-center w-8 h-8 rounded-lg {item.is_completed
        ? 'bg-emerald-500/10 text-emerald-500'
        : 'bg-primary/10 text-primary'}"
    >
      {#if item.is_completed}
        <CheckCircle2 class="w-4 h-4" />
      {:else}
        <Circle class="w-4 h-4" />
      {/if}
    </div>
    <div class="flex flex-col min-w-0">
      <div class="flex items-center gap-2">
        <span class="font-medium text-sm text-foreground truncate {item.is_completed ? 'line-through opacity-70' : ''}">
          {title}
        </span>
        {#if item.is_required && !item.is_completed}
          <span class="text-[10px] uppercase tracking-wider font-semibold px-1.5 py-0.5 rounded bg-amber-500/10 text-amber-500 border border-amber-500/20">
            {isArabic ? "مطلوب" : "Required"}
          </span>
        {/if}
      </div>
    </div>
  </div>

  {#if !item.is_completed}
    <button
      type="button"
      onclick={() => goto(localizeHref(`/${orgId}${href}`))}
      class="shrink-0 inline-flex items-center gap-1 text-xs font-medium text-primary hover:text-primary/80 transition-colors px-2.5 py-1.5 rounded-lg hover:bg-primary/10"
    >
      <span>{isArabic ? "إعداد" : "Configure"}</span>
      {#if isArabic}
        <ArrowLeft class="w-3.5 h-3.5" />
      {:else}
        <ArrowRight class="w-3.5 h-3.5" />
      {/if}
    </button>
  {/if}
</div>
