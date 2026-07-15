<!--
  Step 2: Academic Calendar
  Academic year, semester dates, and automatic month distribution.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";

import { onboardingState } from "../state.svelte";
import LocaleSettings from "$lib/shared/components/locale-settings.svelte";
import TimezoneSettings from "$lib/shared/components/timezone-settings.svelte";
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
      <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4 last:border-b-0">
        <div class="flex flex-col gap-1 text-start">
          <Label for="onboarding_currency" class="text-xs font-semibold text-foreground">
            {m.finance_ledger_currency()}
          </Label>
          <span class="text-[11px] text-muted-foreground leading-relaxed">{m.settings_general_currency_desc()}</span>
        </div>
        <div class="w-full md:w-[480px] shrink-0 flex items-center justify-end">
          <Select type="single" bind:value={onboardingState.profile.currency}>
            <SelectTrigger id="onboarding_currency" class="h-9 text-xs uppercase font-mono bg-background text-end" style="direction: ltr;">
              {#if onboardingState.profile.currency === "YER"}ريال يمني (YER)
              {:else if onboardingState.profile.currency === "SAR"}ريال سعودي (SAR)
              {:else if onboardingState.profile.currency === "AED"}درهم إماراتي (AED)
              {:else if onboardingState.profile.currency === "EGP"}جنيه مصري (EGP)
              {:else if onboardingState.profile.currency === "USD"}دولار أمريكي (USD)
              {:else if onboardingState.profile.currency === "EUR"}يورو (EUR)
              {:else if onboardingState.profile.currency === "GBP"}جنيه إسترليني (GBP)
              {:else}{onboardingState.profile.currency || m.settings_general_currency_placeholder()}
              {/if}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="YER">ريال يمني (YER)</SelectItem>
              <SelectItem value="SAR">ريال سعودي (SAR)</SelectItem>
              <SelectItem value="AED">درهم إماراتي (AED)</SelectItem>
              <SelectItem value="EGP">جنيه مصري (EGP)</SelectItem>
              <SelectItem value="USD">دولار أمريكي (USD)</SelectItem>
              <SelectItem value="EUR">يورو (EUR)</SelectItem>
              <SelectItem value="GBP">جنيه إسترليني (GBP)</SelectItem>
            </SelectContent>
          </Select>
        </div>
      </div>
    </div>
  </div>

  <!-- Academic Year Section -->
  <div class="grid grid-cols-1 md:grid-cols-2 gap-6 bg-card border border-border p-6 rounded-xl">
    <div class="md:col-span-2 text-start">
      <h4 class="text-sm font-semibold text-primary border-b border-border pb-2 mb-4 font-serif">
        {m.common_onboarding_academic_year_section()}
      </h4>
    </div>

    <div class="space-y-2 text-start">
      <Label for="academicYearNameAr">{m.common_onboarding_academic_year_name_ar()}</Label>
      <Input id="academicYearNameAr" bind:value={onboardingState.calendar.academicYearNameAr} placeholder="العام الدراسي 1447 هـ" />
    </div>

    <div class="space-y-2 text-start">
      <Label for="academicYearName">{m.common_onboarding_academic_year_name_en()}</Label>
      <Input id="academicYearName" bind:value={onboardingState.calendar.academicYearName} placeholder="Academic Year 1447 Hijri" class="text-left font-sans" style="direction: ltr;" />
    </div>

    <div class="space-y-2 text-start">
      <Label for="startDate">{m.common_onboarding_start_date()}</Label>
      <Input id="startDate" type="date" bind:value={onboardingState.calendar.startDate} />
    </div>

    <div class="space-y-2 text-start">
      <Label for="endDate">{m.common_onboarding_end_date()}</Label>
      <Input id="endDate" type="date" bind:value={onboardingState.calendar.endDate} />
    </div>
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
      <div class="space-y-2 text-start">
        <Label class="text-xs font-semibold">{m.common_onboarding_template_label()}</Label>
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
      </div>

    </div>
  </div>
</div>
