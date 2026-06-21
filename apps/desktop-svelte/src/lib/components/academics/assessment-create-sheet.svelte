<!--
@component AssessmentCreateSheet
A slide-over Sheet component for creating a new class assessment (exam, quiz, homework).
Uses the shared @orbis/ui-svelte/sheet component.
-->
<script lang="ts">
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { 
  Sheet, SheetContent, SheetHeader, SheetTitle 
} from "@orbis/ui-svelte/sheet";
import { Award } from "lucide-svelte";
import { toast } from "svelte-sonner";
import { getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";

let { 
  open = $bindable(false), 
  classId,
  subjectId,
  cycleId,
  onSuccess 
}: { 
  open: boolean; 
  classId: string;
  subjectId: string;
  cycleId: string;
  onSuccess: () => void;
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
  isSubmitting = true;
  try {
    const request = {
      class_id: classId,
      subject_id: subjectId || undefined,
      term_id: cycleId || undefined,
      name: name || nameAr,
      name_ar: nameAr || name,
      max_score: maxScore ? Number(maxScore) : 100,
      weight: weight ? Number(weight) : 15,
      type: assessmentType,
    };
    await commands.assessments.create(request);
    toast.success(m.assessment_toast_success());
    open = false;
    
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

<Sheet bind:open>
	<SheetContent class="w-full sm:max-w-md bg-card border-l border-border/80 flex flex-col h-full overflow-hidden text-xs">
		<SheetHeader class="pb-4 border-b border-border shrink-0 select-none text-start">
			<SheetTitle class="text-sm font-bold text-foreground uppercase tracking-wider flex items-center gap-2">
				<Award size={14} class="text-primary" />
				<span>{m.assessment_sheet_title()}</span>
			</SheetTitle>
			<p class="text-[10px] text-muted-foreground mt-0.5">
				{m.assessment_sheet_subtitle()}
			</p>
		</SheetHeader>

		<!-- Scrollable Form Container -->
		<form onsubmit={handleSubmit} class="flex-1 overflow-y-auto py-4 flex flex-col gap-4 scrollbar-hide">
			<!-- Name (Arabic) -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="ass_name_ar" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{m.assessment_label_name_ar()}
				</label>
				<Input 
					id="ass_name_ar"
					type="text" 
					bind:value={nameAr}
					placeholder={m.assessment_placeholder_name_ar()}
					class="input-sunken text-xs py-1.5"
					required
				/>
			</div>

			<!-- Name (English) -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="ass_name_en" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{m.assessment_label_name_en()}
				</label>
				<Input 
					id="ass_name_en"
					type="text" 
					bind:value={name}
					placeholder="e.g., Midterm Mathematics Exam"
					class="input-sunken text-xs py-1.5"
				/>
			</div>

			<!-- Assessment Type -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="ass_type" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{m.assessment_label_type()}
				</label>
				<select 
					id="ass_type"
					bind:value={assessmentType}
					class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-ring outline-none"
				>
					<option value="exam">{m.assessment_option_exam()}</option>
					<option value="quiz">{m.assessment_option_quiz()}</option>
					<option value="homework">{m.assessment_option_homework()}</option>
					<option value="classwork">{m.assessment_option_classwork()}</option>
				</select>
			</div>

			<!-- Two columns for scores -->
			<div class="grid grid-cols-2 gap-3">
				<!-- Max Score -->
				<div class="flex flex-col gap-1.5 text-start">
					<label for="ass_max" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
						{m.assessment_label_max()}
					</label>
					<Input 
						id="ass_max"
						type="number" 
						bind:value={maxScore}
						min="1"
						max="1000"
						class="input-sunken text-xs py-1.5"
					/>
				</div>

				<!-- Weight -->
				<div class="flex flex-col gap-1.5 text-start">
					<label for="ass_weight" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
						{m.assessment_label_weight()}
					</label>
					<Input 
						id="ass_weight"
						type="number" 
						bind:value={weight}
						min="1"
						max="100"
						class="input-sunken text-xs py-1.5"
					/>
				</div>
			</div>

			<!-- Form Actions -->
			<div class="flex gap-2 justify-end mt-6 pt-4 border-t border-border shrink-0">
				<Button 
					type="button" 
					variant="ghost" 
					class="text-xs h-9 px-4" 
					onclick={() => open = false}
					disabled={isSubmitting}
				>
					{m.assessment_button_cancel()}
				</Button>
				<Button 
					type="submit" 
					variant="lime" 
					class="text-xs h-9 px-4 font-semibold"
					disabled={isSubmitting}
				>
					{isSubmitting ? m.assessment_toast_saving() : m.assessment_button_save()}
				</Button>
			</div>
		</form>
	</SheetContent>
</Sheet>
