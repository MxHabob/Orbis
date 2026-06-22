<!--
@component SubjectCreateSheet
A slide-over Sheet component for adding a new curriculum subject.
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
import SubjectForm from "../forms/subject-form.svelte";
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
			<SheetTitle class="text-xl font-bold text-foreground">
				{m.subject_sheet_title()}
			</SheetTitle>
			<SheetDescription class="text-sm text-muted-foreground mt-1">
				{m.subject_sheet_subtitle()}
			</SheetDescription>
		</SheetHeader>

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
