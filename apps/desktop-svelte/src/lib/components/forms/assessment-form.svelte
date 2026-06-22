<script lang="ts">
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";

let {
  classId,
  subjectId,
  cycleId,
  onSuccess,
  onCancel,
}: {
  classId: string;
  subjectId: string;
  cycleId: string;
  onSuccess: () => void;
  onCancel: () => void;
} = $props();

// Form state
let isSubmitting = $state(false);
let name = $state("");
let nameAr = $state("");
let maxScore = $state<number>(100);
let weight = $state<number>(15);
let assessmentType = $state<"exam" | "quiz" | "homework" | "classwork">("exam");

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  if (!nameAr && !name) {
    toast.error(m.assessment_toast_error_name());
    return;
  }
  if (!classId || !subjectId || !cycleId) {
    toast.error("Please select a class, subject, and cycle before creating an assessment.");
    return;
  }
  isSubmitting = true;
  try {
    const request = {
      class_id: classId,
      subject_id: subjectId,
      term_id: cycleId,
      title: name || nameAr,
      title_ar: nameAr || name,
      max_score: maxScore ? Number(maxScore) : 100,
      weight: weight ? Number(weight) : 15,
      item_type: assessmentType,
    };
    await commands.assessments.create(request);
    toast.success(m.assessment_toast_success());

    // Reset fields
    name = "";
    nameAr = "";
    maxScore = 100;
    weight = 15;
    assessmentType = "exam";

    onSuccess();
  } catch (e) {
    toast.error(m.assessment_toast_error_submit());
  } finally {
    isSubmitting = false;
  }
}
</script>

<form onsubmit={handleSubmit} class="flex-1 overflow-y-auto py-6 flex flex-col gap-6 scrollbar-hide">
	<!-- Name (Arabic) -->
	<div class="flex flex-col gap-2 text-start">
		<label for="ass_name_ar" class="text-sm font-medium text-foreground">
			{m.assessment_label_name_ar()}
		</label>
		<Input 
			id="ass_name_ar"
			type="text" 
			bind:value={nameAr}
			placeholder={m.assessment_placeholder_name_ar()}
			required
		/>
	</div>

	<!-- Name (English) -->
	<div class="flex flex-col gap-2 text-start">
		<label for="ass_name_en" class="text-sm font-medium text-foreground">
			{m.assessment_label_name_en()}
		</label>
		<Input 
			id="ass_name_en"
			type="text" 
			bind:value={name}
			placeholder="e.g., Midterm Mathematics Exam"
		/>
	</div>

	<!-- Assessment Type -->
	<div class="flex flex-col gap-2 text-start">
		<label for="ass_type" class="text-sm font-medium text-foreground">
			{m.assessment_label_type()}
		</label>
		<select 
			id="ass_type"
			bind:value={assessmentType}
			class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-sm focus:ring-1 focus:ring-ring outline-none h-9"
		>
			<option value="exam">{m.assessment_option_exam()}</option>
			<option value="quiz">{m.assessment_option_quiz()}</option>
			<option value="homework">{m.assessment_option_homework()}</option>
			<option value="classwork">{m.assessment_option_classwork()}</option>
		</select>
	</div>

	<!-- Two columns for scores -->
	<div class="grid grid-cols-2 gap-4">
		<!-- Max Score -->
		<div class="flex flex-col gap-2 text-start">
			<label for="ass_max" class="text-sm font-medium text-foreground">
				{m.assessment_label_max()}
			</label>
			<Input 
				id="ass_max"
				type="number" 
				bind:value={maxScore}
				min="1"
				max="1000"
			/>
		</div>

		<!-- Weight -->
		<div class="flex flex-col gap-2 text-start">
			<label for="ass_weight" class="text-sm font-medium text-foreground">
				{m.assessment_label_weight()}
			</label>
			<Input 
				id="ass_weight"
				type="number" 
				bind:value={weight}
				min="1"
				max="100"
			/>
		</div>
	</div>

	<!-- Form Actions -->
	<div class="flex gap-2 justify-end mt-auto pt-6">
		<Button 
			type="button" 
			variant="ghost" 
			onclick={onCancel}
			disabled={isSubmitting}
		>
			{m.assessment_button_cancel()}
		</Button>
		<Button 
			type="submit" 
			variant="lime" 
			class="font-semibold"
			disabled={isSubmitting}
		>
			{isSubmitting ? m.assessment_toast_saving() : m.assessment_button_save()}
		</Button>
	</div>
</form>
