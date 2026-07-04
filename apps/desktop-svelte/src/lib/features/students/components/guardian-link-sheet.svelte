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
  <Sheet.Content side="right" class="w-[420px] sm:w-[480px] flex flex-col gap-0 p-0">
    <Sheet.Header class="px-6 py-4 border-b border-border/40 shrink-0">
      <Sheet.Title class="text-base font-bold">
        {isEdit ? m.guardians_sheet_edit_title() : m.guardians_sheet_create_title()}
      </Sheet.Title>
      <Sheet.Description class="text-xs text-muted-foreground">
        {m.guardians_desc()}
      </Sheet.Description>
    </Sheet.Header>

    <div class="flex-1 overflow-y-auto p-6 scrollbar-hide">
      <GuardianForm
        {orgId}
        {studentId}
        {editData}
        onSuccess={handleSuccess}
        onCancel={() => (open = false)}
      />
    </div>
  </Sheet.Content>
</Sheet.Root>
