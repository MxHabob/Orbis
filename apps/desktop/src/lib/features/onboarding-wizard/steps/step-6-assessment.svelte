<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { systemStore } from "$lib/stores";
  import AcademicPolicyManager from "$lib/features/settings/academic-policy-manager.svelte";
  import AssessmentSchemesManager from "$lib/features/settings/assessment-schemes-manager.svelte";
  import OnboardingStepHeader from "../onboarding-step-header.svelte";
  import { getTasksForStep } from "../onboarding-steps";
  import { refreshOnboardingState } from "../refresh-onboarding-state";
  import { Badge } from "@orbis/ui/badge";
  import { Button } from "@orbis/ui/button";

  let { orgId }: { orgId: string } = $props();
  const stepTasks = $derived(
    getTasksForStep("assessment", systemStore.onboardingTasks),
  );
  const taskStatus = $derived(stepTasks[0]?.status ?? "not_started");

  let activeTab = $state<"calculation" | "schemes">("calculation");

  async function refreshTasks() {
    await refreshOnboardingState(orgId, m.common_action_failed());
  }

  const YEMEN_STATUTORY_TIERS = [
    {
      title: () => m.onboarding_step6_tier_monthly_title(),
      score: () => m.onboarding_step6_tier_score_100(),
      details: () => m.onboarding_step6_tier_monthly_details(),
      badge: () => m.onboarding_step6_tier_monthly_badge(),
      color: "border-blue-500/30 bg-blue-500/5",
    },
    {
      title: () => m.onboarding_step6_tier_term_title(),
      score: () => m.onboarding_step6_tier_score_50(),
      details: () => m.onboarding_step6_tier_term_details(),
      badge: () => m.onboarding_step6_tier_term_badge(),
      color: "border-primary/30 bg-primary/5",
    },
    {
      title: () => m.onboarding_step6_tier_annual_title(),
      score: () => m.onboarding_step6_tier_score_100(),
      details: () => m.onboarding_step6_tier_annual_details(),
      badge: () => m.onboarding_step6_tier_annual_badge(),
      color: "border-emerald-500/30 bg-emerald-500/5",
    },
    {
      title: () => m.onboarding_step6_tier_certificate_title(),
      score: () => m.onboarding_step6_tier_score_80_20(),
      details: () => m.onboarding_step6_tier_certificate_details(),
      badge: () => m.onboarding_step6_tier_certificate_badge(),
      color: "border-amber-500/30 bg-amber-500/5",
    },
  ];
</script>

<div class="space-y-6">
  <OnboardingStepHeader
    stepNumber={7}
    title={m.onboarding_step6_title()}
    description={m.onboarding_step6_subtitle()}
    isRequired={true}
    status={taskStatus}
  />

  <!-- Official Yemeni Assessment Architecture Overview -->
  <div class="rounded-xl border border-primary/25 bg-card/60 p-4 shadow-xs space-y-4">
    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-2 border-b border-border/60 pb-3">
      <div class="flex items-center gap-2">
        <span class="text-xl">🇾🇪</span>
        <div>
          <h3 class="text-sm font-semibold text-foreground">
            {m.onboarding_step6_yemen_assessment_title()}
          </h3>
          <p class="text-xs text-muted-foreground mt-0.5">
            {m.onboarding_step6_yemen_assessment_description()}
          </p>
        </div>
      </div>
      <Badge variant="outline" class="self-start sm:self-auto text-[11px] px-2.5 py-0.5 bg-background font-mono">
        MoE Sana'a Verified
      </Badge>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3">
      {#each YEMEN_STATUTORY_TIERS as tier}
        <div class="p-3 rounded-lg border {tier.color} flex flex-col justify-between space-y-2">
          <div>
            <div class="flex items-center justify-between gap-1 mb-1">
              <span class="text-xs font-bold text-foreground">{tier.title()}</span>
              <Badge variant="secondary" class="text-[10px] px-1 py-0 h-4">{tier.badge()}</Badge>
            </div>
            <p class="text-[11px] text-muted-foreground leading-relaxed">{tier.details()}</p>
          </div>
          <div class="pt-2 border-t border-border/30 flex items-center justify-between">
            <span class="text-[10px] text-muted-foreground">{m.onboarding_step6_maximum_weight()}</span>
            <span class="text-xs font-mono font-bold text-primary">{tier.score()}</span>
          </div>
        </div>
      {/each}
    </div>
  </div>

  <!-- Navigation Tabs: Calculation Policy vs Assessment Schemes -->
  <div class="flex items-center justify-between border-b border-border/70 pb-2">
    <div class="flex items-center gap-2">
      <Button
        type="button"
        variant={activeTab === "calculation" ? "default" : "outline"}
        size="sm"
        class="h-8 text-xs cursor-pointer"
        onclick={() => (activeTab = "calculation")}
      >
        {m.onboarding_step6_calculation_tab()}
      </Button>
      <Button
        type="button"
        variant={activeTab === "schemes" ? "default" : "outline"}
        size="sm"
        class="h-8 text-xs cursor-pointer"
        onclick={() => (activeTab = "schemes")}
      >
        {m.onboarding_step6_assessment_schemes_tab()}
      </Button>
    </div>

    <span class="text-[11px] text-muted-foreground hidden sm:inline-block">
      {activeTab === "calculation"
        ? m.onboarding_step6_calculation_tab_description()
        : m.onboarding_step6_assessment_schemes_tab_description()}
    </span>
  </div>

  {#if activeTab === "calculation"}
    <AcademicPolicyManager orgId={orgId} onSaved={refreshTasks} />
  {:else}
    <div class="rounded-xl border border-border/70 bg-card p-4">
      <AssessmentSchemesManager orgId={orgId} />
    </div>
  {/if}
</div>
