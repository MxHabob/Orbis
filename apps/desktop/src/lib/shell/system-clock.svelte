<script lang="ts">
import { onMount } from "svelte";
import * as m from "$lib/paraglide/messages";
import { authStore, uiStore } from "$lib/stores";
import { orgContext } from "$lib/stores/org-context.svelte";
import { createOrganizationQuery } from "$lib/features/settings/queries";
import { formatDate } from "$lib/core/i18n";
import { Button } from "@orbis/ui/button";
import { Clock } from "@orbis/icons";

const organizationQuery = createOrganizationQuery(() => authStore.orgId);
let now = $state(new Date());

$effect(() => {
  const organization = organizationQuery.data;
  if (organization) orgContext.setOrganization(organization);
});

const timeZone = $derived(organizationQuery.data?.timezone || "UTC");
const hijriEnabled = $derived(
  organizationQuery.data?.is_hijri_enabled ?? false,
);
const timeLabel = $derived(formatDate(now, "time", { timeZone }));
const gregorianDate = $derived(
  formatDate(now, "date", {
    calendar: "gregory",
    timeZone,
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
  }),
);
const hijriDate = $derived(
  hijriEnabled
    ? formatDate(now, "date", {
        calendar: "islamic-umalqura",
        timeZone,
        year: "numeric",
        month: "2-digit",
        day: "2-digit",
      })
    : "",
);

function toggleOpen() {
  uiStore.setSystemClock(!uiStore.showSystemClock);
}

onMount(() => {
  const updateTime = () => {
    now = new Date();
  };
  const timer = window.setInterval(updateTime, 15_000);
  window.addEventListener("focus", updateTime);

  return () => {
    window.clearInterval(timer);
    window.removeEventListener("focus", updateTime);
  };
});
</script>

<div class="relative">
<Button
  variant="outline"
  size="icon-sm"
  onclick={toggleOpen}
  aria-label={uiStore.showSystemClock ? m.ui_shell_clock_close() : m.ui_shell_clock_open()}
  aria-expanded={uiStore.showSystemClock}
  title={timeLabel}
>
  <Clock size={18} />
</Button>

{#if uiStore.showSystemClock}
  <button
    type="button"
    class="fixed bottom-3 inset-s-14 z-100 flex w-[min(14rem,calc(100vw-4.5rem))] flex-col gap-2 backdrop-blur-md"
    aria-label={m.ui_shell_clock_close()}
    title={m.ui_shell_clock_close()}
    onclick={() => uiStore.setSystemClock(false)}
  >
    <time
      datetime={now.toISOString()}
      class="w-full whitespace-nowrap text-balance text-4xl font-semibold tracking-tight tabular-nums text-foreground"
    >
      {timeLabel}
    </time>
    <span class="flex w-full min-w-0 flex-col items-center gap-2 text-sm">
      {#if hijriEnabled}
        <span class="max-w-full whitespace-nowrap text-balance font-medium tabular-nums">{hijriDate}</span>
      {/if}
      <span class="max-w-full whitespace-nowrap text-balance font-medium tabular-nums">{gregorianDate}</span>
    </span>
  </button>
{/if}
</div>
