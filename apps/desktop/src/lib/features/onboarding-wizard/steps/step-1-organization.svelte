<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import { onMount } from "svelte";
import * as m from "$lib/paraglide/messages";
import { api } from "$lib/api";
import { authStore, systemStore } from "$lib/stores";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Checkbox } from "@orbis/ui/checkbox";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import CurrencySelector from "$lib/shared/components/currency-selector.svelte";
import TimezoneSettings from "$lib/shared/components/timezone-settings.svelte";
import OnboardingStepHeader from "../onboarding-step-header.svelte";
import OnboardingSection from "../onboarding-section.svelte";
import { getTasksForStep } from "../onboarding-steps";
import { navigateOnboardingRoute } from "../navigation";

let { orgId }: { orgId: string } = $props();

let isLoading = $state(true);
let isSaving = $state(false);
let errorMessage = $state<string | null>(null);
let successMessage = $state<string | null>(null);

// Form Fields
let name = $state("");
let nameAr = $state("");
let slug = $state("");
let baseCurrencyCode = $state("SAR");
let timezone = $state("Asia/Riyadh");
let locale = $state("ar");
let isHijriEnabled = $state(false);

const stepTasks = $derived(getTasksForStep("organization", systemStore.onboardingTasks));
const isTaskComplete = $derived(stepTasks.every((t) => t.status === "completed"));

onMount(async () => {
  try {
    isLoading = true;
    const org = (await api.getCurrentOrg().catch(() => null)) || systemStore.onboardingState?.organization || authStore.currentOrg;
    if (org) {
      name = org.name || name;
      nameAr = org.name_ar || nameAr;
      slug = org.slug || slug;
      baseCurrencyCode = org.base_currency_code || (org as any).currency || baseCurrencyCode;
      timezone = org.timezone || timezone;
      locale = org.locale || locale;
      isHijriEnabled = Boolean(org.is_hijri_enabled ?? (org as any).hijri_enabled ?? isHijriEnabled);
    }
    if (!name && authStore.currentOrg?.name) {
      name = authStore.currentOrg.name;
    }
  } catch (err: any) {
    console.error("Failed to load organization profile:", err);
    errorMessage = err?.message || m.common_action_failed();
  } finally {
    isLoading = false;
  }
});

async function saveOrganization() {
  if (!name.trim()) {
    errorMessage = m.onboarding_step1_err_name();
    return;
  }

  isSaving = true;
  errorMessage = null;
  successMessage = null;

  try {
    await api.updateCurrentOrg({
      name: name.trim(),
      name_ar: nameAr.trim() || undefined,
      slug: slug.trim() || undefined,
      base_currency_code: baseCurrencyCode,
      timezone,
      locale,
      is_hijri_enabled: isHijriEnabled,
    });

    // Refresh onboarding DAG state
    await systemStore.loadOnboardingState(orgId);
    successMessage = m.onboarding_step1_saved_success();
  } catch (err: any) {
    console.error("Failed to save organization profile:", err);
    errorMessage = err?.message || m.common_action_failed();
  } finally {
    isSaving = false;
  }
}

async function handleContinue() {
  if (!isTaskComplete) {
    await saveOrganization();
  }
  await navigateOnboardingRoute(`/${orgId}/onboarding/structure`);
}
</script>

<div class="space-y-6">
  <OnboardingStepHeader
    stepNumber={1}
    title={m.onboarding_step1_title()}
    description={m.onboarding_step1_subtitle()}
    isRequired={true}
    status={isTaskComplete ? "completed" : "in_progress"}
  />

  {#if errorMessage}
    <div class="rounded-lg border border-destructive/30 bg-destructive/10 p-4 text-sm text-destructive font-medium flex items-center justify-between">
      <span>{errorMessage}</span>
      <Button variant="ghost" size="sm" onclick={() => errorMessage = null}>✕</Button>
    </div>
  {/if}

  {#if successMessage}
    <div class="rounded-lg border border-emerald-500/30 bg-emerald-500/10 p-4 text-sm text-emerald-600 dark:text-emerald-400 font-medium flex items-center justify-between">
      <span>{successMessage}</span>
      <Button variant="ghost" size="sm" onclick={() => successMessage = null}>✕</Button>
    </div>
  {/if}

  {#if isLoading}
    <div class="flex items-center justify-center p-12 text-muted-foreground text-sm gap-2">
      <LoadingIndicator size={16} class="size-4 text-primary" />
      <span>{m.common_control_loading()}</span>
    </div>
  {:else}
    <!-- Basic Identity Section -->
    <OnboardingSection
      title={m.onboarding_step1_identity_title()}
      description={m.onboarding_step1_identity_desc()}
      isComplete={Boolean(name.trim())}
    >
      <div class="grid gap-4 sm:grid-cols-2">
        <div class="space-y-2">
          <Label for="org-name" class="text-xs font-semibold">
            {m.onboarding_step1_name_en()}
          </Label>
          <Input
            id="org-name"
            bind:value={name}
            placeholder={m.onboarding_step1_name_en_placeholder()}
            required
          />
        </div>

        <div class="space-y-2">
          <Label for="org-name-ar" class="text-xs font-semibold">
            {m.onboarding_step1_name_ar()}
          </Label>
          <Input
            id="org-name-ar"
            bind:value={nameAr}
            placeholder={m.onboarding_step1_name_ar_placeholder()}
          />
        </div>

        <div class="space-y-2 sm:col-span-2">
          <Label for="org-slug" class="text-xs font-semibold">
            {m.onboarding_step1_slug()}
          </Label>
          <Input
            id="org-slug"
            bind:value={slug}
            placeholder={m.onboarding_step1_slug_placeholder()}
            class="font-mono text-xs"
          />
          <p class="text-[11px] text-muted-foreground">
            {m.onboarding_step1_slug_desc()}
          </p>
        </div>
      </div>
    </OnboardingSection>

    <!-- Regional & Financial Preferences -->
    <OnboardingSection
      title={m.onboarding_step1_regional_title()}
      description={m.onboarding_step1_regional_desc()}
      isComplete={Boolean(baseCurrencyCode && timezone)}
    >
      <div class="grid gap-4 sm:grid-cols-2">
        <!-- Currency Select (Database-backed) -->
        <CurrencySelector
          bind:currentCurrency={baseCurrencyCode}
          compact={true}
          title={m.onboarding_step1_currency()}
        />

        <!-- Timezone Select (Location-backed) -->
        <TimezoneSettings
          bind:currentTimezone={timezone}
          compact={true}
          title={m.onboarding_step1_timezone()}
        />

        <!-- System Locale Select -->
        <div class="space-y-2">
          <Label class="text-xs font-semibold">
            {m.onboarding_step1_locale()}
          </Label>
          <Select type="single" bind:value={locale}>
            <SelectTrigger class="w-full">
              {locale === "ar" ? m.onboarding_step1_locale_ar() : m.onboarding_step1_locale_en()}
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="ar">{m.onboarding_step1_locale_ar()}</SelectItem>
              <SelectItem value="en">{m.onboarding_step1_locale_en()}</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <!-- Hijri Calendar Toggle -->
        <div class="flex items-center space-x-3 space-x-reverse rounded-lg border border-border/50 p-3 self-end">
          <Checkbox id="hijri-toggle" bind:checked={isHijriEnabled} />
          <div class="space-y-0.5">
            <Label for="hijri-toggle" class="text-xs font-medium cursor-pointer">
              {m.onboarding_step1_hijri_toggle()}
            </Label>
            <p class="text-[11px] text-muted-foreground">
              {m.onboarding_step1_hijri_desc()}
            </p>
          </div>
        </div>
      </div>
    </OnboardingSection>

    <!-- Inline Actions Bar -->
    <div class="mt-8 flex items-center justify-end gap-3 pb-8 pt-4 border-t border-border/40">
      <Button
        variant="secondary"
        disabled={isSaving}
        onclick={saveOrganization}
        class="text-xs"
      >
        {isSaving ? m.onboarding_saving_btn() : m.onboarding_save_btn()}
      </Button>

      <Button
        variant="default"
        disabled={isSaving || !name.trim()}
        onclick={handleContinue}
        class="text-xs font-semibold shadow-xs"
      >
        {m.onboarding_step1_continue_btn()}
      </Button>
    </div>
  {/if}
</div>
