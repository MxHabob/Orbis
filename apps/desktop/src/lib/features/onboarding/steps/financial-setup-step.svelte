<!--
  Step 6: Tuition & Registration Fees (الرسوم الدراسية)
  Registration fees per grade level with compact 2-column layout.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Input } from "@orbis/ui/input";
import { DollarSign } from "@lucide/svelte";

import { onboardingState } from "../state.svelte";
import { l } from "$lib/core/i18n";

const enabledGradeLevels = $derived(
  onboardingState.academics.gradeLevels.filter((gl) => gl.enabled),
);
</script>

<div class="w-full space-y-3 text-start">
  <div class="flex items-center justify-between border-b border-border/50 pb-1.5">
    <div class="flex items-center gap-2">
      <DollarSign size={16} class="text-primary shrink-0" />
      <h3 class="text-xs font-bold text-foreground">
        {m.common_onboarding_fees_label({ currency: onboardingState.profile.currency })}
      </h3>
    </div>
    <span class="text-[11px] font-bold text-emerald-600 dark:text-emerald-400 bg-emerald-500/10 px-2 py-0.5 rounded-full font-mono">
      {onboardingState.profile.currency}
    </span>
  </div>

  <!-- Registration Fees Multi-Column Responsive Grid (Expanded for Widescreen) -->
  <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 xl:grid-cols-4 gap-3 max-h-[300px] overflow-y-auto scrollbar-hide pr-1">
    {#each enabledGradeLevels as gl (gl.level)}
      <div class="flex items-center justify-between p-2.5 rounded-xl border border-border/70 bg-card hover:bg-muted/15 transition-colors shadow-2xs">
        <div class="flex items-center gap-2 min-w-0">
          <div class="size-2.5 rounded-full shrink-0" style="background-color: {gl.color};"></div>
          <span class="font-bold text-foreground text-xs truncate">{l(gl.name_ar, gl.name)}</span>
        </div>

        <div class="relative w-36 shrink-0">
          <Input
            type="number"
            min="0"
            placeholder="0"
            bind:value={onboardingState.finance.registrationFees[gl.level]}
            onchange={() => onboardingState.save()}
            class="h-7.5 ps-10 text-end text-xs font-bold font-mono bg-background border-border/60 focus:border-primary"
          />
          <div class="absolute inset-y-0 inset-s-2.5 flex items-center pointer-events-none text-[9.5px] font-bold text-muted-foreground uppercase font-sans">
            {onboardingState.profile.currency}
          </div>
        </div>
      </div>
    {:else}
      <div class="col-span-2 py-8 text-center text-xs text-muted-foreground">
        {m.common_onboarding_fees_no_stages({ stage: onboardingState.stageLabel })}
      </div>
    {/each}
  </div>
</div>
