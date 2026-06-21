<!--
@component PaymentRecordSheet
A slide-over Sheet component for recording a student payment.
Uses the shared @orbis/ui-svelte/sheet component.
-->
<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/services/ipc";
import { authStore } from "$lib/stores";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
} from "@orbis/ui-svelte/sheet";
import { CreditCard } from "lucide-svelte";
import { toast } from "svelte-sonner";
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

// State
let isSubmitting = $state(false);
let students = $state<any[]>([]);
let treasuries = $state<any[]>([]);
let fees = $state<any[]>([]);
let isLoadingDropdowns = $state(false);
let isLoadingFees = $state(false);

// Form state
let selectedEnrollmentId = $state("");
let selectedTreasuryId = $state("");
let selectedFeeId = $state("");
let paymentMethod = $state<"Cash" | "Bank Transfer" | "Card">("Cash");
let currency = $state("YER");
let amount = $state<number | null>(null);
let receiptNumber = $state("");
let referenceNumber = $state("");
let notes = $state("");

async function loadDropdownData() {
  if (!orgId) return;
  isLoadingDropdowns = true;
  try {
    const [studentList, treasuryList] = await Promise.all([
      commands.enrollments.getAll(orgId),
      commands.finance.getTreasuries(orgId),
    ]);
    students = studentList;
    treasuries = treasuryList;

    if (studentList.length > 0) {
      selectedEnrollmentId = String(studentList[0].id);
    }
    if (treasuryList.length > 0) {
      selectedTreasuryId = String(treasuryList[0].id);
      currency = String((treasuryList[0] as any).currency || "YER");
    }
  } catch (e) {
    console.error("Error loading dropdown data for payment-record-sheet:", e);
  } finally {
    isLoadingDropdowns = false;
  }
}

async function loadStudentFees(enrollId: string) {
  if (!enrollId || !orgId) return;
  isLoadingFees = true;
  try {
    const list = await commands.finance.getStudentFees(orgId, enrollId);
    fees = list.filter((f) => f.status !== "paid");
    if (fees.length > 0) {
      selectedFeeId = String(fees[0].id);
      amount = Number(fees[0].balance || fees[0].total_amount || 0);
    } else {
      selectedFeeId = "";
      amount = null;
      toast.info(
        m.attendance_status_unknown() === "غير محدد"
          ? "لا توجد رسوم مستحقة غير مدفوعة لهذا الطالب"
          : "No outstanding unpaid fees for this student",
      );
    }
  } catch (e) {
    console.error("Error loading student fees:", e);
  } finally {
    isLoadingFees = false;
  }
}

// Reload dropdowns when sheet is opened
$effect(() => {
  if (open && orgId) {
    loadDropdownData();
  }
});

// Reactively load student fees when student selection changes
$effect(() => {
  if (selectedEnrollmentId) {
    loadStudentFees(selectedEnrollmentId);
  }
});

// Reactively set amount when fee selection changes
$effect(() => {
  if (selectedFeeId) {
    const selectedFee = fees.find((f) => String(f.id) === selectedFeeId);
    if (selectedFee) {
      amount = Number(selectedFee.balance || selectedFee.total_amount || 0);
    }
  }
});

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  if (!selectedEnrollmentId) {
    toast.error(m.payment_toast_error_student());
    return;
  }
  if (!selectedFeeId) {
    toast.error(m.payment_toast_error_fee());
    return;
  }
  if (!selectedTreasuryId) {
    toast.error(m.payment_toast_error_treasury());
    return;
  }
  if (!amount || amount <= 0) {
    toast.error(m.payment_toast_error_amount());
    return;
  }
  if (!currency || currency.trim().length !== 3) {
    toast.error(m.payment_toast_error_currency());
    return;
  }

  isSubmitting = true;
  try {
    const createdBy = authStore.userId || authStore.user?.id || "unknown";
    const request = {
      fee_id: selectedFeeId,
      amount: Number(amount),
      method:
        paymentMethod === "Cash"
          ? "cash"
          : paymentMethod === "Bank Transfer"
            ? "bank_transfer"
            : "card",
      reference: referenceNumber || undefined,
      notes: notes || undefined,
      treasury_id: selectedTreasuryId,
      currency: currency.trim().toUpperCase(),
    };
    await commands.finance.recordPayment(orgId, createdBy, request);
    toast.success(m.payment_toast_success());
    open = false;

    // Reset form
    amount = null;
    receiptNumber = "";
    referenceNumber = "";
    notes = "";

    onSuccess();
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(m.payment_toast_error_submit({ error: errorMsg }));
  } finally {
    isSubmitting = false;
  }
}
</script>

<Sheet bind:open>
	<SheetContent class="w-full sm:max-w-md bg-card border-l border-border/80 flex flex-col h-full overflow-hidden text-xs">
		<SheetHeader class="pb-4 border-b border-border shrink-0 select-none text-start">
			<SheetTitle class="text-sm font-bold text-foreground uppercase tracking-wider flex items-center gap-2">
				<CreditCard size={14} class="text-primary" />
				<span>{m.payment_sheet_title()}</span>
			</SheetTitle>
			<p class="text-[10px] text-muted-foreground mt-0.5">
				{m.payment_sheet_subtitle()}
			</p>
		</SheetHeader>

		<!-- Scrollable Form Container -->
		<form onsubmit={handleSubmit} class="flex-1 overflow-y-auto py-4 flex flex-col gap-4 scrollbar-hide">
			<!-- Student Select -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="pay_student" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{m.payment_label_student()}
				</label>
				<select 
					id="pay_student"
					bind:value={selectedEnrollmentId}
					class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-ring outline-none"
					disabled={isLoadingDropdowns || isSubmitting}
				>
					{#if isLoadingDropdowns}
						<option>{m.attendance_status_unknown() === "غير محدد" ? "جاري تحميل قائمة الطلاب..." : "Loading students..."}</option>
					{:else}
						{#each students as s (s.id)}
							<option value={String(s.id)}>{s.full_name || s.student_number || s.id}</option>
						{/each}
					{/if}
				</select>
			</div>

			<!-- Student Fees Select -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="pay_fee" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{m.payment_label_fee()}
				</label>
				<select 
					id="pay_fee"
					bind:value={selectedFeeId}
					class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-ring outline-none"
					disabled={isLoadingFees || isSubmitting}
				>
					{#if isLoadingFees}
						<option>{m.attendance_status_unknown() === "غير محدد" ? "جاري تحميل الرسوم المستحقة..." : "Loading fees..."}</option>
					{:else if fees.length === 0}
						<option value="">{m.attendance_status_unknown() === "غير محدد" ? "-- لا توجد رسوم مستحقة لهذا الطالب --" : "-- No outstanding fees for this student --"}</option>
					{:else}
						{#each fees as f (f.id)}
							<option value={String(f.id)}>
								{f.notes || f.id.slice(0, 8)} ({m.attendance_status_unknown() === "غير محدد" ? 'المتبقي: ' : 'Balance: '}{f.balance})
							</option>
						{/each}
					{/if}
				</select>
			</div>

			<!-- Treasury Account Select -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="pay_treasury" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{m.payment_label_treasury()}
				</label>
				<select 
					id="pay_treasury"
					bind:value={selectedTreasuryId}
					class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-ring outline-none"
					disabled={isLoadingDropdowns || isSubmitting}
				>
					{#if isLoadingDropdowns}
						<option>{m.loading()}</option>
					{:else}
						{#each treasuries as t (t.id)}
							<option value={String(t.id)}>{t.name_ar || t.name || t.code} ({t.currency})</option>
						{/each}
					{/if}
				</select>
			</div>

			<!-- Payment Method Select -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="pay_method" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{m.payment_label_method()}
				</label>
				<select 
					id="pay_method"
					bind:value={paymentMethod}
					class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-ring outline-none"
					disabled={isSubmitting}
				>
					<option value="Cash">{m.attendance_status_unknown() === "غير محدد" ? "نقدي (كاش)" : "Cash"}</option>
					<option value="Bank Transfer">{m.attendance_status_unknown() === "غير محدد" ? "تحويل بنكي" : "Bank Transfer"}</option>
					<option value="Card">{m.attendance_status_unknown() === "غير محدد" ? "بطاقة دفع / مدى" : "Card"}</option>
				</select>
			</div>

			<!-- Currency Selection -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="pay_currency" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{m.payment_label_currency()}
				</label>
				<div class="flex gap-2">
					<Input 
						id="pay_currency"
						type="text" 
						bind:value={currency}
						placeholder="e.g., YER"
						class="input-sunken text-xs py-1.5 uppercase font-mono max-w-[100px]"
						required
						maxlength={3}
						disabled={isSubmitting}
					/>
					<div class="flex gap-1.5 items-center">
						{#each ['YER', 'USD', 'SAR'] as curr}
							<button
								type="button"
								class="px-2.5 py-1 rounded border text-[10px] font-bold transition-all {currency.toUpperCase() === curr ? 'bg-primary text-primary-foreground border-primary' : 'bg-background hover:bg-accent text-muted-foreground border-border'}"
								onclick={() => currency = curr}
								disabled={isSubmitting}
							>
								{curr}
							</button>
						{/each}
					</div>
				</div>
			</div>

			<!-- Fee Amount -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="pay_amount" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{m.payment_label_amount()}
				</label>
				<Input 
					id="pay_amount"
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

			<!-- Receipt ID -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="pay_receipt" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{m.payment_label_receipt()}
				</label>
				<Input 
					id="pay_receipt"
					type="text" 
					bind:value={receiptNumber}
					placeholder="e.g., REC-10492"
					class="input-sunken text-xs py-1.5"
					disabled={isSubmitting}
				/>
			</div>

			<!-- Reference Number (transaction ID) -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="pay_ref" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{m.payment_label_ref()}
				</label>
				<Input 
					id="pay_ref"
					type="text" 
					bind:value={referenceNumber}
					placeholder="e.g., TXN-998822"
					class="input-sunken text-xs py-1.5"
					disabled={isSubmitting}
				/>
			</div>

			<!-- Description -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="pay_notes" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{m.payment_label_notes()}
				</label>
				<Input 
					id="pay_notes"
					type="text" 
					bind:value={notes}
					placeholder={m.payment_placeholder_notes()}
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
					{m.payment_button_cancel()}
				</Button>
				<Button 
					type="submit" 
					variant="lime" 
					class="text-xs h-9 px-4 font-semibold"
					disabled={isSubmitting || isLoadingDropdowns || isLoadingFees || !selectedFeeId}
				>
					{isSubmitting ? m.loading() : m.payment_button_record()}
				</Button>
			</div>
		</form>
	</SheetContent>
</Sheet>
