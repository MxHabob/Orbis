<!--
  @component OnboardingFooter
  Floating minimal footer with Help button, Back button, optional Skip button,
  and Primary Continue/Launch button.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import SetupWizardFooter from "$lib/shared/components/setup-wizard-footer.svelte";

let {
  canGoBack = false,
  canGoNext = true,
  isSkippable = false,
  isSubmitting = false,
  onPrev,
  onNext,
  onSkip,
}: {
  canGoBack?: boolean;
  canGoNext?: boolean;
  isSkippable?: boolean;
  isSubmitting?: boolean;
  onPrev: () => void;
  onNext: () => void;
  onSkip?: () => void;
} = $props();
</script>

<SetupWizardFooter
  backLabel={m.ui_onboarding_onboarding_footer_back()}
  nextLabel={m.ui_onboarding_onboarding_footer_continue()}
  secondaryLabel={isSkippable ? m.ui_onboarding_onboarding_footer_skip() : undefined}
  {canGoBack}
  {canGoNext}
  {isSubmitting}
  onBack={onPrev}
  onNext={canGoNext ? onNext : undefined}
  onSecondary={isSkippable ? onSkip : undefined}
/>
