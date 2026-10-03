<!--
@component RolesPermissionsManager
Allows administrators to manage system and custom roles, configure granular permissions,
and inspect role assignments according to the Orbis IAM Domain model.
Powered by OrbisGrid and Design System Card primitives.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api } from "$lib/api";
import { confirmStore } from "$lib/stores";
import { Clipboard, Pencil, Plus, Refresh, Search, Trash, UserBadgeCheck } from "@orbis/icons";
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

async function loadRoles() {
  isLoading = true;
  try {
    const fetchedRoles = await api.listRoles();
    roles = fetchedRoles;
  } catch (e) {
    console.error("Error loading roles:", e);
    toast.error(l("فشل تحميل قائمة الأدوار والصلاحيات", "Failed to load roles"));
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
        (r.name_ar && r.name_ar.toLowerCase().includes(q))
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
    typeBadge: r.is_system ? l("نظام قياسي", "System") : l("مخصص", "Custom"),
    permBadge: `${countRolePermissions(r.permissions, r.id)} ${l("صلاحية", "perms")}`,
    name_display: l(r.name_ar, r.name) || r.name || r.id,
  }))
);

// Actions Configuration
const roleActions: ActionItem[] = [
  { id: "edit", label: l("تعديل الصلاحيات", "Edit Permissions"), icon: "Pencil" },
  { id: "duplicate", label: l("نسخ الدور", "Duplicate Role"), icon: "Clipboard" },
  { id: "delete", label: l("حذف الدور", "Delete Role"), icon: "Trash2", variant: "destructive" },
];

// Columns Definition
const columns: OrbisColumnDef[] = [
  { id: "__idx", prop: "__idx", header: "#", name: "#", width: 50, size: 50, readonly: true, pin: "left", align: "center" },
  { id: "id", prop: "id", header: l("المعرف", "Role ID"), name: "Role ID", width: 160, size: 160, readonly: true, pin: "left", cellClass: "font-mono font-bold text-foreground text-xs" },
  { id: "name_ar", prop: "name_ar", header: l("الاسم بالعربية", "Arabic Name"), name: "Arabic Name", width: 200, size: 200, readonly: true, cellClass: "font-semibold text-xs text-foreground" },
  { id: "name", prop: "name", header: l("الاسم بالإنجليزية", "English Name"), name: "English Name", width: 200, size: 200, readonly: true, cellClass: "text-muted-foreground text-xs" },
  { id: "typeBadge", prop: "typeBadge", header: l("نوع الدور", "Type"), name: "Type", width: 110, size: 110, type: "badge", align: "center", readonly: true },
  { id: "permBadge", prop: "permBadge", header: l("الصلاحيات", "Permissions"), name: "Permissions", width: 120, size: 120, align: "center", readonly: true, cellClass: "font-mono text-xs text-muted-foreground" },
  {
    id: "actions",
    prop: "actions",
    header: m.common_je_col_actions ? m.common_je_col_actions() : "الإجراءات",
    name: "Actions",
    width: 90,
    size: 90,
    type: "actions",
    align: "center",
    pin: "right",
    actions: roleActions,
  },
];

function handleGridAction(event: GridActionEvent) {
  const { action, row } = event;
  const original = roles.find((r) => r.id === row.id) || row;
  if (action === "edit") {
    openEditRole(original as RoleDto);
  } else if (action === "duplicate") {
    openDuplicateRole(original as RoleDto);
  } else if (action === "delete") {
    handleDeleteRole(original as RoleDto);
  }
}

function openCreateRole() {
  editingRole = null;
  isSheetOpen = true;
}

function openEditRole(role: RoleDto) {
  editingRole = role;
  isSheetOpen = true;
}

function openDuplicateRole(role: RoleDto) {
  editingRole = {
    ...role,
    id: `${role.id}_copy`,
    name: `${role.name} (Clipboard)`,
    name_ar: role.name_ar ? `${role.name_ar} (نسخة)` : undefined,
    is_system: false,
  };
  isSheetOpen = true;
}

async function handleDeleteRole(roleToDelete: RoleDto) {
  if (roleToDelete.is_system) {
    toast.error(l("لا يمكن حذف أدوار النظام الأساسية", "System roles cannot be deleted"));
    return;
  }

  const confirmed = await confirmStore.prompt({
    title: (m.common_action_delete ? m.common_action_delete() : "حذف") + "؟",
    description: l(
      `هل أنت متأكد من رغبتك في حذف الدور "${roleToDelete.name_ar || roleToDelete.name}"؟`,
      `Are you sure you want to delete role "${roleToDelete.name}"?`
    ),
    confirmText: m.common_action_delete ? m.common_action_delete() : "حذف",
    variant: "destructive",
  });

  if (!confirmed) return;

  try {
    await api.deleteRole(roleToDelete.id);
    await queryClient.invalidateQueries({ queryKey: ["roles"] });
    roles = roles.filter((r) => r.id !== roleToDelete.id);
    toast.success(l("تم حذف الدور بنجاح", "Role deleted successfully"));
  } catch (e: any) {
    toast.error(e?.message || l("فشل حذف الدور", "Failed to delete role"));
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
          placeholder={l("بحث عن دور...", "Search roles...")}
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
          {l("الكل", "All")} ({totalCount})
        </button>
        <button
          type="button"
          class="px-2.5 py-1 rounded-md text-xs font-medium transition-all cursor-pointer {filterType === 'system' ? 'bg-primary text-primary-foreground font-bold shadow-xs' : 'text-muted-foreground hover:text-foreground'}"
          onclick={() => (filterType = "system")}
        >
          {l("نظام", "System")} ({systemCount})
        </button>
        <button
          type="button"
          class="px-2.5 py-1 rounded-md text-xs font-medium transition-all cursor-pointer {filterType === 'custom' ? 'bg-primary text-primary-foreground font-bold shadow-xs' : 'text-muted-foreground hover:text-foreground'}"
          onclick={() => (filterType = "custom")}
        >
          {l("مخصص", "Custom")} ({customCount})
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
        title={m.common_control_refresh ? m.common_control_refresh() : "تحديث"}
      >
        <Refresh size={13} class={isLoading ? "animate-spin" : ""} />
      </Button>

      <Button
        variant="lime"
        size="sm"
        onclick={openCreateRole}
        class="h-9 text-xs flex items-center gap-1.5 font-bold cursor-pointer"
      >
        <Plus size={14} />
        <span>{l("إضافة دور", "Add Role")}</span>
      </Button>
    </div>
  </div>

  <!-- OrbisGrid Content -->
  <div class="flex-1 min-h-0 w-full overflow-hidden rounded-xl border border-border/70 bg-card shadow-xs">
    {#if isLoading}
      <div class="p-6 flex flex-col gap-3">
        {#each Array(6) as _}
          <Skeleton class="h-10 w-full rounded-md" />
        {/each}
      </div>
    {:else if filteredRoles.length === 0}
      <div class="p-16 flex flex-col items-center justify-center text-center gap-3 select-none">
        <div class="size-14 rounded-full bg-primary/10 flex items-center justify-center text-primary mb-2">
          <UserBadgeCheck size={28} />
        </div>
        <h3 class="text-sm font-bold text-foreground">
          {l("لا توجد أدوار مطابقة للبحث", "No matching roles found")}
        </h3>
        <p class="text-xs text-muted-foreground max-w-xs">
          {searchQuery ? l("جرب تعديل عبارة البحث أو الفلتر", "Try adjusting your search query or filter") : l("يمكنك البدء بإضافة دور مخصص جديد للنظام", "You can create a new custom role")}
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
  onsave={handleRoleSaved}
/>
