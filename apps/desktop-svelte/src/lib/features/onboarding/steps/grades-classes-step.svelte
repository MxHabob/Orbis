<!--
  Step 4: Grade Levels & Classes
  Enabling grades, adding/removing sections (divisions).
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Label } from "@orbis/ui-svelte/label";
import { Checkbox } from "@orbis/ui-svelte/checkbox";
import { Badge } from "@orbis/ui-svelte/badge";
import { Input } from "@orbis/ui-svelte/input";
import { Button } from "@orbis/ui-svelte/button";
import { Plus } from "@lucide/svelte";

import { onboardingState } from "../state.svelte";
</script>

<div class="w-full space-y-6">

  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    {#each onboardingState.academics.gradeLevels as gl (gl.level)}
      <div
        class="bg-card border rounded-xl p-4 space-y-3 transition-colors
          {gl.enabled ? 'border-border/60' : 'border-border/30 opacity-40'}"
      >
        <!-- Grade Header -->
        <div class="flex justify-between items-start">
          <div class="flex gap-3 items-center text-start">
            <!-- Color Tag -->
            <div class="w-3.5 h-3.5 rounded-full" style="background-color: {gl.color};"></div>
            <div>
              <h4 class="font-bold text-foreground text-sm font-serif">{gl.name_ar}</h4>
              <p class="text-xs text-muted-foreground font-mono">{gl.name}</p>
            </div>
          </div>

          <div class="flex items-center gap-2">
            <Label for="enable-{gl.level}" class="text-xs text-muted-foreground cursor-pointer">
              {m.common_onboarding_stage_enable()}
            </Label>
            <Checkbox id="enable-{gl.level}" bind:checked={gl.enabled} />
          </div>
        </div>

        <!-- Divisions / Sections manager -->
        {#if gl.enabled}
          <div class="space-y-3 pt-3 border-t border-border/60">
            <Label class="text-xs text-muted-foreground flex justify-start">
              {m.common_onboarding_associated_divisions({ division: onboardingState.divisionLabel })}
            </Label>

            <div class="flex flex-wrap gap-1.5 min-h-8 p-2 bg-muted/20 rounded-lg border border-border/60">
              {#each gl.divisions as div, divisionIndex}
                <Badge variant="secondary" class="bg-card border-border/60 text-foreground text-xs px-2 py-0.5 flex gap-1 items-center font-sans">
                  <Input
                    value={div}
                    maxlength="1"
                    aria-label={`${onboardingState.divisionLabel} ${div}`}
                    class="h-6 w-8 border-0 bg-transparent p-0 text-center text-xs font-mono uppercase focus-visible:ring-0"
                    onchange={(event) => onboardingState.renameDivision(gl.level, divisionIndex, event.currentTarget.value)}
                  />
                  <button
                    onclick={() => onboardingState.removeDivision(gl.level, div)}
                    class="hover:text-destructive font-bold ml-0.5 mr-0.5 text-[10px] cursor-pointer"
                  >
                    &times;
                  </button>
                </Badge>
              {/each}

              {#if gl.divisions.length === 0}
                <span class="text-xs text-muted-foreground/80 self-center">
                  {m.common_onboarding_no_divisions({ division: onboardingState.divisionLabel })}
                </span>
              {/if}
            </div>

            <!-- Quick Add Division Form -->
            <div class="flex gap-2">
              <Input
                placeholder={m.common_onboarding_add_division_placeholder({ division: "A, B, C..." })}
                maxlength="1"
                pattern="[A-Za-z]"
                onkeydown={(event) => { if (event.key === "Enter") onboardingState.addDivision(gl.level); }}
                bind:value={onboardingState.academics.divisionInput[gl.level]}
                class="text-xs h-8"
              />
              <Button
                onclick={() => onboardingState.addDivision(gl.level)}
                size="xs"
                class="h-8 text-xs shrink-0 px-2.5"
              >
                <Plus class="w-3 h-3" />
              </Button>
            </div>
          </div>
        {/if}
      </div>
    {/each}
  </div>
</div>
