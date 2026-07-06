<!--
  @component GuardianLinkSheet
  Side panel for linking/editing a guardian to a student.
-->
<script lang="ts">
import * as Sheet from "@orbis/ui-svelte/sheet";
import GuardianForm from "$lib/features/students/components/guardian-form.svelte";
import type { GuardianLink } from "@orbis/desktop-client";
import * as m from "$lib/paraglide/messages";

let {
  open = $bindable(false),
  orgId,
  studentId,
  editData,
  onSuccess,
}: {
  open?: boolean;
  orgId: string;
  studentId: string;
  editData?: GuardianLink | null;
  onSuccess?: () => void;
} = $props();

const isEdit = $derived(!!editData);

function handleSuccess() {
  open = false;
  onSuccess?.();
}
</script>

<Sheet.Root bind:open>
  <Sheet.Content side="right" class="w-[420px] sm:w-[480px] h-full p-0 flex flex-col overflow-hidden">
    <GuardianForm
      {orgId}
      {studentId}
      {editData}
      onSuccess={handleSuccess}
      onCancel={() => (open = false)}
    />
  </Sheet.Content>
</Sheet.Root>
