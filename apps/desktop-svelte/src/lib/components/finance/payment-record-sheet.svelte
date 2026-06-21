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
  Sheet, SheetContent, SheetHeader, SheetTitle 
} from "@orbis/ui-svelte/sheet";
import { CreditCard } from "lucide-svelte";
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

// State
let isSubmitting = $state(false);
let students = $state<any[]>([]);
let treasuries = $state<any[]>([]);
let isLoadingDropdowns = $state(false);

// Form state
let selectedEnrollmentId = $state("");
let selectedTreasuryId = $state("");
let paymentMethod = $state<"Cash" | "Bank Transfer" | "Card">("Cash");
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
    
    if (studentList.length > 0) selectedEnrollmentId = String(studentList[0].id);
    if (treasuryList.length > 0) selectedTreasuryId = String(treasuryList[0].id);
  } catch (e) {
    console.error("Error loading dropdown data for payment-record-sheet:", e);
  } finally {
    isLoadingDropdowns = false;
  }
}

// Reload when opened
$effect(() => {
  if (open && orgId) {
    loadDropdownData();
  }
});

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  if (!selectedEnrollmentId) {
    toast.error(isAr ? "الرجاء اختيار الطالب" : "Please select a student");
    return;
  }
  if (!selectedTreasuryId) {
    toast.error(isAr ? "الرجاء اختيار الخزينة المستهدفة" : "Please select target treasury");
    return;
  }
  if (!amount || amount <= 0) {
    toast.error(isAr ? "الرجاء إدخال مبلغ صحيح" : "Please enter a valid amount");
    return;
  }

  isSubmitting = true;
  try {
    const createdBy = authStore.userId || authStore.user?.id || "unknown";
    const request = {
      enrollment_id: selectedEnrollmentId,
      treasury_id: selectedTreasuryId,
      payment_method: paymentMethod,
      amount: Number(amount),
      receipt_number: receiptNumber || undefined,
      reference_number: referenceNumber || undefined,
      notes: notes || undefined,
    };
    await commands.finance.recordPayment(orgId, createdBy, request);
    toast.success(isAr ? "تم تسجيل الدفعة بنجاح" : "Payment recorded successfully");
    open = false;
    
    // Reset form
    amount = null;
    receiptNumber = "";
    referenceNumber = "";
    notes = "";

    onSuccess();
  } catch (e) {
    toast.error(
      isAr 
        ? `خطأ أثناء تسجيل الدفعة: ${e instanceof Error ? e.message : String(e)}`
        : `Error recording payment: ${e instanceof Error ? e.message : String(e)}`
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
				<CreditCard size={14} class="text-primary" />
				<span>{isAr ? "تسجيل دفعة جديدة" : "Record Student Payment"}</span>
			</SheetTitle>
			<p class="text-[10px] text-muted-foreground mt-0.5">
				{isAr ? "سجل إيصال دفع جديد بمبلغ وارد من طالب إلى خزينة المدرسة." : "Log a payment transaction receipt received from a student."}
			</p>
		</SheetHeader>

		<!-- Scrollable Form Container -->
		<form onsubmit={handleSubmit} class="flex-1 overflow-y-auto py-4 flex flex-col gap-4 scrollbar-hide">
			<!-- Student Select -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="pay_student" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{isAr ? "طالب *" : "Student *"}
				</label>
				<select 
					id="pay_student"
					bind:value={selectedEnrollmentId}
					class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-ring outline-none"
					disabled={isLoadingDropdowns || isSubmitting}
				>
					{#if isLoadingDropdowns}
						<option>{isAr ? "جاري تحميل قائمة الطلاب..." : "Loading students..."}</option>
					{:else}
						{#each students as s (s.id)}
							<option value={String(s.id)}>{s.full_name || s.student_number || s.id}</option>
						{/each}
					{/if}
				</select>
			</div>

			<!-- Treasury Account Select -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="pay_treasury" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{isAr ? "الخزينة المستلمة *" : "Destination Treasury *"}
				</label>
				<select 
					id="pay_treasury"
					bind:value={selectedTreasuryId}
					class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-ring outline-none"
					disabled={isLoadingDropdowns || isSubmitting}
				>
					{#if isLoadingDropdowns}
						<option>{isAr ? "جاري تحميل الحسابات..." : "Loading treasuries..."}</option>
					{:else}
						{#each treasuries as t (t.id)}
							<option value={String(t.id)}>{t.name_ar || t.name || t.code}</option>
						{/each}
					{/if}
				</select>
			</div>

			<!-- Payment Method Select -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="pay_method" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{isAr ? "طريقة الدفع *" : "Payment Method *"}
				</label>
				<select 
					id="pay_method"
					bind:value={paymentMethod}
					class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-ring outline-none"
					disabled={isSubmitting}
				>
					<option value="Cash">{isAr ? "نقدي (كاش)" : "Cash"}</option>
					<option value="Bank Transfer">{isAr ? "تحويل بنكي" : "Bank Transfer"}</option>
					<option value="Card">{isAr ? "بطاقة دفع / مدى" : "Card"}</option>
				</select>
			</div>

			<!-- Fee Amount -->
			<div class="flex flex-col gap-1.5 text-start">
				<label for="pay_amount" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
					{isAr ? "المبلغ المستلم *" : "Amount Received *"}
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
					{isAr ? "رقم إيصال الدفع" : "Receipt Number"}
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
					{isAr ? "رقم المرجع / الحوالة" : "Reference / Transaction ID"}
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
					{isAr ? "ملاحظات إضافية" : "Notes / Remarks"}
				</label>
				<Input 
					id="pay_notes"
					type="text" 
					bind:value={notes}
					placeholder={isAr ? "أية تفاصيل متعلقة بالدفعة" : "Any details about the receipt"}
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
					disabled={isSubmitting || isLoadingDropdowns}
				>
					{isSubmitting ? (isAr ? "جاري التسجيل..." : "Recording...") : (isAr ? "تسجيل الدفعة" : "Record Receipt")}
				</Button>
			</div>
		</form>
	</SheetContent>
</Sheet>
