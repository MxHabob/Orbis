<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { confirmStore } from "$lib/stores";
import { api } from "$lib/api";
import { Button } from "@orbis/ui/button";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { toast } from "@orbis/ui";
import { localizeHref } from "$lib/paraglide/runtime";
import { getLocale } from "$lib/paraglide/runtime";
import { l, isAr } from "$lib/core/i18n";
import { Plus, Users } from "@lucide/svelte";

import type { User } from "$lib/api";
import { authStore } from "$lib/stores";

import { DataTable } from "$lib/tables";
import { users as usersDef, withIndex } from "$lib/tables/definitions";
import { createUsersPaginatedQuery } from "$lib/features/settings/queries";
import type { UniversalFilter } from "@orbis/ui/filter-builder";
import { userFilterFields } from "$lib/tables/definitions/filter-configs";
import { useQueryClient } from "@tanstack/svelte-query";

let { data }: { data: { orgId: string } } = $props();
const queryClient = useQueryClient();

import { goto } from "$app/navigation";
import { ActionOverlay } from "$lib/tables";
import type { GridActionEvent, ActionItem } from "@orbis/svelte-grid";

// ── State ──────────────────────────────────────────────────────────────
let searchQuery = $state("");
let selectedRole = $state("all");
let overlayEl = $state<any>();
let activeFilter = $state<UniversalFilter>({
  logic: "AND",
  conditions: [],
  groups: [],
});

const userActions: ActionItem[] = [
  { id: "edit", label: "Edit / View", icon: "Pencil" },
  { id: "delete", label: "Delete", icon: "Trash2", variant: "destructive" },
];

// ── Data Loading ────────────────────────────────────────────────────────
const usersQuery = createUsersPaginatedQuery(
  () => data.orgId,
  50,
  () => activeFilter,
);
const users = $derived(
  usersQuery.data?.pages?.flatMap((page: any) => page.items) ?? [],
);
const isLoading = $derived(usersQuery.isLoading);

function loadUsers() {
  usersQuery.refetch();
}

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
      user.roles?.some((r) => r.toLowerCase() === selectedRole.toLowerCase());

    return matchesSearch && matchesRole;
  }),
);

// ── Grid Source ─────────────────────────────────────────────────────────

const gridSource = $derived(
  withIndex(
    filteredUsers.map((u) => ({
      ...u,
      full_name: l(u.name_ar, u.full_name) || u.full_name || "—",
      role: u.roles?.[0] || "staff",
      status: u.is_deleted ? "inactive" : "active",
    })),
  ),
);

// ── Columns ─────────────────────────────────────────────────────────────

const columns = $derived(
  usersDef.columns(
    {
      idx: "#",
      name: m.common_col_name(),
      email: m.common_word_email(),
      phone: m.common_bulk_col_phone(),
      role: m.common_user_form_role(),
      status: m.common_word_status?.() || "Status",
      actions: m.common_je_col_actions?.() || "Actions",
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
    const confirmed = await confirmStore.prompt({
      title: m.common_action_delete() + "؟",
      description: `Are you sure you want to delete ${user.full_name}?`,
      confirmText: m.common_action_delete(),
      variant: "destructive",
    });
    if (confirmed) {
      try {
        await api.updateUser(user.id, { is_active: false } as any);
        await queryClient.invalidateQueries({
          queryKey: ["users", data.orgId],
        });
        await queryClient.invalidateQueries({
          queryKey: ["users_paginated", data.orgId],
        });
        queryClient.removeQueries({ queryKey: ["user", data.orgId, user.id] });
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
  <title>{m.settings_nav_users()} | Orbis</title>
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
              {m.common_filter_all()}
            {:else if selectedRole === "admin"}
              {m.common_users_filter_admin()}
            {:else if selectedRole === "teacher"}
              {m.common_schedule_page_teacher()}
            {:else if selectedRole === "parent"}
              {m.common_users_filter_guardian()}
            {:else if selectedRole === "student"}
              {m.common_word_student_single()}
            {:else}
              {m.settings_users_filter_role_staff()}
            {/if}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">{m.common_filter_all()}</SelectItem>
            <SelectItem value="admin">{m.common_users_filter_admin()}</SelectItem>
            <SelectItem value="teacher">{m.common_schedule_page_teacher()}</SelectItem>
            <SelectItem value="parent">{m.common_users_filter_guardian()}</SelectItem>
            <SelectItem value="student">{m.common_word_student_single()}</SelectItem>
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
  <div class="flex-1 min-h-0 flex flex-col">
    <DataTable
      columns={columns}
      source={gridSource}
      loading={isLoading}
      searchable
      searchPlaceholder={m.settings_users_search_placeholder()}
      bind:searchValue={searchQuery}
      refreshable
      onRefresh={loadUsers}
      printable={true}
      printType="user"
      filterable
      filterFields={userFilterFields}
      bind:filterValue={activeFilter}
      emptyIcon={Users}
      emptyTitle={m.settings_users_empty()}
      emptyDescription={m.settings_users_empty_desc()}
      onaction={handleAction}
    />
    {#if usersQuery.hasNextPage && selectedRole === 'all' && searchQuery === ''}
      <div class="p-2 border-t border-border flex justify-center bg-card shrink-0">
        <Button 
          variant="outline" 
          size="sm" 
          class="text-xs w-full max-w-md cursor-pointer"
          disabled={usersQuery.isFetchingNextPage}
          onclick={() => usersQuery.fetchNextPage()}
        >
          {#if usersQuery.isFetchingNextPage}
            {m.common_loading()}
          {:else}
            {m.settings_users_load_more()}
          {/if}
        </Button>
      </div>
    {/if}
  </div>

  <ActionOverlay bind:this={overlayEl} actions={userActions} onselect={handleAction} />
</div>
