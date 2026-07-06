<!--
@component ClassCreateSheet
A slide-over Sheet component for creating a new academic class.
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
import ClassForm from "./class-form.svelte";
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
	<SheetContent class="flex flex-col h-full sm:max-w-md overflow-hidden p-0 gap-0">
		<ClassForm 
			{orgId} 
			onSuccess={() => {
				open = false;
				onSuccess();
			}} 
			onCancel={() => open = false} 
		/>
	</SheetContent>
</Sheet>
