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
let treasuryType = $state<"cash" | "bank">("cash");
let balance = $state<number>(0);

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  if (!nameAr && !name) {
    toast.error(isAr ? "الرجاء إدخال اسم الخزينة" : "Please enter treasury name");
    return;
  }
  if (!code) {
    toast.error(isAr ? "الرجاء إدخال كود تعريفي" : "Please enter a unique code");
    return;
  }

  isSubmitting = true;
  try {
    const request = {
      name: name || nameAr,
      name_ar: nameAr || name,
      code: code.trim(),
      treasury_type: treasuryType,
      balance: balance ? Number(balance) : 0,
    };
    await commands.finance.createTreasury(orgId, request);
    toast.success(isAr ? "تم إنشاء الخزينة بنجاح" : "Treasury created successfully");
    open = false;
    
    // Reset form fields
    name = "";
    nameAr = "";
    code = "";
    treasuryType = "cash";
    balance = 0;

    onSuccess();
  } catch (e) {
    toast.error(
      isAr 
        ? `خطأ أثناء إنشاء الخزينة: ${e instanceof Error ? e.message : String(e)}`
        : `Error creating treasury: ${e instanceof Error ? e.message : String(e)}`
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
				<span>{isAr ? "إنشاء خزينة جديدة" : "New Treasury safe / Bank"}</span>
			</SheetTitle>
			<p class="text-[10px] text-muted-foreground mt-0.5">
				{isAr ? "عرف صندوق نقدي أو حساب مصرفي لتلقي وإيداع متحصلات المدرسة." : "Add a physical cash register vault or institutional bank account."}
			</p>
		</SheetHeader>

		<!-- Scrollable Form Container -->
		<form onsubmit={handleSubmit} class="flex-1 overflow-y-auto py-4 flex flex-col gap-4 scrollbar-hide">
			<!-- Name (Arabic) -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="treas_name_ar" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{isAr ? "اسم الخزينة (بالعربية) *" : "Treasury Name (Arabic) *"}
				</label>
				<Input 
					id="treas_name_ar"
					type="text" 
					bind:value={nameAr}
					placeholder={isAr ? "مثال: الصندوق الرئيسي للمدرسة" : "e.g., School Main Safe Box"}
					class="input-sunken text-xs py-1.5"
					required
					disabled={isSubmitting}
				/>
			</div>

			<!-- Name (English) -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="treas_name_en" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{isAr ? "اسم الخزينة (بالإنجليزية)" : "Treasury Name (English)"}
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

			<!-- Code -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="treas_code" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{isAr ? "الكود التعريفي (فريد) *" : "Treasury Code (Unique) *"}
				</label>
				<Input 
					id="treas_code"
					type="text" 
					bind:value={code}
					placeholder="e.g., SAFE-MAIN"
					class="input-sunken text-xs py-1.5"
					required
					disabled={isSubmitting}
				/>
			</div>

			<!-- Type Selection -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="treas_type" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{isAr ? "تصنيف الحساب" : "Treasury Category"}
				</label>
				<select 
					id="treas_type"
					bind:value={treasuryType}
					class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-ring outline-none"
					disabled={isSubmitting}
				>
					<option value="cash">{isAr ? "نقدي (كاش)" : "Physical Cashbox / Safe"}</option>
					<option value="bank">{isAr ? "حساب مصرفي / بنك" : "Bank Account"}</option>
				</select>
			</div>

			<!-- Initial Balance -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="treas_bal" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{isAr ? "الرصيد الافتتاحي" : "Opening / Initial Balance"}
				</label>
				<Input 
					id="treas_bal"
					type="number" 
					bind:value={balance}
					min="0"
					step="any"
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
					{isSubmitting ? (isAr ? "جاري الإنشاء..." : "Creating...") : (isAr ? "إنشاء الخزينة" : "Create Treasury")}
				</Button>
			</div>
		</form>
	</SheetContent>
</Sheet>
