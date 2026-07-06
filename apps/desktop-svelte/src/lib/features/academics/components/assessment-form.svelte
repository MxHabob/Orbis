<script lang="ts">
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { SheetHeader, SheetTitle } from "@orbis/ui-svelte/sheet";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { Award } from "lucide-svelte";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";

let {
  classId,
  subjectId,
  cycleId,
  defaultMonth,
  onSuccess,
  onCancel,
}: {
  classId: string;
  subjectId: string;
  cycleId: string;
  defaultMonth?: number;
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

const typeLabels: Record<"exam" | "quiz" | "homework" | "classwork", string> = $derived({
  exam: m.assessment_option_exam(),
  quiz: m.assessment_option_quiz(),
  homework: m.assessment_option_homework(),
  classwork: m.assessment_option_classwork(),
});

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  if (!nameAr && !name) {
    toast.error(m.assessment_toast_error_name());
    return;
  }
  if (!classId || !subjectId || !cycleId) {
    toast.error(
      "Please select a class, subject, and cycle before creating an assessment.",
    );
    return;
  }
  isSubmitting = true;
  try {
    const request = {
      class_id: classId,
      subject_id: subjectId,
      term_id: cycleId,
      month: defaultMonth,
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

<form onsubmit={handleSubmit} class="flex flex-col h-full overflow-hidden gap-0">
	<SheetHeader class="text-start select-none px-6 py-4 border-b border-border/40 shrink-0">
		<SheetTitle class="text-xl font-bold text-foreground flex items-center gap-2">
			<Award size={18} class="text-primary" />
			<span>{m.assessment_sheet_title()}</span>
		</SheetTitle>
	</SheetHeader>

	<div class="flex-1 overflow-y-auto p-6 flex flex-col gap-6 scrollbar-hide">
		<!-- Name (Arabic) -->
		<div class="flex flex-col gap-2 text-start">
			<Label for="ass_name_ar" class="text-sm font-medium text-foreground">
				{m.assessment_label_name_ar()}
			</Label>
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
			<Label for="ass_name_en" class="text-sm font-medium text-foreground">
				{m.assessment_label_name_en()}
			</Label>
			<Input 
				id="ass_name_en"
				type="text" 
				bind:value={name}
				placeholder={m.assessment_placeholder_name_en()}
			/>
		</div>

		<!-- Assessment Type -->
		<div class="flex flex-col gap-2 text-start">
			<Label for="ass_type" class="text-sm font-medium text-foreground">
				{m.assessment_label_type()}
			</Label>
			<Select type="single" value={assessmentType} onValueChange={(v) => (assessmentType = (v as any) ?? "exam")}>
				<SelectTrigger class="h-9 text-sm w-full">
					{typeLabels[assessmentType] || assessmentType}
				</SelectTrigger>
				<SelectContent>
					<SelectItem value="exam">{m.assessment_option_exam()}</SelectItem>
					<SelectItem value="quiz">{m.assessment_option_quiz()}</SelectItem>
					<SelectItem value="homework">{m.assessment_option_homework()}</SelectItem>
					<SelectItem value="classwork">{m.assessment_option_classwork()}</SelectItem>
				</SelectContent>
			</Select>
		</div>

		<!-- Two columns for scores -->
		<div class="grid grid-cols-2 gap-4">
			<!-- Max Score -->
			<div class="flex flex-col gap-2 text-start">
				<Label for="ass_max" class="text-sm font-medium text-foreground">
					{m.assessment_label_max()}
				</Label>
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
				<Label for="ass_weight" class="text-sm font-medium text-foreground">
					{m.assessment_label_weight()}
				</Label>
				<Input 
					id="ass_weight"
					type="number" 
					bind:value={weight}
					min="1"
					max="100"
				/>
			</div>
		</div>
	</div>

	<!-- Form Actions -->
	<div class="px-6 py-4 border-t border-border/40 flex gap-2 justify-end shrink-0 bg-background/50">
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
