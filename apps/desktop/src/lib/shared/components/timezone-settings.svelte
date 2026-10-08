<!--
  @component TimezoneSettings
  A reusable timezone selection component using `@orbis/location`, `SearchableSelect`, and `SettingsRow`.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { getTimezones } from "@orbis/location/timezones";
import SearchableSelect from "$lib/shared/components/searchable-select.svelte";
import SettingsRow from "$lib/shared/components/settings-row.svelte";
import { Label } from "@orbis/ui/label";

let {
  title = m.common_general_timezone(),
  description = m.settings_general_timezone_desc(),
  currentTimezone = $bindable("Asia/Aden"),
  compact = false,
  disabled = false,
  last = false,
  onchange,
}: {
  title?: string;
  description?: string;
  currentTimezone?: string;
  compact?: boolean;
  disabled?: boolean;
  last?: boolean;
  onchange?: (timezone: string) => void;
} = $props();

const timezones = getTimezones();

function getTimezoneValue(tz: any): string {
  return tz?.tzCode || "";
}

function getTimezoneLabel(tz: any): string {
  return tz?.label || tz?.tzCode || "";
}

function getTimezoneSubtitle(tz: any): string | undefined {
  if (tz?.name && tz?.name !== tz?.label) {
    return tz.name;
  }
  return undefined;
}

function getTimezoneBadge(tz: any): string | undefined {
  return tz?.utc || undefined;
}

function handleChange(val: string | null) {
  const next = val || "UTC";
  currentTimezone = next;
  onchange?.(next);
}
</script>

{#if compact}
  <div class="flex flex-col gap-1.5">
    <Label class="text-xs font-semibold">
      {title}
    </Label>
    {#if description}
      <span class="text-[11px] text-muted-foreground leading-relaxed">{description}</span>
    {/if}
    <div class="w-full">
      <SearchableSelect
        bind:value={currentTimezone}
        items={timezones}
        placeholder={m.common_general_timezone()}
        searchPlaceholder={m.shared_ui_search_timezones_placeholder()}
        emptyText={m.shared_ui_no_matching_timezones()}
        {disabled}
        getItemValue={getTimezoneValue}
        getItemLabel={getTimezoneLabel}
        getItemSubtitle={getTimezoneSubtitle}
        getItemBadge={getTimezoneBadge}
        onchange={handleChange}
        class="w-full"
      />
    </div>
  </div>
{:else}
  <SettingsRow {title} {description} {last}>
    {#snippet children()}
      <SearchableSelect
        bind:value={currentTimezone}
        items={timezones}
        placeholder={m.common_general_timezone()}
        searchPlaceholder={m.shared_ui_search_timezones_placeholder()}
        emptyText={m.shared_ui_no_matching_timezones()}
        {disabled}
        getItemValue={getTimezoneValue}
        getItemLabel={getTimezoneLabel}
        getItemSubtitle={getTimezoneSubtitle}
        getItemBadge={getTimezoneBadge}
        onchange={handleChange}
        class="w-full"
      />
    {/snippet}
  </SettingsRow>
{/if}

