<!--
  Step 1: School General Information
  Organization name, root unit, currency, locale, timezone.
-->
<script lang="ts">
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import * as m from "$lib/paraglide/messages";
import { onboardingState } from "../onboarding-state.svelte";
</script>

<div class="max-w-3xl space-y-6">
  <div class="space-y-2 text-start">
    <h3 class="text-foreground font-bold text-xl font-serif">
      {m.onboarding_school_info_title()}
    </h3>
    <p class="text-muted-foreground text-sm">
      {m.onboarding_school_info_desc()}
    </p>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
    <!-- Arabic School Name -->
    <div class="space-y-2">
      <Label for="orgNameAr">{m.onboarding_school_name_ar()}</Label>
      <Input id="orgNameAr" bind:value={onboardingState.orgNameAr} placeholder="مثال: مدرسة صنعاء النموذجية" />
    </div>

    <!-- English School Name -->
    <div class="space-y-2">
      <Label for="orgName">{m.onboarding_school_name_en()}</Label>
      <Input id="orgName" bind:value={onboardingState.orgName} placeholder="Sanaa Model School" class="text-left font-sans" style="direction: ltr;" />
    </div>

    <!-- Arabic Root Unit -->
    <div class="space-y-2">
      <Label for="rootUnitNameAr">{m.onboarding_root_unit_ar()}</Label>
      <Input id="rootUnitNameAr" bind:value={onboardingState.rootUnitNameAr} placeholder="مدرسة صنعاء النموذجية" />
    </div>

    <!-- English Root Unit -->
    <div class="space-y-2">
      <Label for="rootUnitName">{m.onboarding_root_unit_en()}</Label>
      <Input id="rootUnitName" bind:value={onboardingState.rootUnitName} placeholder="Sanaa Model School" class="text-left font-sans" style="direction: ltr;" />
    </div>

    <!-- Currency -->
    <div class="space-y-2">
      <Label for="currency">{m.onboarding_currency()}</Label>
      <Select type="single" value={onboardingState.currency} onValueChange={(v) => (onboardingState.currency = v ?? "YER")}>
        <SelectTrigger class="w-full h-9 text-sm">
          {#if onboardingState.currency === 'YER'}ريال يمني (YER)
          {:else if onboardingState.currency === 'SAR'}ريال سعودي (SAR)
          {:else}دولار أمريكي (USD)
          {/if}
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="YER">ريال يمني (YER)</SelectItem>
          <SelectItem value="SAR">ريال سعودي (SAR)</SelectItem>
          <SelectItem value="USD">دولار أمريكي (USD)</SelectItem>
        </SelectContent>
      </Select>
      <p class="text-[11px] text-muted-foreground">{m.onboarding_currency_hint()}</p>
    </div>

    <!-- Locale -->
    <div class="space-y-2">
      <Label for="locale">{m.onboarding_locale()}</Label>
      <Select type="single" value={onboardingState.locale} onValueChange={(v) => (onboardingState.locale = v ?? "ar")}>
        <SelectTrigger class="w-full h-9 text-sm">
          {onboardingState.locale === 'ar' ? 'العربية (Arabic)' : 'English (الإنجليزية)'}
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="ar">العربية (Arabic)</SelectItem>
          <SelectItem value="en">English (الإنجليزية)</SelectItem>
        </SelectContent>
      </Select>
    </div>

    <!-- Timezone -->
    <div class="space-y-2">
      <Label for="timezone">{m.onboarding_timezone()}</Label>
      <Select type="single" value={onboardingState.timezone} onValueChange={(v) => (onboardingState.timezone = v ?? "Asia/Aden")}>
        <SelectTrigger class="w-full h-9 text-sm">
          {#if onboardingState.timezone === 'Asia/Aden'}توقيت صنعاء / عدن (GMT+3)
          {:else if onboardingState.timezone === 'Asia/Riyadh'}توقيت الرياض (GMT+3)
          {:else}التوقيت العالمي الموحد (UTC)
          {/if}
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="Asia/Aden">توقيت صنعاء / عدن (GMT+3)</SelectItem>
          <SelectItem value="Asia/Riyadh">توقيت الرياض (GMT+3)</SelectItem>
          <SelectItem value="UTC">التوقيت العالمي الموحد (UTC)</SelectItem>
        </SelectContent>
      </Select>
    </div>
  </div>
</div>
