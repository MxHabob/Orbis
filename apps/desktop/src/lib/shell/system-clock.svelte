<script lang="ts">
import { onMount } from "svelte";
import * as m from "$lib/paraglide/messages";
import { authStore } from "$lib/stores";
import { createOrganizationQuery } from "$lib/features/settings/queries";
import { formatDate } from "$lib/core/i18n";
import { Button } from "@orbis/ui/button";
import { Clock, Pin, PinSlash, XMark } from "@orbis/icons";

const organizationQuery = createOrganizationQuery(() => authStore.orgId);
let now = $state(new Date());
let isOpen = $state(false);
let isPinned = $state(false);

const timeZone = $derived(organizationQuery.data?.timezone || "UTC");
const hijriEnabled = $derived(organizationQuery.data?.is_hijri_enabled ?? false);
const timeLabel = $derived(formatDate(now, "time"));
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
  isOpen = !isOpen;
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
  aria-label={isOpen ? m.ui_shell_clock_close() : m.ui_shell_clock_open()}
  aria-expanded={isOpen}
  title={timeLabel}
>
  <Clock size={18} />
</Button>

{#if isOpen}
  <section
    class={"fixed bg-card/60 backdrop-blur-md z-100 transition-[inset,width,border-radius] duration-200 bottom-3 inset-s-14 w-72"}
    aria-label={m.ui_shell_clock_panel_title()}
  >
    <div class="flex flex-col gap-4 p-5">
      <time
        datetime={now.toISOString()}
        class="text-4xl font-semibold tracking-tight tabular-nums text-foreground"
      >
        {timeLabel}
      </time>
      <div class="flex flex-col gap-3 text-sm">
        {#if hijriEnabled}
          <div class="flex items-center justify-between gap-3">
            <span class="text-muted-foreground">{m.ui_shell_clock_hijri_calendar()}</span>
            <span class="font-medium tabular-nums">{hijriDate}</span>
          </div>
        {/if}
        <div class="flex items-center justify-between gap-3">
          <span class="text-muted-foreground">{m.ui_shell_clock_gregorian_calendar()}</span>
          <span class="font-medium tabular-nums">{gregorianDate}</span>
        </div>
      </div>
    </div>
  </section>
{/if}
</div>
