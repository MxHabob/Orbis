<!--
  @component ClassSubjectSheet
  Side panel for creating or editing a class-subject-teacher assignment.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import * as Sheet from "@orbis/ui-svelte/sheet";
import ClassSubjectForm from "$lib/features/academics/components/class-subject-form.svelte";
import type { ClassSubject } from "@orbis/sdk-core";

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

const isEdit = $derived(!!editData);

function handleSuccess() {
  open = false;
  onSuccess?.();
}
</script>

<Sheet.Root bind:open>
  <Sheet.Content side="right" class="w-[420px] sm:w-[480px] flex flex-col gap-0 p-0">
    <Sheet.Header class="px-6 py-4 border-b border-border/40 shrink-0">
      <Sheet.Title class="text-base font-bold">
        {isEdit ? m.common_action_edit() : m.common_cs_sheet_create_title()}
      </Sheet.Title>
      <Sheet.Description class="text-xs text-muted-foreground">
        {m.common_cs_desc()}
      </Sheet.Description>
    </Sheet.Header>

    <div class="flex-1 overflow-y-auto p-6 scrollbar-hide">
      <ClassSubjectForm
        {orgId}
        fixedClassId={classId}
        {editData}
        onSuccess={handleSuccess}
        onCancel={() => (open = false)}
      />
    </div>
  </Sheet.Content>
</Sheet.Root>
