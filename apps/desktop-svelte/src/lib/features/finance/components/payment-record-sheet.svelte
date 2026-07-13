<!--
@component PaymentRecordSheet
A slide-over Sheet component for recording a student payment.
Uses the shared @orbis/ui-svelte/sheet component.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetDescription,
} from "@orbis/ui-svelte/sheet";
import { CreditCard } from "lucide-svelte";
import PaymentRecordForm from "./payment-record-form.svelte";

let {
  open = $bindable(false),
  orgId,
  onSuccess,
}: {
  open: boolean;
  orgId: string;
  onSuccess: () => void;
} = $props();
</script>

<Sheet bind:open>
	<SheetContent class="flex flex-col h-full sm:max-w-md">
		<SheetHeader class="text-start select-none">
			<SheetTitle class="text-xl font-bold text-foreground flex items-center gap-2">
				<CreditCard size={18} class="text-primary" />
				<span>{m.common_payment_sheet_title()}</span>
			</SheetTitle>
			<SheetDescription class="text-sm text-muted-foreground mt-1">
				{m.common_payment_sheet_subtitle()}
			</SheetDescription>
		</SheetHeader>

		<PaymentRecordForm 
			{orgId} 
			onSuccess={() => {
				open = false;
				onSuccess();
			}} 
			onCancel={() => open = false} 
		/>
	</SheetContent>
</Sheet>
