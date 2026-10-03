<!--
  @component RoleSelect
  Dynamic Role Selector component powered by TanStack Query & SearchableSelect.
  Fetches authoritative organization roles from the backend rather than using hardcoded lists.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { l } from "$lib/core/i18n";
import { api, type RoleDto } from "$lib/api";
import SearchableSelect from "$lib/shared/components/searchable-select.svelte";
import { createQuery } from "@tanstack/svelte-query";

interface Props {
  value?: string | null;
  allowAll?: boolean;
  allLabel?: string;
  placeholder?: string;
  disabled?: boolean;
  size?: "sm" | "default";
  class?: string;
  id?: string;
  onValueChange?: (val: string) => void;
  onchange?: (val: string | null) => void;
}

let {
  value = $bindable(""),
  allowAll = false,
  allLabel = m.common_filter_all(),
  placeholder = m.common_select_role(),
  disabled = false,
  size = "default",
  class: className = "",
  id = undefined,
  onValueChange,
  onchange,
}: Props = $props();

const rolesQuery = createQuery<RoleDto[]>(() => ({
  queryKey: ["roles"],
  queryFn: () => api.listRoles(),
  staleTime: 30 * 1000,
}));

const roles = $derived(rolesQuery.data ?? []);

function getRoleValue(r: RoleDto): string {
  return String(r.id || "");
}

function getRoleLabel(r: RoleDto): string {
  return l(r.name_ar, r.name) || r.name || String(r.id || "");
}

function getRoleSubtitle(r: RoleDto): string | undefined {
  return r.description || undefined;
}

function getRoleBadge(r: RoleDto): string | undefined {
  return String(r.id || "");
}

function handleChange(val: string | null) {
  const next = val || "";
  value = next;
  onValueChange?.(next);
  onchange?.(val);
}
</script>

<SearchableSelect
  bind:value
  items={roles}
  placeholder={rolesQuery.isLoading ? "جارٍ تحميل الأدوار..." : placeholder}
  searchPlaceholder="ابحث عن اسم الدور الوظيفي أو الرمز..."
  emptyText="لا توجد أدوار مطابقة للبحث"
  showAllOption={allowAll}
  {allLabel}
  disabled={disabled || rolesQuery.isLoading}
  {size}
  {id}
  getItemValue={getRoleValue}
  getItemLabel={getRoleLabel}
  getItemSubtitle={getRoleSubtitle}
  getItemBadge={getRoleBadge}
  onchange={handleChange}
  class={className}
/>
