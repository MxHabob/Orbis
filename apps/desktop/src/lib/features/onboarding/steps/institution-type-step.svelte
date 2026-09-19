<!--
  Step 1: Institution Type Selection
  Shows institution type options (School, University, Institute, Training Center, Custom).
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Card } from "@orbis/ui/card";
import { Label } from "@orbis/ui/label";
import { Input } from "@orbis/ui/input";
import { FormField } from "@orbis/ui";
import { Badge } from "@orbis/ui/badge";

import { onboardingState } from "../state.svelte";
import { INSTITUTION_PROFILES } from "../templates";
import {
  School,
  GraduationCap,
  BookOpen,
  Presentation,
  Sliders,
  CheckCircle2,
  Info,
} from "@lucide/svelte";
import { l } from "$lib/core/i18n";

const profiles = [
  { id: "school", icon: School },
  { id: "university", icon: GraduationCap },
  { id: "institute", icon: BookOpen },
  { id: "training_center", icon: Presentation },
  { id: "custom", icon: Sliders },
];

// Check if institution type is already set
let isPreConfigured = $derived(
  onboardingState.profile.orgName !== "" &&
    onboardingState.profile.institutionType !== "",
);

function selectProfile(type: string) {
  onboardingState.selectInstitutionType(type);
}
</script>

<div class="w-full space-y-6">

  <!-- Notice when pre-configured -->
  {#if isPreConfigured}
    <div class="flex items-center gap-3 p-3 rounded-xl border border-primary/20 bg-primary/5 text-primary text-xs font-sans">
      <Info size={16} class="shrink-0" />
      <span>
        {l("تم ضبط نوع المؤسسة مسبقاً أثناء التهيئة. يمكنك تعديل الخيار أدناه.", "Institution type is pre-configured. You can adjust it below if needed.")}
      </span>
    </div>
  {/if}

  <!-- Profiles Grid -->
  <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4">
    {#each profiles as prof}
      {@const data = onboardingState.localizedProfile(prof.id)}
      <button
        type="button"
        class="text-start relative group rounded-xl border bg-card p-4 cursor-pointer transition-all duration-200 flex flex-col justify-between gap-4 h-40 {onboardingState.profile.institutionType === prof.id ? 'border-primary bg-primary/5 text-primary' : 'border-border/60 hover:bg-muted/30 hover:border-muted-foreground/30'}"
        onclick={() => selectProfile(prof.id)}
      >
        <div class="flex items-start justify-between w-full">
          <div class="p-2 rounded-lg {onboardingState.profile.institutionType === prof.id ? 'bg-primary/20 text-primary' : 'bg-muted text-muted-foreground group-hover:bg-primary/10 group-hover:text-primary'} transition-all duration-200">
            <prof.icon size={20} />
          </div>
          {#if onboardingState.profile.institutionType === prof.id}
            <div class="text-primary">
              <CheckCircle2 size={16} />
            </div>
          {/if}
        </div>

        <div class="space-y-1">
          <h4 class="text-xs font-bold text-foreground group-hover:text-primary transition-colors">
            {data.name}
          </h4>
          <p class="text-[10px] text-muted-foreground line-clamp-2 leading-relaxed font-sans">
            {data.description}
          </p>
        </div>
      </button>
    {/each}
  </div>

  <!-- Terminology Information / Preview -->
  {#if onboardingState.profile.institutionType}
    <div class="p-4 border border-border/60 rounded-xl bg-muted/10 space-y-3 text-start">
      <h4 class="text-xs font-bold text-foreground flex items-center gap-2 font-serif">
        <span class="w-1 h-2.5 rounded-full bg-primary"></span>
        {m.common_onboarding_profile_terminology_title()}
      </h4>
      <div class="grid grid-cols-2 sm:grid-cols-4 gap-4 text-xs font-sans">
        <div class="space-y-1">
          <span class="text-[10px] text-muted-foreground block">{m.common_class_label_grade()}</span>
          <span class="font-semibold text-foreground">{onboardingState.stageLabel}</span>
        </div>
        <div class="space-y-1">
          <span class="text-[10px] text-muted-foreground block">{m.common_onboarding_term_classroom()}</span>
          <span class="font-semibold text-foreground">{onboardingState.classroomLabel}</span>
        </div>
        <div class="space-y-1">
          <span class="text-[10px] text-muted-foreground block">{m.common_onboarding_term_division()}</span>
          <span class="font-semibold text-foreground">{onboardingState.divisionLabel}</span>
        </div>
        <div class="space-y-1">
          <span class="text-[10px] text-muted-foreground block">{m.common_onboarding_term_subject()}</span>
          <span class="font-semibold text-foreground">{onboardingState.subjectLabel}</span>
        </div>
      </div>
    </div>
  {/if}

  <!-- Automatic Numbering Customization -->
  <div class="p-4 border border-border/60 rounded-xl bg-card space-y-3 text-start">
    <FormField id="numbering_pattern" label={m.common_onboarding_profile_numbering_title()} required>
      {#snippet children()}
      <p class="text-[10px] text-muted-foreground">
        {m.common_onboarding_profile_numbering_desc()}
      </p>
      <Input 
      id="numbering_pattern" 
      bind:value={onboardingState.profile.studentNumberPattern} 
      class="h-9 text-xs border border-input max-w-md font-mono" 
      />
      {/snippet}
    </FormField>
  </div>
</div>
