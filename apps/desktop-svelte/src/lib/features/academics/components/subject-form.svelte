<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { commands } from "$lib/core/ipc";
import { createSubjectInputSchema } from "@orbis/schemas";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { SheetHeader, SheetTitle } from "@orbis/ui-svelte/sheet";
import InputColor from "$lib/shared/components/input-color.svelte";
import { toast } from "svelte-sonner";
type SubjectDraft = {
	name: string;
	name_ar: string;
	code: string;
	color: string | null;
};

const form = new FormController<SubjectDraft>({
	initialValues: { name: "", name_ar: "", code: "", color: null },
	schema: createSubjectInputSchema,
});

function fieldError(field: string): string | undefined {
	return form.errors[field]?.[0];
}
let {
	onSuccess,
	onCancel,
}: {
	orgId: string;
	onSuccess: () => void;
	onCancel: () => void;
} = $props();

async function handleSubmit(e: SubmitEvent) {
	e.preventDefault();
	const submitted = await form.submit(async (draft) => {
		await commands.academic.createSubject({
			...draft,
			name: draft.name || draft.name_ar || "",
		});
	});
	if (submitted) {
		form.reset();
		onSuccess();
	} else if (form.formError) {
		toast.error(form.formError);
	} else if (Object.keys(form.errors).length > 0) {
		toast.error(m.common_subject_toast_error_name());
	}
}
</script>

<form onsubmit={handleSubmit} class="flex flex-col h-full overflow-hidden gap-0">
	<SheetHeader class="text-start select-none px-6 py-4 border-b border-border/40 shrink-0">
		<SheetTitle class="text-xl font-bold text-foreground">
			{m.common_subject_sheet_title()}
		</SheetTitle>
	</SheetHeader>

	<div class="flex-1 overflow-y-auto p-6 flex flex-col gap-5 scrollbar-hide">
		<!-- Name (Arabic) -->
		<div class="flex flex-col gap-2 text-start">
			<label for="sub_name_ar" class="text-sm font-medium text-foreground">
				{m.common_subject_label_name_ar()}
			</label>
			<InputColor
				placeholder={m.common_subject_placeholder_name_ar()}
				bind:value={form.values.name_ar}
				bind:color={form.values.color}
				onblur={() => form.markTouched("name_ar")}
				required
			/>
			{#if fieldError("name_ar")}
				<p id="sub_name_ar_error" class="text-xs text-destructive">{fieldError("name_ar")}</p>
			{/if}
		</div>

		<!-- Name (English) -->
		<div class="flex flex-col gap-2 text-start">
			<label for="sub_name_en" class="text-sm font-medium text-foreground">
				{m.common_subject_label_name_en()}
			</label>
			<Input 
				id="sub_name_en"
				type="text" 
				bind:value={form.values.name}
				onblur={() => form.markTouched("name")}
				aria-invalid={Boolean(fieldError("name"))}
				aria-describedby={fieldError("name") ? "sub_name_error" : undefined}
				placeholder={m.common_subject_placeholder_name_en()}
			/>
			{#if fieldError("name")}
				<p id="sub_name_error" class="text-xs text-destructive">{fieldError("name")}</p>
			{/if}
		</div>

		<!-- Subject Code -->
		<div class="flex flex-col gap-2 text-start">
			<label for="sub_code" class="text-sm font-medium text-foreground">
				{m.common_subject_label_code()}
			</label>
			<Input 
				id="sub_code"
				type="text" 
				bind:value={form.values.code}
				onblur={() => form.markTouched("code")}
				aria-invalid={Boolean(fieldError("code"))}
				aria-describedby={fieldError("code") ? "sub_code_error" : undefined}
				placeholder={m.common_subject_placeholder_code()}
			/>
			{#if fieldError("code")}
				<p id="sub_code_error" class="text-xs text-destructive">{fieldError("code")}</p>
			{/if}
		</div>

		<!-- Credit Hours -->
		<div class="flex flex-col gap-2 text-start">
			<label for="sub_weight" class="text-sm font-medium text-foreground">
				{m.common_subject_label_periods()}
			</label>
			<Input 
				id="sub_weight"
				type="number" 
				value="3"
				min="1"
				max="20"
			/>
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
