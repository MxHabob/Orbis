<!--
  @component ClassSubjectSheet
  Side panel for creating or editing a class-subject-teacher assignment.
-->
<script lang="ts">
import * as Sheet from "@orbis/ui/sheet";
import ClassSubjectForm from "$lib/features/academics/components/class-subject-form.svelte";
import type { ClassSubject } from "@orbis/sdk";

let {
  open = $bindable(false),
  orgId,
  classId = "",
  editData,
  onSuccess,
}: {
  open?: boolean;
  orgId: string;
  classId?: string;
  editData?: ClassSubject | null;
  onSuccess?: () => void;
} = $props();

function handleSuccess() {
  open = false;
  onSuccess?.();
}
</script>

<Sheet.Root bind:open>
  <Sheet.Content side="right" class="flex h-full w-full flex-col gap-0 overflow-hidden p-0 sm:max-w-md">
    <ClassSubjectForm
      {orgId}
      fixedClassId={classId}
      {editData}
      onSuccess={handleSuccess}
      onCancel={() => (open = false)}
    />
  </Sheet.Content>
</Sheet.Root>
