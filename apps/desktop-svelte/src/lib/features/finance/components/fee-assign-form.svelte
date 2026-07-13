<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { toast } from "svelte-sonner";

let {
  orgId,
  onSuccess,
  onCancel,
}: {
  orgId: string;
  onSuccess: () => void;
  onCancel: () => void;
} = $props();

// State
let isSubmitting = $state(false);
let students = $state<any[]>([]);
let templates = $state<any[]>([]);
let isLoadingDropdowns = $state(false);

// Form state
let selectedEnrollmentId = $state("");
let selectedFeeTemplateId = $state("");
let amount = $state<number | null>(null);
let description = $state("");
let dueDate = $state(new Date().toISOString().split("T")[0]);

async function loadDropdownData() {
  if (!orgId) return;
  isLoadingDropdowns = true;
  try {
    const [studentList, templateList] = await Promise.all([
      commands.academic.listEnrollments(orgId),
      commands.finance.listActiveFeeTemplates(orgId),
    ]);
    students = studentList;
    templates = templateList;

    if (studentList.length > 0)
      selectedEnrollmentId = String(studentList[0].id);
    if (templateList.length > 0)
      selectedFeeTemplateId = String(templateList[0].id);
  } catch (e) {
    console.error("Error loading dropdown data for fee-assign-form:", e);
  } finally {
    isLoadingDropdowns = false;
  }
}

// Reload dropdowns when form is mounted
$effect(() => {
  if (orgId) {
    loadDropdownData();
  }
});

// Auto-fill template amount when template changes
$effect(() => {
  if (selectedFeeTemplateId && templates.length > 0) {
    const tpl = templates.find(
      (t) => String(t.id) === String(selectedFeeTemplateId),
    );
    if (tpl) {
      amount =
        tpl.default_amount !== undefined ? Number(tpl.default_amount) : null;
    }
  }
});

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  if (!selectedEnrollmentId) {
    toast.error(m.common_fee_toast_error_student());
    return;
  }
  if (!selectedFeeTemplateId) {
    toast.error(m.common_fee_toast_error_ledger());
    return;
  }
  if (!amount || amount <= 0) {
    toast.error(m.common_fee_toast_error_amount());
    return;
  }

  isSubmitting = true;
  try {
    const request = {
      enrollment_id: selectedEnrollmentId,
      fee_template_id: selectedFeeTemplateId,
      amount: Number(amount),
      notes: description || undefined,
      due_date: dueDate,
    };
    await commands.finance.assignStudentFee(orgId, request);
    toast.success(m.common_fee_toast_success());

    // Reset form
    amount = null;
    description = "";
    dueDate = new Date().toISOString().split("T")[0];

    onSuccess();
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(m.common_fee_toast_error_submit({ error: errorMsg }));
  } finally {
    isSubmitting = false;
  }
}
</script>

<form onsubmit={handleSubmit} class="flex-1 overflow-y-auto py-6 flex flex-col gap-6 scrollbar-hide">
	<!-- Student Select -->
	<div class="flex flex-col gap-2 text-start">
		<label for="fee_student" class="text-sm font-medium text-foreground">
			{m.common_fee_label_student()}
		</label>
		<select 
			id="fee_student"
			bind:value={selectedEnrollmentId}
			class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-sm focus:ring-1 focus:ring-ring outline-none h-9 w-full"
			disabled={isLoadingDropdowns || isSubmitting}
		>
			{#if isLoadingDropdowns}
				<option>{m.common_fee_dropdown_loading_students()}</option>
			{:else}
				{#each students as s (s.id)}
					<option value={String(s.id)}>{s.full_name || s.student_number || s.id}</option>
				{/each}
			{/if}
		</select>
	</div>

	<!-- Fee Template Select -->
	<div class="flex flex-col gap-2 text-start">
		<label for="fee_ledger" class="text-sm font-medium text-foreground">
			{m.common_fee_label_ledger()}
		</label>
		<select 
			id="fee_ledger"
			bind:value={selectedFeeTemplateId}
			class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-sm focus:ring-1 focus:ring-ring outline-none h-9 w-full"
			disabled={isLoadingDropdowns || isSubmitting}
		>
			{#if isLoadingDropdowns}
				<option>{m.common_fee_dropdown_loading_ledgers()}</option>
			{:else}
				{#each templates as t (t.id)}
					<option value={String(t.id)}>{t.name_ar || t.name || t.code}</option>
				{/each}
			{/if}
		</select>
	</div>

	<!-- Fee Amount -->
	<div class="flex flex-col gap-2 text-start">
		<label for="fee_amount" class="text-sm font-medium text-foreground">
			{m.common_fee_label_amount()}
		</label>
		<Input 
			id="fee_amount"
			type="number" 
			bind:value={amount}
			min="1"
			step="any"
			placeholder="0.00"
			required
			disabled={isSubmitting}
		/>
	</div>

	<!-- Due Date -->
	<div class="flex flex-col gap-2 text-start">
		<label for="fee_due" class="text-sm font-medium text-foreground">
			{m.common_fee_label_due()}
		</label>
		<Input 
			id="fee_due"
			type="date" 
			bind:value={dueDate}
			disabled={isSubmitting}
		/>
	</div>

	<!-- Description -->
	<div class="flex flex-col gap-2 text-start">
		<label for="fee_desc" class="text-sm font-medium text-foreground">
			{m.common_fee_label_desc()}
		</label>
		<Input 
			id="fee_desc"
			type="text" 
			bind:value={description}
			placeholder={m.common_fee_placeholder_desc()}
			disabled={isSubmitting}
		/>
	</div>

	<!-- Form Actions -->
	<div class="flex gap-2 justify-end mt-auto pt-6">
		<Button 
			type="button" 
			variant="ghost" 
			onclick={onCancel}
			disabled={isSubmitting}
		>
			{m.common_action_cancel()}
		</Button>
		<Button 
			type="submit" 
			variant="lime" 
			class="font-semibold"
			disabled={isSubmitting || isLoadingDropdowns}
		>
			{isSubmitting ? m.common_fee_toast_assigning() : m.common_fee_button_assign()}
		</Button>
	</div>
</form>
