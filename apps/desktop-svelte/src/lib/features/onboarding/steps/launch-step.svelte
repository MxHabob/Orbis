<!--
  Step 9: Launch Setup
  Animated Avatar, final launch button, and progress indicators.
-->
<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import { Progress } from "@orbis/ui-svelte/progress";
import { Sparkles } from "lucide-svelte";
import { Avatar } from "@orbis/avatar-engine";
import * as common from "$lib/paraglide/messages/common";
import { onboardingState } from "../state.svelte";
import { authStore } from "$lib/stores";
import { goto } from "$app/navigation";
import { localizeHref } from "$lib/paraglide/runtime";

let isLaunching = $state(false);
let launchProgress = $state(0);

let message = $derived.by(() => {
  const isAr = onboardingState.profile.locale.startsWith("ar");
  const bldCount = onboardingState.facilities.buildings.length;
  const roomCount = onboardingState.facilities.rooms.length;
  const capacity = onboardingState.facilities.totalCapacity;
  const stageCount = onboardingState.academics.enabledStages;

  if (isAr) {
    const bldText = bldCount === 1 ? "مبنى واحد" : `${bldCount} مباني`;
    const roomText = roomCount === 1 ? "قاعة دراسية واحدة" : `${roomCount} قاعة دراسية`;
    const stageText = stageCount === 1 ? "مرحلة دراسية واحدة" : `${stageCount} مراحل دراسية`;
    return `لقد قمت بإعداد ${bldText} تحتوي على ${roomText} بسعة استيعابية إجمالية تبلغ ${capacity} طالباً. سأبدأ الآن بتنظيم هذه البنية، وربط المراحل الـ ${stageText} التي قمت بتفعيلها لبناء بيئة العمل الخاصة بمدرستك.`;
  } else {
    const bldText = bldCount === 1 ? "1 building" : `${bldCount} buildings`;
    const roomText = roomCount === 1 ? "1 room" : `${roomCount} rooms`;
    const stageText = stageCount === 1 ? "1 academic stage" : `${stageCount} academic stages`;
    return `You have configured ${bldText} containing ${roomText} with a total capacity of ${capacity} students. I will now link this infrastructure with the ${stageText} you activated to initialize your school's workspace.`;
  }
});

async function handleCompleteSetup() {
  const orgId = authStore.orgId;
  if (!orgId) return;

  isLaunching = true;
  launchProgress = 0;

  // Start background complete setup IPC request
  const setupPromise = onboardingState.completeSetup(orgId);

  // Animate progress bar (takes about 3 seconds)
  const interval = setInterval(() => {
    if (launchProgress < 95) {
      launchProgress += Math.floor(Math.random() * 8) + 3;
    }
  }, 150);

  try {
    const success = await setupPromise;
    if (success) {
      clearInterval(interval);
      launchProgress = 100;
      setTimeout(() => {
        isLaunching = false;
        authStore.markSetupComplete();
        goto(localizeHref(`/${orgId}`));
      }, 800); // slight delay for smooth visual transition
    } else {
      clearInterval(interval);
      isLaunching = false;
    }
  } catch (err) {
    clearInterval(interval);
    isLaunching = false;
  }
}
</script>

<div class="min-h-[55vh] flex flex-col items-center justify-center px-6 select-none w-full max-w-3xl mx-auto space-y-12 animate-in fade-in duration-700">

  <!-- Header Title -->
  <div class="text-center">
    <h1 class="text-3xl sm:text-5xl font-light tracking-tight text-foreground font-sans leading-tight">
      {onboardingState.profile.locale.startsWith("ar") ? "تهيئة بنية المدرسة" : "Prepare School Infrastructure"}
    </h1>
  </div>

  <!-- Avatar & Message Details -->
  <div class="w-full max-w-2xl flex flex-col sm:flex-row items-center gap-6 sm:gap-8 justify-center">
    
    <!-- Avatar Section -->
    <div class="shrink-0 flex items-center justify-center">
      <Avatar hat="graduation" width="96px" height="96px" />
    </div>

    <!-- Description text & buttons/progress -->
    <div class="text-center sm:text-start self-center max-w-lg w-full space-y-6">
      <p class="text-base sm:text-lg text-foreground/90 leading-relaxed font-sans font-normal">
        {message}
      </p>

      <div class="w-full max-w-sm mx-auto sm:mx-0">
        {#if !isLaunching}
          <Button
            onclick={handleCompleteSetup}
            size="lg"
            class="px-8 py-6 text-sm font-bold bg-primary text-primary-foreground hover:bg-primary/90 rounded-xl shadow-md flex gap-2 items-center justify-center transition-all duration-200 hover:scale-[1.01] active:scale-[0.99] w-full sm:w-auto"
          >
            <Sparkles class="w-4 h-4 animate-pulse" />
            <span>{common.onboarding_review_launch_btn()}</span>
          </Button>
        {:else}
          <div class="space-y-2.5">
            <Progress value={launchProgress} class="h-1.5 w-full bg-muted" />
            <div class="flex justify-between text-[10px] font-bold font-mono text-muted-foreground uppercase tracking-wider">
              <span>{onboardingState.profile.locale.startsWith("ar") ? "جاري تهيئة النظام..." : "Initializing..."}</span>
              <span>{launchProgress}%</span>
            </div>
          </div>
        {/if}

        {#if onboardingState.errorMessage}
          <div class="mt-4 p-3 rounded-xl border border-destructive/20 bg-destructive/5 text-destructive text-xs text-start font-sans leading-relaxed animate-in fade-in duration-200">
            <span class="font-bold">{onboardingState.profile.locale.startsWith("ar") ? "حدث خطأ أثناء التأسيس:" : "Setup Error:"}</span>
            <p class="font-mono mt-1 text-[10px] opacity-90 break-words">{onboardingState.errorMessage}</p>
          </div>
        {/if}
      </div>
    </div>

  </div>
</div>
