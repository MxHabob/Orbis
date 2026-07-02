<!--
@component LocaleSettings
A reusable locale/language selection card using `@orbis/location` and `ComboboxDropdown`.
-->
<script lang="ts">
import { Label } from "@orbis/ui-svelte/label";
import { countries } from "@orbis/location/countries-intl";
import { ComboboxDropdown } from "@orbis/ui-svelte/combobox-dropdown";

let {
  title = "Language & Region",
  description = "Choose your preferred language for the interface.",
  currentLocale = $bindable("ar"),
  onchange,
}: {
  title?: string;
  description?: string;
  currentLocale?: string;
  onchange?: (locale: string) => void;
} = $props();

const localeItems = Object.values(countries).map((c, index) => ({
  id: index.toString(),
  label: `${c.name} (${c.default_locale})`,
  value: c.default_locale,
}));

const selectedItem = $derived(
  localeItems.find((item) => item.value === currentLocale),
);

function handleSelect(item: { id: string; label: string; value: string }) {
  if (item.value !== currentLocale) {
    currentLocale = item.value;
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
      placeholder="Select language..."
      searchPlaceholder="Search language..."
      {selectedItem}
      items={localeItems}
      onSelect={handleSelect}
      class="w-full"
    />
  </div>
</div>

