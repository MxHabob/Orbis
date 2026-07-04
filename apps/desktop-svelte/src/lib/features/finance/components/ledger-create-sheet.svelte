<!--
@component LedgerCreateSheet
A slide-over Sheet component for creating a new ledger account.
Uses the shared @orbis/ui-svelte/sheet component.
-->
<script lang="ts">
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetDescription,
} from "@orbis/ui-svelte/sheet";
import { Landmark } from "lucide-svelte";
import LedgerForm from "../forms/ledger-form.svelte";
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
</script>

<Sheet bind:open>
	<SheetContent class="flex flex-col h-full sm:max-w-md">
		<SheetHeader class="text-start select-none">
			<SheetTitle class="text-xl font-bold text-foreground flex items-center gap-2">
				<Landmark size={18} class="text-primary" />
				<span>{m.ledger_sheet_title()}</span>
			</SheetTitle>
			<SheetDescription class="text-sm text-muted-foreground mt-1">
				{m.ledger_sheet_subtitle()}
			</SheetDescription>
		</SheetHeader>

		<LedgerForm 
			{orgId} 
			onSuccess={() => {
				open = false;
				onSuccess();
			}} 
			onCancel={() => open = false} 
		/>
	</SheetContent>
</Sheet>
