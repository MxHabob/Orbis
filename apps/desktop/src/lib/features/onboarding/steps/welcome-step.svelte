<!--
  Step 0: Welcome
  Shows org name and summary of initialized settings.
  Guides the user into the academic setup wizard.
-->
<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { Building2, CheckCircle2, Database, Shield } from "@lucide/svelte";
  import { Avatar } from "@orbis/avatar-engine";
  import { Badge } from "@orbis/ui/badge";

  import { l } from "$lib/core/i18n";
  import { onboardingState } from "../state.svelte";

  let orgName = $derived(
    l(onboardingState.profile.orgNameAr, onboardingState.profile.orgName) ||
      "Orbis",
  );

  // Show if basic organization profile data is present
  let hasOrgData = $derived(
    Boolean(
      onboardingState.profile.orgName || onboardingState.profile.orgNameAr,
    ),
  );
</script>

<div
  class="flex flex-col items-center justify-center px-6 select-none w-full max-w-4xl mx-auto space-y-6 sm:space-y-8 animate-in fade-in duration-700 py-4 my-auto"
>
  <!-- School Name Title -->
  <div class="text-center">
    <h1
      class="text-3xl sm:text-4xl font-light tracking-tight text-foreground font-sans leading-tight"
    >
      {m.common_onboarding_welcome_title({ orgName })}
    </h1>
  </div>

  <!-- Avatar & Description -->
  <div
    class="w-full max-w-2xl flex flex-col sm:flex-row items-center gap-6 sm:gap-8 justify-center"
  >
    <!-- Avatar Section -->
    <div class="shrink-0 flex items-center justify-center">
      <Avatar hat="graduation" width="88px" height="88px" />
    </div>

    <!-- Description Text -->
    <div class="text-center sm:text-start self-center max-w-lg space-y-3.5">
      <p
        class="text-base text-foreground/90 leading-relaxed font-sans font-normal"
      >
        {m.common_onboarding_welcome_message()}
      </p>

      <!-- Initial Organization Settings Summary -->
      {#if hasOrgData}
        <div class="flex flex-wrap gap-2 justify-center sm:justify-start">
          <Badge
            variant="secondary"
            class="flex items-center gap-1.5 text-[10px] py-1 px-2.5"
          >
            <Building2 size={11} />
            <span>{orgName}</span>
          </Badge>
          {#if onboardingState.profile.currency}
            <Badge
              variant="secondary"
              class="flex items-center gap-1.5 text-[10px] py-1 px-2.5"
            >
              <Shield size={11} />
              <span>{onboardingState.profile.currency}</span>
            </Badge>
          {/if}
          {#if onboardingState.profile.timezone}
            <Badge
              variant="secondary"
              class="flex items-center gap-1.5 text-[10px] py-1 px-2.5"
            >
              <Database size={11} />
              <span>{onboardingState.profile.timezone}</span>
            </Badge>
          {/if}
          <Badge
            variant="outline"
            class="flex items-center gap-1.5 text-[10px] py-1 px-2.5 text-emerald-500 border-emerald-500/30"
          >
            <CheckCircle2 size={11} />
            <span
              >{l("جاهز للتهيئة الأكاديمية", "Ready for Academic Setup")}</span
            >
          </Badge>
        </div>
      {/if}
    </div>
  </div>
</div>
