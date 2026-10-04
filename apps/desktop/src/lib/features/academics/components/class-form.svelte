<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { api, type CreateClassRequest, queryKeys } from "$lib/api";
import { academicContext } from "$lib/stores";
import { createPeriodsQuery } from "$lib/features/academics/queries";
import { createQuery } from "@tanstack/svelte-query";
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

const periodsQuery = createPeriodsQuery(() => orgId);
const periods = $derived(periodsQuery.data ?? []);

const unitsQuery = createQuery(() => ({
  queryKey: queryKeys.org.units(orgId),
  queryFn: () => api.listOrgUnits(),
  enabled: !!orgId,
}));
const units = $derived(unitsQuery.data ?? []);

// Form state
const form = new FormController({
  initialValues: {
    unit_id: "",
    period_id: "",
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
    const periodId =
      values.period_id ||
      academicContext.effectivePeriodId ||
      (periods[0]?.id ? String(periods[0].id) : "");
    if (!periodId) {
      throw new Error(m.shared_ui_academic_period_required());
    }
    const unitId =
      values.unit_id ||
      (units[0]?.id ? String(units[0].id) : "");
    if (!unitId) {
      throw new Error(m.shared_ui_organizational_unit_required());
    }

    const request: CreateClassRequest = {
      unit_id: unitId,
      period_id: periodId,
      name: values.name_ar || values.name,
      level: values.grade_level || "1",
    };
    await api.createClass(request);
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

	<div class="flex-1 overflow-y-auto p-6 flex flex-col gap-5 scrollbar-hide text-start">
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

		<!-- Unit & Period Selection -->
		<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
			{#if units.length > 0}
				<FormField id="unit_select" label={m.common_word_unit()} error={fieldError("unit_id")}>
					{#snippet children()}
					<Select
						type="single"
						value={form.values.unit_id || (units[0]?.id ? String(units[0].id) : "")}
						onValueChange={(v) => (form.values.unit_id = v ?? "")}
					>
						<SelectTrigger class="h-9 text-xs w-full">
							{units.find(u => String(u.id) === (form.values.unit_id || String(units[0]?.id)))?.name_ar || units.find(u => String(u.id) === (form.values.unit_id || String(units[0]?.id)))?.name || m.common_word_unit()}
						</SelectTrigger>
						<SelectContent class="max-h-52">
							{#each units as u}
								<SelectItem value={String(u.id)}>
									{u.name_ar || u.name}
								</SelectItem>
							{/each}
						</SelectContent>
					</Select>
					{/snippet}
				</FormField>
			{/if}

			{#if periods.length > 0}
				<FormField id="period_select" label={m.common_period_form_title_create()} error={fieldError("period_id")}>
					{#snippet children()}
					<Select
						type="single"
						value={form.values.period_id || academicContext.effectivePeriodId || (periods[0]?.id ? String(periods[0].id) : "")}
						onValueChange={(v) => (form.values.period_id = v ?? "")}
					>
						<SelectTrigger class="h-9 text-xs w-full">
							{periods.find(p => String(p.id) === (form.values.period_id || academicContext.effectivePeriodId || String(periods[0]?.id)))?.name_ar || periods.find(p => String(p.id) === (form.values.period_id || academicContext.effectivePeriodId || String(periods[0]?.id)))?.name || m.common_period_form_title_create()}
						</SelectTrigger>
						<SelectContent class="max-h-52">
							{#each periods as p}
								<SelectItem value={String(p.id)}>
									{p.name_ar || p.name}
								</SelectItem>
							{/each}
						</SelectContent>
					</Select>
					{/snippet}
				</FormField>
			{/if}
		</div>

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
