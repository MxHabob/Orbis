<!--
  Step 5: Curriculum & Subjects (المواد الدراسية)
  List of subjects with toggle checkboxes, add custom subjects, full height utilization.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Input } from "@orbis/ui/input";
import { FormField } from "@orbis/ui";
import { Checkbox } from "@orbis/ui/checkbox";
import { Button } from "@orbis/ui/button";
import { Plus, BookOpen, Trash2 } from "@lucide/svelte";

import { onboardingState } from "../state.svelte";
import { l } from "$lib/core/i18n";

let newSubCode = $state("");
let newSubName = $state("");
let newSubNameAr = $state("");
let newSubColor = $state("#3b82f6");

function handleAddSubject() {
  const nameAr = newSubNameAr.trim() || newSubName.trim();
  const nameEn = newSubName.trim() || newSubNameAr.trim();
  const code = newSubCode.trim().toUpperCase() || nameEn.slice(0, 3).toUpperCase();
  if (!nameAr) return;

  const success = onboardingState.addSubject(
    code,
    nameEn,
    nameAr,
    newSubColor,
  );
  if (success) {
    newSubCode = "";
    newSubName = "";
    newSubNameAr = "";
  }
}
</script>

<div class="w-full h-full flex flex-col min-h-0 space-y-2.5 text-start">
  <div class="flex items-center justify-between border-b border-border/50 pb-1.5 shrink-0">
    <div class="flex items-center gap-2">
      <BookOpen size={16} class="text-primary shrink-0" />
      <h3 class="text-xs font-bold text-foreground">
        {onboardingState.subjectLabel}
      </h3>
    </div>
    <span class="text-[11px] font-bold text-primary bg-primary/10 px-2 py-0.5 rounded-full font-mono">
      {onboardingState.academics.enabledSubjects} {m.common_word_active ? m.common_word_active() : "مفعلة"}
    </span>
  </div>

  <!-- Inline Quick Add Subject Card -->
  <div class="rounded-2xl border border-primary/20 bg-primary/5 p-2.5 shadow-2xs shrink-0">
    <div class="grid grid-cols-[36px_2fr_1fr_2fr_auto] gap-2 items-center">
      <input type="color" bind:value={newSubColor} class="size-7 rounded-lg border border-border/70 bg-transparent cursor-pointer p-0 shrink-0" />
      
      <Input bind:value={newSubNameAr} placeholder="اسم المادة (عربي)" class="h-7.5 text-xs font-bold font-sans bg-background" />
      <Input bind:value={newSubCode} placeholder="CHM" class="h-7.5 text-xs font-mono uppercase bg-background text-center" />
      <Input bind:value={newSubName} placeholder="Subject Name (English)" class="h-7.5 text-xs font-sans bg-background text-left" style="direction: ltr;" />

      <Button size="sm" class="h-7.5 px-3 text-xs gap-1 font-bold cursor-pointer" onclick={handleAddSubject} disabled={!newSubNameAr.trim() && !newSubName.trim()}>
        <Plus size={13} />
        <span>{m.common_onboarding_add()}</span>
      </Button>
    </div>
  </div>

  <!-- Subjects Multi-Column Responsive Grid (Expands to fill 100% available height) -->
  <div class="flex-1 min-h-0 overflow-y-auto scrollbar-hide grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-2.5 p-1">
    {#each onboardingState.academics.subjects as sub (sub.code)}
      <div class="flex items-center justify-between p-2.5 rounded-2xl border bg-card hover:bg-muted/15 transition-all {sub.enabled ? 'border-border/80 shadow-2xs' : 'border-border/30 opacity-40'}">
        <div class="flex items-center gap-2.5 min-w-0">
          <div class="size-3 rounded-full shrink-0" style="background-color: {sub.color};"></div>
          <div class="flex items-center gap-1.5 min-w-0">
            <span class="font-bold text-foreground text-xs truncate">{l(sub.name_ar, sub.name)}</span>
            <span class="text-[9.5px] font-mono text-muted-foreground bg-muted px-1.5 py-0.2 rounded border border-border/50 shrink-0 uppercase">
              {sub.code}
            </span>
          </div>
        </div>

        <Checkbox bind:checked={sub.enabled} class="cursor-pointer shrink-0 ms-2" />
      </div>
    {/each}
  </div>
</div>
