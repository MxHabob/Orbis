<!--
  @component TeacherSelect
  Self-fetching Searchable Teacher Selector powered by TanStack Query & Employee Affairs repository.
  Includes a sticky search bar and optional "Add new teacher" creation button.
-->
<script lang="ts">
import { page } from "$app/state";
import { l } from "$lib/core/i18n";
import { createEmployeesQuery } from "$lib/features/employees/queries";
import * as m from "$lib/paraglide/messages";
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
  placeholder = m.common_cs_form_select_teacher?.() || "اختر المعلم...",
  searchPlaceholder = "ابحث بالاسم أو الرقم الوظيفي...",
  disabled = false,
  id = undefined,
  showAllOption = false,
  allOptionLabel = m.common_cs_form_all_teachers?.() || "جميع المعلمين",
  createLabel = "إضافة معلم جديد...",
  onCreate = undefined,
  class: className = "",
  onchange = undefined,
}: Props = $props();

const resolvedOrgId = $derived(
  orgId || page.params.orgId || authStore.orgId || "default",
);

// Self-contained TanStack Query
const teachersQuery = createEmployeesQuery(
  () => resolvedOrgId,
  () => ({ employee_type: "teacher", status: "active" }),
);

const teachers = $derived(teachersQuery.data?.items ?? []);

function getTeacherLabel(t: any): string {
  if (!t) return "";
  const arName = t.name_ar || t.nameAr || t.full_name;
  const enName = t.name_en || t.nameEn || t.full_name || t.email;
  const localized = l(arName, enName);
  if (localized) return localized;
  const composite = `${t.first_name || ""} ${t.last_name || ""}`.trim();
  if (composite) return composite;
  return String(t.full_name || t.email || t.employee_number || t.id || "");
}

function getTeacherSubtitle(t: any): string | undefined {
  if (t?.employee_number) return `#${t.employee_number}`;
  if (t?.job_title) return t.job_title;
  if (t?.department) return t.department;
  return undefined;
}
</script>

<SearchableSelect
  bind:value
  items={teachers}
  {placeholder}
  {searchPlaceholder}
  emptyText={m.common_cs_form_no_teachers?.() || "لا يوجد معلمون مطابقون"}
  {disabled}
  {id}
  {showAllOption}
  {allOptionLabel}
  {createLabel}
  {onCreate}
  class={className}
  getItemValue={(t) => String(t?.id ?? "")}
  getItemLabel={getTeacherLabel}
  getItemSubtitle={getTeacherSubtitle}
  {onchange}
/>
