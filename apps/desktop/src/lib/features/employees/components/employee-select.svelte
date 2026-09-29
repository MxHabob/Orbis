<!--
@component EmployeeSelect
Self-fetching Searchable Employee Selector powered by TanStack Query & Employee Affairs domain.
Includes a sticky search bar, subtitle with job title/employee number, and support for Orbis Design System.
-->
<script lang="ts">
import { page } from "$app/state";
import { l } from "$lib/core/i18n";
import { createEmployeesQuery } from "$lib/features/employees/queries";
import SearchableSelect from "$lib/shared/components/searchable-select.svelte";
import { authStore } from "$lib/stores";

interface Props {
  value?: string | null;
  orgId?: string;
  placeholder?: string;
  searchPlaceholder?: string;
  disabled?: boolean;
  id?: string;
  showAllOption?: boolean;
  allOptionLabel?: string;
  createLabel?: string;
  onCreate?: () => void;
  class?: string;
  onchange?: (val: string | null) => void;
}

let {
  value = $bindable<string | null>(null),
  orgId,
  placeholder = l("اختر الموظف / المسؤول...", "Select employee / manager..."),
  searchPlaceholder = l("ابحث بالاسم، المسمى الوظيفي، أو الرقم...", "Search by name, job title, or ID..."),
  disabled = false,
  id = undefined,
  showAllOption = false,
  allOptionLabel = l("الكل", "All"),
  createLabel = undefined,
  onCreate = undefined,
  class: className = "",
  onchange = undefined,
}: Props = $props();

const resolvedOrgId = $derived(
  orgId || (page.params.orgId as string) || authStore.orgId || "default",
);

// TanStack Query for Active Employees
const employeesQuery = createEmployeesQuery(
  () => resolvedOrgId,
  () => ({ status: "active", limit: 300 }),
);

const employees = $derived(
  Array.isArray(employeesQuery.data)
    ? employeesQuery.data
    : Array.isArray((employeesQuery.data as any)?.items)
    ? (employeesQuery.data as any).items
    : []
);

function getEmployeeLabel(e: any): string {
  if (!e) return "";
  const arName = [e.first_name_ar, e.last_name_ar].filter(Boolean).join(" ");
  const enName = [e.first_name, e.last_name].filter(Boolean).join(" ");
  const localized = l(arName, enName);
  if (localized) return localized;
  return String(e.full_name || e.name || e.employee_number || e.id || "");
}

function getEmployeeSubtitle(e: any): string | undefined {
  const parts = [];
  if (e?.job_title) parts.push(e.job_title);
  if (e?.employee_number) parts.push(`#${e.employee_number}`);
  return parts.length > 0 ? parts.join(" • ") : undefined;
}
</script>

<SearchableSelect
  bind:value
  items={employees}
  {placeholder}
  {searchPlaceholder}
  emptyText={l("لا يوجد موظفون مطابقون", "No matching employees found")}
  {disabled}
  {id}
  {showAllOption}
  {allOptionLabel}
  {createLabel}
  {onCreate}
  class={className}
  getItemValue={(e) => String(e?.id ?? "")}
  getItemLabel={getEmployeeLabel}
  getItemSubtitle={getEmployeeSubtitle}
  {onchange}
/>
