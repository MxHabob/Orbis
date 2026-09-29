<!--
@component BranchSheet
A slide-over Sheet component for creating or editing an organizational branch / campus.
-->
<script lang="ts">
import { Sheet, SheetContent } from "@orbis/ui/sheet";
import BranchForm from "./branch-form.svelte";
import type { OrgUnitDto } from "$lib/api";

let {
  open = $bindable(false),
  orgId,
  branch = null,
  onSuccess,
}: {
  open: boolean;
  orgId: string;
  branch?: OrgUnitDto | null;
  onSuccess: (unit: OrgUnitDto) => void;
} = $props();
</script>

<Sheet bind:open>
  <SheetContent class="flex flex-col h-full sm:max-w-2xl overflow-hidden p-0 gap-0">
    <BranchForm
      {orgId}
      {branch}
      onSuccess={(unit) => {
        open = false;
        onSuccess(unit);
      }}
      onCancel={() => (open = false)}
    />
  </SheetContent>
</Sheet>
