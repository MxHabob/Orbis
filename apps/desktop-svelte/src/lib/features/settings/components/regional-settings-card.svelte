<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Label } from "@orbis/ui-svelte/label";
import { Input } from "@orbis/ui-svelte/input";
import { Switch } from "@orbis/ui-svelte/switch";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import TimezoneSettings from "$lib/shared/components/timezone-settings.svelte";
import LocaleSettings from "$lib/shared/components/locale-settings.svelte";

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

<div class="border border-border rounded-md overflow-hidden flex flex-col">
  <!-- Card Header -->
  <div class="bg-muted/50 px-6 py-3 border-b border-border text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
    {m.settings_general_section_regional()}
  </div>

  <div class="flex flex-col">
    <!-- Currency -->
    <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
      <div class="flex flex-col gap-1 text-start">
        <Label for="org_currency" class="text-xs font-semibold text-foreground">
          {m.settings_general_currency()}
        </Label>
        <span class="text-[11px] text-muted-foreground leading-relaxed">{m.settings_general_currency_desc()}</span>
      </div>
      <div class="w-full md:w-[480px] shrink-0 flex items-center justify-end">
        <Input
          id="org_currency"
          type="text"
          bind:value={currency}
          placeholder={m.settings_general_currency_placeholder()}
          class="uppercase font-mono w-full text-start h-9 bg-background"
          required
          disabled={isSaving}
        />
      </div>
    </div>

    <!-- Timezone -->
    <TimezoneSettings
      title={m.common_general_timezone()}
      description={m.settings_general_timezone_desc()}
      bind:currentTimezone={timezone}
    />

    <LocaleSettings
      title={m.settings_general_locale()}
      description={m.settings_general_locale_desc()}
      bind:currentLocale={locale}
      onchange={onLanguageChange}
    />

    <!-- Date Format -->
    <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
      <div class="flex flex-col gap-1 text-start">
        <Label class="text-xs font-semibold text-foreground">
          {m.settings_general_date_format()}
        </Label>
        <span class="text-[11px] text-muted-foreground leading-relaxed">{m.settings_general_date_format_desc()}</span>
      </div>
      <div class="w-full md:w-[480px] shrink-0 flex items-center justify-end">
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
      </div>
    </div>

    <!-- Hijri Calendar Toggle -->
    <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between last:border-b-0 gap-4">
      <div class="flex flex-col gap-1 text-start">
        <Label for="hijri_enabled" class="text-xs font-semibold text-foreground">
          {m.settings_general_hijri()}
        </Label>
        <span class="text-[11px] text-muted-foreground leading-relaxed">{m.settings_general_hijri_desc()}</span>
      </div>
      <div class="w-full md:w-[480px] shrink-0 flex items-center justify-start md:justify-start">
        <Switch
          id="hijri_enabled"
          bind:checked={hijriEnabled}
          disabled={isSaving}
        />
      </div>
    </div>
  </div>
</div>
