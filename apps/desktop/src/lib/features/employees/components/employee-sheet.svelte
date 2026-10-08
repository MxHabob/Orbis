<!--
  @component EmployeeSheet
  Slide-over panel for creating and editing employee dossiers in HR,
  matching the exact architecture of FeeTemplateSheet and Orbis Design System.
-->
<script lang="ts">
import * as Sheet from "@orbis/ui/sheet";
import type { EmployeeDetailDto } from "$lib/api/types";
import EmployeeForm from "./employee-form.svelte";

interface Props {
  open: boolean;
  orgId: string;
  mode?: "preview" | "create" | "edit";
  employee?: EmployeeDetailDto | null;
  onSuccess?: (emp: EmployeeDetailDto) => void;
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
  <Sheet.Content side="end" class="w-full sm:max-w-xl md:max-w-2xl lg:max-w-3xl h-full p-0 flex flex-col overflow-hidden">
    {#if open}
      <EmployeeForm
        {orgId}
        {mode}
        {employee}
        onSuccess={(emp) => {
          open = false;
          onSuccess?.(emp);
        }}
        onCancel={() => {
          open = false;
          onCancel?.();
        }}
      />
    {/if}
  </Sheet.Content>
</Sheet.Root>
