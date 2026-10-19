<!--
  @component SectionSubjectSheet
  Side panel for creating or editing a class-subject-teacher assignment.
-->
<script lang="ts">
import * as Sheet from "@orbis/ui/sheet";
import SectionSubjectForm from "$lib/features/academics/components/class-subject-form.svelte";
import type { SectionSubjectDto } from "$lib/api";

let {
  open = $bindable(false),
  orgId,
  classId = "",
  periodId = "",
  editData,
  onSuccess,
}: {
  open?: boolean;
  orgId: string;
  classId?: string;
  periodId?: string;
  editData?: SectionSubjectDto | null;
  onSuccess?: () => void;
} = $props();

function handleSuccess() {
  open = false;
  onSuccess?.();
}
</script>

<Sheet.Root bind:open>
  <Sheet.Content
    side="end"
    contentClass="data-[side=end]:sm:!max-w-md"
    class="flex h-full w-full flex-col gap-0 overflow-hidden p-0"
  >
    <SectionSubjectForm
      {orgId}
      fixedClassId={classId}
      fixedPeriodId={periodId}
      {editData}
      onSuccess={handleSuccess}
      onCancel={() => (open = false)}
    />
  </Sheet.Content>
</Sheet.Root>
