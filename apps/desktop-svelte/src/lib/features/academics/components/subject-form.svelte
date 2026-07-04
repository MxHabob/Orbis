<script lang="ts">
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import InputColor from "$lib/shared/components/input-color.svelte";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";

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
let isSubmitting = $state(false);
let name = $state("");
let nameAr = $state("");
let code = $state("");
let creditHours = $state<number>(3);
let color = $state("");

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  if (!nameAr && !name) {
    toast.error(m.subject_toast_error_name());
    return;
  }
  isSubmitting = true;
  try {
    const request = {
      name: name || nameAr,
      name_ar: nameAr || name,
      code: code || undefined,
      credit_hours: creditHours ? Number(creditHours) : 3,
      color: color || undefined,
    };
    await commands.subjects.create(orgId, request);
    toast.success(m.subject_toast_success());

    // Reset fields
    name = "";
    nameAr = "";
    code = "";
    creditHours = 3;
    color = "";

    onSuccess();
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(m.subject_toast_error_submit({ error: errorMsg }));
  } finally {
    isSubmitting = false;
  }
}
</script>

<form onsubmit={handleSubmit} class="flex-1 overflow-y-auto py-6 flex flex-col gap-6 scrollbar-hide">
	<!-- Name (Arabic) -->
	<div class="flex flex-col gap-2 text-start">
		<label for="sub_name_ar" class="text-sm font-medium text-foreground">
			{m.subject_label_name_ar()}
		</label>
		<InputColor
			placeholder={m.subject_placeholder_name_ar()}
			bind:value={nameAr}
			bind:color={color}
			required
		/>
	</div>

	<!-- Name (English) -->
	<div class="flex flex-col gap-2 text-start">
		<label for="sub_name_en" class="text-sm font-medium text-foreground">
			{m.subject_label_name_en()}
		</label>
		<Input 
			id="sub_name_en"
			type="text" 
			bind:value={name}
			placeholder="e.g., Mathematics"
		/>
	</div>

	<!-- Subject Code -->
	<div class="flex flex-col gap-2 text-start">
		<label for="sub_code" class="text-sm font-medium text-foreground">
			{m.subject_label_code()}
		</label>
		<Input 
			id="sub_code"
			type="text" 
			bind:value={code}
			placeholder="e.g., MATH-101"
		/>
	</div>

	<!-- Credit Hours -->
	<div class="flex flex-col gap-2 text-start">
		<label for="sub_weight" class="text-sm font-medium text-foreground">
			{m.subject_label_periods()}
		</label>
		<Input 
			id="sub_weight"
			type="number" 
			bind:value={creditHours}
			min="1"
			max="20"
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
			{m.subject_button_cancel()}
		</Button>
		<Button 
			type="submit" 
			variant="lime" 
			class="font-semibold"
			disabled={isSubmitting}
		>
			{isSubmitting ? m.subject_toast_saving() : m.subject_button_save()}
		</Button>
	</div>
</form>
