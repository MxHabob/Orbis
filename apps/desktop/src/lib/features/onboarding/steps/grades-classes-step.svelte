<!--
  Step 4: Grade Levels & Classes (المراحل الدراسية والفصول)
  Enabling grades, adding/removing sections (divisions) with full height utilization.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Label } from "@orbis/ui/label";
import { Checkbox } from "@orbis/ui/checkbox";
import { Badge } from "@orbis/ui/badge";
import { Input } from "@orbis/ui/input";
import { Button } from "@orbis/ui/button";
import { Plus, GraduationCap } from "@lucide/svelte";

import { onboardingState } from "../state.svelte";
import { l } from "$lib/core/i18n";
</script>

<div class="w-full h-full flex flex-col min-h-0 space-y-2.5 text-start">
  <div class="flex items-center justify-between border-b border-border/50 pb-1.5 shrink-0">
    <div class="flex items-center gap-2">
      <GraduationCap size={16} class="text-primary shrink-0" />
      <h3 class="text-xs font-bold text-foreground">
        {onboardingState.stageLabel} & {onboardingState.divisionLabel}
      </h3>
    </div>
    <span class="text-[11px] font-bold text-primary bg-primary/10 px-2 py-0.5 rounded-full font-mono">
      {onboardingState.academics.enabledStages} {onboardingState.stagesLabelPlural} / {onboardingState.academics.totalDivisions} {onboardingState.divisionLabel}
    </span>
  </div>

  <!-- Responsive Multi-Column Grade Grid (Expands to fill available vertical space) -->
  <div class="flex-1 min-h-0 overflow-y-auto scrollbar-hide grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-3 p-1">
    {#each onboardingState.academics.gradeLevels as gl (gl.level)}
      <div
        class="bg-card border rounded-2xl p-3.5 flex flex-col justify-between gap-2.5 transition-all duration-150 shadow-2xs hover:shadow-xs
          {gl.enabled ? 'border-border/80 bg-card' : 'border-border/30 opacity-40 bg-muted/10'}"
      >
        <!-- Grade Header -->
        <div class="flex justify-between items-center gap-2">
          <div class="flex items-center gap-2 min-w-0">
            <div class="size-3 rounded-full shrink-0" style="background-color: {gl.color};"></div>
            <div class="min-w-0">
              <h4 class="font-bold text-foreground text-xs truncate">{l(gl.name_ar, gl.name)}</h4>
            </div>
          </div>

          <Checkbox id="enable-{gl.level}" bind:checked={gl.enabled} class="cursor-pointer" />
        </div>

        <!-- Divisions / Sections manager -->
        {#if gl.enabled}
          <div class="space-y-2 pt-2 border-t border-border/40">
            <!-- Division Badges -->
            <div class="flex flex-wrap gap-1 min-h-6 items-center">
              {#each gl.divisions as div, divisionIndex}
                <Badge variant="secondary" class="bg-muted/60 border border-border/50 text-foreground text-[10px] px-2 py-0.5 flex gap-1 items-center font-sans">
                  <span>{div}</span>
                  <button
                    type="button"
                    onclick={() => onboardingState.removeDivision(gl.level, div)}
                    class="hover:text-destructive font-bold text-[10px] cursor-pointer"
                  >
                    &times;
                  </button>
                </Badge>
              {/each}

              {#if gl.divisions.length === 0}
                <span class="text-[10px] text-muted-foreground/70">
                  {m.common_onboarding_no_divisions({ division: onboardingState.divisionLabel })}
                </span>
              {/if}
            </div>

            <!-- Quick Add Division Input -->
            <div class="flex gap-1.5">
              <Input
                placeholder="أ، ب، Boys..."
                onkeydown={(event) => { if (event.key === "Enter") onboardingState.addDivision(gl.level); }}
                bind:value={onboardingState.academics.divisionInput[gl.level]}
                class="text-[11px] h-7 px-2.5 bg-background font-sans"
              />
              <Button
                type="button"
                onclick={() => onboardingState.addDivision(gl.level)}
                size="icon-xs"
                class="h-7 w-7 shrink-0 rounded-lg cursor-pointer"
              >
                <Plus size={12} />
              </Button>
            </div>
          </div>
        {/if}
      </div>
    {/each}
  </div>
</div>
