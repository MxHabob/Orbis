<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { systemStore } from "$lib/stores";
  import AcademicPolicyManager from "$lib/features/settings/academic-policy-manager.svelte";
  import OnboardingStepHeader from "../onboarding-step-header.svelte";
  import { getTasksForStep } from "../onboarding-steps";
  import { refreshOnboardingState } from "../refresh-onboarding-state";

  let { orgId }: { orgId: string } = $props();
  const stepTasks = $derived(
    getTasksForStep("assessment", systemStore.onboardingTasks),
  );
  const taskStatus = $derived(stepTasks[0]?.status ?? "not_started");

  async function refreshTasks() {
    await refreshOnboardingState(orgId, m.common_action_failed());
  }
</script>

<div class="space-y-6">
  <OnboardingStepHeader
    stepNumber={7}
    title={m.onboarding_step6_title()}
    description={m.onboarding_step6_subtitle()}
    isRequired={true}
    status={taskStatus}
  />

  <AcademicPolicyManager orgId={orgId} onSaved={refreshTasks} />
</div>
