<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Switch } from "@orbis/ui/switch";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import CurrencySelector from "$lib/shared/components/currency-selector.svelte";
import TimezoneSettings from "$lib/shared/components/timezone-settings.svelte";
import LocaleSettings from "$lib/shared/components/locale-settings.svelte";
import SettingsCard from "$lib/shared/components/settings-card.svelte";
import SettingsRow from "$lib/shared/components/settings-row.svelte";

interface Props {
  currency: string;
  timezone: string;
  locale: string;
  dateFormat: string;
  hijriEnabled: boolean;
  isSaving?: boolean;
  onLanguageChange: (newLocale: string) => void;
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
  { value: "YYYY-MM-DD", label: "YYYY-MM-DD (2026-06-24)" },
  { value: "DD/MM/YYYY", label: "DD/MM/YYYY (24/06/2026)" },
  { value: "MM/DD/YYYY", label: "MM/DD/YYYY (06/24/2026)" },
];
</script>

<SettingsCard title={m.settings_general_section_regional()}>
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
  />

  <!-- Locale / Language -->
  <LocaleSettings
    title={m.settings_general_locale()}
    description={m.settings_general_locale_desc()}
    bind:currentLocale={locale}
    onchange={onLanguageChange}
  />

  <!-- Date Format -->
  <SettingsRow
    title={m.settings_general_date_format()}
    description={m.settings_general_date_format_desc()}
  >
    {#snippet children()}
      <Select type="single" value={dateFormat} onValueChange={(v: string | undefined) => (dateFormat = v ?? "")} disabled={isSaving}>
        <SelectTrigger class="h-9 w-full">
          {dateFormats.find(df => df.value === dateFormat)?.label || dateFormat}
        </SelectTrigger>
        <SelectContent>
          {#each dateFormats as df}
            <SelectItem value={df.value}>{df.label}</SelectItem>
          {/each}
        </SelectContent>
      </Select>
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
</SettingsCard>
