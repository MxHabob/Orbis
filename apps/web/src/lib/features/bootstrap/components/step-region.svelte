<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  COUNTRIES_LIST,
  COLOR_PRESETS,
  type OnboardingFormModel,
} from "../types";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Checkbox } from "@orbis/ui/checkbox";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { Check } from "@orbis/icons";
import { Card, CardContent } from "@orbis/ui/card";
import CurrencySelector from "$lib/shared/components/currency-selector.svelte";
import TimezoneSettings from "$lib/shared/components/timezone-settings.svelte";

let {
  form = $bindable<OnboardingFormModel>(),
  errors = {},
}: {
  form: OnboardingFormModel;
  errors?: Record<string, string>;
} = $props();

function selectCountry(countryCode: string) {
  form.countryCode = countryCode;
  const country = COUNTRIES_LIST.find((item) => item.code === countryCode);
  if (country?.defaultCurrency) form.baseCurrencyCode = country.defaultCurrency;
  if (country?.defaultTimezone) form.timezone = country.defaultTimezone;
}
</script>

<section class="mx-auto w-full max-w-3xl space-y-6 py-2">
  <header class="space-y-2 text-start">
    <p class="max-w-2xl text-sm leading-relaxed text-muted-foreground">
      {m.operations_ui_organization_setup_description()}
    </p>
  </header>

  <Card>
    <CardContent class="space-y-6 p-5 sm:p-7">
    <div class="grid grid-cols-1 gap-5 sm:grid-cols-2">
      <div class="space-y-1.5">
        <Label for="org-country" class="text-xs font-medium text-foreground">
          {m.ui_onboarding_organization_country_region()}
        </Label>
        <Select
          type="single"
          value={form.countryCode}
          onValueChange={(value) => value && selectCountry(value)}
        >
          <SelectTrigger id="org-country" class="h-10 w-full">
            {@const selectedCountry = COUNTRIES_LIST.find((item) => item.code === form.countryCode)}
            {selectedCountry ? `${selectedCountry.flag} ${selectedCountry.name()}` : form.countryCode}
          </SelectTrigger>
          <SelectContent>
            {#each COUNTRIES_LIST as country (country.code)}
              <SelectItem value={country.code}>
                {country.flag} {country.name()}
              </SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>

      <div class="space-y-1.5">
        <Label for="org-city" class="text-xs font-medium text-foreground">
          {m.ui_onboarding_organization_city_campus()}
        </Label>
        <Input
          id="org-city"
          type="text"
          placeholder={m.ui_onboarding_organization_e_g_riyadh()}
          bind:value={form.city}
          class="h-10"
        />
      </div>
    </div>

    <div class="grid grid-cols-1 gap-5 sm:grid-cols-2">
      <div class="space-y-1.5">
        <Label for="org-phone" class="text-sm font-medium">
          {m.setup_bootstrap_organization_phone_optional()}
        </Label>
        <Input id="org-phone" type="tel" bind:value={form.phone} autocomplete="tel" />
      </div>
      <div class="space-y-1.5">
        <Label for="org-website" class="text-sm font-medium">
          {m.setup_bootstrap_website_optional()}
        </Label>
        <Input id="org-website" type="url" bind:value={form.website} autocomplete="url" />
      </div>
      <div class="space-y-1.5">
        <Label for="org-tax-id" class="text-sm font-medium">
          {m.setup_bootstrap_tax_id_optional()}
        </Label>
        <Input id="org-tax-id" bind:value={form.taxId} />
      </div>
      <div class="space-y-1.5">
        <Label for="org-license-number" class="text-sm font-medium">
          {m.setup_bootstrap_license_number_optional()}
        </Label>
        <Input id="org-license-number" bind:value={form.licenseNumber} />
      </div>
    </div>

    <div class="space-y-1.5">
      <Label for="org-locale" class="text-xs font-medium text-foreground">
        {m.settings_appearance_language()}
        <span class="text-destructive" aria-hidden="true"> *</span>
      </Label>
      <Select
        type="single"
        value={form.locale}
        onValueChange={(value) => (form.locale = value ?? form.locale)}
      >
        <SelectTrigger
          id="org-locale"
          class="h-10 w-full sm:max-w-sm"
          aria-invalid={errors.locale ? "true" : undefined}
          aria-describedby={errors.locale ? "org-locale-error" : undefined}
        >
          {form.locale === "en-US"
            ? m.shared_ui_locale_english_united_states()
            : m.shared_ui_locale_arabic_saudi()}
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="ar-SA">{m.shared_ui_locale_arabic_saudi()}</SelectItem>
          <SelectItem value="en-US">{m.shared_ui_locale_english_united_states()}</SelectItem>
        </SelectContent>
      </Select>
      {#if errors.locale}
        <p id="org-locale-error" class="text-xs text-destructive" role="alert">
          {errors.locale}
        </p>
      {/if}
    </div>

    <div class="grid grid-cols-1 gap-5 sm:grid-cols-2">
      <CurrencySelector
        bind:currentCurrency={form.baseCurrencyCode}
        compact={true}
        title={m.ui_onboarding_organization_base_currency()}
      />
      <div class="space-y-1.5">
        <TimezoneSettings
          bind:currentTimezone={form.timezone}
          compact={true}
          title={m.ui_onboarding_organization_timezone()}
          required
        />
        {#if errors.timezone}
          <p class="text-xs text-destructive" role="alert">{errors.timezone}</p>
        {/if}
      </div>
    </div>

    <div class="flex items-center justify-between gap-4 rounded-xl border border-border/60 bg-muted/20 p-4">
      <div class="space-y-1">
        <Label for="hijri-toggle" class="cursor-pointer text-sm font-medium text-foreground">
          {m.ui_onboarding_organization_dual_hijri_gregorian_calendar()}
        </Label>
        <p class="text-xs leading-relaxed text-muted-foreground">
          {m.ui_onboarding_organization_display_corresponding_umm_al_qura_hijri_dates_across_modules()}
        </p>
      </div>
      <Checkbox id="hijri-toggle" bind:checked={form.isHijriEnabled} />
    </div>

    <fieldset class="space-y-3">
      <legend class="text-sm font-medium text-foreground">
        {m.ui_onboarding_organization_accent_color_theme()}
      </legend>
      <div class="flex flex-wrap items-center gap-3">
        {#each COLOR_PRESETS as color (color.name)}
          {@const isColorActive = form.primaryColor === color.value}
          <button
            type="button"
            aria-label={color.label()}
            aria-pressed={isColorActive}
            onclick={() => (form.primaryColor = color.value)}
            class="flex size-9 items-center justify-center rounded-full transition-transform focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2 {isColorActive
              ? 'scale-110 ring-2 ring-foreground ring-offset-2 ring-offset-background'
              : 'hover:scale-105'}"
            style="background-color: {color.value};"
            title={color.label()}
          >
            {#if isColorActive}
              <Check size={16} class="text-white" strokeWidth={3} />
            {/if}
          </button>
        {/each}
      </div>
    </fieldset>
    </CardContent>
  </Card>
</section>
