<script lang="ts">
import { onMount } from "svelte";
import * as m from "$lib/paraglide/messages";
import { authStore } from "$lib/stores";
import { orgContext } from "$lib/stores/org-context.svelte";
import { createOrganizationQuery } from "$lib/features/settings/queries";
import { formatDate } from "$lib/core/i18n";
import { Clock } from "@orbis/icons";
import { Popover, PopoverContent, PopoverTrigger } from "@orbis/ui/popover";

const organizationQuery = createOrganizationQuery(() => authStore.orgId);
let now = $state(new Date());
let isOpen = $state(false);

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

<Popover bind:open={isOpen}>
  <PopoverTrigger>
    {#snippet child({ props }: { props: any })}
      <button
        {...props}
        type="button"
        class="flex h-7 shrink-0 items-center gap-1.5 px-2 text-[11px] tabular-nums text-muted-foreground transition-colors hover:bg-accent hover:text-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-ring"
        title={m.ui_shell_clock_open()}
        aria-label={m.ui_shell_clock_open()}
      >
        <Clock size={14} />
        <time datetime={now.toISOString()}>{timeLabel}</time>
      </button>
    {/snippet}
  </PopoverTrigger>

  <PopoverContent
    side="top"
    align="end"
    sideOffset={6}
    class="w-[min(18rem,calc(100dvw-1rem))] rounded-md border border-border bg-card p-4 text-foreground shadow-md"
  >
    <div class="flex flex-col items-center gap-3">
      <time
        datetime={now.toISOString()}
        class="text-4xl font-semibold tracking-tight tabular-nums"
      >
        {timeLabel}
      </time>
      {#if hijriEnabled}
        <time
          datetime={now.toISOString()}
          class="text-sm font-medium tabular-nums text-muted-foreground"
        >
          {hijriDate}
        </time>
      {/if}
      <time
        datetime={now.toISOString()}
        class="text-sm font-medium tabular-nums text-muted-foreground"
      >
        {gregorianDate}
      </time>
    </div>
  </PopoverContent>
</Popover>
