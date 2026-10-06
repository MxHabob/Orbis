<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Switch } from "@orbis/ui/switch";
import { Card, CardHeader, CardTitle, CardContent } from "@orbis/ui/card";
import CurrencySelector from "$lib/shared/components/currency-selector.svelte";
import TimezoneSettings from "$lib/shared/components/timezone-settings.svelte";
import LocaleSettings from "$lib/shared/components/locale-settings.svelte";
import SearchableSelect from "$lib/shared/components/searchable-select.svelte";
import SettingsRow from "$lib/shared/components/settings-row.svelte";

interface Props {
  currency: string;
  timezone: string;
  locale: string;
  dateFormat: string;
  hijriEnabled: boolean;
  isSaving?: boolean;
  onLanguageChange: (newLocale: string) => void | boolean | Promise<void | boolean>;
}

let {
  currency = $bindable(),
  timezone = $bindable(),
  locale = $bindable(),
  dateFormat = $bindable(),
  hijriEnabled = $bindable(),
  isSaving = false,
  onLanguageChange,
}: Props = $props();

const dateFormats = [
  { value: "YYYY-MM-DD", label: "YYYY-MM-DD (2026-06-24)", example: "ISO Standard" },
  { value: "DD/MM/YYYY", label: "DD/MM/YYYY (24/06/2026)", example: "British / Arabic Standard" },
  { value: "MM/DD/YYYY", label: "MM/DD/YYYY (06/24/2026)", example: "US Standard" },
];
</script>

<Card>
  <CardHeader>
    <CardTitle>{m.settings_general_section_regional()}</CardTitle>
  </CardHeader>
  <CardContent>
    <!-- Currency -->
    <CurrencySelector
      title={m.finance_ledger_currency()}
      description={m.settings_general_currency_desc()}
      bind:currentCurrency={currency}
      disabled={isSaving}
    />

    <!-- Timezone -->
    <TimezoneSettings
      title={m.common_general_timezone()}
      description={m.settings_general_timezone_desc()}
      bind:currentTimezone={timezone}
      disabled={isSaving}
    />

    <!-- Locale / Language -->
    <LocaleSettings
      title={m.settings_general_locale()}
      description={m.settings_general_locale_desc()}
      bind:currentLocale={locale}
      selectionMode="regional"
      disabled={isSaving}
      onchange={onLanguageChange}
    />

    <!-- Date Format -->
    <SettingsRow
      title={m.settings_general_date_format()}
      description={m.settings_general_date_format_desc()}
    >
      {#snippet children()}
        <SearchableSelect
          bind:value={dateFormat}
          items={dateFormats}
          placeholder={m.settings_general_date_format()}
          searchPlaceholder={m.settings_ui_date_format_search_placeholder()}
          disabled={isSaving}
          getItemValue={(df) => df.value}
          getItemLabel={(df) => df.label}
          getItemSubtitle={(df) => df.example}
          getItemBadge={(df) => df.value}
          class="w-full"
        />
      {/snippet}
    </SettingsRow>

    <!-- Hijri Calendar Toggle -->
    <SettingsRow
      title={m.settings_general_hijri()}
      description={m.settings_general_hijri_desc()}
      labelFor="hijri_enabled"
      controlClass="justify-start md:justify-start"
      last
    >
      {#snippet children()}
        <Switch
          id="hijri_enabled"
          bind:checked={hijriEnabled}
          disabled={isSaving}
        />
      {/snippet}
    </SettingsRow>
  </CardContent>
</Card>
