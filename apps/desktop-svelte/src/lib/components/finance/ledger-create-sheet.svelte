<!--
@component LedgerCreateSheet
A slide-over Sheet component for creating a new ledger account.
Uses the shared @orbis/ui-svelte/sheet component.
-->
<script lang="ts">
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { 
  Sheet, SheetContent, SheetHeader, SheetTitle 
} from "@orbis/ui-svelte/sheet";
import { Landmark } from "lucide-svelte";
import { toast } from "svelte-sonner";
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

// Form state
let isSubmitting = $state(false);
let name = $state("");
let nameAr = $state("");
let code = $state("");
let accountType = $state<"revenue" | "expense" | "asset" | "liability">("revenue");
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
    open = false;
    
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

<Sheet bind:open>
	<SheetContent class="w-full sm:max-w-md bg-card border-l border-border/80 flex flex-col h-full overflow-hidden text-xs">
		<SheetHeader class="pb-4 border-b border-border shrink-0 select-none text-start">
			<SheetTitle class="text-sm font-bold text-foreground uppercase tracking-wider flex items-center gap-2">
				<Landmark size={14} class="text-primary" />
				<span>{m.ledger_sheet_title()}</span>
			</SheetTitle>
			<p class="text-[10px] text-muted-foreground mt-0.5">
				{m.ledger_sheet_subtitle()}
			</p>
		</SheetHeader>

		<!-- Scrollable Form Container -->
		<form onsubmit={handleSubmit} class="flex-1 overflow-y-auto py-4 flex flex-col gap-4 scrollbar-hide">
			<!-- Name (Arabic) -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="led_name_ar" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{m.ledger_label_name_ar()}
				</label>
				<Input 
					id="led_name_ar"
					type="text" 
					bind:value={nameAr}
					placeholder={m.ledger_placeholder_name_ar()}
					class="input-sunken text-xs py-1.5"
					required
					disabled={isSubmitting}
				/>
			</div>

			<!-- Name (English) -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="led_name_en" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{m.ledger_label_name_en()}
				</label>
				<Input 
					id="led_name_en"
					type="text" 
					bind:value={name}
					placeholder="e.g., Tuition Fee Revenue"
					class="input-sunken text-xs py-1.5"
					disabled={isSubmitting}
				/>
			</div>

			<!-- Code -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="led_code" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{m.ledger_label_code()}
				</label>
				<Input 
					id="led_code"
					type="text" 
					bind:value={code}
					placeholder="e.g., TUI-REV"
					class="input-sunken text-xs py-1.5"
					required
					disabled={isSubmitting}
				/>
			</div>

			<!-- Currency Selection -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="led_currency" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{m.ledger_label_currency()}
				</label>
				<div class="flex gap-2">
					<Input 
						id="led_currency"
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

			<!-- Account Type Selection -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="led_type" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{m.ledger_label_type()}
				</label>
				<select 
					id="led_type"
					bind:value={accountType}
					class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-ring outline-none"
					disabled={isSubmitting}
				>
					<option value="revenue">{m.finance_type_revenue()}</option>
					<option value="expense">{m.finance_type_expense()}</option>
					<option value="asset">{m.finance_type_asset()}</option>
					<option value="liability">{m.finance_type_liability()}</option>
				</select>
			</div>

			<!-- Description -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="led_desc" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{m.ledger_label_description()}
				</label>
				<Input 
					id="led_desc"
					type="text" 
					bind:value={description}
					placeholder={m.ledger_placeholder_description()}
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
					{m.ledger_button_cancel()}
				</Button>
				<Button 
					type="submit" 
					variant="lime" 
					class="text-xs h-9 px-4 font-semibold"
					disabled={isSubmitting}
				>
					{isSubmitting ? m.loading() : m.ledger_button_create()}
				</Button>
			</div>
		</form>
	</SheetContent>
</Sheet>
