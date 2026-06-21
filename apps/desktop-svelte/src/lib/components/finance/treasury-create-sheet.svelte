<!--
@component TreasuryCreateSheet
A slide-over Sheet component for creating a new Treasury safe/bank account.
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
let ledgers = $state<any[]>([]);
let isLoadingLedgers = $state(false);

let name = $state("");
let nameAr = $state("");
let code = $state("");
let treasuryType = $state<"cash" | "bank">("cash");
let selectedLedgerId = $state("");
let currency = $state("YER");
let bankName = $state("");
let accountNumber = $state("");
let balance = $state<number>(0);

async function loadLedgerAccounts() {
  if (!orgId) return;
  isLoadingLedgers = true;
  try {
    const list = await commands.finance.getLedgerAccounts(orgId);
    ledgers = list.filter((l: any) => String(l.item_type || "").toLowerCase() === 'asset' || l.is_bank_account === 1 || l.is_cash_account === 1);
    if (ledgers.length > 0) {
      selectedLedgerId = ledgers[0].id;
    }
  } catch (e) {
    console.error("Error loading ledger accounts in treasury creation:", e);
  } finally {
    isLoadingLedgers = false;
  }
}

$effect(() => {
  if (open && orgId) {
    loadLedgerAccounts();
  }
});

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  if (!nameAr && !name) {
    toast.error(m.treasury_toast_error_name());
    return;
  }
  if (!selectedLedgerId) {
    toast.error(m.treasury_toast_error_ledger());
    return;
  }
  if (!currency || currency.trim().length !== 3) {
    toast.error(m.treasury_toast_error_currency());
    return;
  }

  isSubmitting = true;
  try {
    const request = {
      name: name || nameAr,
      name_ar: nameAr || name,
      item_type: treasuryType,
      ledger_account_id: selectedLedgerId,
      currency: currency.trim().toUpperCase(),
      bank_name: treasuryType === "bank" ? (bankName || undefined) : undefined,
      account_number: treasuryType === "bank" ? (accountNumber || undefined) : undefined,
    };
    await commands.finance.createTreasury(orgId, request);
    toast.success(m.treasury_toast_success());
    open = false;
    
    // Reset form fields
    name = "";
    nameAr = "";
    code = "";
    treasuryType = "cash";
    selectedLedgerId = "";
    currency = "YER";
    bankName = "";
    accountNumber = "";
    balance = 0;

    onSuccess();
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(m.treasury_toast_error_submit({ error: errorMsg }));
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
				<span>{m.treasury_sheet_title()}</span>
			</SheetTitle>
			<p class="text-[10px] text-muted-foreground mt-0.5">
				{m.treasury_sheet_subtitle()}
			</p>
		</SheetHeader>

		<!-- Scrollable Form Container -->
		<form onsubmit={handleSubmit} class="flex-1 overflow-y-auto py-4 flex flex-col gap-4 scrollbar-hide">
			<!-- Name (Arabic) -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="treas_name_ar" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{m.treasury_label_name_ar()}
				</label>
				<Input 
					id="treas_name_ar"
					type="text" 
					bind:value={nameAr}
					placeholder={m.treasury_placeholder_name_ar()}
					class="input-sunken text-xs py-1.5"
					required
					disabled={isSubmitting}
				/>
			</div>

			<!-- Name (English) -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="treas_name_en" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{m.treasury_label_name_en()}
				</label>
				<Input 
					id="treas_name_en"
					type="text" 
					bind:value={name}
					placeholder="e.g., School Main Safe Box"
					class="input-sunken text-xs py-1.5"
					disabled={isSubmitting}
				/>
			</div>

			<!-- Type Selection -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="treas_type" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{m.treasury_label_type()}
				</label>
				<select 
					id="treas_type"
					bind:value={treasuryType}
					class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-ring outline-none"
					disabled={isSubmitting}
				>
					<option value="cash">{m.attendance_status_unknown() === "غير محدد" ? "نقدي (كاش)" : "Physical Cashbox"}</option>
					<option value="bank">{m.attendance_status_unknown() === "غير محدد" ? "حساب مصرفي / بنك" : "Bank Account"}</option>
				</select>
			</div>

			<!-- Associated Ledger Account Select -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="treas_ledger" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{m.treasury_label_ledger()}
				</label>
				<select 
					id="treas_ledger"
					bind:value={selectedLedgerId}
					class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-ring outline-none"
					disabled={isLoadingLedgers || isSubmitting}
				>
					{#if isLoadingLedgers}
						<option>{m.loading()}</option>
					{:else}
						{#each ledgers as l (l.id)}
							<option value={String(l.id)}>{l.name_ar || l.name} ({l.code})</option>
						{/each}
					{/if}
				</select>
			</div>

			<!-- Currency Selection -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="treas_currency" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{m.treasury_label_currency()}
				</label>
				<div class="flex gap-2">
					<Input 
						id="treas_currency"
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

			<!-- Conditional Bank Fields -->
			{#if treasuryType === "bank"}
				<!-- Bank Name -->
				<div class="flex flex-col gap-1.5 text-start">
					<label for="treas_bank_name" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
						{m.treasury_label_bank_name()}
					</label>
					<Input 
						id="treas_bank_name"
						type="text" 
						bind:value={bankName}
						placeholder={m.treasury_placeholder_bank_name()}
						class="input-sunken text-xs py-1.5"
						disabled={isSubmitting}
					/>
				</div>

				<!-- Account Number -->
				<div class="flex flex-col gap-1.5 text-start">
					<label for="treas_acc_num" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
						{m.treasury_label_acc_num()}
					</label>
					<Input 
						id="treas_acc_num"
						type="text" 
						bind:value={accountNumber}
						placeholder="e.g., 123456789"
						class="input-sunken text-xs py-1.5"
						disabled={isSubmitting}
					/>
				</div>
			{/if}

			<!-- Form Actions -->
			<div class="flex gap-2 justify-end mt-6 pt-4 border-t border-border shrink-0">
				<Button 
					type="button" 
					variant="ghost" 
					class="text-xs h-9 px-4" 
					onclick={() => open = false}
					disabled={isSubmitting}
				>
					{m.treasury_button_cancel()}
				</Button>
				<Button 
					type="submit" 
					variant="lime" 
					class="text-xs h-9 px-4 font-semibold"
					disabled={isSubmitting || isLoadingLedgers}
				>
					{isSubmitting ? m.loading() : m.treasury_button_create()}
				</Button>
			</div>
		</form>
	</SheetContent>
</Sheet>
