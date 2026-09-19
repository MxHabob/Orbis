<!--
  @page PermissionsSettingsPage
  Allows administrators to manage system and custom roles, configure granular permissions,
  and inspect role assignments according to the Orbis IAM Domain model.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api } from "$lib/api";
import { confirmStore } from "$lib/stores";
import {
  Plus,
  RefreshCw,
  Edit2,
  Trash2,
  Shield,
  Check,
  Copy,
} from "@lucide/svelte";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
  toast,
} from "@orbis/ui";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import { Skeleton } from "@orbis/ui/skeleton";
import { fade } from "svelte/transition";
import { l } from "$lib/core/i18n";
import type { RoleDto } from "$lib/api/types";

import RoleCreationSheet from "$lib/features/settings/components/role-creation-sheet.svelte";
import {
  SYSTEM_ROLE_DEFINITIONS,
  countRolePermissions,
  isSystemRole,
} from "$lib/core/iam";

let { data }: { data: { orgId: string } } = $props();

// State
let roles = $state<RoleDto[]>([]);
let isLoading = $state(true);

// Sheet State
let isSheetOpen = $state(false);
let editingRole = $state<RoleDto | null>(null);

async function loadRoles() {
  isLoading = true;
  try {
    const fetchedRoles = await api.listRoles().catch(() => []);

    // Merge system role definitions if not already in DB
    const merged: RoleDto[] = [...fetchedRoles];
    for (const sys of SYSTEM_ROLE_DEFINITIONS) {
      if (!merged.some((r) => r.id === sys.id)) {
        merged.push({
          id: sys.id,
          name: sys.nameEn,
          name_ar: sys.nameAr,
          is_system: true,
          permissions: sys.defaultPermissions,
        });
      }
    }

    roles = merged;
  } catch (e) {
    console.error("Error loading roles:", e);
    toast.error(l("فشل تحميل قائمة الأدوار والصلاحيات", "Failed to load roles"));
  } finally {
    isLoading = false;
  }
}

onMount(loadRoles);

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
    name: `${role.name} (Copy)`,
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
    roles = roles.filter((r) => r.id !== roleToDelete.id);
    toast.success(l("تم حذف الدور بنجاح", "Role deleted successfully"));
  } catch (e: any) {
    toast.error(e?.message || l("فشل حذف الدور", "Failed to delete role"));
  }
}
</script>

<svelte:head>
  <title>{m.settings_nav_permissions ? m.settings_nav_permissions() : "الصلاحيات"} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 scrollbar-hide">
  <!-- Top Header Bar -->
  <div class="flex items-center justify-between gap-4 pb-5 select-none border-b border-border/40 font-sans">
    <div class="flex items-center gap-2">
      {#if !isLoading}
        <span class="text-[10px] px-2 py-0.5 rounded bg-muted font-mono text-muted-foreground font-bold">
          {roles.length} {l("أدوار", "roles")}
        </span>
      {/if}
    </div>

    <!-- Actions -->
    <div class="flex items-center gap-2">
      <Button
        variant="outline"
        size="sm"
        onclick={loadRoles}
        class="h-9 w-9 p-0 flex items-center justify-center cursor-pointer"
        title={m.common_control_refresh ? m.common_control_refresh() : "تحديث"}
      >
        <RefreshCw size={13} class={isLoading ? "animate-spin" : ""} />
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

  <!-- Content Section: Standard Table -->
  <div class="w-full">
    {#if isLoading}
      <div class="flex flex-col gap-3">
        {#each Array(4) as _}
          <Skeleton class="h-14 w-full rounded-lg" />
        {/each}
      </div>
    {:else if roles.length === 0}
      <!-- Empty state -->
      <div class="border border-dashed border-border/80 rounded-2xl p-16 flex flex-col items-center justify-center text-center gap-3 select-none my-4">
        <div class="w-14 h-14 rounded-full bg-primary/10 flex items-center justify-center text-primary mb-2">
          <Shield size={28} />
        </div>
        <h3 class="text-sm font-bold text-foreground">
          {l("لا توجد أدوار مسجلة", "No roles found")}
        </h3>
        <p class="text-xs text-muted-foreground max-w-xs">
          {l("يمكنك البدء بإضافة دور مخصص جديد للنظام", "You can create a new custom role")}
        </p>
      </div>
    {:else}
      <div in:fade={{ duration: 150 }} class="overflow-hidden">
        <Table>
          <TableHeader class="bg-muted/40 select-none">
            <TableRow>
              <TableHead class="text-start font-bold text-xs">{l("معرف الدور (Code)", "Role ID")}</TableHead>
              <TableHead class="text-start font-bold text-xs">{l("الاسم بالعربية", "Arabic Name")}</TableHead>
              <TableHead class="text-start font-bold text-xs">{l("الاسم بالإنجليزية", "English Name")}</TableHead>
              <TableHead class="text-start font-bold text-xs">{l("نوع الدور", "Type")}</TableHead>
              <TableHead class="text-start font-bold text-xs">{l("عدد الصلاحيات", "Permissions")}</TableHead>
              <TableHead class="text-end font-bold text-xs w-36">{m.common_je_col_actions ? m.common_je_col_actions() : "الإجراءات"}</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {#each roles as role (role.id)}
              {@const permCount = countRolePermissions(role.permissions, role.id)}
              <TableRow class="hover:bg-muted/10 transition-all duration-150">
                <!-- ID / Code -->
                <TableCell class="text-start font-mono text-xs text-foreground font-bold">
                  {role.id}
                </TableCell>

                <!-- Arabic Name -->
                <TableCell class="text-start text-xs font-semibold">
                  {role.name_ar || "—"}
                </TableCell>

                <!-- English Name -->
                <TableCell class="text-start text-xs text-muted-foreground">
                  {role.name || "—"}
                </TableCell>

                <!-- Type Badge -->
                <TableCell class="text-start text-xs">
                  {#if role.is_system}
                    <Badge variant="secondary" class="text-[9px] font-semibold">
                      {l("نظام قياسي", "System")}
                    </Badge>
                  {:else}
                    <Badge variant="success" class="text-[9px] font-bold">
                      {l("مخصص", "Custom")}
                    </Badge>
                  {/if}
                </TableCell>

                <!-- Permissions Count -->
                <TableCell class="text-start text-xs">
                  <Badge variant="outline" class="text-[9px] font-mono font-bold text-muted-foreground">
                    {permCount} {l("صلاحية", "perms")}
                  </Badge>
                </TableCell>

                <!-- Actions -->
                <TableCell class="text-end text-xs">
                  <div class="flex items-center justify-end gap-1.5">
                    <Button
                      variant="outline"
                      size="icon"
                      onclick={() => openEditRole(role)}
                      class="h-7 w-7 cursor-pointer hover:text-primary hover:bg-primary/5"
                      title={l("تعديل الصلاحيات", "Edit Permissions")}
                    >
                      <Edit2 size={12} />
                    </Button>

                    <Button
                      variant="outline"
                      size="icon"
                      onclick={() => openDuplicateRole(role)}
                      class="h-7 w-7 cursor-pointer hover:text-foreground hover:bg-muted/50"
                      title={l("نسخ الدور", "Duplicate Role")}
                    >
                      <Copy size={12} />
                    </Button>

                    {#if !role.is_system}
                      <Button
                        variant="outline"
                        size="icon"
                        onclick={() => handleDeleteRole(role)}
                        class="h-7 w-7 cursor-pointer text-destructive hover:bg-destructive/10 border-destructive/20"
                        title={m.common_action_delete ? m.common_action_delete() : "حذف"}
                      >
                        <Trash2 size={12} />
                      </Button>
                    {:else}
                      <span class="w-7"></span>
                    {/if}
                  </div>
                </TableCell>
              </TableRow>
            {/each}
          </TableBody>
        </Table>
      </div>
    {/if}
  </div>
</div>

<!-- Role Creation / Edit Sheet -->
<RoleCreationSheet
  bind:open={isSheetOpen}
  editingRole={editingRole}
  onsave={async () => {
    await loadRoles();
  }}
/>
