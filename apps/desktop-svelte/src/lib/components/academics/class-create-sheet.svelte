<!--
@component ClassCreateSheet
A slide-over Sheet component for creating a new academic class.
Uses the shared @orbis/ui-svelte/sheet component.
-->
<script lang="ts">
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
} from "@orbis/ui-svelte/sheet";
import { toast } from "svelte-sonner";
import { getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";

let {
  open = $bindable(false),
  orgId,
  onSuccess,
}: {
  open: boolean;
  orgId: string;
  onSuccess: () => void;
} = $props();

// Form state
let isSubmitting = $state(false);
let name = $state("");
let nameAr = $state("");
let code = $state("");
let gradeLevel = $state("");
let capacity = $state<number>(30);

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
    };
    await commands.classes.create(orgId, request);
    toast.success(m.class_toast_success());
    open = false;

    // Reset fields
    name = "";
    nameAr = "";
    code = "";
    gradeLevel = "";
    capacity = 30;

    onSuccess();
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(m.class_toast_error_submit({ error: errorMsg }));
  } finally {
    isSubmitting = false;
  }
}
</script>

<Sheet bind:open>
	<SheetContent class="w-full sm:max-w-md bg-card border-l border-border/80 flex flex-col h-full overflow-hidden text-xs">
		<SheetHeader class="pb-4 border-b border-border shrink-0 select-none text-start">
			<SheetTitle class="text-sm font-bold text-foreground uppercase tracking-wider">
				{m.class_sheet_title()}
			</SheetTitle>
			<p class="text-[10px] text-muted-foreground mt-0.5">
				{m.class_sheet_subtitle()}
			</p>
		</SheetHeader>

		<!-- Scrollable Form Container -->
		<form onsubmit={handleSubmit} class="flex-1 overflow-y-auto py-4 flex flex-col gap-4 scrollbar-hide">
			<!-- Name (Arabic) -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="name_ar" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{m.class_label_name_ar()}
				</label>
				<Input 
					id="name_ar"
					type="text" 
					bind:value={nameAr}
					placeholder={m.class_placeholder_name_ar()}
					class="input-sunken text-xs py-1.5"
					required
				/>
			</div>

			<!-- Name (English) -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="name_en" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{m.class_label_name_en()}
				</label>
				<Input 
					id="name_en"
					type="text" 
					bind:value={name}
					placeholder="e.g., Grade 1 - Group A"
					class="input-sunken text-xs py-1.5"
				/>
			</div>

			<!-- Two Columns Row -->
			<div class="grid grid-cols-2 gap-3">
				<!-- Class Code -->
				<div class="flex flex-col gap-1.5 text-start">
					<label for="code" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
						{m.class_label_code()}
					</label>
					<Input 
						id="code"
						type="text" 
						bind:value={code}
						placeholder="e.g., G1-A"
						class="input-sunken text-xs py-1.5"
					/>
				</div>

				<!-- Grade Level -->
				<div class="flex flex-col gap-1.5 text-start">
					<label for="grade" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
						{m.class_label_grade()}
					</label>
					<Input 
						id="grade"
						type="text" 
						bind:value={gradeLevel}
						placeholder="e.g., 1"
						class="input-sunken text-xs py-1.5"
					/>
				</div>
			</div>

			<!-- Capacity -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="capacity" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{m.class_label_capacity()}
				</label>
				<Input 
					id="capacity"
					type="number" 
					bind:value={capacity}
					min="1"
					max="100"
					class="input-sunken text-xs py-1.5"
				/>
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
					{m.class_button_cancel()}
				</Button>
				<Button 
					type="submit" 
					variant="lime" 
					class="text-xs h-9 px-4 font-semibold"
					disabled={isSubmitting}
				>
					{isSubmitting ? m.class_toast_saving() : m.class_button_save()}
				</Button>
			</div>
		</form>
	</SheetContent>
</Sheet>
