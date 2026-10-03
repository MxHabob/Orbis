<!--
  @component OnboardingFooter
  Floating minimal footer with Help button, Back button, optional Skip button,
  and Primary Continue/Launch button.
-->
<script lang="ts">
import { Button } from "@orbis/ui/button";
import { QuestionMarkCircle } from "@orbis/icons";

let {
  canGoBack = false,
  isSkippable = false,
  isLastStep = false,
  isSubmitting = false,
  onPrev,
  onNext,
  onSkip,
  onHelp,
  isRTL = false,
}: {
  canGoBack?: boolean;
  isSkippable?: boolean;
  isLastStep?: boolean;
  isSubmitting?: boolean;
  onPrev: () => void;
  onNext: () => void;
  onSkip?: () => void;
  onHelp: () => void;
  isRTL?: boolean;
} = $props();
</script>

<footer class="sticky bottom-2 z-30 w-full max-w-xl mx-auto rounded-full bg-card/90 backdrop-blur-xl border border-border/80 shadow-md shadow-black/5 px-4 sm:px-6 py-2.5 flex items-center justify-between transition-all shrink-0 mt-6">
  <!-- Start Side: Help Trigger Button -->
  <div class="flex items-center">
    <Button
      type="button"
      variant="ghost"
      size="sm"
      onclick={onHelp}
      class="h-8 px-3 text-xs font-normal gap-1.5 rounded-full text-muted-foreground hover:text-foreground hover:bg-muted/50"
    >
      <QuestionMarkCircle size={14} />
      <span>{isRTL ? "المساعدة" : "Help"}</span>
    </Button>
  </div>

  <!-- End Side: Navigation Action Buttons -->
  <div class="flex items-center gap-2">
    {#if canGoBack && !isSubmitting}
      <Button
        type="button"
        variant="ghost"
        size="sm"
        onclick={onPrev}
        class="h-8 px-4 text-xs font-medium rounded-full text-muted-foreground hover:text-foreground hover:bg-muted/50"
      >
        <span>{isRTL ? "السابق" : "Back"}</span>
      </Button>
    {/if}

    {#if isSkippable && !isSubmitting}
      <Button
        type="button"
        variant="ghost"
        size="sm"
        onclick={onSkip}
        class="h-8 px-3.5 text-xs font-medium rounded-full text-muted-foreground hover:text-foreground"
      >
        <span>{isRTL ? "تخطي الآن" : "Skip"}</span>
      </Button>
    {/if}

    {#if !isLastStep}
      <Button
        type="button"
        variant="default"
        size="sm"
        onclick={onNext}
        disabled={isSubmitting}
        class="h-8 px-5 text-xs font-semibold rounded-full shadow-sm shadow-primary/20"
      >
        <span>{isRTL ? "المتابعة" : "Continue"}</span>
      </Button>
    {/if}
  </div>
</footer>
