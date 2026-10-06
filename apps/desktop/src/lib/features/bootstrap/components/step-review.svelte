<!--
  @component StepReview
  Step 5: Pre-Flight Verification & System Bootstrap Launch.
  Spacious overview dossier of all parameters + authentic 5-phase initialization execution pipeline.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  type OnboardingFormModel,
  CURRENCY_OPTIONS,
  TIMEZONE_OPTIONS,
  COUNTRIES_LIST,
  EDUCATIONAL_TRACK_OPTIONS,
  ORGANIZATION_SIZE_OPTIONS,
} from "../types";
import { getBlueprint } from "@orbis/blueprints";
import { Button } from "@orbis/ui/button";
import { Check } from "@orbis/icons";

let {
  form,
  isSubmitting = false,
  isComplete = false,
  error = null,
  activeTaskIndex = 0,
  onLaunch,
  onEnterWorkspace,
  isRTL = false,
}: {
  form: OnboardingFormModel;
  isSubmitting?: boolean;
  isComplete?: boolean;
  error?: string | null;
  activeTaskIndex?: number;
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

const tasks = $derived([
  m.ui_onboarding_review_task_identity_records(),
  m.ui_onboarding_review_task_academic_structure(),
  m.ui_onboarding_review_task_admin_credentials(),
  m.ui_onboarding_review_task_financial_ledger(),
  m.ui_onboarding_review_task_node_readiness(),
]);
</script>

<div class="space-y-6 max-w-5xl mx-auto py-2">
  <div class="space-y-2 text-start">
    <div class="flex items-center gap-2">
      <span class="text-xs font-semibold text-primary uppercase font-mono tracking-wider">
        {m.ui_onboarding_review_step_6_of_6()}
      </span>
    </div>
    <h2 class="text-2xl font-bold tracking-tight text-foreground">
      {isComplete
        ? (m.ui_onboarding_review_system_initialized_successfully())
        : isSubmitting
          ? (m.ui_onboarding_review_initializing_autonomous_node())
          : (m.ui_onboarding_review_review_launch_setup())}
    </h2>
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
          {form.organizationNameAr || form.organizationName}
        </h3>
        <p class="text-xs text-muted-foreground leading-relaxed">
          {m.ui_onboarding_review_success_description()}
        </p>
      </div>

      <div class="pt-4">
        <Button
          type="button"
          size="lg"
          onclick={onEnterWorkspace}
          class="h-11 px-8 text-sm font-semibold shadow-md shadow-primary/20"
        >
          <span>{m.ui_onboarding_review_enter_workspace()}</span>
        </Button>
      </div>
    </div>
  {:else if isSubmitting}
    <!-- Real-Time Pipeline Progress Execution -->
    <div class="rounded-2xl border border-border/70 bg-card p-8 space-y-6 shadow-md text-start">
      <div class="flex items-center justify-between border-b border-border/40 pb-4">
        <div class="space-y-1">
          <div class="text-sm font-bold text-foreground">
            {m.ui_onboarding_review_execution_pipeline()}
          </div>
          <div class="text-xs text-muted-foreground font-mono">
            {m.ui_onboarding_review_phase_of({ value1: Math.min(activeTaskIndex + 1, tasks.length), value2: tasks.length })}
          </div>
        </div>

        <div class="size-6 rounded-full border-2 border-primary border-t-transparent animate-spin"></div>
      </div>

      <div class="space-y-3">
        {#each tasks as task, idx}
          {@const isDone = idx < activeTaskIndex}
          {@const isCurrent = idx === activeTaskIndex}

          <div class="flex items-center gap-3 p-3 rounded-xl transition-all {isCurrent ? 'bg-primary/5 border border-primary/25' : isDone ? 'bg-muted/15' : 'opacity-40'}">
            <div class="size-6 rounded-full flex items-center justify-center shrink-0 text-xs font-mono {isDone ? 'bg-emerald-500 text-white' : isCurrent ? 'bg-primary text-primary-foreground animate-pulse' : 'bg-muted text-muted-foreground'}">
              {#if isDone}
                <Check size={13} strokeWidth={3} />
              {:else}
                {idx + 1}
              {/if}
            </div>

            <div class="text-xs font-medium {isCurrent ? 'text-foreground font-semibold' : isDone ? 'text-foreground/80' : 'text-muted-foreground'}">
              {task}
            </div>
          </div>
        {/each}
      </div>
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
      <div class="p-4 rounded-xl bg-destructive/10 border border-destructive/25 text-destructive text-xs space-y-1 text-start">
        <div class="font-bold">{m.ui_onboarding_review_initialization_failed()}</div>
        <p>{error}</p>
      </div>
    {/if}

    <div class="pt-4 flex justify-end">
      <Button
        type="button"
        size="lg"
        onclick={onLaunch}
        class="h-11 px-8 text-sm font-semibold shadow-md shadow-primary/20"
      >
        <span>{m.ui_onboarding_review_launch_bootstrap()}</span>
      </Button>
    </div>
  {/if}
</div>
