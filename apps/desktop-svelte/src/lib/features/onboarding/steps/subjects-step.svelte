<!--
  Step 6: School Subjects Setup
  List of subjects with toggle checkboxes, add custom subjects.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Input } from "@orbis/ui-svelte/input";
import { FormField } from "@orbis/ui-svelte";
import { Checkbox } from "@orbis/ui-svelte/checkbox";
import { Button } from "@orbis/ui-svelte/button";
import { Plus } from "@lucide/svelte";

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

<div class="w-full space-y-6">

  <!-- Subjects List (Clean, borderless list of rows) -->
  <div class="w-full overflow-x-auto scrollbar-hide border border-border/60 rounded-2xl bg-card shadow-sm">
    <div class="min-w-3xl divide-y divide-border/60">
      <!-- Header Row -->
      <div class="grid grid-cols-[50px_1.8fr_1fr_2fr_auto] gap-4 p-4 bg-muted/20 text-xs font-semibold text-muted-foreground select-none text-start items-center">
        <span>{m.common_td_text_color()}</span>
        <span>{m.common_onboarding_subject_name_ar()}</span>
        <span>{m.common_subject_label_code()}</span>
        <span>{m.common_onboarding_subjects_table_name_en()}</span>
        <span class="w-12 text-center">{m.common_word_status()}</span>
      </div>

      <!-- Inline Add Subject Row -->
      <div class="grid grid-cols-[50px_1.8fr_1fr_2fr_auto] gap-4 px-4 py-2.5 items-center bg-muted/5 transition-colors text-start">
        <!-- Color Picker -->
        <div class="flex justify-center items-center">
          <input type="color" bind:value={newSubColor} class="w-7 h-7 rounded-full border border-border/60 bg-transparent cursor-pointer p-0 overflow-hidden shrink-0" />
        </div>

        <!-- Name Arabic -->
        <FormField label={m.common_onboarding_subject_name_ar()} required>{#snippet children()}<Input bind:value={newSubNameAr} placeholder={m.common_onboarding_subjects_placeholder_ar()} class="h-8 text-xs font-bold font-sans bg-background/50 border-dashed border-border/80" />{/snippet}</FormField>

        <!-- Code -->
        <FormField label={m.common_subject_label_code()} required>{#snippet children()}<Input bind:value={newSubCode} placeholder="CHM" class="h-8 text-xs font-mono uppercase bg-background/50 border-dashed border-border/80 text-center" />{/snippet}</FormField>

        <!-- Name English -->
        <FormField label={m.common_onboarding_subjects_table_name_en()}>{#snippet children()}<Input bind:value={newSubName} placeholder={m.common_onboarding_subjects_placeholder_ar()} class="h-8 text-xs font-sans bg-background/50 border-dashed border-border/80 text-left" style="direction: ltr;" />{/snippet}</FormField>

        <!-- Add Button -->
        <div class="flex justify-center items-center">
          <Button variant="default" size="icon-sm" class="h-8 w-8 rounded-lg bg-primary text-primary-foreground hover:bg-primary/90 animate-pulse" onclick={handleAddSubject}>
            <Plus size={14} />
          </Button>
        </div>
      </div>

      <!-- Subject Rows -->
      {#each onboardingState.academics.subjects as sub (sub.code)}
        <div class="grid grid-cols-[50px_1.8fr_1fr_2fr_auto] gap-4 px-4 py-3 items-center hover:bg-muted/10 transition-colors text-start {sub.enabled ? '' : 'opacity-50'}">
          <!-- Color Dot -->
          <div class="flex justify-center items-center">
            <div class="w-3 h-3 rounded-full shrink-0" style="background-color: {sub.color};"></div>
          </div>

          <!-- Name Arabic -->
          <span class="font-bold text-foreground text-sm font-sans">{sub.name_ar}</span>

          <!-- Code -->
          <div class="text-start">
            <span class="text-xs text-muted-foreground font-mono bg-muted border border-border/60 px-1.5 py-0.5 rounded uppercase">
              {sub.code}
            </span>
          </div>

          <!-- Name English -->
          <span class="text-xs text-muted-foreground font-sans">{sub.name}</span>

          <!-- Status Checkbox -->
          <div class="flex justify-center items-center w-12">
            <Checkbox bind:checked={sub.enabled} />
          </div>
        </div>
      {/each}
    </div>
  </div>
</div>
