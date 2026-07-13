<script lang="ts">
import { Label } from "@orbis/ui-svelte/label";
import { Input } from "@orbis/ui-svelte/input";
import { Coins } from "lucide-svelte";
import * as m from "$lib/paraglide/messages";
import { onboardingState } from "../state.svelte";

const enabledGradeLevels = $derived(
  onboardingState.gradeLevels.filter((gl) => gl.enabled),
);
</script>

<div class="max-w-4xl space-y-6">


  <!-- Registration Fees Grid -->
  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    {#each enabledGradeLevels as gl (gl.level)}
      <div class="bg-card border border-border rounded-xl p-4 flex items-center justify-between gap-4">
        <div class="flex gap-3 items-center text-start flex-1 min-w-0">
          <!-- Color indicator -->
          <div
            class="w-10 h-10 rounded-lg flex items-center justify-center shrink-0"
            style="background-color: {gl.color}20; border: 1px solid {gl.color}40;"
          >
            <Coins class="w-5 h-5 animate-pulse" style="color: {gl.color};" />
          </div>
          <div class="truncate text-start">
            <h4 class="font-bold text-foreground text-sm font-serif">
              {gl.name_ar}
            </h4>
            <p class="text-xs text-muted-foreground font-mono mt-0.5">{gl.name}</p>
          </div>
        </div>

        <div class="w-[180px] space-y-1.5 text-start shrink-0">
          <Label class="text-[11px] font-semibold text-muted-foreground">
            {m.onboarding_fees_label({ currency: onboardingState.currency })}
          </Label>
          <div class="relative">
            <Input
              type="number"
              min="0"
              placeholder="0.00"
              bind:value={onboardingState.registrationFees[gl.level]}
              class="h-9 pr-12 text-sm text-end font-bold font-mono"
            />
            <div class="absolute inset-y-0 left-3 flex items-center pointer-events-none text-[10px] font-bold text-muted-foreground uppercase">
              {onboardingState.currency}
            </div>
          </div>
        </div>
      </div>
    {/each}
    {#if enabledGradeLevels.length === 0}
      <div class="col-span-2 text-center py-10 text-muted-foreground border border-dashed rounded-xl">
        {m.onboarding_fees_no_stages({ stage: onboardingState.stageLabel })}
      </div>
    {/if}
  </div>
</div>
