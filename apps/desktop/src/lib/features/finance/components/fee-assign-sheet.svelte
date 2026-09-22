<!--
@component FeeAssignSheet
A slide-over Sheet component for assigning a fee to a student.
Uses the shared @orbis/ui/sheet component.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Sheet, SheetContent, SheetHeader, SheetTitle } from "@orbis/ui/sheet";
import { Document } from "@orbis/icons";
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
				<Document size={18} class="text-primary" />
				<span>{m.common_fee_sheet_title()}</span>
			</SheetTitle>
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
