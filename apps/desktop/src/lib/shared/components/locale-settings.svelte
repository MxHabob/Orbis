<!--
@component LocaleSettings
A reusable locale/language selection row using `@orbis/location`, `ComboboxDropdown`, and `SettingsRow`.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { countries } from "@orbis/location/countries-intl";
import { ComboboxDropdown } from "@orbis/ui/combobox-dropdown";
import SettingsRow from "$lib/shared/components/settings-row.svelte";

let {
  title = m.settings_appearance_language
    ? m.settings_appearance_language()
    : "Language & Region",
  description = m.common_account_preferences_lang_desc
    ? m.common_account_preferences_lang_desc()
    : "Choose your preferred language for the interface.",
  currentLocale = $bindable("ar"),
  last = false,
  onchange,
}: {
  title?: string;
  description?: string;
  currentLocale?: string;
  last?: boolean;
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

<SettingsRow {title} {description} {last}>
  {#snippet children()}
    <ComboboxDropdown
      placeholder="Select language..."
      searchPlaceholder="Search language..."
      {selectedItem}
      items={localeItems}
      onSelect={handleSelect}
      class="w-full"
    />
  {/snippet}
</SettingsRow>
