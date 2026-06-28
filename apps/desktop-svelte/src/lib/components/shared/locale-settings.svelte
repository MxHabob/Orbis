<!--
@component LocaleSettings
A reusable locale/language selection card using `@orbis/location` and `ComboboxDropdown`.
-->
<script lang="ts">
import { Card, CardTitle, CardDescription } from "@orbis/ui-svelte/card";
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

<Card class="flex flex-col sm:flex-row justify-between items-start sm:items-center p-6 gap-4">
  <div class="space-y-1 text-start">
    <CardTitle class="text-base font-semibold text-foreground">{title}</CardTitle>
    <CardDescription class="text-xs text-muted-foreground">{description}</CardDescription>
  </div>

  <div class="w-full sm:w-[250px] shrink-0">
    <ComboboxDropdown
      placeholder="Select language..."
      searchPlaceholder="Search language..."
      {selectedItem}
      items={localeItems}
      onSelect={handleSelect}
    />
  </div>
</Card>

