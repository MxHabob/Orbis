<!--
  Step 1: Academic Calendar & Semesters Setup
  Configures primary academic year and semester split without redundant regional settings.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Input } from "@orbis/ui/input";
import { FormField } from "@orbis/ui";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { Calendar, Layers } from "@lucide/svelte";

import { onboardingState } from "../state.svelte";
import { l } from "$lib/core/i18n";
</script>

<div class="w-full grid grid-cols-1 md:grid-cols-2 gap-4 text-start">

  <!-- Left Column: Academic Year Details -->
  <div class="bg-card border border-border/70 rounded-2xl p-4.5 flex flex-col justify-between shadow-xs space-y-3.5">
    <div class="flex items-center gap-2 border-b border-border/50 pb-2.5">
      <Calendar size={16} class="text-primary shrink-0" />
      <h3 class="text-xs font-bold text-foreground">
        {m.common_onboarding_academic_year_section()}
      </h3>
    </div>

    <div class="space-y-2.5 flex-1 flex flex-col justify-center">
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-2.5">
        <FormField id="academicYearNameAr" label={m.common_onboarding_academic_year_name_ar()} required>
          {#snippet children()}<Input id="academicYearNameAr" bind:value={onboardingState.calendar.academicYearNameAr} placeholder="العام الدراسي 1447 هـ" class="h-8.5 text-xs font-bold font-sans" />{/snippet}
        </FormField>

        <FormField id="academicYearName" label={m.common_onboarding_academic_year_name_en()}>
          {#snippet children()}<Input id="academicYearName" bind:value={onboardingState.calendar.academicYearName} placeholder="Academic Year 2025-2026" class="h-8.5 text-xs font-sans text-left" style="direction: ltr;" />{/snippet}
        </FormField>
      </div>

      <div class="grid grid-cols-1 sm:grid-cols-2 gap-2.5">
        <FormField id="startDate" label={m.common_onboarding_start_date()} required>
          {#snippet children()}<Input id="startDate" type="date" bind:value={onboardingState.calendar.startDate} class="h-8.5 text-xs font-mono" />{/snippet}
        </FormField>

        <FormField id="endDate" label={m.common_onboarding_end_date()} required>
          {#snippet children()}<Input id="endDate" type="date" bind:value={onboardingState.calendar.endDate} class="h-8.5 text-xs font-mono" />{/snippet}
        </FormField>
      </div>
    </div>
  </div>

  <!-- Right Column: Semesters & Structure -->
  <div class="bg-card border border-border/70 rounded-2xl p-4.5 flex flex-col justify-between shadow-xs space-y-3.5">
    <div class="flex items-center justify-between border-b border-border/50 pb-2.5">
      <div class="flex items-center gap-2">
        <Layers size={16} class="text-primary shrink-0" />
        <h3 class="text-xs font-bold text-foreground">
          {m.common_onboarding_semester_split()}
        </h3>
      </div>
      <span class="text-[10px] text-muted-foreground font-mono">
        {m.common_onboarding_semester_weight_hint()}
      </span>
    </div>

    <div class="space-y-2.5 flex-1 flex flex-col justify-center">
      <!-- Template Selection -->
      <FormField label={m.common_onboarding_template_label()} required>
        {#snippet children()}
        <Select type="single" bind:value={onboardingState.calendar.selectedTemplateId}>
          <SelectTrigger class="w-full h-8.5 text-xs font-sans bg-background">
            {#if onboardingState.calendar.selectedTemplateId}
              {@const selectedTpl = onboardingState.calendar.periodTemplates.find(t => t.id === onboardingState.calendar.selectedTemplateId)}
              {#if selectedTpl}
                {l(selectedTpl.name_ar, selectedTpl.name)}
              {:else}
                {m.common_onboarding_template_placeholder()}
              {/if}
            {:else}
              {m.common_onboarding_template_placeholder()}
            {/if}
          </SelectTrigger>
          <SelectContent>
            {#each onboardingState.calendar.periodTemplates as tpl}
              <SelectItem value={tpl.id}>{l(tpl.name_ar, tpl.name)}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
        {/snippet}
      </FormField>

      <!-- Terms Weight Preview -->
      {#if onboardingState.calendar.selectedTemplateId}
        {@const selectedTpl = onboardingState.calendar.periodTemplates.find(t => t.id === onboardingState.calendar.selectedTemplateId)}
        {#if selectedTpl?.structure && selectedTpl.structure.length > 0}
          <div class="pt-1">
            <div class="text-[10.5px] font-medium text-muted-foreground mb-1.5 text-start">
              {m.common_onboarding_period_distribution()}
            </div>
            <div class="grid grid-cols-2 sm:grid-cols-3 gap-2">
              {#each selectedTpl.structure as term}
                <div class="p-2 bg-muted/30 border border-border/60 rounded-lg flex items-center justify-between text-xs">
                  <span class="font-bold text-foreground text-[11px] truncate">{l(term.name_ar, term.name)}</span>
                  <span class="px-1.5 py-0.5 bg-primary/10 text-primary font-mono font-bold rounded text-[10px]">
                    {term.weight}%
                  </span>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      {/if}
    </div>
  </div>

</div>
