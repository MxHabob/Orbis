<!--
@component TimezoneSettings
A reusable timezone selection row using `@orbis/location`, `ComboboxDropdown`, and `SettingsRow`.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { getTimezones } from "@orbis/location/timezones";
import { ComboboxDropdown } from "@orbis/ui/combobox-dropdown";
import SettingsRow from "$lib/shared/components/settings-row.svelte";

let {
  title = "Timezone",
  description = "Choose the default timezone for the organization.",
  currentTimezone = $bindable("UTC"),
  last = false,
  onchange,
}: {
  title?: string;
  description?: string;
  currentTimezone?: string;
  last?: boolean;
  onchange?: (timezone: string) => void;
} = $props();

const timezoneItems = getTimezones().map((tz, index) => ({
  id: index.toString(),
  label: tz.label,
  value: tz.tzCode,
}));

const selectedItem = $derived(
  timezoneItems.find((item) => item.value === currentTimezone),
);

function handleSelect(item: { id: string; label: string; value: string }) {
  if (item.value !== currentTimezone) {
    currentTimezone = item.value;
    onchange?.(item.value);
  }
}
</script>

<SettingsRow {title} {description} {last}>
  {#snippet children()}
    <ComboboxDropdown
      placeholder="Select timezone..."
      searchPlaceholder="Search timezone..."
      {selectedItem}
      items={timezoneItems}
      onSelect={handleSelect}
      class="w-full"
    />
  {/snippet}
</SettingsRow>
