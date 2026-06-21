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
import { getLocale } from "$lib/paraglide/runtime";

let { 
  open = $bindable(false), 
  orgId, 
  onSuccess 
}: { 
  open: boolean; 
  orgId: string; 
  onSuccess: () => void;
} = $props();

const isAr = $derived(getLocale() === "ar");

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
    toast.error(isAr ? "الرجاء إدخال اسم الحساب" : "Please enter ledger account name");
    return;
  }
  if (!code) {
    toast.error(isAr ? "الرجاء إدخال رمز الحساب" : "Please enter account code");
    return;
  }
  if (!currency || currency.trim().length !== 3) {
    toast.error(isAr ? "الرجاء إدخال رمز عملة صحيح من 3 أحرف" : "Please enter a valid 3-letter currency code");
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
    toast.success(isAr ? "تم إنشاء الحساب الدفتري بنجاح" : "Ledger account created successfully");
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
    toast.error(
      isAr 
        ? `خطأ أثناء إنشاء الحساب: ${e instanceof Error ? e.message : String(e)}`
        : `Error creating ledger account: ${e instanceof Error ? e.message : String(e)}`
    );
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
				<span>{isAr ? "إنشاء حساب دفتري جديد" : "New Ledger Account"}</span>
			</SheetTitle>
			<p class="text-[10px] text-muted-foreground mt-0.5">
				{isAr ? "عرف فئة لتصنيف الإيرادات أو المصروفات الخاصة بالمدرسة في شجرة الحسابات." : "Define classification accounts for tuition income, operational costs, etc."}
			</p>
		</SheetHeader>

		<!-- Scrollable Form Container -->
		<form onsubmit={handleSubmit} class="flex-1 overflow-y-auto py-4 flex flex-col gap-4 scrollbar-hide">
			<!-- Name (Arabic) -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="led_name_ar" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{isAr ? "اسم الحساب (بالعربية) *" : "Account Name (Arabic) *"}
				</label>
				<Input 
					id="led_name_ar"
					type="text" 
					bind:value={nameAr}
					placeholder={isAr ? "مثال: إيرادات الرسوم الدراسية" : "e.g., Tuition Fee Revenue"}
					class="input-sunken text-xs py-1.5"
					required
					disabled={isSubmitting}
				/>
			</div>

			<!-- Name (English) -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="led_name_en" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{isAr ? "اسم الحساب (بالإنجليزية)" : "Account Name (English)"}
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
					{isAr ? "رمز الحساب (فريد) *" : "Account Code (Unique) *"}
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
					{isAr ? "العملة *" : "Currency *"}
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
					{isAr ? "نوع الحساب" : "Account Type"}
				</label>
				<select 
					id="led_type"
					bind:value={accountType}
					class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-ring outline-none"
					disabled={isSubmitting}
				>
					<option value="revenue">{isAr ? "إيرادات" : "Revenue / Income"}</option>
					<option value="expense">{isAr ? "مصروفات" : "Expense"}</option>
					<option value="asset">{isAr ? "أصول" : "Asset"}</option>
					<option value="liability">{isAr ? "خصوم / التزامات" : "Liability"}</option>
				</select>
			</div>

			<!-- Description -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="led_desc" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{isAr ? "تفاصيل إضافية / الوصف" : "Description / Memo"}
				</label>
				<Input 
					id="led_desc"
					type="text" 
					bind:value={description}
					placeholder={isAr ? "أية تفاصيل إضافية" : "Optional description of the ledger"}
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
					{isAr ? "إلغاء" : "Cancel"}
				</Button>
				<Button 
					type="submit" 
					variant="lime" 
					class="text-xs h-9 px-4 font-semibold"
					disabled={isSubmitting}
				>
					{isSubmitting ? (isAr ? "جاري الإنشاء..." : "Creating...") : (isAr ? "إنشاء الحساب" : "Create Account")}
				</Button>
			</div>
		</form>
	</SheetContent>
</Sheet>
