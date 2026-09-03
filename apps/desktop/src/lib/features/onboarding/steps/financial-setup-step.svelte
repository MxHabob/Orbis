<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Input } from "@orbis/ui/input";
import { FormField } from "@orbis/ui";

import { getLocale } from "$lib/paraglide/runtime";
import { onboardingState } from "../state.svelte";

const enabledGradeLevels = $derived(
  onboardingState.academics.gradeLevels.filter((gl) => gl.enabled),
);
</script>

<div class="w-full space-y-6">

  <!-- Registration Fees List (Clean, borderless list of rows) -->
  <div class="w-full overflow-x-auto scrollbar-hide border border-border/60 rounded-2xl bg-card shadow-sm">
    <div class="min-w-160 divide-y divide-border/60">
      <!-- Header Row -->
      <div class="grid grid-cols-[50px_2.5fr_1.5fr] gap-4 p-4 bg-muted/20 text-xs font-semibold text-muted-foreground select-none text-start items-center">
        <span>{m.common_td_text_color()}</span>
        <span>{onboardingState.stageLabel}</span>
        <span class="text-end">{m.common_onboarding_fees_label({ currency: onboardingState.profile.currency })}</span>
      </div>

      {#each enabledGradeLevels as gl (gl.level)}
        <div class="grid grid-cols-[50px_2.5fr_1.5fr] gap-4 px-4 py-3 items-center hover:bg-muted/10 transition-colors text-start">
          <!-- Color Dot -->
          <div class="flex justify-center items-center">
            <div class="w-3 h-3 rounded-full shrink-0" style="background-color: {gl.color};"></div>
          </div>

          <!-- Grade Level Name -->
          <div class="flex flex-col sm:flex-row sm:items-center gap-1.5 sm:gap-4">
            <span class="font-bold text-foreground text-sm font-sans">{gl.name_ar}</span>
            <span class="text-xs text-muted-foreground font-mono">{gl.name}</span>
          </div>

          <!-- Fee Input -->
          <div class="flex justify-end items-center">
            <FormField label={m.common_onboarding_fees_label({ currency: onboardingState.profile.currency })}>
              {#snippet children()}
            <div class="relative w-45">
              <Input
                type="number"
                min="0"
                placeholder="0"
                bind:value={onboardingState.finance.registrationFees[gl.level]}
                onchange={() => onboardingState.save()}
                class="h-8 ps-12 text-end text-sm font-bold font-mono bg-background/50 border-border/60 focus:border-primary"
              />
              <div class="absolute inset-y-0 inset-s-3 flex items-center pointer-events-none text-[10px] font-bold text-muted-foreground uppercase font-sans">
                {onboardingState.profile.currency}
              </div>
            </div>
              {/snippet}
            </FormField>
          </div>
        </div>
      {/each}

      {#if enabledGradeLevels.length === 0}
        <div class="text-center py-12 text-muted-foreground text-sm font-sans">
          {m.common_onboarding_fees_no_stages({ stage: onboardingState.stageLabel })}
        </div>
      {/if}
    </div>
  </div>
</div>
