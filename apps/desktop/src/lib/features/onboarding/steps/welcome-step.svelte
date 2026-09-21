<!--
  Step 0: Welcome
  Clean, warm welcome screen with minimal metadata indicators.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Avatar } from "@orbis/avatar-engine";
import { l } from "$lib/core/i18n";
import { onboardingState } from "../state.svelte";

let orgName = $derived(
  l(onboardingState.profile.orgNameAr, onboardingState.profile.orgName) ||
    "Orbis",
);

let hasOrgData = $derived(
  Boolean(
    onboardingState.profile.orgName || onboardingState.profile.orgNameAr,
  ),
);
</script>

<div class="flex flex-col items-center justify-center px-4 select-none w-full max-w-2xl mx-auto my-auto py-6 animate-in fade-in duration-500">
  
  <div class="w-full bg-card border border-border/70 rounded-3xl p-6 sm:p-8 shadow-2xs text-center flex flex-col items-center space-y-5">
    
    <!-- Avatar with subtle glow ring -->
    <div class="relative p-1 rounded-full bg-primary/5 ring-1 ring-primary/20">
      <Avatar hat="graduation" width="84px" height="84px" />
    </div>

    <!-- Title & Greeting -->
    <div class="space-y-2 max-w-md">
      <h1 class="text-2xl sm:text-3xl font-bold tracking-tight text-foreground font-sans">
        {m.common_onboarding_welcome_title({ orgName })}
      </h1>
      <p class="text-xs sm:text-sm text-muted-foreground leading-relaxed font-sans font-normal">
        {m.common_onboarding_welcome_message()}
      </p>
    </div>

    <!-- Organization Metadata Strip (Minimalist & Quiet) -->
    {#if hasOrgData}
      <div class="pt-2 flex flex-wrap items-center justify-center gap-2 text-xs">
        <span class="px-3 py-1 rounded-full bg-muted/50 border border-border/50 text-foreground font-bold font-sans">
          {orgName}
        </span>
        {#if onboardingState.profile.currency}
          <span class="px-2.5 py-1 rounded-full bg-muted/40 border border-border/40 text-muted-foreground font-mono font-medium text-[11px]">
            {onboardingState.profile.currency}
          </span>
        {/if}
        {#if onboardingState.profile.timezone}
          <span class="px-2.5 py-1 rounded-full bg-muted/40 border border-border/40 text-muted-foreground font-mono text-[11px]">
            {onboardingState.profile.timezone}
          </span>
        {/if}
      </div>
    {/if}

  </div>

</div>

