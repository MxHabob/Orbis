<!--
@component SubjectCreateSheet
A slide-over Sheet component for adding a new curriculum subject.
Uses the shared @orbis/ui/sheet component.
-->
<script lang="ts">
import { Sheet, SheetContent, SheetHeader, SheetTitle } from "@orbis/ui/sheet";
import SubjectForm from "./subject-form.svelte";
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
	<SheetContent
		contentClass="data-[side=end]:sm:max-w-md"
		class="flex flex-col h-full overflow-hidden p-0 gap-0"
	>
		<SubjectForm 
			{orgId} 
			onSuccess={() => {
				open = false;
				onSuccess();
			}} 
			onCancel={() => open = false} 
		/>
	</SheetContent>
</Sheet>
