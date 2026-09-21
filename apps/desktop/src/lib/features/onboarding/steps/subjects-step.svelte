<!--
  Step 5: Curriculum & Subjects (المواد الدراسية)
  List of subjects with unified card design and whole-card click-to-toggle matching grade cards.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Input } from "@orbis/ui/input";
import { Button } from "@orbis/ui/button";
import { Checkbox } from "@orbis/ui/checkbox";
import { Plus } from "@lucide/svelte";

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

<div class="w-full h-full flex flex-col min-h-0 space-y-3 text-start animate-in fade-in duration-300">
  <div class="flex items-center justify-between border-b border-border/50 pb-2 shrink-0">
    <h3 class="text-xs font-bold text-foreground">
      {onboardingState.subjectLabel}
    </h3>
    <span class="text-[11px] font-bold text-primary bg-primary/10 px-2.5 py-0.5 rounded-full font-mono">
      {onboardingState.academics.enabledSubjects} / {onboardingState.academics.subjects.length} {l("مفعلة", "Active")}
    </span>
  </div>

  <!-- Inline Quick Add Subject Card -->
  <div class="rounded-2xl border border-border/70 bg-card p-2.5 shadow-2xs shrink-0">
    <div class="grid grid-cols-[36px_2fr_1fr_2fr_auto] gap-2 items-center">
      <input type="color" bind:value={newSubColor} class="size-7.5 rounded-lg border border-border/70 bg-transparent cursor-pointer p-0 shrink-0" />
      
      <Input bind:value={newSubNameAr} placeholder="اسم المادة (عربي)" class="h-8 text-xs font-bold font-sans bg-background" />
      <Input bind:value={newSubCode} placeholder="CHM" class="h-8 text-xs font-mono uppercase bg-background text-center" />
      <Input bind:value={newSubName} placeholder="Subject Name (English)" class="h-8 text-xs font-sans bg-background text-left" style="direction: ltr;" />

      <Button size="sm" class="h-8 px-4 text-xs gap-1 font-bold cursor-pointer" onclick={handleAddSubject} disabled={!newSubNameAr.trim() && !newSubName.trim()}>
        <Plus size={13} />
        <span>{m.common_onboarding_add()}</span>
      </Button>
    </div>
  </div>

  <!-- Subjects Multi-Column Responsive Grid (Identical to Grade Cards) -->
  <div class="flex-1 min-h-0 overflow-y-auto scrollbar-hide grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-3 p-1">
    {#each onboardingState.academics.subjects as sub (sub.code)}
      <div
        role="button"
        tabindex="0"
        onclick={() => {
          sub.enabled = !sub.enabled;
          onboardingState.save();
        }}
        onkeydown={(e) => {
          if (e.key === "Enter" || e.key === " ") {
            sub.enabled = !sub.enabled;
            onboardingState.save();
          }
        }}
        class="bg-card border rounded-2xl p-3.5 flex flex-col justify-between gap-2.5 transition-all duration-150 shadow-2xs hover:shadow-xs cursor-pointer select-none
          {sub.enabled
            ? 'border-border/80 bg-card ring-1 ring-primary/20'
            : 'border-border/30 opacity-40 bg-muted/10 hover:opacity-70'}"
      >
        <!-- Subject Header -->
        <div class="flex justify-between items-center gap-2">
          <div class="flex items-center gap-2.5 min-w-0">
            <div
              class="size-3 rounded-full shrink-0"
              style="background-color: {sub.color};"
            ></div>
            <h4 class="font-bold text-foreground text-xs truncate">
              {l(sub.name_ar, sub.name)}
            </h4>
          </div>

          <div class="pointer-events-none shrink-0 flex items-center justify-center">
            <Checkbox checked={sub.enabled} />
          </div>
        </div>

        <!-- Subject Bottom Details -->
        <div class="flex items-center justify-between pt-2 border-t border-border/40 text-[10px] text-muted-foreground">
          <span class="font-mono font-bold px-2 py-0.5 rounded-md bg-muted/60 border border-border/40 text-foreground uppercase text-[9.5px]">
            {sub.code}
          </span>
          <span>
            {sub.enabled ? l("مفعلة", "Active") : l("غير مفعلة", "Inactive")}
          </span>
        </div>
      </div>
    {/each}
  </div>
</div>


