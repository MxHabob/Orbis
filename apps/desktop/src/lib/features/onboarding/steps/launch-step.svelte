<!--
  Step 8: Launch Setup
  Animated Avatar, final launch button, and progress indicators.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Progress } from "@orbis/ui/progress";
import { ArrowLeft, ArrowRight, Sparkles } from "@orbis/icons";
import { Avatar } from "@orbis/avatar-engine";
import { getLocale } from "$lib/paraglide/runtime";

import { onboardingState } from "../state.svelte";
import { authStore, systemStore } from "$lib/stores";
import { goto } from "$app/navigation";
import { localizeHref } from "$lib/paraglide/runtime";
import { l } from "$lib/core/i18n";

const isRTL = $derived(getLocale().startsWith("ar"));

let isLaunching = $state(false);
let launchProgress = $state(0);

let message = $derived.by(() => {
  const isAr = onboardingState.profile.locale.startsWith("ar");
  const bldCount = onboardingState.facilities.buildings.length;
  const roomCount = onboardingState.facilities.rooms.length;
  const capacity = onboardingState.facilities.totalCapacity;
  const stageCount = onboardingState.academics.enabledStages;
  const orgDisplayName = l(
    onboardingState.profile.orgNameAr,
    onboardingState.profile.orgName
  ) || l("المؤسسة", "your institution");

  if (isAr) {
    const bldText = bldCount === 1 ? "مبنى واحد" : `${bldCount} مباني`;
    const roomText =
      roomCount === 1 ? "قاعة دراسية واحدة" : `${roomCount} قاعة دراسية`;
    const stageText =
      stageCount === 1 ? "مرحلة دراسية واحدة" : `${stageCount} مراحل دراسية`;
    return `تم تجهيز ${bldText} بإجمالي ${roomText} وسعة استيعابية تبلغ ${capacity} طالباً، مع ربط وتفعيل ${stageText}. جاهزون الآن لإطلاق بيئة العمل الخاصة بـ ${orgDisplayName}.`;
  } else {
    const bldText = bldCount === 1 ? "1 building" : `${bldCount} buildings`;
    const roomText = roomCount === 1 ? "1 room" : `${roomCount} rooms`;
    const stageText =
      stageCount === 1 ? "1 academic stage" : `${stageCount} academic stages`;
    return `Configured ${bldText} with ${roomText} and total capacity of ${capacity} students, linking ${stageText}. Ready to initialize workspace for ${orgDisplayName}.`;
  }
});

async function handleCompleteSetup() {
  const orgId = authStore.orgId;
  if (!orgId) return;

  isLaunching = true;
  launchProgress = 0;

  // Start background complete setup IPC request
  const setupPromise = onboardingState.completeSetup(orgId);

  // Animate progress bar
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
      setTimeout(async () => {
        isLaunching = false;
        authStore.markSetupComplete();
        await systemStore.loadReadiness();
        goto(localizeHref(`/${orgId}`), {
          invalidateAll: true,
          replaceState: true,
        });
      }, 700);
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

<div class="flex flex-col items-center justify-center px-4 select-none w-full max-w-2xl mx-auto my-auto py-6 animate-in fade-in duration-500">

  <div class="w-full bg-card border border-border/70 rounded-3xl p-6 sm:p-8 shadow-2xs text-center flex flex-col items-center space-y-5">
    
    <!-- Avatar with subtle ring -->
    <div class="relative p-1 rounded-full bg-primary/5 ring-1 ring-primary/20">
      <Avatar hat="graduation" width="84px" height="84px" />
    </div>

    <!-- Title & Message -->
    <div class="space-y-2 max-w-md">
      <h1 class="text-2xl sm:text-3xl font-bold tracking-tight text-foreground font-sans">
        {m.common_onboarding_launch_title()}
      </h1>
      <p class="text-xs sm:text-sm text-muted-foreground leading-relaxed font-sans font-normal">
        {message}
      </p>
    </div>

    <!-- Action Area -->
    <div class="w-full max-w-xs pt-2">
      {#if !isLaunching}
        <Button
          onclick={handleCompleteSetup}
          size="default"
          class="w-full h-9 text-xs font-bold gap-2 cursor-pointer shadow-xs"
        >
          <Sparkles class="w-3.5 h-3.5" />
          <span>{m.common_onboarding_review_launch_btn()}</span>
          {#if isRTL}
            <ArrowLeft class="w-3.5 h-3.5" />
          {:else}
            <ArrowRight class="w-3.5 h-3.5" />
          {/if}
        </Button>
      {:else}
        <div class="space-y-2">
          <Progress value={launchProgress} class="h-1.5 w-full bg-muted" />
          <div class="flex justify-between text-[10px] font-bold font-mono text-muted-foreground">
            <span>{m.common_onboarding_launch_loading()}</span>
            <span>{launchProgress}%</span>
          </div>
        </div>
      {/if}

      {#if onboardingState.errorMessage}
        <div class="mt-3 p-2.5 rounded-xl border border-destructive/20 bg-destructive/10 text-destructive text-xs text-start font-sans leading-relaxed animate-in fade-in">
          <span class="font-bold">{m.common_onboarding_launch_error()}</span>
          <p class="font-mono mt-1 text-[10px] opacity-90">{onboardingState.errorMessage}</p>
        </div>
      {/if}
    </div>

  </div>

</div>

