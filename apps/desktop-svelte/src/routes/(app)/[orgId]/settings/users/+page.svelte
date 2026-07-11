<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { toast } from "svelte-sonner";
import { localizeHref } from "$lib/paraglide/runtime";
import { getLocale } from "$lib/paraglide/runtime";
import { l, isAr } from "$lib/core/i18n";
import { Plus, Users } from "lucide-svelte";
import * as m from "$lib/paraglide/messages";
import type { User } from "$lib/core/ipc";
import { authStore } from "$lib/stores";

import { DataTable } from "$lib/tables";
import { users as usersDef, withIndex } from "$lib/tables/definitions";

let { data }: { data: { orgId: string } } = $props();

import { goto } from "$app/navigation";
import { ActionOverlay } from "$lib/tables";
import type { GridActionEvent, ActionItem } from "@orbis/svelte-grid";

// ── State ──────────────────────────────────────────────────────────────
let users = $state<User[]>([]);
let isLoading = $state(true);
let searchQuery = $state("");
let selectedRole = $state("all");
let overlayEl = $state<any>();

const userActions: ActionItem[] = [
  { id: "edit", label: "Edit / View", icon: "Pencil" },
  { id: "delete", label: "Delete", icon: "Trash2", variant: "destructive" },
];

// ── Data Loading ────────────────────────────────────────────────────────

async function loadUsers() {
  isLoading = true;
  try {
    users = await commands.iam.listUsers(data.orgId);
  } catch (e) {
    toast.error((e as any)?.message || "Failed to load users");
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (data.orgId) loadUsers();
});

// ── Filtering ───────────────────────────────────────────────────────────

const filteredUsers = $derived(
  users.filter((user) => {
    const q = searchQuery.toLowerCase();
    const matchesSearch =
      !q ||
      (user.full_name || "").toLowerCase().includes(q) ||
      (user.name_ar || "").toLowerCase().includes(q) ||
      (user.email || "").toLowerCase().includes(q);

    const matchesRole =
      selectedRole === "all" ||
      String(user.primary_role || user.role || "").toLowerCase() ===
        selectedRole.toLowerCase();

    return matchesSearch && matchesRole;
  }),
);

// ── Grid Source ─────────────────────────────────────────────────────────

const gridSource = $derived(
  withIndex(
    filteredUsers.map((u) => ({
      ...u,
      full_name: l(u.name_ar, u.full_name) || u.full_name || "—",
      role: String(u.primary_role || u.role || "staff"),
      status: u.is_deleted ? "inactive" : "active",
    })),
  ),
);

// ── Columns ─────────────────────────────────────────────────────────────

const columns = $derived(
  usersDef.columns(
    {
      idx: "#",
      name: m.settings_users_col_name(),
      email: m.settings_users_col_email(),
      phone: m.settings_users_col_phone(),
      role: m.settings_users_col_role(),
      status: m.settings_users_col_status?.() || "Status",
      actions: m.settings_users_col_actions?.() || "Actions",
    },
    userActions,
  ),
);

// ── Handlers ───────────────────────────────────────────────────────────

async function handleAction(event: GridActionEvent) {
  const user = event.row as User;
  if (event.actionId === "__menu__") {
    overlayEl?.show(event);
  } else if (event.actionId === "edit" || event.actionId === "dblclick") {
    goto(localizeHref(`/${data.orgId}/settings/users/${user.id}`));
  } else if (event.actionId === "delete") {
    if (confirm(`Are you sure you want to delete ${user.full_name}?`)) {
      try {
        await commands.iam.deleteUser(user.id, "admin");
        toast.success("User deleted successfully");
        loadUsers();
      } catch (e) {
        toast.error((e as any)?.message || "Failed to delete user");
      }
    }
  }
}
</script>

<svelte:head>
  <title>{m.settings_users_window_title()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-4 scrollbar-hide">
  <!-- Header: Filters + Add Button -->
  <div class="flex items-center justify-between gap-4 flex-nowrap shrink-0">
    <h1 class="text-lg font-bold text-foreground truncate">{m.settings_users_title()}</h1>
    <div class="flex items-center gap-2 flex-nowrap shrink-0">
      <!-- Role Filter -->
      <div class="w-40 text-start">
        <Select type="single" value={selectedRole} onValueChange={(v) => (selectedRole = v ?? "all")}>
          <SelectTrigger class="h-8 w-full text-xs bg-card border-border/60">
            {#if selectedRole === "all"}
              {m.settings_users_filter_all()}
            {:else if selectedRole === "admin"}
              {m.settings_users_filter_admin()}
            {:else if selectedRole === "teacher"}
              {m.settings_users_filter_teacher()}
            {:else if selectedRole === "parent"}
              {m.settings_users_filter_guardian()}
            {:else if selectedRole === "student"}
              {m.settings_users_filter_student()}
            {:else}
              {m.settings_users_filter_role_staff()}
            {/if}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">{m.settings_users_filter_all()}</SelectItem>
            <SelectItem value="admin">{m.settings_users_filter_admin()}</SelectItem>
            <SelectItem value="teacher">{m.settings_users_filter_teacher()}</SelectItem>
            <SelectItem value="parent">{m.settings_users_filter_guardian()}</SelectItem>
            <SelectItem value="student">{m.settings_users_filter_student()}</SelectItem>
            <SelectItem value="staff">Staff</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <Button variant="lime" class="text-xs h-8 px-4 font-semibold" href={localizeHref(`/${data.orgId}/settings/users/new`)}>
        <Plus size={14} class="me-1.5" />
        {m.settings_users_add()}
      </Button>
    </div>
  </div>

  <!-- DataTable -->
  <div class="flex-1 min-h-0">
    <DataTable
      {columns}
      source={gridSource}
      loading={isLoading}
      searchable
      searchPlaceholder={m.settings_users_search_placeholder()}
      bind:searchValue={searchQuery}
      emptyIcon={Users}
      emptyTitle={m.settings_users_empty()}
      emptyDescription={m.settings_users_empty_desc()}
      rtl={getLocale() === "ar"}
      refreshable
      onRefresh={loadUsers}
      onaction={handleAction}
    />
  </div>

  <ActionOverlay bind:this={overlayEl} actions={userActions} onselect={handleAction} />
</div>
