<!--
  Step 0: Welcome
  Shows org name (from CLI init) and a summary of what was configured.
  Guides the user into the academic setup wizard.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Avatar } from "@orbis/avatar-engine";
import { Badge } from "@orbis/ui/badge";
import { CheckCircle2, Building2, Shield, Database } from "@lucide/svelte";

import { onboardingState } from "../state.svelte";
import { getLocale } from "$lib/paraglide/runtime";

let orgName = $derived(
  getLocale() === "ar"
    ? onboardingState.profile.orgNameAr || onboardingState.profile.orgName
    : onboardingState.profile.orgName || onboardingState.profile.orgNameAr,
);

// Show what CLI already configured
let hasOrgData = $derived(onboardingState.profile.orgName !== "" || onboardingState.profile.orgNameAr !== "");
</script>

<div class="min-h-[55vh] flex flex-col items-center justify-center px-6 select-none w-full max-w-3xl mx-auto space-y-12 animate-in fade-in duration-1000">

  <!-- School Name Title -->
  <div class="text-center">
    <h1 class="text-3xl sm:text-5xl font-light tracking-tight text-foreground font-sans leading-tight">
      {m.common_onboarding_welcome_title({ orgName: orgName || "Orbis" })}
    </h1>
  </div>

  <!-- Avatar & Description -->
  <div class="w-full max-w-2xl flex flex-col sm:flex-row items-center gap-6 sm:gap-8 justify-center">
    
    <!-- Avatar Section -->
    <div class="shrink-0 flex items-center justify-center">
      <Avatar hat="graduation" width="96px" height="96px" />
    </div>

    <!-- Description Text -->
    <div class="text-center sm:text-start self-center max-w-lg space-y-4">
      <p class="text-base sm:text-lg text-foreground/90 leading-relaxed font-sans font-normal">
        {m.common_onboarding_welcome_message()}
      </p>

      <!-- CLI Setup Summary (show what's already configured) -->
      {#if hasOrgData}
        <div class="flex flex-wrap gap-2 justify-center sm:justify-start">
          <Badge variant="secondary" class="flex items-center gap-1.5 text-[10px] py-1 px-2.5">
            <Building2 size={11} />
            <span>{orgName}</span>
          </Badge>
          {#if onboardingState.profile.currency}
            <Badge variant="secondary" class="flex items-center gap-1.5 text-[10px] py-1 px-2.5">
              <Shield size={11} />
              <span>{onboardingState.profile.currency}</span>
            </Badge>
          {/if}
          {#if onboardingState.profile.timezone}
            <Badge variant="secondary" class="flex items-center gap-1.5 text-[10px] py-1 px-2.5">
              <Database size={11} />
              <span>{onboardingState.profile.timezone}</span>
            </Badge>
          {/if}
          <Badge variant="outline" class="flex items-center gap-1.5 text-[10px] py-1 px-2.5 text-emerald-500 border-emerald-500/30">
            <CheckCircle2 size={11} />
            <span>جاهز للاستخدام</span>
          </Badge>
        </div>
      {/if}
    </div>

  </div>
</div>
