<script lang="ts">
import { Avatar } from "@orbis/avatar-engine";
import * as m from "$lib/paraglide/messages";
import { onboardingState } from "../state.svelte";
import { getLocale } from "$lib/paraglide/runtime";

let orgName = $derived(
  getLocale() === "ar" ? onboardingState.orgNameAr : onboardingState.orgName
);
</script>

<div class="min-h-[55vh] flex flex-col items-center justify-center px-6 select-none w-full max-w-3xl mx-auto space-y-12 animate-in fade-in duration-1000">

  <!-- School Name Title (Google-style thin-to-normal weight typography) -->
  <div class="text-center">
    <h1 class="text-3xl sm:text-5xl font-light tracking-tight text-foreground font-sans leading-tight">
      {m.onboarding_welcome_title({ orgName })}
    </h1>
  </div>

  <!-- Avatar & Description (No container card, direct horizontal flex flow) -->
  <div class="w-full max-w-2xl flex flex-col sm:flex-row items-center gap-6 sm:gap-8 justify-center">
    
    <!-- Avatar Section (Clean and minimalist) -->
    <div class="shrink-0 flex items-center justify-center">
      <Avatar hat="graduation" width="96px" height="96px" />
    </div>

    <!-- Description Text (Clear, friendly, readable) -->
    <div class="text-center sm:text-start self-center max-w-lg">
      <p class="text-base sm:text-lg text-foreground/90 leading-relaxed font-sans font-normal">
        {m.onboarding_welcome_message()}
      </p>
    </div>

  </div>
</div>
