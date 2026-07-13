<script lang="ts">
import { Avatar } from "@orbis/avatar-engine";
import * as m from "$lib/paraglide/messages";
import { onboardingState } from "../state.svelte";
import { getLocale } from "$lib/paraglide/runtime";

let orgName = $derived(
  getLocale() === "ar" ? onboardingState.orgNameAr : onboardingState.orgName
);
</script>

<div class="flex-1 flex flex-col items-center justify-center px-6 pb-8 select-none max-w-2xl mx-auto space-y-10 animate-in fade-in slide-in-from-bottom-4 duration-500">
  <!-- Avatar Section -->
  <div class="flex justify-center w-full">
    <Avatar hat="graduation" width="112px" height="112px" />
  </div>

  <!-- Content Section -->
  <div class="space-y-4 text-center">
    <h1 class="text-2xl sm:text-3xl font-bold tracking-tight text-foreground font-serif">
      {m.onboarding_welcome_title({ orgName })}
    </h1>
    
    <p class="text-sm sm:text-[15px] text-muted-foreground leading-relaxed max-w-lg mx-auto">
      {m.onboarding_welcome_message()}
    </p>
  </div>
</div>
