<!--
@component TreasuryCreateSheet
A slide-over Sheet component for creating a new Treasury safe/bank account.
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
import TreasuryForm from "../forms/treasury-form.svelte";
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
				<span>{m.treasury_sheet_title()}</span>
			</SheetTitle>
			<SheetDescription class="text-sm text-muted-foreground mt-1">
				{m.treasury_sheet_subtitle()}
			</SheetDescription>
		</SheetHeader>

		<TreasuryForm 
			{orgId} 
			onSuccess={() => {
				open = false;
				onSuccess();
			}} 
			onCancel={() => open = false} 
		/>
	</SheetContent>
</Sheet>
