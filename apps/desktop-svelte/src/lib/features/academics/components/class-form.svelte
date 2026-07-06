<script lang="ts">
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { SheetHeader, SheetTitle } from "@orbis/ui-svelte/sheet";
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
let gradeLevel = $state("");
let capacity = $state<number>(30);
let color = $state("");

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  if (!nameAr && !name) {
    toast.error(m.class_toast_error_name());
    return;
  }
  isSubmitting = true;
  try {
    const request = {
      name: name || nameAr,
      name_ar: nameAr || name,
      code: code || undefined,
      grade_level: gradeLevel || undefined,
      capacity: capacity ? Number(capacity) : 30,
      color: color || undefined,
    };
    await commands.classes.create(orgId, request);
    toast.success(m.class_toast_success());

    // Reset fields
    name = "";
    nameAr = "";
    code = "";
    gradeLevel = "";
    capacity = 30;
    color = "";

    onSuccess();
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(m.class_toast_error_submit({ error: errorMsg }));
  } finally {
    isSubmitting = false;
  }
}
</script>

<form onsubmit={handleSubmit} class="flex flex-col h-full overflow-hidden gap-0">
	<SheetHeader class="text-start select-none px-6 py-4 border-b border-border/40 shrink-0">
		<SheetTitle class="text-xl font-bold text-foreground">
			{m.class_sheet_title()}
		</SheetTitle>
	</SheetHeader>

	<div class="flex-1 overflow-y-auto p-6 flex flex-col gap-6 scrollbar-hide">
		<!-- Name (Arabic) -->
		<div class="flex flex-col gap-2 text-start">
			<label for="name_ar" class="text-sm font-medium text-foreground">
				{m.class_label_name_ar()}
			</label>
			<InputColor
				placeholder={m.class_placeholder_name_ar()}
				bind:value={nameAr}
				bind:color={color}
				required
			/>
		</div>

		<!-- Name (English) -->
		<div class="flex flex-col gap-2 text-start">
			<label for="name_en" class="text-sm font-medium text-foreground">
				{m.class_label_name_en()}
			</label>
			<Input 
				id="name_en"
				type="text" 
				bind:value={name}
				placeholder="e.g., Grade 1 - Group A"
			/>
		</div>

		<!-- Two Columns Row -->
		<div class="grid grid-cols-2 gap-4">
			<!-- Class Code -->
			<div class="flex flex-col gap-2 text-start">
				<label for="code" class="text-sm font-medium text-foreground">
					{m.class_label_code()}
				</label>
				<Input 
					id="code"
					type="text" 
					bind:value={code}
					placeholder="e.g., G1-A"
				/>
			</div>

			<!-- Grade Level -->
			<div class="flex flex-col gap-2 text-start">
				<label for="grade" class="text-sm font-medium text-foreground">
					{m.class_label_grade()}
				</label>
				<Input 
					id="grade"
					type="text" 
					bind:value={gradeLevel}
					placeholder="e.g., 1"
				/>
			</div>
		</div>

		<!-- Capacity -->
		<div class="flex flex-col gap-2 text-start">
			<label for="capacity" class="text-sm font-medium text-foreground">
				{m.class_label_capacity()}
			</label>
			<Input 
				id="capacity"
				type="number" 
				bind:value={capacity}
				min="1"
				max="100"
			/>
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
			{m.class_button_cancel()}
		</Button>
		<Button 
			type="submit" 
			variant="lime" 
			class="font-semibold"
			disabled={isSubmitting}
		>
			{isSubmitting ? m.class_toast_saving() : m.class_button_save()}
		</Button>
	</div>
</form>
