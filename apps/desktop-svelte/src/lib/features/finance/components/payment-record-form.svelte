<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { commands } from "$lib/core/ipc";
import { authStore } from "$lib/stores";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { toast } from "svelte-sonner";

import { slide } from "svelte/transition";

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
let treasuries = $state<any[]>([]);
let fees = $state<any[]>([]);
let activeCurrencies = $state<any[]>([]);
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

// Derived exchange rate & conversion
const exchangeRate = $derived.by(() => {
  const selectedCurr = activeCurrencies.find(
    (c) => c.code === currency.toUpperCase(),
  );
  return selectedCurr ? selectedCurr.exchange_rate : 1.0;
});

const convertedAmount = $derived.by(() => {
  if (!amount) return 0.0;
  return amount * exchangeRate;
});

async function loadDropdownData() {
  if (!orgId) return;
  isLoadingDropdowns = true;
  try {
    const [studentList, treasuryList, currencyList] = await Promise.all([
      commands.academic.listEnrollments(),
			commands.finance
				.listAccounts()
				.then((accounts) => accounts.filter((account: any) => ["cash", "bank"].includes(account.sub_type))),
      commands.finance.listCurrencies(),
    ]);
    students = studentList;
    treasuries = treasuryList;
    activeCurrencies = currencyList;

    if (studentList.length > 0) {
      selectedEnrollmentId = String(studentList[0].id);
    }
    if (treasuryList.length > 0) {
      selectedTreasuryId = String(treasuryList[0].id);
      currency = String((treasuryList[0] as any).currency || "YER");
    }
  } catch (e) {
    console.error("Error loading dropdown data for payment-record-form:", e);
  } finally {
    isLoadingDropdowns = false;
  }
}

async function loadStudentFees(enrollId: string) {
  if (!enrollId || !orgId) return;
  isLoadingFees = true;
  try {
    const list = await commands.finance.listStudentFeesByEnrollment(enrollId);
    fees = list.filter((f) => f.status !== "paid");
    if (fees.length > 0) {
      selectedFeeId = String(fees[0].id);
      amount = Number(fees[0].balance || fees[0].total_amount || 0);
    } else {
      selectedFeeId = "";
      amount = null;
      toast.info(
        m.common_status_undefined() === "غير محدد"
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

// Reload dropdowns on mount
$effect(() => {
  if (orgId) {
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
    toast.error(m.common_fee_toast_error_student());
    return;
  }
  if (!selectedFeeId) {
    toast.error(m.common_payment_toast_error_fee());
    return;
  }
  if (!selectedTreasuryId) {
    toast.error(m.common_payment_toast_error_treasury());
    return;
  }
  if (!amount || amount <= 0) {
    toast.error(m.common_fee_toast_error_amount());
    return;
  }
  if (!currency || currency.trim().length !== 3) {
    toast.error(m.common_ledger_toast_error_currency());
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
	cash_account_id: selectedTreasuryId,
      currency: currency.trim().toUpperCase(),
    };
    await commands.finance.recordPayment(createdBy, request);
    toast.success(m.common_payment_toast_success());

    // Reset form
    amount = null;
    receiptNumber = "";
    referenceNumber = "";
    notes = "";

    onSuccess();
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(m.common_payment_toast_error_submit({ error: errorMsg }));
  } finally {
    isSubmitting = false;
  }
}
</script>

<form onsubmit={handleSubmit} class="flex-1 overflow-y-auto py-6 flex flex-col gap-6 scrollbar-hide">
	<!-- Student Select -->
	<div class="flex flex-col gap-2 text-start">
		<label for="pay_student" class="text-sm font-medium text-foreground">
			{m.common_fee_label_student()}
		</label>
		<select 
			id="pay_student"
			bind:value={selectedEnrollmentId}
			class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-sm focus:ring-1 focus:ring-ring outline-none h-9 w-full"
			disabled={isLoadingDropdowns || isSubmitting}
		>
			{#if isLoadingDropdowns}
				<option>{m.common_status_undefined() === "غير محدد" ? m.common_fee_dropdown_loading_students() : "Loading students..."}</option>
			{:else}
				{#each students as s (s.id)}
					<option value={String(s.id)}>{s.full_name || s.student_number || s.id}</option>
				{/each}
			{/if}
		</select>
	</div>

	<!-- Student Fees Select -->
	<div class="flex flex-col gap-2 text-start">
		<label for="pay_fee" class="text-sm font-medium text-foreground">
			{m.common_payment_label_fee()}
		</label>
		<select 
			id="pay_fee"
			bind:value={selectedFeeId}
			class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-sm focus:ring-1 focus:ring-ring outline-none h-9 w-full"
			disabled={isLoadingFees || isSubmitting}
		>
			{#if isLoadingFees}
				<option>{m.common_status_undefined() === "غير محدد" ? m.finance_payment_loading_fees() : "Loading fees..."}</option>
			{:else if fees.length === 0}
				<option value="">{m.common_status_undefined() === "غير محدد" ? m.finance_payment_no_fees() : "-- No outstanding fees for this student --"}</option>
			{:else}
				{#each fees as f (f.id)}
					<option value={String(f.id)}>
						{f.notes || f.id.slice(0, 8)} ({m.common_status_undefined() === "غير محدد" ? 'المتبقي: ' : 'Balance: '}{f.balance})
					</option>
				{/each}
			{/if}
		</select>
	</div>

	<!-- Treasury Account Select -->
	<div class="flex flex-col gap-2 text-start">
		<label for="pay_treasury" class="text-sm font-medium text-foreground">
			{m.common_payment_label_treasury()}
		</label>
		<select 
			id="pay_treasury"
			bind:value={selectedTreasuryId}
			class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-sm focus:ring-1 focus:ring-ring outline-none h-9 w-full"
			disabled={isLoadingDropdowns || isSubmitting}
		>
			{#if isLoadingDropdowns}
				<option>{m.common_loading()}</option>
			{:else}
				{#each treasuries as t (t.id)}
					<option value={String(t.id)}>{t.name_ar || t.name || t.code} ({t.currency})</option>
				{/each}
			{/if}
		</select>
	</div>

	<!-- Payment Method Select -->
	<div class="flex flex-col gap-2 text-start">
		<label for="pay_method" class="text-sm font-medium text-foreground">
			{m.common_payment_label_method()}
		</label>
		<select 
			id="pay_method"
			bind:value={paymentMethod}
			class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-sm focus:ring-1 focus:ring-ring outline-none h-9 w-full"
			disabled={isSubmitting}
		>
			<option value="Cash">{m.common_status_undefined() === "غير محدد" ? m.finance_pos_cash() : "Cash"}</option>
			<option value="Bank Transfer">{m.common_status_undefined() === "غير محدد" ? m.finance_pos_bank_transfer() : "Bank Transfer"}</option>
			<option value="Card">{m.common_status_undefined() === "غير محدد" ? m.finance_payment_type_card() : "Card"}</option>
		</select>
	</div>

	<!-- Currency Selection -->
	<div class="flex flex-col gap-2 text-start">
		<label for="pay_currency" class="text-sm font-medium text-foreground">
			{m.common_payment_label_currency()}
		</label>
		<select 
			id="pay_currency"
			bind:value={currency}
			class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-sm focus:ring-1 focus:ring-ring outline-none h-9 w-full font-sans"
			disabled={isLoadingDropdowns || isSubmitting}
		>
			{#each activeCurrencies as curr (curr.code)}
				<option value={curr.code}>
					{curr.code} - {curr.name_ar || curr.name || curr.code} {curr.symbol ? `(${curr.symbol})` : ''}
				</option>
			{/each}
		</select>
	</div>

	<!-- Fee Amount -->
	<div class="flex flex-col gap-2 text-start">
		<label for="pay_amount" class="text-sm font-medium text-foreground">
			{m.common_payment_label_amount()}
		</label>
		<Input 
			id="pay_amount"
			type="number" 
			bind:value={amount}
			min="1"
			step="any"
			placeholder="0.00"
			required
			disabled={isSubmitting}
		/>
		{#if currency.toUpperCase() !== 'YER' && amount && amount > 0}
			<div class="text-xs text-emerald-500 font-bold select-none mt-1" transition:slide={{ duration: 150 }}>
				 يعادل بالريال: {formatCurrency(convertedAmount, 'YER')} (سعر الصرف: {exchangeRate.toFixed(2)})
			</div>
		{/if}
	</div>

	<!-- Receipt ID -->
	<div class="flex flex-col gap-2 text-start">
		<label for="pay_receipt" class="text-sm font-medium text-foreground">
			{m.common_payment_label_receipt()}
		</label>
		<Input 
			id="pay_receipt"
			type="text" 
			bind:value={receiptNumber}
			placeholder="e.g., REC-10492"
			disabled={isSubmitting}
		/>
	</div>

	<!-- Reference Number (transaction ID) -->
	<div class="flex flex-col gap-2 text-start">
		<label for="pay_ref" class="text-sm font-medium text-foreground">
			{m.common_payment_label_ref()}
		</label>
		<Input 
			id="pay_ref"
			type="text" 
			bind:value={referenceNumber}
			placeholder="e.g., TXN-998822"
			disabled={isSubmitting}
		/>
	</div>

	<!-- Description -->
	<div class="flex flex-col gap-2 text-start">
		<label for="pay_notes" class="text-sm font-medium text-foreground">
			{m.common_payment_label_notes()}
		</label>
		<Input 
			id="pay_notes"
			type="text" 
			bind:value={notes}
			placeholder={m.common_payment_placeholder_notes()}
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
			disabled={isSubmitting || isLoadingDropdowns || isLoadingFees || !selectedFeeId}
		>
			{isSubmitting ? m.common_loading() : m.common_payment_button_record()}
		</Button>
	</div>
</form>
