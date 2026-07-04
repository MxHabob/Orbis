<!--
  Step 4: Grade Levels & Classes
  Enabling grades, adding/removing sections (divisions).
-->
<script lang="ts">
import { Label } from "@orbis/ui-svelte/label";
import { Checkbox } from "@orbis/ui-svelte/checkbox";
import { Badge } from "@orbis/ui-svelte/badge";
import { Input } from "@orbis/ui-svelte/input";
import { Button } from "@orbis/ui-svelte/button";
import { Plus } from "lucide-svelte";
import * as m from "$lib/paraglide/messages";
import { onboardingState } from "../onboarding-state.svelte";
</script>

<div class="max-w-4xl space-y-6">
  <div class="space-y-2 text-start">
    <h3 class="text-foreground font-bold text-xl font-serif">
      {m.onboarding_configure_stage({ stage: onboardingState.stageLabel })}
    </h3>
    <p class="text-muted-foreground text-sm">
      {m.onboarding_configure_stage_desc({ stage: onboardingState.stageLabel, division: onboardingState.divisionLabel })}
    </p>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    {#each onboardingState.gradeLevels as gl (gl.level)}
      <div
        class="bg-card border rounded-xl p-5 space-y-4 transition-colors
          {gl.enabled ? 'border-border' : 'border-border/40 opacity-50'}"
      >
        <!-- Grade Header -->
        <div class="flex justify-between items-start">
          <div class="flex gap-3 items-center text-start">
            <!-- Color Tag -->
            <div class="w-4 h-4 rounded-full" style="background-color: {gl.color};"></div>
            <div>
              <h4 class="font-bold text-foreground text-sm font-serif">{gl.name_ar}</h4>
              <p class="text-xs text-muted-foreground font-mono">{gl.name}</p>
            </div>
          </div>

          <div class="flex items-center gap-2">
            <Label for="enable-{gl.level}" class="text-xs text-muted-foreground cursor-pointer">
              {m.onboarding_stage_enable()}
            </Label>
            <Checkbox id="enable-{gl.level}" bind:checked={gl.enabled} />
          </div>
        </div>

        <!-- Divisions / Sections manager -->
        {#if gl.enabled}
          <div class="space-y-3 pt-3 border-t border-border">
            <Label class="text-xs text-muted-foreground flex justify-start">
              {m.onboarding_associated_divisions({ division: onboardingState.divisionLabel })}
            </Label>

            <div class="flex flex-wrap gap-1.5 min-h-[32px] p-2 bg-muted/40 rounded-lg border border-border">
              {#each gl.divisions as div}
                <Badge variant="secondary" class="bg-card border-border text-foreground text-xs px-2.5 py-0.5 flex gap-1 items-center font-sans">
                  <span>
                    {onboardingState.divisionLabel} {div}
                  </span>
                  <button
                    onclick={() => onboardingState.removeDivision(gl.level, div)}
                    class="hover:text-destructive font-bold ml-0.5 mr-0.5 text-[10px]"
                  >
                    &times;
                  </button>
                </Badge>
              {/each}

              {#if gl.divisions.length === 0}
                <span class="text-xs text-muted-foreground/80 self-center">
                  {m.onboarding_no_divisions({ division: onboardingState.divisionLabel })}
                </span>
              {/if}
            </div>

            <!-- Quick Add Division Form -->
            <div class="flex gap-2">
              <Input
                placeholder={m.onboarding_add_division_placeholder({ division: onboardingState.divisionLabel })}
                bind:value={onboardingState.divisionInput[gl.level]}
                class="text-xs h-8"
              />
              <Button
                onclick={() => onboardingState.addDivision(gl.level)}
                size="sm"
                class="h-8 text-xs shrink-0 px-3"
              >
                <Plus class="w-3.5 h-3.5" />
              </Button>
            </div>
          </div>
        {/if}
      </div>
    {/each}
  </div>
</div>
