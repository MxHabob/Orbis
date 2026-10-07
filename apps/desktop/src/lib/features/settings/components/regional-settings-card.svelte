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
  formatLocale: string;
  timeFormat: string;
  hijriEnabled: boolean;
  isSaving?: boolean;
  onLanguageChange: (newLocale: string) => void | boolean | Promise<void | boolean>;
}

let {
  currency = $bindable(),
  timezone = $bindable(),
  locale = $bindable(),
  dateFormat = $bindable(),
  formatLocale = $bindable(),
  timeFormat = $bindable(),
  hijriEnabled = $bindable(),
  isSaving = false,
  onLanguageChange,
}: Props = $props();

const dateFormats = [
  { value: "YYYY-MM-DD", label: m.settings_general_date_iso(), example: "2026-06-24" },
  { value: "DD/MM/YYYY", label: m.settings_general_date_numeric_dmy(), example: "24/06/2026" },
  { value: "MM/DD/YYYY", label: m.settings_general_date_numeric_mdy(), example: "06/24/2026" },
  { value: "DD MMM YYYY", label: m.settings_general_date_short_dmy(), example: "24 Jun 2026" },
  { value: "MMM DD, YYYY", label: m.settings_general_date_short_mdy(), example: "Jun 24, 2026" },
  { value: "DD MMMM YYYY", label: m.settings_general_date_long_dmy(), example: "24 June 2026" },
  { value: "MMMM DD, YYYY", label: m.settings_general_date_long_mdy(), example: "June 24, 2026" },
  { value: "EEEE, DD MMMM YYYY", label: m.settings_general_date_full_dmy(), example: "Wednesday, 24 June 2026" },
];

const formatLocales = [
  { value: "auto", label: m.settings_general_format_locale_auto() },
  { value: "ar-SA", label: m.settings_general_format_locale_arabic() },
  { value: "ar-SA-u-nu-latn", label: m.settings_general_format_locale_arabic_latin_digits() },
  { value: "en-US", label: m.settings_general_format_locale_english_us() },
  { value: "en-GB", label: m.settings_general_format_locale_english_uk() },
];

const timeFormats = [
  { value: "locale", label: m.settings_general_time_format_locale() },
  { value: "12-hour", label: m.settings_general_time_format_12_hour() },
  { value: "24-hour", label: m.settings_general_time_format_24_hour() },
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

    <SettingsRow
      title={m.settings_general_format_locale()}
      description={m.settings_general_format_locale_desc()}
    >
      {#snippet children()}
        <SearchableSelect
          bind:value={formatLocale}
          items={formatLocales}
          placeholder={m.settings_general_format_locale()}
          disabled={isSaving}
          getItemValue={(item) => item.value}
          getItemLabel={(item) => item.label}
          class="w-full"
        />
      {/snippet}
    </SettingsRow>

    <SettingsRow
      title={m.settings_general_time_format()}
      description={m.settings_general_time_format_desc()}
    >
      {#snippet children()}
        <SearchableSelect
          bind:value={timeFormat}
          items={timeFormats}
          placeholder={m.settings_general_time_format()}
          disabled={isSaving}
          getItemValue={(item) => item.value}
          getItemLabel={(item) => item.label}
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
