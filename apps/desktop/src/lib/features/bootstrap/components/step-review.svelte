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
import { Button } from "@orbis/ui/button";
import { Check } from "@orbis/icons";

let {
  form,
  isSubmitting = false,
  isComplete = false,
  error = null,
  postBootstrapError = null,
  authenticationRequired = false,
  warnings = [],
  result = null,
  onLaunch,
  onEnterWorkspace,
  isRTL = false,
}: {
  form: OnboardingFormModel;
  isSubmitting?: boolean;
  isComplete?: boolean;
  error?: string | null;
  postBootstrapError?: string | null;
  authenticationRequired?: boolean;
  warnings?: string[];
  result?: BootstrapResult | null;
  onLaunch: () => void;
  onEnterWorkspace: () => void;
  isRTL?: boolean;
} = $props();

const blueprint = $derived(getBlueprint(form.institutionType));
const tz = $derived(TIMEZONE_OPTIONS.find((t) => t.value === form.timezone));
const cur = $derived(CURRENCY_OPTIONS.find((c) => c.code === form.baseCurrencyCode));
const country = $derived(COUNTRIES_LIST.find((c) => c.code === form.countryCode));
const track = $derived(EDUCATIONAL_TRACK_OPTIONS.find((t) => t.value === form.educationalTrack));
const size = $derived(ORGANIZATION_SIZE_OPTIONS.find((s) => s.value === form.organizationSize));

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
    <div class="rounded-2xl border border-border/70 bg-card shadow-lg text-center p-10 space-y-6">
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

      {#if postBootstrapError}
        <div class="mx-auto max-w-xl rounded-xl border border-destructive/30 bg-destructive/10 p-4 text-start text-sm text-destructive" role="alert">
          <p class="font-semibold">{m.ui_onboarding_review_post_bootstrap_action_failed()}</p>
          <p class="mt-1">{postBootstrapError}</p>
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
    </div>
  {:else if isSubmitting}
    <div class="flex min-h-64 flex-col items-center justify-center gap-4 rounded-2xl border border-border/70 bg-card p-8 text-center">
      <LoadingIndicator size={32} class="size-8 text-primary" />
      <p class="max-w-lg text-sm leading-relaxed text-muted-foreground" aria-live="polite">
        {m.ui_onboarding_review_please_wait_while_orbis_provisions_initial_records_and_configurations()}
      </p>
    </div>
  {:else}
    <!-- Summary Dossier Matrix -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-5 text-start">
      <!-- Blueprint & Organization -->
      <div class="rounded-2xl border border-border/70 bg-card p-6 space-y-4 shadow-xs">
        <div class="border-b border-border/40 pb-2.5">
          <h3 class="text-xs font-bold text-muted-foreground uppercase tracking-wider font-mono">
            {m.ui_onboarding_review_blueprint_institution()}
          </h3>
        </div>

        <div class="space-y-3 text-xs">
          <div class="flex justify-between items-center">
            <span class="text-muted-foreground">{m.ui_onboarding_review_blueprint()}</span>
            <span class="font-semibold text-foreground">{isRTL ? blueprint.nameAr : blueprint.nameEn}</span>
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
      </div>

      <!-- Regional & Administrator -->
      <div class="rounded-2xl border border-border/70 bg-card p-6 space-y-4 shadow-xs">
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
      </div>
    </div>

    {#if error}
      <div class="p-4 rounded-xl bg-destructive/10 border border-destructive/25 text-destructive text-xs space-y-1 text-start" role="alert">
        <div class="font-bold">{m.ui_onboarding_review_initialization_failed()}</div>
        <p>{error}</p>
      </div>
    {/if}

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
