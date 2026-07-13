<!--
@component FeeAssignSheet
A slide-over Sheet component for assigning a fee to a student.
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
import { Wallet } from "lucide-svelte";
import FeeAssignForm from "./fee-assign-form.svelte";

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
				<Wallet size={18} class="text-primary" />
				<span>{m.common_fee_sheet_title()}</span>
			</SheetTitle>
			<SheetDescription class="text-sm text-muted-foreground mt-1">
				{m.common_fee_sheet_subtitle()}
			</SheetDescription>
		</SheetHeader>

		<FeeAssignForm 
			{orgId} 
			onSuccess={() => {
				open = false;
				onSuccess();
			}} 
			onCancel={() => open = false} 
		/>
	</SheetContent>
</Sheet>
