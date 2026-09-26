<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { EmployeeForm, createEmployeeQuery } from "$lib/features/employees";
import { ArrowPath } from "@orbis/icons";
import { l } from "$lib/core/i18n";
import { extractErrorMessage } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { goto } from "$app/navigation";
import { localizeHref } from "$lib/paraglide/runtime";

const orgId = $derived(page.params.orgId || "");
const employeeId = $derived(page.params.id || "");

const employeeQuery = createEmployeeQuery(() => employeeId);
</script>

<svelte:head>
  <title>{m.employees_edit_title ? m.employees_edit_title() : "Edit Employee"} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  {#if employeeQuery.isLoading}
    <div class="h-full w-full flex flex-col items-center justify-center gap-3 text-muted-foreground">
      <ArrowPath size={24} class="animate-spin text-primary" />
      <span class="text-xs">{l("جاري تحميل بيانات الموظف...", "Loading employee...")}</span>
    </div>
  {:else if employeeQuery.error || !employeeQuery.data}
    <div class="h-full w-full flex flex-col items-center justify-center p-6 text-center gap-3">
      <p class="text-xs text-destructive font-semibold">{extractErrorMessage(employeeQuery.error) || "Employee not found"}</p>
      <Button variant="outline" size="sm" class="text-xs" onclick={() => goto(localizeHref(`/${orgId}/employees`))}>
        {l("العودة لدليل الموظفين", "Back to Directory")}
      </Button>
    </div>
  {:else}
    <EmployeeForm {orgId} mode="edit" employee={employeeQuery.data} />
  {/if}
</div>
