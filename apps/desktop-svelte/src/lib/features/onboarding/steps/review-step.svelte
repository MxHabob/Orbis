<!--
  Step 8: Review & Launch
  Summary of configuration and final launch button.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";

import { onboardingState } from "../state.svelte";
</script>

<div class="w-full space-y-8 max-w-4xl mx-auto">
  
  <!-- Review Summary List (Clean, borderless sections) -->
  <div class="border border-border/60 rounded-2xl bg-card overflow-hidden shadow-sm divide-y divide-border/60">
    
    <!-- Section 1: Institution Info -->
    <div class="p-6 space-y-4">
      <h4 class="text-xs font-bold text-primary uppercase tracking-wider text-start font-sans">
        {m.common_onboarding_review_institution_info()}
      </h4>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-3 text-sm">
        <div class="flex justify-between py-1 border-b border-border/30">
          <span class="text-muted-foreground">{m.common_onboarding_review_institution_name()}</span>
          <span class="font-bold text-foreground">{onboardingState.profile.orgNameAr}</span>
        </div>
        <div class="flex justify-between py-1 border-b border-border/30">
          <span class="text-muted-foreground">{m.common_onboarding_review_org_id()}</span>
          <span class="font-mono text-xs text-muted-foreground bg-muted px-1.5 py-0.5 rounded">{onboardingState.profile.orgName}</span>
        </div>
        <div class="flex justify-between py-1 border-b border-border/30">
          <span class="text-muted-foreground">{m.common_onboarding_review_currency()}</span>
          <span class="font-bold text-emerald-500">{onboardingState.profile.currency}</span>
        </div>
        <div class="flex justify-between py-1 border-b border-border/30">
          <span class="text-muted-foreground">{m.common_onboarding_review_locale_timezone()}</span>
          <span class="font-bold text-foreground">
            {onboardingState.localizedLanguageName} / {onboardingState.profile.timezone}
          </span>
        </div>
      </div>
    </div>

    <!-- Section 2: Academic Calendar -->
    <div class="p-6 space-y-4">
      <h4 class="text-xs font-bold text-primary uppercase tracking-wider text-start font-sans">
        {onboardingState.calendarLabel}
      </h4>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-3 text-sm">
        <div class="flex justify-between py-1 border-b border-border/30">
          <span class="text-muted-foreground">{onboardingState.calendarLabel}</span>
          <span class="font-bold text-foreground">{onboardingState.calendar.academicYearNameAr}</span>
        </div>
        <div class="flex justify-between py-1 border-b border-border/30">
          <span class="text-muted-foreground">{m.common_onboarding_review_year_duration()}</span>
          <span class="font-bold text-foreground font-mono">
            {m.common_onboarding_review_date_range({ start: onboardingState.calendar.startDate, end: onboardingState.calendar.endDate })}
          </span>
        </div>
        {#if onboardingState.calendar.selectedTemplateId}
          {@const activeTemplate = onboardingState.calendar.periodTemplates.find((t) => t.id === onboardingState.calendar.selectedTemplateId)}
          {#if activeTemplate}
            <div class="flex justify-between py-1 border-b border-border/30 md:col-span-2">
              <span class="text-muted-foreground">{m.common_onboarding_template_label()}</span>
              <span class="font-bold text-foreground">{activeTemplate[onboardingState.profile.locale.startsWith("ar") ? "name_ar" : "name"]}</span>
            </div>
          {/if}
        {/if}
      </div>
    </div>

    <!-- Section 3: Buildings & Rooms -->
    <div class="p-6 space-y-4">
      <h4 class="text-xs font-bold text-primary uppercase tracking-wider text-start font-sans">
        {m.common_onboarding_review_buildings_halls()}
      </h4>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-3 text-sm">
        {#each onboardingState.facilities.buildings as bld}
          {@const roomCount = onboardingState.facilities.rooms.filter((r) => r.building === bld).length}
          <div class="flex justify-between py-1 border-b border-border/30">
            <span class="text-muted-foreground">{bld}</span>
            <span class="font-bold text-foreground">
              {roomCount} {onboardingState.classroomsLabelPlural}
            </span>
          </div>
        {/each}
        <div class="flex justify-between py-1 border-b border-border/30 md:col-span-2">
          <span class="text-muted-foreground">{m.common_onboarding_review_capacity()}</span>
          <span class="font-bold text-primary">
            {m.common_insight_students_link({ count: onboardingState.facilities.totalCapacity })}
          </span>
        </div>
      </div>
    </div>

    <!-- Section 4: Stages & Subjects -->
    <div class="p-6 space-y-4">
      <h4 class="text-xs font-bold text-primary uppercase tracking-wider text-start font-sans">
        {onboardingState.stageLabel} & {onboardingState.subjectLabel}
      </h4>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-3 text-sm">
        <div class="flex justify-between py-1 border-b border-border/30">
          <span class="text-muted-foreground">{m.common_onboarding_review_enabled_stages({ stage: onboardingState.stageLabel })}</span>
          <span class="font-bold text-foreground">
            {onboardingState.academics.enabledStages} {onboardingState.stagesLabelPlural}
          </span>
        </div>
        <div class="flex justify-between py-1 border-b border-border/30">
          <span class="text-muted-foreground">{m.common_onboarding_review_divisions_label({ division: onboardingState.divisionLabel })}</span>
          <span class="font-bold text-foreground">
            {onboardingState.academics.totalDivisions}
          </span>
        </div>
        <div class="flex justify-between py-1 border-b border-border/30">
          <span class="text-muted-foreground">{m.common_onboarding_review_subjects_label({ subject: onboardingState.subjectLabel })}</span>
          <span class="font-bold text-foreground">
            {onboardingState.academics.enabledSubjects}
          </span>
        </div>
        <div class="flex justify-between py-1 border-b border-border/30">
          <span class="text-muted-foreground">{m.common_onboarding_review_time_slots()}</span>
          <span class="font-bold text-foreground">
            {m.common_onboarding_review_slots_value({ count: onboardingState.academics.totalSlots })}
          </span>
        </div>
      </div>
    </div>

  </div>
</div>
