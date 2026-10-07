<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { api } from "$lib/api";
import { createAssessmentInputSchema } from "@orbis/schemas";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { FormField } from "@orbis/ui";
import { SheetHeader, SheetTitle } from "@orbis/ui/sheet";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { AdjustmentsHorizontal } from "@orbis/icons";
import { toast } from "@orbis/ui";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
import { BilingualNameFields } from "@orbis/ui/bilingual-name-fields";
import { suggestLocalTranslation } from "$lib/shared/translation/local-engine";

let {
  classId,
  subjectId,
  periodId,
  defaultMonth,
  onSuccess,
  onCancel,
}: {
  classId: string;
  subjectId: string;
  periodId: string;
  defaultMonth?: number;
  onSuccess: () => void;
  onCancel: () => void;
} = $props();

// Form state
const form = new FormController({
  initialValues: {
    title: "",
    title_ar: "",
    max_score: 100,
    weight: 15,
    item_type: "exam" as const,
  },
  schema: createAssessmentInputSchema,
  normalize: (values) => ({
    class_id: classId,
    subject_id: subjectId,
    term_id: periodId,
    month: defaultMonth,
    ...values,
  }),
});

function fieldError(field: string): string | undefined {
  return form.errors[field]?.[0];
}

const typeLabels: Record<"exam" | "quiz" | "homework" | "classwork", string> =
  $derived({
    exam: m.common_assessment_option_exam(),
    quiz: m.common_assessment_option_quiz(),
    homework: m.common_assessments_type_homework(),
    classwork: m.common_assessment_option_classwork(),
  });

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  const submitted = await form.submit(async (payload) => {
    const title = payload.title_ar || payload.title || "Assessment";
    await api.createAssessmentScheme({
      name: title,
      description: payload.title || payload.title_ar,
      max_score: Number(payload.max_score) || 100,
      formula: {
        item_type: payload.item_type,
        weight: Number(payload.weight) || 15,
        class_id: classId,
        subject_id: subjectId,
        period_id: periodId,
        month: defaultMonth ?? null,
      },
    });
    toast.success(m.common_assessment_toast_success());
  });
  if (submitted) {
    form.reset();
    onSuccess();
  } else if (form.formError) {
    toast.error(form.formError);
  } else if (Object.keys(form.errors).length > 0) {
    toast.error(m.common_assessment_toast_error_name());
  }
}
</script>

<form onsubmit={handleSubmit} class="flex flex-col h-full overflow-hidden gap-0">
	<SheetHeader class="text-start select-none px-6 py-4 border-b border-border/40 shrink-0">
		<SheetTitle class="text-xl font-bold text-foreground flex items-center gap-2">
			<AdjustmentsHorizontal size={18} class="text-primary" />
			<span>{m.common_assessment_sheet_title()}</span>
		</SheetTitle>
	</SheetHeader>

	<div class="flex-1 overflow-y-auto p-6 flex flex-col gap-5 scrollbar-hide">
		<BilingualNameFields
			arId="ass_name_ar"
			enId="ass_name_en"
			arLabel={m.common_assessment_label_name_ar()}
			enLabel={m.common_assessment_label_name_en()}
			bind:arValue={form.values.title_ar}
			bind:enValue={form.values.title}
			arPlaceholder={m.common_assessment_placeholder_name_ar()}
			enPlaceholder={m.common_assessment_placeholder_name_en()}
			arError={fieldError("title_ar")}
			enError={fieldError("title")}
			onArBlur={() => form.markTouched("title_ar")}
			onEnBlur={() => form.markTouched("title")}
			kind="label"
			requestSuggestion={suggestLocalTranslation}
			arRequired
			disabled={form.isSubmitting}
			loadingMessage={m.bilingual_entry_loading()}
			failureMessage={m.bilingual_entry_failure()}
			generatedMessage={m.bilingual_entry_generated()}
		/>

		<!-- Assessment Type -->
		<FormField id="ass_type" label={m.common_assessment_label_type()} error={fieldError("item_type")} required>
			{#snippet children()}
			<Select type="single" value={form.values.item_type} onValueChange={(v: string | undefined) => (form.values.item_type = (v as typeof form.values.item_type) ?? "exam")}>
				<SelectTrigger class="h-9 text-sm w-full">
					{typeLabels[form.values.item_type] || form.values.item_type}
				</SelectTrigger>
				<SelectContent>
					<SelectItem value="exam">{m.common_assessment_option_exam()}</SelectItem>
					<SelectItem value="quiz">{m.common_assessment_option_quiz()}</SelectItem>
					<SelectItem value="homework">{m.common_assessments_type_homework()}</SelectItem>
					<SelectItem value="classwork">{m.common_assessment_option_classwork()}</SelectItem>
				</SelectContent>
			</Select>
			{/snippet}
		</FormField>

		<!-- Two columns for scores -->
		<div class="grid grid-cols-2 gap-4">
			<!-- Max Score -->
			<FormField id="ass_max" label={m.common_exams_subjects_score()} error={fieldError("max_score")} required>
				{#snippet children()}
				<Input 
					id="ass_max"
					type="number" 
					bind:value={form.values.max_score}
					onblur={() => form.markTouched("max_score")}
					min="1"
					max="1000"
				/>
				{/snippet}
			</FormField>

			<!-- Weight -->
			<FormField id="ass_weight" label={m.common_assessment_label_weight()} error={fieldError("weight")} required>
				{#snippet children()}
				<Input 
					id="ass_weight"
					type="number" 
					bind:value={form.values.weight}
					onblur={() => form.markTouched("weight")}
					min="1"
					max="100"
				/>
				{/snippet}
			</FormField>
		</div>
	</div>

	<!-- Form Actions -->
	<div class="px-6 py-3 border-t border-border/40 flex gap-2 justify-end shrink-0 bg-background/50">
		<Button 
			type="button" 
			variant="ghost" 
			onclick={onCancel}
			disabled={form.isSubmitting}
		>
			{m.common_action_cancel()}
		</Button>
		<Button 
			type="submit" 
			variant="lime" 
			class="font-semibold"
			disabled={form.isSubmitting}
		>
			{form.isSubmitting ? m.common_state_saving() : m.common_action_save()}
		</Button>
	</div>
</form>
