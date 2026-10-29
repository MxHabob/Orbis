<!--
@component RolesPermissionsManager
Allows administrators to manage system and custom roles, configure granular permissions,
and inspect role assignments according to the Orbis IAM Domain model.
Powered by OrbisGrid and Design System Card primitives.
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api } from "$lib/api";
import { confirmStore } from "$lib/stores";
import { Plus, Refresh, Search, UserBadgeCheck } from "@orbis/icons";
import { toast } from "@orbis/ui";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Skeleton } from "@orbis/ui/skeleton";
import { fade } from "svelte/transition";
import { l } from "$lib/core/i18n";
import { useQueryClient } from "@tanstack/svelte-query";
import type { RoleDto } from "$lib/api/types";
import RoleCreationSheet from "./components/role-creation-sheet.svelte";
import { countRolePermissions } from "$lib/core/iam";
import {
  OrbisGrid,
  type OrbisColumnDef,
  type GridActionEvent,
  type ActionItem,
} from "@orbis/svelte-grid";

let { orgId }: { orgId: string } = $props();
const queryClient = useQueryClient();

// State
let roles = $state<RoleDto[]>([]);
let isLoading = $state(true);
let searchQuery = $state("");
let filterType = $state<"all" | "system" | "custom">("all");

// Sheet State
let isSheetOpen = $state(false);
let editingRole = $state<RoleDto | null>(null);
let isDuplicate = $state(false);

async function loadRoles() {
  isLoading = true;
  try {
    const fetchedRoles = await api.listRoles();
    roles = fetchedRoles;
  } catch (e) {
    console.error("Error loading roles:", e);
    toast.error(m.ui_copy_failed_to_load_roles_445d4a19());
  } finally {
    isLoading = false;
  }
}

onMount(loadRoles);

// Statistics
const totalCount = $derived(roles.length);
const systemCount = $derived(roles.filter((r) => r.is_system).length);
const customCount = $derived(roles.filter((r) => !r.is_system).length);

// Filtered Data Source for OrbisGrid
const filteredRoles = $derived.by(() => {
  let list = roles;
  if (filterType === "system") {
    list = list.filter((r) => r.is_system);
  } else if (filterType === "custom") {
    list = list.filter((r) => !r.is_system);
  }
  if (searchQuery.trim()) {
    const q = searchQuery.toLowerCase().trim();
    list = list.filter(
      (r) =>
        r.id.toLowerCase().includes(q) ||
        r.name.toLowerCase().includes(q) ||
        (r.name_ar && r.name_ar.toLowerCase().includes(q)),
    );
  }
  return list;
});

const gridSource = $derived(
  filteredRoles.map((r, i) => ({
    ...r,
    __idx: i + 1,
    permCount: countRolePermissions(r.permissions, r.id),
    roleType: r.is_system ? "system" : "custom",
    typeBadge: r.is_system
      ? m.ui_copy_system_e3980179()
      : m.common_onboarding_break_custom(),
    permBadge: `${countRolePermissions(r.permissions, r.id)} ${m.ui_copy_perms_dd19d499()}`,
    name_display: l(r.name_ar, r.name) || r.name || r.id,
  })),
);

// Actions Configuration
const roleActions: ActionItem[] = [
  { id: "edit", label: m.ui_copy_edit_permissions_bc90fdaf(), icon: "Pencil" },
  {
    id: "duplicate",
    label: m.ui_copy_duplicate_role_82f6bf4c(),
    icon: "Clipboard",
  },
  {
    id: "delete",
    label: m.ui_copy_delete_role_981a0e24(),
    icon: "Trash2",
    variant: "destructive",
    visible: (row) => !row.is_system,
  },
];

// Columns Definition
const columns: OrbisColumnDef[] = [
  {
    id: "__idx",
    prop: "__idx",
    header: "#",
    name: "#",
    width: 50,
    size: 50,
    readonly: true,
    pin: "left",
    align: "center",
  },
  {
    id: "id",
    prop: "id",
    header: m.ui_copy_role_id_efa8c46a(),
    name: m.ui_copy_role_id_efa8c46a(),
    width: 160,
    size: 160,
    readonly: true,
    pin: "left",
    cellClass: "font-mono font-bold text-foreground text-xs",
  },
  {
    id: "name_ar",
    prop: "name_ar",
    header: m.common_org_unit_name_ar(),
    name: m.common_org_unit_name_ar(),
    width: 200,
    size: 200,
    readonly: true,
    cellClass: "font-semibold text-xs text-foreground",
  },
  {
    id: "name",
    prop: "name",
    header: m.common_org_unit_name_en(),
    name: m.common_org_unit_name_en(),
    width: 200,
    size: 200,
    readonly: true,
    cellClass: "text-muted-foreground text-xs",
  },
  {
    id: "typeBadge",
    prop: "typeBadge",
    header: m.ui_copy_type_0d257656(),
    name: m.ui_copy_type_0d257656(),
    width: 110,
    size: 110,
    type: "badge",
    align: "center",
    readonly: true,
  },
  {
    id: "permBadge",
    prop: "permBadge",
    header: m.settings_permissions_col_permissions(),
    name: m.settings_permissions_col_permissions(),
    width: 120,
    size: 120,
    align: "center",
    readonly: true,
    cellClass: "font-mono text-xs text-muted-foreground",
  },
  {
    id: "actions",
    prop: "actions",
    header: m.common_je_col_actions(),
    name: m.ui_copy_actions_c48127a5(),
    width: 90,
    size: 90,
    type: "action",
    align: "center",
    pin: "right",
    readonly: true,
    meta: { items: roleActions },
  },
];

function handleGridAction(event: GridActionEvent) {
  const role = roles.find((candidate) => candidate.id === String(event.row.id));
  if (!role) return;

  if (event.actionId === "edit" || event.actionId === "dblclick") {
    openEditRole(role);
  } else if (event.actionId === "duplicate") {
    openDuplicateRole(role);
  } else if (event.actionId === "delete") {
    handleDeleteRole(role);
  }
}

function openCreateRole() {
  editingRole = null;
  isDuplicate = false;
  isSheetOpen = true;
}

function openEditRole(role: RoleDto) {
  editingRole = role;
  isDuplicate = false;
  isSheetOpen = true;
}

function openDuplicateRole(role: RoleDto) {
  let duplicateId = `${role.id}_copy`;
  let suffix = 2;
  while (roles.some((candidate) => candidate.id === duplicateId)) {
    duplicateId = `${role.id}_copy_${suffix++}`;
  }

  editingRole = {
    ...role,
    id: duplicateId,
    name: `${role.name} (${m.settings_ui_copy_suffix()})`,
    name_ar: role.name_ar
      ? `${role.name_ar} (${m.settings_ui_copy_suffix()})`
      : undefined,
    is_system: false,
  };
  isDuplicate = true;
  isSheetOpen = true;
}

async function handleDeleteRole(roleToDelete: RoleDto) {
  if (roleToDelete.is_system) {
    toast.error(m.ui_copy_system_roles_cannot_be_deleted_78a2db52());
    return;
  }

  const confirmed = await confirmStore.prompt({
    title: m.common_action_delete(),
    description: m.settings_ui_role_delete_confirm({
      role: roleToDelete.name_ar || roleToDelete.name,
    }),
    confirmText: m.common_action_delete(),
    variant: "destructive",
  });

  if (!confirmed) return;

  try {
    await api.deleteRole(roleToDelete.id);
    await queryClient.invalidateQueries({ queryKey: ["roles"] });
    roles = roles.filter((r) => r.id !== roleToDelete.id);
    toast.success(m.ui_copy_role_deleted_successfully_df286726());
  } catch (e: any) {
    toast.error(e?.message || m.ui_copy_failed_to_delete_role_50ad627e());
  }
}

async function handleRoleSaved() {
  await queryClient.invalidateQueries({ queryKey: ["roles"] });
  await loadRoles();
}
</script>

<div class="h-full w-full overflow-hidden flex flex-col p-6 gap-4 text-start">
  <!-- Top Settings Toolbar -->
  <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3 shrink-0">
    <div class="flex items-center gap-2">
      <!-- Search Input -->
      <div class="relative w-48 sm:w-64">
        <Search size={14} class="absolute start-2.5 top-2.5 text-muted-foreground" />
        <Input
          type="search"
          placeholder={m.ui_copy_search_roles_9a3861a1()}
          class="ps-8 h-9 text-xs bg-card"
          bind:value={searchQuery}
        />
      </div>

      <!-- Filter Pill Buttons -->
      <div class="flex rounded-lg border border-border/60 p-0.5 bg-card text-xs">
        <button
          type="button"
          class="px-2.5 py-1 rounded-md text-xs font-medium transition-all cursor-pointer {filterType === 'all' ? 'bg-primary text-primary-foreground font-bold shadow-xs' : 'text-muted-foreground hover:text-foreground'}"
          onclick={() => (filterType = "all")}
        >
          {m.common_filter_all()} ({totalCount})
        </button>
        <button
          type="button"
          class="px-2.5 py-1 rounded-md text-xs font-medium transition-all cursor-pointer {filterType === 'system' ? 'bg-primary text-primary-foreground font-bold shadow-xs' : 'text-muted-foreground hover:text-foreground'}"
          onclick={() => (filterType = "system")}
        >
          {m.ui_copy_system_9f6abbe1()} ({systemCount})
        </button>
        <button
          type="button"
          class="px-2.5 py-1 rounded-md text-xs font-medium transition-all cursor-pointer {filterType === 'custom' ? 'bg-primary text-primary-foreground font-bold shadow-xs' : 'text-muted-foreground hover:text-foreground'}"
          onclick={() => (filterType = "custom")}
        >
          {m.common_onboarding_break_custom()} ({customCount})
        </button>
      </div>
    </div>

    <!-- Actions -->
    <div class="flex items-center gap-2">
      <Button
        variant="outline"
        size="icon"
        onclick={loadRoles}
        class="h-9 w-9 cursor-pointer"
        title={m.common_control_refresh()}
      >
        {#if isLoading}<LoadingIndicator  size={13} />{:else}<Refresh size={13}   />{/if}
      </Button>

      <Button
        variant="lime"
        size="sm"
        onclick={openCreateRole}
        class="h-9 text-xs flex items-center gap-1.5 font-bold cursor-pointer"
      >
        <Plus size={14} />
        <span>{m.ui_copy_add_role_00aeff97()}</span>
      </Button>
    </div>
  </div>

  <!-- OrbisGrid Content -->
  <div class="flex-1 min-h-0 w-full overflow-hidden rounded-xl border border-border/70 bg-card shadow-xs">
    {#if isLoading}
      <div class="p-6 flex flex-col gap-3">
        {#each Array(6) as _, i (i)}
          <Skeleton class="h-10 w-full rounded-md" />
        {/each}
      </div>
    {:else if filteredRoles.length === 0}
      <div class="p-16 flex flex-col items-center justify-center text-center gap-3 select-none">
        <div class="size-14 rounded-full bg-primary/10 flex items-center justify-center text-primary mb-2">
          <UserBadgeCheck size={28} />
        </div>
        <h3 class="text-sm font-bold text-foreground">
          {m.ui_copy_no_matching_roles_found_f955bb5e()}
        </h3>
        <p class="text-xs text-muted-foreground max-w-xs">
          {searchQuery ? m.ui_copy_try_adjusting_your_search_query_or_filter_07bb19e0() : m.ui_copy_you_can_create_a_new_custom_role_2bcf83d1()}
        </p>
      </div>
    {:else}
      <div in:fade={{ duration: 150 }} class="h-full w-full">
        <OrbisGrid
          {columns}
          source={gridSource}
          onaction={handleGridAction}
          loading={isLoading}
          class="h-full border-0"
        />
      </div>
    {/if}
  </div>
</div>


<!-- Role Creation / Pencil Sheet -->
<RoleCreationSheet
  bind:open={isSheetOpen}
  {editingRole}
  {isDuplicate}
  onsave={handleRoleSaved}
/>
