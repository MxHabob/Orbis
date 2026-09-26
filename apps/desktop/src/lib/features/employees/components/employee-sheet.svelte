<!--
  @component EmployeeSheet
  Slide-over panel for creating, editing, and previewing employee dossiers,
  matching the exact architecture of FeeTemplateSheet and Orbis Design System.
-->
<script lang="ts">
import * as Sheet from "@orbis/ui/sheet";
import type { EmployeeDetailDto, AccountProvisioningResultDto } from "$lib/api/types";
import EmployeeForm from "./employee-form.svelte";

interface Props {
  open: boolean;
  orgId: string;
  mode?: "preview" | "create" | "edit";
  employee?: EmployeeDetailDto | null;
  onSuccess?: (emp: EmployeeDetailDto, prov?: AccountProvisioningResultDto | null) => void;
  onCancel?: () => void;
}

let {
  open = $bindable(false),
  orgId,
  mode = $bindable("create"),
  employee = null,
  onSuccess,
  onCancel,
}: Props = $props();
</script>

<Sheet.Root bind:open>
  <Sheet.Content side="right" class="w-full sm:max-w-xl md:max-w-2xl lg:max-w-3xl h-full p-0 flex flex-col overflow-hidden">
    {#if open}
      <EmployeeForm
        {orgId}
        {mode}
        {employee}
        onSuccess={(emp, prov) => {
          open = false;
          onSuccess?.(emp, prov);
        }}
        onCancel={() => {
          open = false;
          onCancel?.();
        }}
      />
    {/if}
  </Sheet.Content>
</Sheet.Root>
