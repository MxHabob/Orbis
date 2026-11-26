<!--
@component SubjectCreateSheet
A slide-over Sheet component for adding a new curriculum subject.
Uses the shared @orbis/ui/sheet component.
-->
<script lang="ts">
import { Sheet, SheetContent, SheetHeader, SheetTitle } from "@orbis/ui/sheet";
import SubjectForm from "./subject-form.svelte";
import * as m from "$lib/paraglide/messages";
import type { SubjectDto } from "$lib/api";

let {
  open = $bindable(false),
  orgId,
  subject,
  onSuccess,
}: {
  open: boolean;
  orgId: string;
  subject?: SubjectDto | null;
  onSuccess: () => void;
} = $props();
</script>

<Sheet bind:open>
	<SheetContent
		contentClass="data-[side=end]:sm:!max-w-md"
		class="flex flex-col h-full overflow-hidden p-0 gap-0"
	>
		{#key subject?.id ?? "__create_subject__"}
		<SubjectForm
			{orgId}
			{subject}
			onSuccess={() => {
				open = false;
				onSuccess();
			}} 
			onCancel={() => open = false} 
		/>
		{/key}
	</SheetContent>
</Sheet>
