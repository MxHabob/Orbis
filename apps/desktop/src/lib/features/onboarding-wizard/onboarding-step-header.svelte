<script lang="ts">
import type { OnboardingStepStatus } from "./onboarding-steps";

let {
  actionButton = undefined,
}: {
  stepNumber?: number;
  totalSteps?: number;
  title?: string;
  description?: string;
  isRequired?: boolean;
  status?: OnboardingStepStatus;
  actionButton?: any;
} = $props();
</script>

{#if actionButton}
  <div class="mb-4 flex items-center justify-end">
    {@render actionButton()}
  </div>
{/if}
