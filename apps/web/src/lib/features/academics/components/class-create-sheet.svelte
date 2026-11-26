<!--
@component ClassCreateSheet
A slide-over Sheet component for creating a new academic class.
Uses the shared @orbis/ui/sheet component.
-->
<script lang="ts">
import { Sheet, SheetContent, SheetHeader, SheetTitle } from "@orbis/ui/sheet";
import ClassForm from "./class-form.svelte";
import * as m from "$lib/paraglide/messages";
import type { SectionDto } from "$lib/api";

let {
  open = $bindable(false),
  orgId,
  section,
  onSuccess,
}: {
  open: boolean;
  orgId: string;
  section?: SectionDto | null;
  onSuccess: () => void;
} = $props();
</script>

<Sheet bind:open>
	<SheetContent
		contentClass="data-[side=end]:sm:!max-w-md"
		class="flex flex-col h-full overflow-hidden p-0 gap-0"
	>
		{#key section?.id ?? "__create__"}
			<ClassForm
				{orgId}
				{section}
				onSuccess={() => {
					open = false;
					onSuccess();
				}}
				onCancel={() => (open = false)}
			/>
		{/key}
	</SheetContent>
</Sheet>
