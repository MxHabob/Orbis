<script lang="ts">
import { onMount } from "svelte";
import { slide } from "svelte/transition";
import { DocumentPage } from "@orbis/icons";
import { api } from "$lib/api";
import { formatCurrency } from "$lib/core/i18n";
import * as m from "$lib/paraglide/messages";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
import { paymentRecordInputSchema } from "@orbis/schemas";
import { FormField, toast } from "@orbis/ui";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import {
  createAvailablePaymentMethodsQuery,
  type AvailablePaymentMethod,
} from "$lib/features/finance";

let {
  orgId,
  onSuccess,
  onCancel,
}: {
  orgId: string;
  onSuccess: () => void;
  onCancel: () => void;
} = $props();

// Dynamic payment methods from Integration Layer
const paymentMethodsQuery = createAvailablePaymentMethodsQuery(() => orgId);
const availablePaymentMethods = $derived(paymentMethodsQuery.data ?? []);

// State
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
let paymentMethod = $state<string>("cash");
let currency = $state("YER");
let amount = $state<number | null>(null);
let receiptNumber = $state("");
let referenceNumber = $state("");
let sourceToken = $state("");
let notes = $state("");

const selectedPaymentMethodObj = $derived(
  availablePaymentMethods.find((m) => m.id === paymentMethod) || availablePaymentMethods[0]
);

const form = new FormController({
  initialValues: {
    fee_id: "",
    amount: 0,
    method: "cash",
    reference: "",
    notes: "",
    cash_account_id: "",
    currency: "YER",
  },
  schema: paymentRecordInputSchema,
});
const isSubmitting = $derived(form.isSubmitting);

// Derived exchange rate & conversion
const exchangeRate = $derived.by(() => {
  const selectedCurr = activeCurrencies.find(
    (c) => c.code === currency.toUpperCase(),
  );
  const rate = Number(selectedCurr?.exchange_rate ?? 1.0);
  return Number.isFinite(rate) && rate > 0 ? rate : 1.0;
});

const convertedAmount = $derived(amount ? amount * exchangeRate : 0.0);

async function loadDropdownData() {
  if (!orgId) return;
  isLoadingDropdowns = true;
  try {
    const [studentList, treasuryList, currencyList] = await Promise.all([
      api.listEnrollments(),
      api
        .listAccounts()
        .then((accounts) =>
          accounts.filter((account: any) =>
            ["cash", "bank"].includes(account.sub_type),
          ),
        ),
      api.listCurrencies(),
    ]);
    students = studentList;
    treasuries = treasuryList;
    activeCurrencies = currencyList;

    if (studentList.length > 0) {
      selectedEnrollmentId = String(studentList[0].id);
      await loadStudentFees(String(studentList[0].id));
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
    const list = await api.listStudentFees({ enrollmentId: enrollId });
    fees = list.filter((f) => f.status !== "paid");
    if (fees.length > 0) {
      selectedFeeId = String(fees[0].id);
      amount = Number(fees[0].balance || fees[0].total_amount || 0);
    } else {
      selectedFeeId = "";
      amount = null;
      toast.info(m.finance_payment_no_fees());
    }
  } catch (e) {
    console.error("Error loading student fees:", e);
  } finally {
    isLoadingFees = false;
  }
}

function handleStudentChange(e: Event) {
  const val = (e.currentTarget as HTMLSelectElement).value;
  selectedEnrollmentId = val;
  loadStudentFees(val);
}

function handleFeeChange(e: Event) {
  const val = (e.currentTarget as HTMLSelectElement).value;
  selectedFeeId = val;
  const selectedFee = fees.find((f) => String(f.id) === val);
  if (selectedFee) {
    amount = Number(selectedFee.balance || selectedFee.total_amount || 0);
  }
}

onMount(() => {
  loadDropdownData();
});

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  if (!selectedEnrollmentId) {
    toast.error(m.common_fee_toast_error_student());
    return;
  }
  form.values = {
    fee_id: selectedFeeId,
    amount: Number(amount),
    method: paymentMethod,
    reference: referenceNumber || undefined,
    notes: notes || undefined,
    cash_account_id: selectedTreasuryId,
    currency: currency.trim().toUpperCase(),
  };
  const submitted = await form.submit(async (payload) => {
    await api.processPayment({
      fee_id: payload.fee_id,
      account_id: payload.cash_account_id,
      amount: payload.amount,
      method: payload.method,
      reference: payload.reference,
      source: sourceToken.trim() || undefined,
    });
    toast.success(m.common_payment_toast_success());
  });
  if (submitted) {
    form.reset();
    amount = null;
    receiptNumber = "";
    referenceNumber = "";
    sourceToken = "";
    notes = "";
    onSuccess();
  } else if (form.formError) {
    toast.error(form.formError);
  } else {
    toast.error(m.common_fee_toast_error_amount());
  }
}
</script>

<form onsubmit={handleSubmit} class="flex-1 overflow-y-auto py-6 flex flex-col gap-6 scrollbar-hide">
	<!-- Student Select -->
	<FormField id="pay_student" label={m.common_fee_label_student()} required>
		{#snippet children()}
		<select 
			id="pay_student"
			value={selectedEnrollmentId}
			onchange={handleStudentChange}
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
		{/snippet}
	</FormField>

	<!-- Student Fees Select -->
	<FormField id="pay_fee" label={m.common_payment_label_fee()} required>
		{#snippet children()}
		<select 
			id="pay_fee"
			value={selectedFeeId}
			onchange={handleFeeChange}
			class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-sm focus:ring-1 focus:ring-ring outline-none h-9 w-full"
			disabled={isLoadingFees || isSubmitting}
		>
			{#if isLoadingFees}
				<option>{m.finance_payment_loading_fees()}</option>
			{:else if fees.length === 0}
				<option value="">{m.finance_payment_no_fees()}</option>
			{:else}
				{#each fees as f (f.id)}
					<option value={String(f.id)}>
						{f.notes || f.id.slice(0, 8)} ({m.common_payment_label_balance()}: {f.balance})
					</option>
				{/each}
			{/if}
		</select>
		{/snippet}
	</FormField>

	<!-- Treasury Account Select -->
	<FormField id="pay_treasury" label={m.common_payment_label_cash_account()} required>
		{#snippet children()}
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
		{/snippet}
	</FormField>

	<!-- Payment Method Select (Dynamically Resolved via Integration Layer) -->
	<FormField id="pay_method" label={m.common_payment_label_method()} required>
		{#snippet children()}
		<div class="flex flex-col gap-1.5">
			<select 
				id="pay_method"
				bind:value={paymentMethod}
				class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-sm focus:ring-1 focus:ring-ring outline-none h-9 w-full"
				disabled={isSubmitting}
			>
				{#each availablePaymentMethods as method (method.id)}
					<option value={method.id}>
						{method.name} {method.is_digital_gateway ? m.finance_ui_gateway_provider({ provider: method.provider_name ?? "" }) : ""}
					</option>
				{/each}
			</select>
			{#if selectedPaymentMethodObj?.is_digital_gateway}
				<div class="text-[10px] text-primary flex items-center gap-1 font-medium bg-primary/5 border border-primary/20 rounded px-2 py-1">
					<span>{m.finance_ui_payment_electronic_processing()} <strong>{selectedPaymentMethodObj.provider_name}</strong></span>
				</div>
			{/if}
		</div>
		{/snippet}
	</FormField>

	<!-- Electronic Gateway Source Token -->
	{#if selectedPaymentMethodObj?.requires_source_token}
		<FormField id="pay_source_token" label={m.finance_ui_payment_gateway_token_label()}>
			{#snippet children()}
			<Input
				id="pay_source_token"
				type="text"
				bind:value={sourceToken}
				placeholder={m.finance_ui_payment_gateway_token_placeholder()}
				class="font-mono text-xs"
				disabled={isSubmitting}
			/>
			{/snippet}
		</FormField>
	{/if}

	<!-- Currency Selection -->
	<FormField id="pay_currency" label={m.common_payment_label_currency()} required>
		{#snippet children()}
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
		{/snippet}
	</FormField>

	<!-- Fee Amount -->
	<FormField id="pay_amount" label={m.common_payment_label_amount()} required>
		{#snippet children()}
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
				 {m.common_payment_converted_yer({ amount: formatCurrency(convertedAmount, "YER"), rate: Number.isFinite(exchangeRate) && exchangeRate > 0 ? exchangeRate.toFixed(2) : "1.00" })}
			</div>
		{/if}
		{/snippet}
	</FormField>

	<!-- DocumentPage ID -->
	<FormField id="pay_receipt" label={m.common_payment_label_receipt()}>
		{#snippet children()}
		<Input 
			id="pay_receipt"
			type="text" 
			bind:value={receiptNumber}
			placeholder={m.common_payment_receipt_placeholder()}
			disabled={isSubmitting}
		/>
		{/snippet}
	</FormField>

	<!-- Reference Number (transaction ID) -->
	<FormField id="pay_ref" label={m.common_payment_label_ref()}>
		{#snippet children()}
		<Input 
			id="pay_ref"
			type="text" 
			bind:value={referenceNumber}
			placeholder={m.common_payment_reference_placeholder()}
			disabled={isSubmitting}
		/>
		{/snippet}
	</FormField>

	<!-- Description -->
	<FormField id="pay_notes" label={m.common_payment_label_notes()}>
		{#snippet children()}
		<Input 
			id="pay_notes"
			type="text" 
			bind:value={notes}
			placeholder={m.common_payment_placeholder_notes()}
			disabled={isSubmitting}
		/>
		{/snippet}
	</FormField>

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
