<!--
  Step 6: School Subjects Setup
  List of subjects with toggle checkboxes, add custom subjects.
-->
<script lang="ts">
import { Label } from "@orbis/ui-svelte/label";
import { Input } from "@orbis/ui-svelte/input";
import { Checkbox } from "@orbis/ui-svelte/checkbox";
import { Button } from "@orbis/ui-svelte/button";
import { Plus } from "lucide-svelte";
import * as m from "$lib/paraglide/messages";
import { onboardingState } from "../state.svelte";

let newSubCode = $state("");
let newSubName = $state("");
let newSubNameAr = $state("");
let newSubColor = $state("#f59e0b");

function handleAddSubject() {
  const success = onboardingState.addSubject(
    newSubCode,
    newSubName,
    newSubNameAr,
    newSubColor,
  );
  if (success) {
    newSubCode = "";
    newSubName = "";
    newSubNameAr = "";
  }
}
</script>

<div class="max-w-4xl space-y-6">

  <!-- Add Subject form -->
  <div class="p-6 bg-card rounded-2xl border border-border/60 grid grid-cols-1 md:grid-cols-5 gap-4 items-end shadow-sm">
    <div class="space-y-2 text-start">
      <Label class="text-xs font-semibold text-muted-foreground">{m.onboarding_subject_code_label({ subject: onboardingState.subjectLabel })}</Label>
      <Input bind:value={newSubCode} placeholder="PHY" class="text-xs h-9 uppercase font-sans" />
    </div>

    <div class="space-y-2 text-start">
      <Label class="text-xs font-semibold text-muted-foreground">{m.onboarding_subject_name_ar_label({ subject: onboardingState.subjectLabel })}</Label>
      <Input bind:value={newSubNameAr} placeholder="الفيزياء" class="text-xs h-9" />
    </div>

    <div class="space-y-2 font-sans text-start">
      <Label class="text-xs font-semibold text-muted-foreground">{m.onboarding_subject_name_en_label({ subject: onboardingState.subjectLabel })}</Label>
      <Input bind:value={newSubName} placeholder="Physics" class="text-xs h-9 text-left" style="direction: ltr;" />
    </div>

    <div class="space-y-2 text-start">
      <Label class="text-xs font-semibold text-muted-foreground">{m.onboarding_subject_color()}</Label>
      <div class="flex gap-2 items-center">
        <input type="color" bind:value={newSubColor} class="w-9 h-9 border border-border/60 rounded bg-background cursor-pointer" />
        <span class="text-xs font-mono text-muted-foreground">{newSubColor}</span>
      </div>
    </div>

    <Button onclick={handleAddSubject} class="w-full h-9 text-xs flex gap-1.5 items-center justify-center rounded-xl bg-primary text-primary-foreground hover:bg-primary/90">
      <Plus class="w-4 h-4" />
      <span>{m.onboarding_add_label()}</span>
    </Button>
  </div>

  <!-- Subjects List (Clean, borderless list of rows) -->
  <div class="divide-y divide-border/60 border border-border/60 rounded-2xl bg-card overflow-hidden shadow-sm">
    {#each onboardingState.subjects as sub (sub.code)}
      <div class="flex items-center justify-between p-4 transition-colors hover:bg-muted/10 {sub.enabled ? '' : 'opacity-50'}">
        <div class="flex items-center gap-4 text-start">
          <!-- Color Dot (Simple & elegant indicator) -->
          <div class="w-3 h-3 rounded-full shrink-0" style="background-color: {sub.color};"></div>
          
          <div class="flex flex-col sm:flex-row sm:items-center gap-1.5 sm:gap-4 text-start">
            <span class="font-bold text-foreground text-sm font-sans">
              {sub.name_ar}
            </span>
            <span class="text-xs text-muted-foreground font-mono bg-muted border border-border/60 px-1.5 py-0.5 rounded uppercase">
              {sub.code}
            </span>
            <span class="text-xs text-muted-foreground sm:border-s sm:border-border/60 sm:ps-4 font-sans">
              {sub.name}
            </span>
          </div>
        </div>
        
        <Checkbox bind:checked={sub.enabled} />
      </div>
    {/each}
  </div>
</div>
