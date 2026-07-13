<!--
@component TimezoneSettings
A reusable timezone selection component using `@orbis/location` and `ComboboxDropdown`.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Label } from "@orbis/ui-svelte/label";
import { getTimezones } from "@orbis/location/timezones";
import { ComboboxDropdown } from "@orbis/ui-svelte/combobox-dropdown";

let {
  title = "Timezone",
  description = "Choose the default timezone for the organization.",
  currentTimezone = $bindable("UTC"),
  onchange,
}: {
  title?: string;
  description?: string;
  currentTimezone?: string;
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

<div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
  <div class="flex flex-col gap-1 text-start">
    <Label class="text-xs font-semibold text-foreground">
      {title}
    </Label>
    <span class="text-[11px] text-muted-foreground leading-relaxed">{description}</span>
  </div>
  <div class="w-full md:w-[480px] shrink-0 flex items-center justify-end">
    <ComboboxDropdown
      placeholder="Select timezone..."
      searchPlaceholder="Search timezone..."
      {selectedItem}
      items={timezoneItems}
      onSelect={handleSelect}
      class="w-full"
    />
  </div>
</div>
