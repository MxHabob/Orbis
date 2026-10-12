<!-- @component StepReview -->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
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
import { Check } from "@orbis/icons";

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
  onRedeemKey,
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
  onRedeemKey: (productKey: string) => Promise<boolean>;
} = $props();

let productKey = $state("");
const blueprint = $derived(getBlueprint(form.institutionType));
const tz = $derived(TIMEZONE_OPTIONS.find((t) => t.value === form.timezone));
const cur = $derived(CURRENCY_OPTIONS.find((c) => c.code === form.baseCurrencyCode));
const country = $derived(COUNTRIES_LIST.find((c) => c.code === form.countryCode));
const track = $derived(EDUCATIONAL_TRACK_OPTIONS.find((t) => t.value === form.educationalTrack));
const size = $derived(ORGANIZATION_SIZE_OPTIONS.find((s) => s.value === form.organizationSize));
const isArabic = $derived(getLocale().startsWith("ar"));

async function redeemProductKey(event: SubmitEvent) {
  event.preventDefault();
  const key = productKey.trim();
  if (!key || licenseLoading) return;
  if (await onRedeemKey(key)) productKey = "";
}

</script>

<div class="space-y-6 max-w-5xl mx-auto py-2">
  <div class="space-y-2 text-start">
    {#if isComplete}
      <h2 class="text-2xl font-bold tracking-tight text-foreground">
        {m.ui_onboarding_review_system_initialized_successfully()}
      </h2>
    {/if}
    <p class="text-sm text-muted-foreground max-w-2xl leading-relaxed">
      {isComplete
        ? (m.ui_onboarding_review_your_workspace_has_been_provisioned_and_is_ready_for_production())
        : isSubmitting
          ? (m.ui_onboarding_review_please_wait_while_orbis_provisions_initial_records_and_configurations())
          : (m.ui_onboarding_review_review_your_settings_below_before_initiating_autonomous_bootstrap())}
    </p>
  </div>

  {#if isComplete}
    <Card>
      <CardContent class="space-y-6 p-10 text-center">
      <div class="size-16 rounded-2xl bg-emerald-500/10 text-emerald-500 flex items-center justify-center mx-auto border border-emerald-500/20 shadow-sm">
        <Check size={32} strokeWidth={2.5} />
      </div>

      <div class="space-y-2 max-w-md mx-auto">
        <h3 class="text-xl font-bold text-foreground">
          {result?.organization_name || form.organizationNameAr || form.organizationName}
        </h3>
        {#if result?.organization_slug}
          <p class="font-mono text-xs text-muted-foreground">{result.organization_slug}</p>
        {/if}
        <p class="text-xs text-muted-foreground leading-relaxed">
          {m.ui_onboarding_review_success_description()}
        </p>
      </div>

      {#if warnings.length > 0}
        <div class="mx-auto max-w-xl space-y-2 rounded-xl border border-amber-500/30 bg-amber-500/10 p-4 text-start text-sm text-amber-800 dark:text-amber-200" role="status">
          <p class="font-semibold">{m.ui_onboarding_review_initialization_warnings()}</p>
          <ul class="list-inside list-disc space-y-1">
            {#each warnings as warning (warning)}
              <li>{warning}</li>
            {/each}
          </ul>
        </div>
      {/if}

      {#if !authenticationRequired}
        <div class="mx-auto max-w-xl space-y-4 rounded-xl border border-border bg-muted/20 p-5 text-start">
          <div class="space-y-1">
            <h4 class="text-sm font-semibold text-foreground">
              {m.license_activation_title()}
            </h4>
            <p class="text-xs leading-relaxed text-muted-foreground">
              {m.license_activation_desc()}
            </p>
          </div>

          {#if licenseIsOperational}
            <p class="text-sm text-emerald-600 dark:text-emerald-400" role="status">
              {m.license_activation_success()}
            </p>
          {:else}
            <form class="flex flex-col gap-2 sm:flex-row" onsubmit={redeemProductKey}>
              <Input
                bind:value={productKey}
                aria-label={m.license_key_label()}
                placeholder={m.license_key_placeholder()}
                disabled={licenseLoading}
                autocomplete="off"
              />
              <Button type="submit" disabled={licenseLoading || !productKey.trim()}>
                {m.license_activate_btn()}
              </Button>
            </form>
            <Button
              type="button"
              variant="outline"
              disabled={licenseLoading}
              onclick={onStartTrial}
            >
              {m.license_start_trial_btn()}
            </Button>
            {#if licenseError}
              <p class="text-sm text-destructive" role="alert">{licenseError}</p>
            {/if}
          {/if}
        </div>
      {/if}

      <div class="pt-4">
        <Button
          type="button"
          size="lg"
          onclick={onEnterWorkspace}
          class="h-11 px-8 text-sm font-semibold shadow-md shadow-primary/20"
        >
          <span>
            {authenticationRequired
              ? m.ui_onboarding_review_continue_to_sign_in()
              : m.ui_onboarding_review_enter_workspace()}
          </span>
        </Button>
      </div>
      </CardContent>
    </Card>
  {:else if isSubmitting}
    <Card>
      <CardContent class="flex min-h-64 flex-col items-center justify-center gap-4 p-8 text-center">
      <LoadingIndicator size={32} class="size-8 text-primary" />
      <p class="max-w-lg text-sm leading-relaxed text-muted-foreground" aria-live="polite">
        {m.ui_onboarding_review_please_wait_while_orbis_provisions_initial_records_and_configurations()}
      </p>
      </CardContent>
    </Card>
  {:else}
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
        {#if isSubmitting}
          <LoadingIndicator size={16} class="me-2 size-4" />
        {/if}
        <span>{m.ui_onboarding_review_launch_bootstrap()}</span>
      </Button>
    </div>
  {/if}
</div>
