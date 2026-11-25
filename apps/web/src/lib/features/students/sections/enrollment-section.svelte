<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { BookOpen } from "@orbis/icons";
import { Card, CardContent, CardHeader, CardTitle } from "@orbis/ui/card";
import { Label } from "@orbis/ui/label";
import { Select, SelectContent, SelectItem, SelectTrigger } from "@orbis/ui/select";
import { ClassSelect } from "$lib/features/academics";
import CapacityIndicator from "$lib/shared/components/capacity-indicator.svelte";
import type { StudentFormState } from "../form-state.svelte";

let { state }: { state: StudentFormState } = $props();

const studyModes: { value: StudentFormState["studyMode"]; label: string }[] = [
  { value: "regular", label: m.operations_ui_study_mode_regular() },
  { value: "part_time", label: m.operations_ui_study_mode_part_time() },
  { value: "distance_learning", label: m.operations_ui_study_mode_distance() },
  { value: "homeschooled", label: m.operations_ui_study_mode_homeschooled() },
  { value: "auditor", label: m.operations_ui_study_mode_auditor() },
  { value: "visiting", label: m.operations_ui_study_mode_visiting() },
  { value: "exchange", label: m.operations_ui_study_mode_exchange() },
  { value: "special_needs", label: m.operations_ui_study_mode_special_needs() },
];
</script>

<Card class="gap-0">
  <CardHeader class="border-b border-border px-4 py-4 sm:px-6">
    <CardTitle class="flex items-center gap-2 text-sm font-semibold">
      <BookOpen class="size-4 text-primary" aria-hidden="true" />
      {m.student_registration_step_enrollment()}
    </CardTitle>
  </CardHeader>
  <CardContent class="p-0">
    <div class="grid items-start gap-3 px-4 py-3 sm:px-6">
      <div class="flex flex-col gap-1 text-start">
        <Label for="classSelect" class="text-xs font-semibold text-foreground flex items-center gap-1">
          {m.common_form_target_class()}
          <span class="text-destructive">*</span>
        </Label>
        <div class="mt-1 flex min-w-0 flex-col gap-2">
          <ClassSelect
            id="classSelect"
            bind:value={state.personal.classId}
          />
          {#if state.selectedClass}
            {@const enrolledCount = state.selectedClass.current_students ?? 0}
            {@const classCapacity = state.selectedClass.capacity ?? state.admissionPolicy.maxClassCapacity}
            <div class="flex items-center justify-between px-1">
              <span class="text-[10px] text-muted-foreground">{m.ui_copy_capacity_ebf7f7c9()}</span>
              <CapacityIndicator
                current={enrolledCount}
                max={classCapacity}
                size="sm"
                class="w-44"
              />
            </div>
            {#if classCapacity > 0 && enrolledCount >= classCapacity}
              <p class="px-1 text-xs text-amber-600 dark:text-amber-400" role="status">
                {m.student_registration_capacity_review()}
              </p>
            {/if}
          {/if}
        </div>
      </div>
      <div class="flex flex-col gap-1.5 text-start w-full min-w-0">
        <Label for="student-study-mode" class="text-xs font-semibold text-foreground">{m.ui_copy_study_mode_enrollment_track_2468575d()}</Label>
        <Select
          type="single"
          value={state.studyMode}
          onValueChange={(value) => {
            const selected = studyModes.find((mode) => mode.value === value);
            if (selected) state.studyMode = selected.value;
          }}
        >
          <SelectTrigger id="student-study-mode" class="h-9 text-xs bg-background">
            {studyModes.find((mode) => mode.value === state.studyMode)?.label ?? state.studyMode}
          </SelectTrigger>
          <SelectContent class="text-start max-h-64 overflow-y-auto" align="end">
            {#each studyModes as mode (mode.value)}
              <SelectItem value={mode.value} class="text-xs">
                {mode.label}
              </SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>
    </div>
  </CardContent>
</Card>
