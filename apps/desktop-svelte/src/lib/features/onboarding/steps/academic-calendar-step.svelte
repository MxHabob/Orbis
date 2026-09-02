<!--
  Step 2: Academic Calendar
  Academic year, semester dates, and automatic month distribution.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Input } from "@orbis/ui-svelte/input";
import { FormField } from "@orbis/ui-svelte";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";

import { onboardingState } from "../state.svelte";
import LocaleSettings from "$lib/shared/components/locale-settings.svelte";
import TimezoneSettings from "$lib/shared/components/timezone-settings.svelte";
import CurrencySelector from "$lib/shared/components/currency-selector.svelte";
import { getLocale, setLocale } from "$lib/paraglide/runtime";

async function handleLanguageChange(newLocale: string) {
  const lang = newLocale.split("-")[0]?.toLowerCase() || "ar";
  const targetLang = lang === "ar" ? "ar" : "en";

  // Persist current wizard input states to localStorage before reloading
  onboardingState.save();

  if (targetLang === getLocale()) return;

  setLocale(targetLang);
}
</script>

<div class="w-full space-y-6">

  <!-- Regional Settings Section -->
  <div class="border border-border rounded-xl overflow-hidden flex flex-col bg-card">
    <div class="bg-muted/50 px-6 py-3 border-b border-border text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none font-serif">
      {m.settings_general_section_regional()}
    </div>

    <div class="flex flex-col">
      <!-- Timezone -->
      <TimezoneSettings
        title={m.common_general_timezone()}
        description={m.settings_general_timezone_desc()}
        bind:currentTimezone={onboardingState.profile.timezone}
      />

      <!-- Language -->
      <LocaleSettings
        title={m.settings_general_locale()}
        description={m.settings_general_locale_desc()}
        bind:currentLocale={onboardingState.profile.locale}
        onchange={handleLanguageChange}
      />

      <!-- Currency -->
      <CurrencySelector
        title={m.finance_ledger_currency()}
        description={m.settings_general_currency_desc()}
        bind:currentCurrency={onboardingState.profile.currency}
      />
    </div>
  </div>

  <!-- Academic Year Section -->
  <div class="grid grid-cols-1 md:grid-cols-2 gap-6 bg-card border border-border p-6 rounded-xl">
    <div class="md:col-span-2 text-start">
      <h4 class="text-sm font-semibold text-primary border-b border-border pb-2 mb-4 font-serif">
        {m.common_onboarding_academic_year_section()}
      </h4>
    </div>

    <FormField id="academicYearNameAr" label={m.common_onboarding_academic_year_name_ar()} required>
      {#snippet children()}<Input id="academicYearNameAr" bind:value={onboardingState.calendar.academicYearNameAr} placeholder="العام الدراسي 1447 هـ" />{/snippet}
    </FormField>

    <FormField id="academicYearName" label={m.common_onboarding_academic_year_name_en()}>
      {#snippet children()}<Input id="academicYearName" bind:value={onboardingState.calendar.academicYearName} placeholder="Academic Year 1447 Hijri" class="text-left font-sans" style="direction: ltr;" />{/snippet}
    </FormField>

    <FormField id="startDate" label={m.common_onboarding_start_date()} required>
      {#snippet children()}<Input id="startDate" type="date" bind:value={onboardingState.calendar.startDate} />{/snippet}
    </FormField>

    <FormField id="endDate" label={m.common_onboarding_end_date()} required>
      {#snippet children()}<Input id="endDate" type="date" bind:value={onboardingState.calendar.endDate} />{/snippet}
    </FormField>
  </div>

  <!-- Semesters Section -->
  <div class="grid grid-cols-1 md:grid-cols-2 gap-6 bg-card border border-border p-6 rounded-xl">
    <div class="md:col-span-2 flex items-center justify-between border-b border-border pb-2 mb-4">
      <h4 class="text-sm font-semibold text-primary font-serif">
        {m.common_onboarding_semester_split()}
      </h4>
      <span class="text-xs text-muted-foreground font-mono">
        {m.common_onboarding_semester_weight_hint()}
      </span>
    </div>

    <!-- Template Selection -->
    <div class="md:col-span-2 space-y-4">
      <FormField label={m.common_onboarding_template_label()} required>
        {#snippet children()}
        <Select type="single" bind:value={onboardingState.calendar.selectedTemplateId}>
          <SelectTrigger class="w-full h-9 text-xs font-sans">
            {#if onboardingState.calendar.selectedTemplateId}
              {@const selectedTpl = onboardingState.calendar.periodTemplates.find(t => t.id === onboardingState.calendar.selectedTemplateId)}
              {#if selectedTpl}
                {getLocale() === "ar" ? selectedTpl.name_ar : selectedTpl.name}
              {:else}
                {m.common_onboarding_template_placeholder()}
              {/if}
            {:else}
              {m.common_onboarding_template_placeholder()}
            {/if}
          </SelectTrigger>
          <SelectContent>
            {#each onboardingState.calendar.periodTemplates as tpl}
              <SelectItem value={tpl.id}>{getLocale() === "ar" ? tpl.name_ar : tpl.name}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
        {/snippet}
      </FormField>

      {#if onboardingState.calendar.selectedTemplateId}
        {@const selectedTpl = onboardingState.calendar.periodTemplates.find(t => t.id === onboardingState.calendar.selectedTemplateId)}
        {#if selectedTpl?.structure && selectedTpl.structure.length > 0}
          <div class="pt-2">
            <div class="text-[11px] font-medium text-muted-foreground mb-2 text-start">
              {m.common_onboarding_period_distribution()}
            </div>
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-2">
              {#each selectedTpl.structure as term}
                <div class="p-3 bg-muted/40 border border-border rounded-lg flex items-center justify-between text-xs">
                  <span class="font-medium text-foreground">{getLocale() === "ar" ? term.name_ar : term.name}</span>
                  <span class="px-2 py-0.5 bg-primary/10 text-primary font-mono font-semibold rounded text-[11px]">
                    {term.weight}%
                  </span>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      {/if}
    </div>
  </div>
</div>
