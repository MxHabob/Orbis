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
import AssessmentForm from "./assessment-form.svelte";
import * as m from "$lib/paraglide/messages";

let {
  open = $bindable(false),
  classId,
  subjectId,
  periodId,
  month,
  onSuccess,
}: {
  open: boolean;
  classId: string;
  subjectId: string;
  periodId: string;
  month?: number;
  onSuccess: () => void;
} = $props();
</script>

<Sheet bind:open>
	<SheetContent class="flex flex-col h-full sm:max-w-md overflow-hidden p-0 gap-0">
		<AssessmentForm 
			{classId} 
			{subjectId} 
			{periodId} 
			defaultMonth={month}
			onSuccess={() => {
				open = false;
				onSuccess();
			}} 
			onCancel={() => open = false} 
		/>
	</SheetContent>
</Sheet>
