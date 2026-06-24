<!--
  Step 6: School Subjects Setup
  List of subjects with toggle checkboxes, add custom subjects.
-->
<script lang="ts">
import { Label } from "@orbis/ui-svelte/label";
import { Input } from "@orbis/ui-svelte/input";
import { Checkbox } from "@orbis/ui-svelte/checkbox";
import { Badge } from "@orbis/ui-svelte/badge";
import { Button } from "@orbis/ui-svelte/button";
import { Plus, BookOpen } from "lucide-svelte";
import * as m from "$lib/paraglide/messages";
import { onboardingState } from "../onboarding-state.svelte";

let newSubCode = $state("");
let newSubName = $state("");
let newSubNameAr = $state("");
let newSubColor = $state("#f59e0b");

function handleAddSubject() {
  const success = onboardingState.addSubject(
    newSubCode,
    newSubName,
    newSubNameAr,
    newSubColor
  );
  if (success) {
    newSubCode = "";
    newSubName = "";
    newSubNameAr = "";
  }
}
</script>

<div class="max-w-4xl space-y-6">
  <div class="space-y-2 text-start">
    <h3 class="text-foreground font-bold text-xl font-serif">
      {m.onboarding_subjects_title()}
    </h3>
    <p class="text-muted-foreground text-sm">
      {m.onboarding_subjects_desc()}
    </p>
  </div>

  <!-- Add Subject form -->
  <div class="p-6 bg-muted/40 rounded-xl border border-border grid grid-cols-1 md:grid-cols-5 gap-4 items-end">
    <div class="space-y-2 text-start">
      <Label class="text-xs">{m.onboarding_subject_code()}</Label>
      <Input bind:value={newSubCode} placeholder="PHY" class="text-xs h-9 uppercase font-sans" />
    </div>

    <div class="space-y-2 text-start">
      <Label class="text-xs">{m.onboarding_subject_name_ar()}</Label>
      <Input bind:value={newSubNameAr} placeholder="الفيزياء" class="text-xs h-9" />
    </div>

    <div class="space-y-2 font-sans text-start">
      <Label class="text-xs">{m.onboarding_subject_name_en()}</Label>
      <Input bind:value={newSubName} placeholder="Physics" class="text-xs h-9 text-left" style="direction: ltr;" />
    </div>

    <div class="space-y-2 text-start">
      <Label class="text-xs">{m.onboarding_subject_color()}</Label>
      <div class="flex gap-2 items-center">
        <input type="color" bind:value={newSubColor} class="w-9 h-9 border border-border rounded bg-background cursor-pointer" />
        <span class="text-xs font-mono text-muted-foreground">{newSubColor}</span>
      </div>
    </div>

    <Button onclick={handleAddSubject} class="w-full h-9 text-xs flex gap-1.5 items-center justify-center">
      <Plus class="w-4 h-4" />
      <span>{m.onboarding_subjects_add_subject()}</span>
    </Button>
  </div>

  <!-- Subjects List Grid -->
  <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
    {#each onboardingState.subjects as sub (sub.code)}
      <div
        class="bg-card border rounded-xl p-4 space-y-3 transition-colors
          {sub.enabled ? 'border-border' : 'border-border/40 opacity-50'}"
      >
        <div class="flex justify-between items-start">
          <div class="flex gap-3 items-center text-start">
            <!-- Color indicator -->
            <div
              class="w-8 h-8 rounded-lg flex items-center justify-center"
              style="background-color: {sub.color}20; border: 1px solid {sub.color}40;"
            >
              <BookOpen class="w-4 h-4" style="color: {sub.color};" />
            </div>
            <div class="text-start">
              <div class="font-bold text-foreground text-sm flex gap-1.5 items-center font-serif">
                <span>{sub.name_ar}</span>
                <Badge variant="outline" class="text-[9px] px-1 font-mono uppercase bg-muted border-border text-muted-foreground">
                  {sub.code}
                </Badge>
              </div>
              <p class="text-xs text-muted-foreground font-mono mt-0.5">{sub.name}</p>
            </div>
          </div>

          <Checkbox bind:checked={sub.enabled} />
        </div>
      </div>
    {/each}
  </div>
</div>
