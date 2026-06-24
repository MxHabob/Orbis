<!--
  Step 2: Academic Calendar
  Academic year, semester dates, and automatic month distribution.
-->
<script lang="ts">
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import * as m from "$lib/paraglide/messages";
import { onboardingState } from "../onboarding-state.svelte";
</script>

<div class="max-w-3xl space-y-6">
  <div class="space-y-2 text-start">
    <h3 class="text-foreground font-bold text-xl font-serif">
      {m.onboarding_calendar_title()}
    </h3>
    <p class="text-muted-foreground text-sm">
      {m.onboarding_calendar_desc()}
    </p>
  </div>

  <!-- Academic Year Section -->
  <div class="grid grid-cols-1 md:grid-cols-2 gap-6 bg-card border border-border p-6 rounded-xl">
    <div class="md:col-span-2 text-start">
      <h4 class="text-sm font-semibold text-primary border-b border-border pb-2 mb-4 font-serif">
        {m.onboarding_academic_year_section()}
      </h4>
    </div>

    <div class="space-y-2">
      <Label for="academicYearNameAr">{m.onboarding_academic_year_name_ar()}</Label>
      <Input id="academicYearNameAr" bind:value={onboardingState.academicYearNameAr} placeholder="العام الدراسي 1447 هـ" />
    </div>

    <div class="space-y-2">
      <Label for="academicYearName">{m.onboarding_academic_year_name_en()}</Label>
      <Input id="academicYearName" bind:value={onboardingState.academicYearName} placeholder="Academic Year 1447 Hijri" class="text-left font-sans" style="direction: ltr;" />
    </div>

    <div class="space-y-2">
      <Label for="startDate">{m.onboarding_start_date()}</Label>
      <Input id="startDate" type="date" bind:value={onboardingState.startDate} />
    </div>

    <div class="space-y-2">
      <Label for="endDate">{m.onboarding_end_date()}</Label>
      <Input id="endDate" type="date" bind:value={onboardingState.endDate} />
    </div>
  </div>

  <!-- Semesters Section -->
  <div class="grid grid-cols-1 md:grid-cols-2 gap-6 bg-card border border-border p-6 rounded-xl">
    <div class="md:col-span-2 flex items-center justify-between border-b border-border pb-2 mb-4">
      <h4 class="text-sm font-semibold text-primary font-serif">
        {m.onboarding_semester_split()}
      </h4>
      <span class="text-xs text-muted-foreground font-mono">
        {m.onboarding_semester_weight_hint()}
      </span>
    </div>

    <!-- Semester 1 -->
    <div class="space-y-4">
      <div class="space-y-1 text-start">
        <h5 class="text-xs font-semibold text-foreground font-serif">
          {m.onboarding_semester_1()}
        </h5>
      </div>
      <div class="space-y-2">
        <Label class="text-xs">{m.onboarding_semester_name_ar()}</Label>
        <Input bind:value={onboardingState.semester1NameAr} class="h-9 text-xs" />
      </div>
      <div class="space-y-2">
        <Label class="text-xs">{m.onboarding_semester_start()}</Label>
        <Input type="date" bind:value={onboardingState.semester1Start} class="h-9 text-xs" />
      </div>
      <div class="space-y-2">
        <Label class="text-xs">{m.onboarding_semester_end()}</Label>
        <Input type="date" bind:value={onboardingState.semester1End} class="h-9 text-xs" />
      </div>
    </div>

    <!-- Semester 2 -->
    <div class="space-y-4 ps-6 border-s border-border">
      <div class="space-y-1 text-start">
        <h5 class="text-xs font-semibold text-foreground font-serif">
          {m.onboarding_semester_2()}
        </h5>
      </div>
      <div class="space-y-2">
        <Label class="text-xs">{m.onboarding_semester_name_ar()}</Label>
        <Input bind:value={onboardingState.semester2NameAr} class="h-9 text-xs" />
      </div>
      <div class="space-y-2">
        <Label class="text-xs">{m.onboarding_semester_start()}</Label>
        <Input type="date" bind:value={onboardingState.semester2Start} class="h-9 text-xs" />
      </div>
      <div class="space-y-2">
        <Label class="text-xs">{m.onboarding_semester_end()}</Label>
        <Input type="date" bind:value={onboardingState.semester2End} class="h-9 text-xs" />
      </div>
    </div>

    <!-- Hint -->
    <div class="md:col-span-2 pt-2 text-start">
      <p class="text-[11px] text-muted-foreground leading-relaxed">
        {m.onboarding_calendar_months_hint()}
      </p>
    </div>
  </div>
</div>
