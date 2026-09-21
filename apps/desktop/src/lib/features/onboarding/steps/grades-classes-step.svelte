<!--
  Step 4: Grade Levels & Classes (المراحل الدراسية والفصول)
  Enabling grades and managing sections/divisions with unified card design and whole-card click-to-toggle.
-->
<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { Checkbox } from "@orbis/ui/checkbox";
  import { Badge } from "@orbis/ui/badge";
  import { Input } from "@orbis/ui/input";
  import { Button } from "@orbis/ui/button";
  import { Plus } from "@lucide/svelte";

  import { onboardingState } from "../state.svelte";
  import { l } from "$lib/core/i18n";
</script>

<div class="w-full h-full flex flex-col min-h-0 space-y-3 text-start animate-in fade-in duration-300">
  <div class="flex items-center justify-between border-b border-border/50 pb-2 shrink-0">
    <h3 class="text-xs font-bold text-foreground">
      {onboardingState.stageLabel} & {onboardingState.divisionLabel}
    </h3>
    <span class="text-[11px] font-bold text-primary bg-primary/10 px-2.5 py-0.5 rounded-full font-mono">
      {onboardingState.academics.enabledStages} {onboardingState.stagesLabelPlural} / {onboardingState.academics.totalDivisions} {onboardingState.divisionLabel}
    </span>
  </div>

  <!-- Responsive Multi-Column Grade Grid -->
  <div class="flex-1 min-h-0 overflow-y-auto scrollbar-hide grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-3 p-1">
    {#each onboardingState.academics.gradeLevels as gl (gl.level)}
      <div
        role="button"
        tabindex="0"
        onclick={() => {
          gl.enabled = !gl.enabled;
          onboardingState.save();
        }}
        onkeydown={(e) => {
          if (e.key === "Enter" || e.key === " ") {
            gl.enabled = !gl.enabled;
            onboardingState.save();
          }
        }}
        class="bg-card border rounded-2xl p-3.5 flex flex-col justify-between gap-2.5 transition-all duration-150 shadow-2xs hover:shadow-xs cursor-pointer select-none
          {gl.enabled
            ? 'border-border/80 bg-card ring-1 ring-primary/20'
            : 'border-border/30 opacity-40 bg-muted/10 hover:opacity-70'}"
      >
        <!-- Grade Header -->
        <div class="flex justify-between items-center gap-2">
          <div class="flex items-center gap-2.5 min-w-0">
            <div class="size-3 rounded-full shrink-0" style="background-color: {gl.color};"></div>
            <h4 class="font-bold text-foreground text-xs truncate">{l(gl.name_ar, gl.name)}</h4>
          </div>

          <div class="pointer-events-none shrink-0 flex items-center justify-center">
            <Checkbox checked={gl.enabled} />
          </div>
        </div>

        <!-- Divisions / Sections manager -->
        {#if gl.enabled}
          <div
            class="space-y-2 pt-2.5 border-t border-border/40 animate-in fade-in duration-150"
            role="presentation"
            onclick={(e) => e.stopPropagation()}
            onkeydown={(e) => e.stopPropagation()}
          >
            <!-- Division Badges -->
            <div class="flex flex-wrap gap-1.5 min-h-6 items-center">
              {#each gl.divisions as div}
                <Badge variant="secondary" class="bg-muted/60 border border-border/50 text-foreground text-[10.5px] px-2 py-0.5 flex gap-1.5 items-center font-sans">
                  <span>{div}</span>
                  <button
                    type="button"
                    onclick={(e) => {
                      e.stopPropagation();
                      onboardingState.removeDivision(gl.level, div);
                    }}
                    class="text-muted-foreground hover:text-destructive font-bold text-xs cursor-pointer focus:outline-none"
                    aria-label="Remove division"
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
                onclick={(e) => e.stopPropagation()}
                onkeydown={(event) => {
                  event.stopPropagation();
                  if (event.key === "Enter") onboardingState.addDivision(gl.level);
                }}
                bind:value={onboardingState.academics.divisionInput[gl.level]}
                class="text-[11px] h-7 px-2.5 bg-background font-sans"
              />
              <Button
                type="button"
                onclick={(e) => {
                  e.stopPropagation();
                  onboardingState.addDivision(gl.level);
                }}
                size="icon-xs"
                class="h-7 w-7 shrink-0 rounded-lg cursor-pointer"
              >
                <Plus size={12} />
              </Button>
            </div>
          </div>
        {:else}
          <div class="flex items-center justify-between pt-2 border-t border-border/30 text-[10px] text-muted-foreground">
            <span class="font-mono text-[9.5px]">
              {gl.divisions.length} {onboardingState.divisionLabel}
            </span>
            <span>{l("غير مفعل", "Disabled")}</span>
          </div>
        {/if}
      </div>
    {/each}
  </div>
</div>
