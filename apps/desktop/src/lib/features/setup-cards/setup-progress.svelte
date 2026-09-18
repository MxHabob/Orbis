<!--
  Setup Progress Banner / Section — progressive configuration widget on the dashboard.
  Displays readiness status, completion percentage bar, and actionable cards.
-->
<script lang="ts">
import { getLocale } from "$lib/paraglide/runtime";
import { systemStore } from "$lib/stores/system.svelte";
import SetupCard from "./setup-card.svelte";
import { Sparkles, X } from "@lucide/svelte";

let { orgId }: { orgId: string } = $props();

const isArabic = $derived(getLocale().startsWith("ar"));
let isDismissed = $state(false);

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

const items = $derived(systemStore.orgReadiness?.pending_setup_items ?? []);
const pendingCount = $derived(systemStore.pendingSetupItems.length);
const totalCount = $derived(items.length);
const percentage = $derived(systemStore.setupProgressPercentage);

// Show only if there are pending items and not dismissed
const shouldShow = $derived(!isDismissed && pendingCount > 0);
</script>

{#if shouldShow}
  <div class="mb-8 rounded-2xl border border-primary/20 bg-gradient-to-b from-card/90 to-card/50 p-5 shadow-lg backdrop-blur-sm relative overflow-hidden">
    <!-- Background glow -->
    <div class="absolute -top-24 -right-24 w-48 h-48 bg-primary/10 rounded-full blur-3xl pointer-events-none"></div>

    <!-- Header -->
    <div class="flex items-center justify-between mb-4">
      <div class="flex items-center gap-2.5">
        <div class="w-8 h-8 rounded-lg bg-primary/15 text-primary flex items-center justify-center">
          <Sparkles class="w-4 h-4" />
        </div>
        <div>
          <h3 class="font-semibold text-sm text-foreground">
            {isArabic ? "إكمال إعداد النظام" : "Complete System Setup"}
          </h3>
          <p class="text-xs text-muted-foreground">
            {isArabic
              ? `تم إنجاز ${totalCount - pendingCount} من ${totalCount} عناصر`
              : `${totalCount - pendingCount} of ${totalCount} items completed`}
          </p>
        </div>
      </div>

      <div class="flex items-center gap-3">
        <div class="text-xs font-semibold text-primary px-2.5 py-1 rounded-full bg-primary/10">
          {percentage}%
        </div>
        <button
          type="button"
          onclick={() => (isDismissed = true)}
          class="text-muted-foreground hover:text-foreground p-1 rounded-md hover:bg-muted/50 transition-colors"
          title={isArabic ? "إخفاء" : "Dismiss"}
        >
          <X class="w-4 h-4" />
        </button>
      </div>
    </div>

    <!-- Progress bar -->
    <div class="w-full bg-muted/60 h-2 rounded-full overflow-hidden mb-4">
      <div
        class="bg-gradient-to-r from-primary to-emerald-500 h-full rounded-full transition-all duration-500 ease-out"
        style="width: {percentage}%"
      ></div>
    </div>

    <!-- Cards Grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-2.5">
      {#each items as item (item.key)}
        <SetupCard
          {item}
          {orgId}
          href={routeMap[item.key] ?? "/settings"}
        />
      {/each}
    </div>
  </div>
{/if}
