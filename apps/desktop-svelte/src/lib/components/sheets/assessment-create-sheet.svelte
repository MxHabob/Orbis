<!--
@component AssessmentCreateSheet
A slide-over Sheet component for creating a new class assessment (exam, quiz, homework).
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
import { Award } from "lucide-svelte";
import AssessmentForm from "../forms/assessment-form.svelte";
import * as m from "$lib/paraglide/messages";

let {
  open = $bindable(false),
  classId,
  subjectId,
  cycleId,
  month,
  onSuccess,
}: {
  open: boolean;
  classId: string;
  subjectId: string;
  cycleId: string;
  month?: number;
  onSuccess: () => void;
} = $props();
</script>

<Sheet bind:open>
	<SheetContent class="flex flex-col h-full sm:max-w-md">
		<SheetHeader class="text-start select-none">
			<SheetTitle class="text-xl font-bold text-foreground flex items-center gap-2">
				<Award size={18} class="text-primary" />
				<span>{m.assessment_sheet_title()}</span>
			</SheetTitle>
			<SheetDescription class="text-sm text-muted-foreground mt-1">
				{m.assessment_sheet_subtitle()}
			</SheetDescription>
		</SheetHeader>

		<AssessmentForm 
			{classId} 
			{subjectId} 
			{cycleId} 
			defaultMonth={month}
			onSuccess={() => {
				open = false;
				onSuccess();
			}} 
			onCancel={() => open = false} 
		/>
	</SheetContent>
</Sheet>
