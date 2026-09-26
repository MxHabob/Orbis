<!--
  @component RoleSelect
  Dynamic Role Selector component. Fetches authoritative organization roles
  from the backend rather than using hardcoded lists.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { l } from "$lib/core/i18n";
import { api, type RoleDto } from "$lib/api";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { createQuery } from "@tanstack/svelte-query";

interface Props {
  value?: string;
  allowAll?: boolean;
  allLabel?: string;
  placeholder?: string;
  disabled?: boolean;
  size?: "sm" | "default";
  class?: string;
  onValueChange?: (val: string) => void;
}

let {
  value = $bindable(""),
  allowAll = false,
  allLabel = m.common_filter_all?.() || "جميع الأدوار",
  placeholder = m.common_select_role?.() || "اختر الدور الوظيفي...",
  disabled = false,
  size = "default",
  class: className = "",
  onValueChange,
}: Props = $props();

const rolesQuery = createQuery<RoleDto[]>(() => ({
  queryKey: ["roles"],
  queryFn: () => api.listRoles(),
  staleTime: 30 * 1000,
}));

const roles = $derived(rolesQuery.data ?? []);

function getRoleLabel(roleId: string): string {
  if (allowAll && (roleId === "all" || roleId === "")) {
    return allLabel;
  }
  const match = roles.find((r) => r.id === roleId);
  if (match) {
    return l(match.name_ar, match.name) || match.name || match.id;
  }
  return roleId || placeholder;
}

function handleChange(v: string | undefined) {
  const next = v ?? "";
  value = next;
  onValueChange?.(next);
}
</script>

<Select
  type="single"
  value={value || (allowAll ? "all" : undefined)}
  onValueChange={handleChange}
  disabled={disabled || rolesQuery.isLoading}
>
  <SelectTrigger class="{size === 'sm' ? 'h-8 text-xs' : 'h-9 text-xs'} bg-card {className}">
    <span>{getRoleLabel(value)}</span>
  </SelectTrigger>
  <SelectContent class="text-xs font-sans max-h-56">
    {#if allowAll}
      <SelectItem value="all">{allLabel}</SelectItem>
    {/if}
    {#each roles as r (r.id)}
      <SelectItem value={r.id}>
        <div class="flex items-center justify-between w-full gap-2">
          <span>{l(r.name_ar, r.name) || r.name}</span>
          <span class="text-[10px] font-mono text-muted-foreground opacity-70">({r.id})</span>
        </div>
      </SelectItem>
    {/each}
  </SelectContent>
</Select>
