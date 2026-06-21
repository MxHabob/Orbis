<!--
@component FeeAssignSheet
A slide-over Sheet component for assigning a fee to a student.
Uses the shared @orbis/ui-svelte/sheet component.
-->
<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { 
  Sheet, SheetContent, SheetHeader, SheetTitle 
} from "@orbis/ui-svelte/sheet";
import { Wallet } from "lucide-svelte";
import { toast } from "svelte-sonner";
import { getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";

let { 
  open = $bindable(false), 
  orgId, 
  onSuccess 
}: { 
  open: boolean; 
  orgId: string; 
  onSuccess: () => void;
} = $props();

// State
let isSubmitting = $state(false);
let students = $state<any[]>([]);
let ledgers = $state<any[]>([]);
let isLoadingDropdowns = $state(false);

// Form state
let selectedEnrollmentId = $state("");
let selectedLedgerId = $state("");
let amount = $state<number | null>(null);
let description = $state("");
let dueDate = $state("");

async function loadDropdownData() {
  if (!orgId) return;
  isLoadingDropdowns = true;
  try {
    const [studentList, ledgerList] = await Promise.all([
      commands.enrollments.getAll(orgId),
      commands.finance.getLedgerAccounts(orgId),
    ]);
    students = studentList;
    ledgers = ledgerList;
    
    if (studentList.length > 0) selectedEnrollmentId = String(studentList[0].id);
    if (ledgerList.length > 0) selectedLedgerId = String(ledgerList[0].id);
  } catch (e) {
    console.error("Error loading dropdown data for fee-assign-sheet:", e);
  } finally {
    isLoadingDropdowns = false;
  }
}

// Reload dropdowns when sheet is opened
$effect(() => {
  if (open && orgId) {
    loadDropdownData();
  }
});

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  if (!selectedEnrollmentId) {
    toast.error(m.fee_toast_error_student());
    return;
  }
  if (!selectedLedgerId) {
    toast.error(m.fee_toast_error_ledger());
    return;
  }
  if (!amount || amount <= 0) {
    toast.error(m.fee_toast_error_amount());
    return;
  }

  isSubmitting = true;
  try {
    const request = {
      enrollment_id: selectedEnrollmentId,
      ledger_account_id: selectedLedgerId,
      amount: Number(amount),
      description: description || undefined,
      due_date: dueDate || undefined,
    };
    await commands.finance.assignStudentFee(orgId, request);
    toast.success(m.fee_toast_success());
    open = false;
    
    // Reset form
    amount = null;
    description = "";
    dueDate = "";

    onSuccess();
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(m.fee_toast_error_submit({ error: errorMsg }));
  } finally {
    isSubmitting = false;
  }
}
</script>

<Sheet bind:open>
	<SheetContent class="w-full sm:max-w-md bg-card border-l border-border/80 flex flex-col h-full overflow-hidden text-xs">
		<SheetHeader class="pb-4 border-b border-border shrink-0 select-none text-start">
			<SheetTitle class="text-sm font-bold text-foreground uppercase tracking-wider flex items-center gap-2">
				<Wallet size={14} class="text-primary" />
				<span>{m.fee_sheet_title()}</span>
			</SheetTitle>
			<p class="text-[10px] text-muted-foreground mt-0.5">
				{m.fee_sheet_subtitle()}
			</p>
		</SheetHeader>

		<!-- Scrollable Form Container -->
		<form onsubmit={handleSubmit} class="flex-1 overflow-y-auto py-4 flex flex-col gap-4 scrollbar-hide">
			<!-- Student Select -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="fee_student" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{m.fee_label_student()}
				</label>
				<select 
					id="fee_student"
					bind:value={selectedEnrollmentId}
					class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-ring outline-none"
					disabled={isLoadingDropdowns || isSubmitting}
				>
					{#if isLoadingDropdowns}
						<option>{m.fee_dropdown_loading_students()}</option>
					{:else}
						{#each students as s (s.id)}
							<option value={String(s.id)}>{s.full_name || s.student_number || s.id}</option>
						{/each}
					{/if}
				</select>
			</div>

			<!-- Ledger Account Select -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="fee_ledger" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{m.fee_label_ledger()}
				</label>
				<select 
					id="fee_ledger"
					bind:value={selectedLedgerId}
					class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-ring outline-none"
					disabled={isLoadingDropdowns || isSubmitting}
				>
					{#if isLoadingDropdowns}
						<option>{m.fee_dropdown_loading_ledgers()}</option>
					{:else}
						{#each ledgers as l (l.id)}
							<option value={String(l.id)}>{l.name_ar || l.name || l.code}</option>
						{/each}
					{/if}
				</select>
			</div>

			<!-- Fee Amount -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="fee_amount" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{m.fee_label_amount()}
				</label>
				<Input 
					id="fee_amount"
					type="number" 
					bind:value={amount}
					min="1"
					step="any"
					placeholder="0.00"
					class="input-sunken text-xs py-1.5"
					required
					disabled={isSubmitting}
				/>
			</div>

			<!-- Due Date -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="fee_due" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{m.fee_label_due()}
				</label>
				<Input 
					id="fee_due"
					type="date" 
					bind:value={dueDate}
					class="input-sunken text-xs py-1.5"
					disabled={isSubmitting}
				/>
			</div>

			<!-- Description -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="fee_desc" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{m.fee_label_desc()}
				</label>
				<Input 
					id="fee_desc"
					type="text" 
					bind:value={description}
					placeholder={m.fee_placeholder_desc()}
					class="input-sunken text-xs py-1.5"
					disabled={isSubmitting}
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
					{m.fee_button_cancel()}
				</Button>
				<Button 
					type="submit" 
					variant="lime" 
					class="text-xs h-9 px-4 font-semibold"
					disabled={isSubmitting || isLoadingDropdowns}
				>
					{isSubmitting ? m.fee_toast_assigning() : m.fee_button_assign()}
				</Button>
			</div>
		</form>
	</SheetContent>
</Sheet>
