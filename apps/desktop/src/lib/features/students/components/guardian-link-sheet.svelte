<!--
  @component GuardianLinkSheet
  Side panel for linking/editing a guardian to a student.
-->
<script lang="ts">
import * as Sheet from "@orbis/ui/sheet";
import GuardianForm from "$lib/features/students/components/guardian-form.svelte";
import type { GuardianDto } from "$lib/api";

let {
  open = $bindable(false),
  orgId,
  studentId,
  editData = null,
  onSuccess,
}: {
  open?: boolean;
  orgId: string;
  studentId: string;
  editData?: GuardianDto | null;
  onSuccess?: () => void;
} = $props();

function handleSuccess() {
  open = false;
  onSuccess?.();
}
</script>

<Sheet.Root bind:open>
  <Sheet.Content side="right" class="w-105 sm:w-120 h-full p-0 flex flex-col overflow-hidden">
    <GuardianForm
      {orgId}
      {studentId}
      {editData}
      onSuccess={handleSuccess}
      onCancel={() => (open = false)}
    />
  </Sheet.Content>
</Sheet.Root>
