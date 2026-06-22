<script lang="ts">
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
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
let accountType = $state<"revenue" | "expense" | "asset" | "liability">(
  "revenue",
);
let currency = $state("YER");
let description = $state("");

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  if (!nameAr && !name) {
    toast.error(m.ledger_toast_error_name());
    return;
  }
  if (!code) {
    toast.error(m.ledger_toast_error_code());
    return;
  }
  if (!currency || currency.trim().length !== 3) {
    toast.error(m.ledger_toast_error_currency());
    return;
  }

  isSubmitting = true;
  try {
    const request = {
      code: code.trim(),
      name: name || nameAr,
      name_ar: nameAr || name,
      item_type: accountType,
      currency: currency.trim().toUpperCase(),
      opening_balance: 0.0,
      sub_type: undefined,
      parent_id: undefined,
    };
    await commands.finance.createLedgerAccount(orgId, request);
    toast.success(m.ledger_toast_success());

    // Reset form fields
    name = "";
    nameAr = "";
    code = "";
    accountType = "revenue";
    currency = "YER";
    description = "";

    onSuccess();
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(m.ledger_toast_error_submit({ error: errorMsg }));
  } finally {
    isSubmitting = false;
  }
}
</script>

<form onsubmit={handleSubmit} class="flex-1 overflow-y-auto py-6 flex flex-col gap-6 scrollbar-hide">
	<!-- Name (Arabic) -->
	<div class="flex flex-col gap-2 text-start">
		<label for="led_name_ar" class="text-sm font-medium text-foreground">
			{m.ledger_label_name_ar()}
		</label>
		<Input 
			id="led_name_ar"
			type="text" 
			bind:value={nameAr}
			placeholder={m.ledger_placeholder_name_ar()}
			required
			disabled={isSubmitting}
		/>
	</div>

	<!-- Name (English) -->
	<div class="flex flex-col gap-2 text-start">
		<label for="led_name_en" class="text-sm font-medium text-foreground">
			{m.ledger_label_name_en()}
		</label>
		<Input 
			id="led_name_en"
			type="text" 
			bind:value={name}
			placeholder="e.g., Tuition Fee Revenue"
			disabled={isSubmitting}
		/>
	</div>

	<!-- Code -->
	<div class="flex flex-col gap-2 text-start">
		<label for="led_code" class="text-sm font-medium text-foreground">
			{m.ledger_label_code()}
		</label>
		<Input 
			id="led_code"
			type="text" 
			bind:value={code}
			placeholder="e.g., TUI-REV"
			required
			disabled={isSubmitting}
		/>
	</div>

	<!-- Currency Selection -->
	<div class="flex flex-col gap-2 text-start">
		<label for="led_currency" class="text-sm font-medium text-foreground">
			{m.ledger_label_currency()}
		</label>
		<div class="flex gap-3">
			<Input 
				id="led_currency"
				type="text" 
				bind:value={currency}
				placeholder="e.g., YER"
				class="uppercase font-mono max-w-[100px]"
				required
				maxlength={3}
				disabled={isSubmitting}
			/>
			<div class="flex gap-2 items-center">
				{#each ['YER', 'USD', 'SAR'] as curr}
					<button
						type="button"
						class="px-3 py-1.5 rounded border text-xs font-semibold transition-all {currency.toUpperCase() === curr ? 'bg-primary text-primary-foreground border-primary' : 'bg-background hover:bg-accent text-muted-foreground border-border'}"
						onclick={() => currency = curr}
						disabled={isSubmitting}
					>
						{curr}
					</button>
				{/each}
			</div>
		</div>
	</div>

	<!-- Account Type Selection -->
	<div class="flex flex-col gap-2 text-start">
		<label for="led_type" class="text-sm font-medium text-foreground">
			{m.ledger_label_type()}
		</label>
		<select 
			id="led_type"
			bind:value={accountType}
			class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-sm focus:ring-1 focus:ring-ring outline-none h-9 w-full"
			disabled={isSubmitting}
		>
			<option value="revenue">{m.finance_type_revenue()}</option>
			<option value="expense">{m.finance_type_expense()}</option>
			<option value="asset">{m.finance_type_asset()}</option>
			<option value="liability">{m.finance_type_liability()}</option>
		</select>
	</div>

	<!-- Description -->
	<div class="flex flex-col gap-2 text-start">
		<label for="led_desc" class="text-sm font-medium text-foreground">
			{m.ledger_label_description()}
		</label>
		<Input 
			id="led_desc"
			type="text" 
			bind:value={description}
			placeholder={m.ledger_placeholder_description()}
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
			{m.ledger_button_cancel()}
		</Button>
		<Button 
			type="submit" 
			variant="lime" 
			class="font-semibold"
			disabled={isSubmitting}
		>
			{isSubmitting ? m.loading() : m.ledger_button_create()}
		</Button>
	</div>
</form>
