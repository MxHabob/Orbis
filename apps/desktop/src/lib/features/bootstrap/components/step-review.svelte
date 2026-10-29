<!-- @component StepReview -->
<script lang="ts">
import { onDestroy } from "svelte";
import * as m from "$lib/paraglide/messages";
import {
  type OnboardingFormModel,
  CURRENCY_OPTIONS,
  TIMEZONE_OPTIONS,
  COUNTRIES_LIST,
  EDUCATIONAL_TRACK_OPTIONS,
  ORGANIZATION_SIZE_OPTIONS,
} from "../types";
import type { BootstrapResult } from "$lib/api/types";
import { getBlueprint } from "@orbis/blueprints";
import { getLocale } from "$lib/paraglide/runtime";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Card, CardContent } from "@orbis/ui/card";
import { Avatar, AvatarController } from "@orbis/avatar-engine";
import { normalizeLicenseProductKey } from "$lib/utils/license";

let {
  form,
  isSubmitting = false,
  isComplete = false,
  authenticationRequired = false,
  warnings = [],
  result = null,
  licenseError = null,
  licenseLoading = false,
  licenseIsOperational = false,
  onLaunch,
  onEnterWorkspace,
  onStartTrial,
  onActivateKey,
}: {
  form: OnboardingFormModel;
  isSubmitting?: boolean;
  isComplete?: boolean;
  authenticationRequired?: boolean;
  warnings?: string[];
  result?: BootstrapResult | null;
  licenseError?: string | null;
  licenseLoading?: boolean;
  licenseIsOperational?: boolean;
  onLaunch: () => void;
  onEnterWorkspace: () => void;
  onStartTrial: () => Promise<boolean>;
  onActivateKey: (productKey: string) => Promise<boolean>;
} = $props();

const avatarController = new AvatarController();
let productKey = $state("");
let productKeyError = $state<string | null>(null);
let setupStage = $state(0);
const blueprint = $derived(getBlueprint(form.institutionType));
const tz = $derived(TIMEZONE_OPTIONS.find((t) => t.value === form.timezone));
const cur = $derived(
  CURRENCY_OPTIONS.find((c) => c.code === form.baseCurrencyCode),
);
const country = $derived(
  COUNTRIES_LIST.find((c) => c.code === form.countryCode),
);
const track = $derived(
  EDUCATIONAL_TRACK_OPTIONS.find((t) => t.value === form.educationalTrack),
);
const size = $derived(
  ORGANIZATION_SIZE_OPTIONS.find((s) => s.value === form.organizationSize),
);
const isArabic = $derived(getLocale().startsWith("ar"));
const organizationName = $derived(
  (isArabic
    ? form.organizationNameAr || result?.organization_name
    : result?.organization_name || form.organizationName) ||
    result?.organization_name ||
    form.organizationNameAr ||
    form.organizationName,
);
const administratorName = $derived(
  (isArabic
    ? form.adminFirstNameAr || form.adminFirstName
    : form.adminFirstName || form.adminFirstNameAr) ||
    m.ui_onboarding_default_admin_name(),
);
const institutionTypeLabel = $derived(
  {
    k12_school: () => m.ui_onboarding_institution_type_school(),
    university: () => m.ui_onboarding_institution_type_university(),
    vocational_institute: () =>
      m.ui_onboarding_institution_type_vocational_institute(),
    language_center: () => m.ui_onboarding_institution_type_language_center(),
  }[form.institutionType](),
);

$effect(() => {
  if (isComplete) avatarController.happy();
});

onDestroy(() => avatarController.destroy());

$effect(() => {
  if (!isSubmitting) return;

  setupStage = 0;
  const stageTimeouts = [
    window.setTimeout(() => (setupStage = 1), 1600),
    window.setTimeout(() => (setupStage = 2), 4200),
  ];

  return () => stageTimeouts.forEach((timeout) => window.clearTimeout(timeout));
});

async function submitProductKey() {
  const key = productKey.trim();
  if (!key || licenseLoading) return;
  productKeyError = null;
  const activationKey = normalizeLicenseProductKey(key);
  if (!activationKey) {
    productKeyError = m.license_invalid_key_format();
    return;
  }
  const succeeded = await onActivateKey(activationKey);
  if (succeeded) productKey = "";
}
</script>

{#if isSubmitting}
  <section class="flex min-h-[64vh] items-center justify-center px-4 text-center" role="status">
    <div class="min-h-20">
      {#key setupStage}
        <p class="setup-message max-w-5xl text-3xl font-medium leading-relaxed tracking-tight text-foreground sm:text-5xl lg:text-6xl" aria-live="polite">
          {#if setupStage === 0}
            {m.ui_onboarding_setup_getting_everything_ready()}
          {:else if setupStage === 1}
            {m.ui_onboarding_setup_preparing_institution_workspace()}
          {:else}
            {m.ui_onboarding_setup_almost_done({ name: administratorName })}
          {/if}
        </p>
      {/key}
    </div>
  </section>
{:else if isComplete}
  <section class="mx-auto flex min-h-[64vh] max-w-4xl flex-col items-center justify-center px-4 py-10 text-center">
    <Avatar
      controller={avatarController}
      hat="graduation"
      width="160px"
      height="160px"
      label="Orbis"
      class="mb-4"
    />
    <h1 class="max-w-3xl text-3xl font-semibold leading-tight tracking-tight text-foreground sm:text-5xl" dir={isArabic ? "rtl" : "ltr"}>
      {m.ui_onboarding_setup_welcome({
        name: organizationName,
        administrator: administratorName,
        institutionType: institutionTypeLabel,
      })}
    </h1>

    {#if warnings.length > 0}
      <div class="mt-7 w-full max-w-xl space-y-2 rounded-xl border border-amber-500/30 bg-amber-500/10 p-4 text-start text-sm text-amber-800 dark:text-amber-200" role="status">
        <p class="font-semibold">{m.ui_onboarding_review_initialization_warnings()}</p>
        <ul class="list-inside list-disc space-y-1">
          {#each warnings as warning (warning)}
            <li>{warning}</li>
          {/each}
        </ul>
      </div>
    {/if}

    <Button
      type="button"
      size="lg"
      onclick={onEnterWorkspace}
      class="mt-8 h-12 px-9 text-sm font-semibold"
    >
      {authenticationRequired
        ? m.ui_onboarding_review_continue_to_sign_in()
        : m.ui_onboarding_review_enter_workspace()}
    </Button>

    {#if !authenticationRequired}
      <div class="mt-9 w-full max-w-xl space-y-4 border-t border-border/60 pt-6 text-start">
        <div class="space-y-1">
          <h2 class="text-sm font-semibold text-foreground">
            {m.license_activation_title()}
          </h2>
          <p class="text-xs leading-relaxed text-muted-foreground">
            {m.license_activation_desc()}
          </p>
        </div>

        {#if licenseIsOperational}
          <p class="text-sm text-emerald-600 dark:text-emerald-400" role="status">
            {m.license_activation_success()}
          </p>
        {/if}
        <div class="flex flex-col gap-2">
          <Input
            bind:value={productKey}
            aria-label={m.license_key_label()}
            placeholder={m.license_key_placeholder()}
            disabled={licenseLoading}
            autocomplete="off"
          />
          <Button
            type="button"
            disabled={licenseLoading || !productKey.trim()}
            onclick={() => void submitProductKey()}
          >
            {licenseLoading ? m.license_activating() : m.license_activate_btn()}
          </Button>
          {#if productKeyError}
            <p class="text-sm text-destructive" role="alert">{productKeyError}</p>
          {/if}
          {#if licenseError}
            <p class="text-sm text-destructive" role="alert">{licenseError}</p>
          {/if}
        </div>
        {#if !licenseIsOperational}
          <Button
            type="button"
            variant="outline"
            disabled={licenseLoading}
            onclick={onStartTrial}
          >
            {m.license_start_trial_btn()}
          </Button>
        {/if}
      </div>
    {/if}
  </section>
{:else}
  <div class="mx-auto max-w-5xl space-y-6 py-2">
    <div class="space-y-2 text-start">
      <p class="max-w-2xl text-sm leading-relaxed text-muted-foreground">
        {m.ui_onboarding_review_review_your_settings_below_before_initiating_autonomous_bootstrap()}
      </p>
    </div>

    <!-- Summary Dossier Matrix -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-5 text-start">
      <!-- Blueprint & Organization -->
      <Card>
        <CardContent class="space-y-4 p-6">
        <div class="border-b border-border/40 pb-2.5">
          <h3 class="text-xs font-bold text-muted-foreground uppercase tracking-wider font-mono">
            {m.ui_onboarding_review_blueprint_institution()}
          </h3>
        </div>

        <div class="space-y-3 text-xs">
          <div class="flex justify-between items-center">
            <span class="text-muted-foreground">{m.ui_onboarding_review_blueprint()}</span>
            <span class="font-semibold text-foreground">{isArabic ? blueprint.nameAr : blueprint.nameEn}</span>
          </div>

          <div class="flex justify-between items-center">
            <span class="text-muted-foreground">{m.common_rooms_label_name_ar()}</span>
            <span class="font-semibold text-foreground">{form.organizationNameAr || "—"}</span>
          </div>

          <div class="flex justify-between items-center">
            <span class="text-muted-foreground">{m.common_rooms_label_name_en()}</span>
            <span class="font-semibold text-foreground">{form.organizationName || "—"}</span>
          </div>

          <div class="flex justify-between items-center">
            <span class="text-muted-foreground">{m.ui_onboarding_review_track()}</span>
            <span class="font-semibold text-foreground">{track?.label()}</span>
          </div>

          <div class="flex justify-between items-center gap-4">
            <span class="shrink-0 text-muted-foreground">{m.ui_onboarding_organization_expected_student_capacity()}</span>
            <span class="text-end font-semibold text-foreground">{size?.label() ?? form.organizationSize}</span>
          </div>
        </div>
        </CardContent>
      </Card>

      <!-- Regional & Administrator -->
      <Card>
        <CardContent class="space-y-4 p-6">
        <div class="border-b border-border/40 pb-2.5">
          <h3 class="text-xs font-bold text-muted-foreground uppercase tracking-wider font-mono">
            {m.ui_onboarding_review_region_master_admin()}
          </h3>
        </div>

        <div class="space-y-3 text-xs">
          <div class="flex justify-between items-center">
            <span class="text-muted-foreground">{m.ui_onboarding_review_country_region()}</span>
            <span class="font-semibold text-foreground">{country?.flag} {country?.name()}</span>
          </div>

          <div class="flex justify-between items-center gap-4">
            <span class="shrink-0 text-muted-foreground">{m.ui_onboarding_organization_timezone()}</span>
            <span class="text-end font-semibold text-foreground">{tz?.label() ?? form.timezone}</span>
          </div>

          <div class="flex justify-between items-center">
            <span class="text-muted-foreground">{m.setup_bootstrap_currency_label()}</span>
            <span class="font-semibold text-foreground font-mono">{cur?.code} ({cur?.symbol()})</span>
          </div>

          <div class="flex justify-between items-center">
            <span class="text-muted-foreground">{m.ui_onboarding_admin_administrator()}</span>
            <span class="font-semibold text-foreground">{form.adminFirstName} {form.adminLastName}</span>
          </div>

          <div class="flex justify-between items-center">
            <span class="text-muted-foreground">{m.common_word_email()}</span>
            <span class="font-semibold text-foreground font-mono truncate max-w-[200px]">{form.adminEmail}</span>
          </div>
        </div>
        </CardContent>
      </Card>
    </div>

    <div class="pt-4 flex justify-end">
      <Button
        type="button"
        size="lg"
        onclick={onLaunch}
        disabled={isSubmitting}
        class="h-11 px-8 text-sm font-semibold shadow-md shadow-primary/20"
      >
        <span>{m.ui_onboarding_review_launch_bootstrap()}</span>
      </Button>
    </div>
  </div>
{/if}

<style>
  .setup-message {
    animation: setup-message-in 700ms cubic-bezier(0.22, 1, 0.36, 1) both;
  }

  @keyframes setup-message-in {
    from {
      opacity: 0;
      transform: translateY(8px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .setup-message {
      animation: none;
    }
  }
</style>
