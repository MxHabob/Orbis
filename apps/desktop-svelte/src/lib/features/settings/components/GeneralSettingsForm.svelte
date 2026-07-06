<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import * as m from "$lib/paraglide/messages";
import {
  InfoSettingsCard,
  BrandingSettingsCard,
  RegionalSettingsCard,
  AvatarSettingsCard,
  DangerZoneSettingsCard,
} from "$lib/features/settings";

let { org, isSaving, onsave, onsaveavatar, onremove, onlanguagechange } = $props<{
  org: any;
  isSaving: boolean;
  onsave: (request: any) => void;
  onsaveavatar: (logo: string | null) => void;
  onremove: () => void;
  onlanguagechange: (newLocale: string, currentValues: any) => void;
}>();

// Form state initialized once with props
let name = $state(org.name || "");
let slug = $state(org.slug || "");
let id = $state(org.id || "");
let logo = $state(org.logo || "");
let primaryColor = $state(org.primary_color || "#84cc16");
let currency = $state(org.currency || "YER");
let timezone = $state(org.timezone || "Asia/Aden");
let locale = $state(org.locale || "ar");
let dateFormat = $state(org.date_format || "YYYY-MM-DD");
let hijriEnabled = $state(org.hijri_enabled === 1);

function handleSubmit(e: Event) {
  e.preventDefault();
  onsave({
    name,
    logo: logo || null,
    primary_color: primaryColor,
    currency,
    timezone,
    locale,
    date_format: dateFormat,
    hijri_enabled: hijriEnabled ? 1 : 0,
  });
}

function handleLanguageChange(newLocale: string) {
  onlanguagechange(newLocale, {
    name,
    logo: logo || null,
    primary_color: primaryColor,
    currency,
    timezone,
    locale: newLocale,
    date_format: dateFormat,
    hijri_enabled: hijriEnabled ? 1 : 0,
  });
}

function handleSaveAvatar(e: Event) {
  e.preventDefault();
  onsaveavatar(logo || null);
}
</script>

<div class="h-full w-full overflow-y-auto scrollbar-hide">
  <div class="max-w-6xl mx-auto p-6 flex flex-col gap-6">
    <form onsubmit={handleSubmit} class="flex flex-col gap-6">
      <InfoSettingsCard
        {slug}
        bind:name
        {id}
        {isSaving}
      />

      <BrandingSettingsCard
        bind:primaryColor
        {isSaving}
      />

      <RegionalSettingsCard
        bind:currency
        bind:timezone
        bind:locale
        bind:dateFormat
        bind:hijriEnabled
        {isSaving}
        onLanguageChange={handleLanguageChange}
      />

      <!-- Action Buttons -->
      <div class="flex justify-end gap-3 pb-8 border-b border-border">
        <Button
          type="submit"
          class="px-6 font-semibold h-9 text-xs"
          disabled={isSaving}
        >
          {isSaving ? m.settings_general_saving() : m.settings_general_save()}
        </Button>
      </div>
    </form>

    <AvatarSettingsCard
      bind:logo
      {name}
      {isSaving}
      onsave={handleSaveAvatar}
    />

    <DangerZoneSettingsCard
      onremove={onremove}
    />
  </div>
</div>
