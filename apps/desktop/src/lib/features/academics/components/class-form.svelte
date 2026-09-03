<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { FormField } from "@orbis/ui";
import { SheetHeader, SheetTitle } from "@orbis/ui/sheet";
import InputColor from "$lib/shared/components/input-color.svelte";
import { toast } from "@orbis/ui";
import { classFormInputSchema } from "@orbis/schemas";
import { FormController } from "$lib/shared/forms/form-controller.svelte";

let {
  orgId,
  onSuccess,
  onCancel,
}: {
  orgId: string;
  onSuccess: () => void;
  onCancel: () => void;
} = $props();

// Form state
const form = new FormController({
	initialValues: {
		name: "",
		name_ar: "",
		code: "",
		grade_level: "",
		capacity: 30,
		color: "",
	},
	schema: classFormInputSchema,
});

function fieldError(field: string): string | undefined {
	return form.errors[field]?.[0];
}

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
	const submitted = await form.submit(async (values) => {
		await commands.academic.createClass({
			...values,
			name: values.name || values.name_ar,
			name_ar: values.name_ar || values.name,
			code: values.code || undefined,
			grade_level: values.grade_level || undefined,
			color: values.color || undefined,
		} as any);
		toast.success(m.academics_class_toast_success());
	});
	if (submitted) {
		form.reset();
		onSuccess();
	} else if (form.formError) {
		toast.error(form.formError);
	} else if (Object.keys(form.errors).length > 0) {
		toast.error(m.academics_class_toast_error_name());
  }
}
</script>

<form onsubmit={handleSubmit} class="flex flex-col h-full overflow-hidden gap-0">
	<SheetHeader class="text-start select-none px-6 py-4 border-b border-border/40 shrink-0">
		<SheetTitle class="text-xl font-bold text-foreground">
			{m.academics_class_sheet_title()}
		</SheetTitle>
	</SheetHeader>

	<div class="flex-1 overflow-y-auto p-6 flex flex-col gap-5 scrollbar-hide">
		<!-- Name (Arabic) -->
		<FormField id="name_ar" label={m.academics_class_label_name_ar()} error={fieldError("name_ar")} required>
			{#snippet children()}
			<InputColor
				placeholder={m.academics_class_placeholder_name_ar()}
				bind:value={form.values.name_ar}
				bind:color={form.values.color}
				required
			/>
			{/snippet}
		</FormField>

		<!-- Name (English) -->
		<FormField id="name_en" label={m.academics_class_label_name_en()} error={fieldError("name")}>
			{#snippet children()}
			<Input 
				id="name_en"
				type="text" 
				bind:value={form.values.name}
				onblur={() => form.markTouched("name")}
				aria-invalid={Boolean(fieldError("name"))}
				placeholder={m.common_class_placeholder_name_en()}
			/>
			{/snippet}
		</FormField>

		<!-- Two Columns Row -->
		<div class="grid grid-cols-2 gap-4">
			<!-- Class Code -->
			<FormField id="code" label={m.academics_class_label_code()} error={fieldError("code")}>
				{#snippet children()}
				<Input 
					id="code"
					type="text" 
					bind:value={form.values.code}
					onblur={() => form.markTouched("code")}
					placeholder={m.academics_class_placeholder_code()}
				/>
				{/snippet}
			</FormField>

			<!-- Grade Level -->
			<FormField id="grade" label={m.common_class_label_grade()} error={fieldError("grade_level")}>
				{#snippet children()}
				<Input 
					id="grade"
					type="text" 
					bind:value={form.values.grade_level}
					onblur={() => form.markTouched("grade_level")}
					placeholder={m.academics_class_placeholder_grade()}
				/>
				{/snippet}
			</FormField>
		</div>

		<!-- Capacity -->
		<FormField id="capacity" label={m.academics_class_label_capacity()} error={fieldError("capacity")} required>
			{#snippet children()}
			<Input 
				id="capacity"
				type="number" 
				bind:value={form.values.capacity}
				onblur={() => form.markTouched("capacity")}
				min="1"
				max="100"
			/>
			{/snippet}
		</FormField>
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
			{form.isSubmitting ? m.common_state_saving() : m.academics_class_button_save()}
		</Button>
	</div>
</form>
