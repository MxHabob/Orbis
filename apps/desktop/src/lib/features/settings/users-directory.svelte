<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { confirmStore } from "$lib/stores";
import { api } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { toast } from "@orbis/ui";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import { l } from "$lib/core/i18n";
import { Plus, Users, Search, Refresh } from "@orbis/icons";

import type { User } from "$lib/api";
import { OrbisGrid, type OrbisColumnDef, type GridActionEvent, type ActionItem } from "@orbis/svelte-grid";
import { createUsersPaginatedQuery } from "$lib/features/settings/queries";
import { FilterTrigger, type UniversalFilter } from "@orbis/ui/filter-builder";
import { userFilterFields } from "$lib/core/filter-configs";
import { useQueryClient } from "@tanstack/svelte-query";
import { goto } from "$app/navigation";

let { orgId }: { orgId: string } = $props();
const queryClient = useQueryClient();

// ── State ──────────────────────────────────────────────────────────────
let searchQuery = $state("");
let selectedRole = $state("all");
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
  () => orgId,
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

// ── Filtering & Grid Source ──────────────────────────────────────────────
const effectiveFilter = $derived.by<UniversalFilter>(() => {
  const conds = [...(activeFilter?.conditions || [])];
  if (selectedRole && selectedRole !== "all") {
    conds.push({
      id: "role-filter",
      field: "role",
      operator: "eq",
      value: selectedRole,
    });
  }
  return {
    logic: activeFilter?.logic || "AND",
    conditions: conds,
    groups: activeFilter?.groups || [],
  };
});

const gridSource = $derived(
  users.map((u, i) => ({
    ...u,
    __idx: i + 1,
    full_name: l(u.name_ar, u.full_name) || u.full_name || "—",
    role: u.roles?.[0] || "staff",
    status: u.is_deleted ? "inactive" : "active",
  })),
);

// ── Columns ─────────────────────────────────────────────────────────────
const columns: OrbisColumnDef[] = [
  { id: "__idx", prop: "__idx", header: "#", name: "#", width: 48, size: 48, readonly: true, pin: "left", align: "center" },
  { id: "full_name", prop: "full_name", header: m.common_col_name(), name: m.common_col_name(), width: 220, size: 220, type: "avatar", readonly: true, pin: "left" },
  { id: "email", prop: "email", header: m.common_word_email(), name: m.common_word_email(), width: 200, size: 200, readonly: true },
  { id: "phone", prop: "phone", header: m.common_bulk_col_phone(), name: m.common_bulk_col_phone(), width: 140, size: 140, readonly: true, cellClass: "font-mono" },
  { id: "role", prop: "role", header: m.common_user_form_role(), name: m.common_user_form_role(), width: 110, size: 110, readonly: true, align: "center" },
  { id: "status", prop: "status", header: m.common_word_status?.() || "Status", name: "Status", width: 100, size: 100, type: "badge", align: "center", readonly: true },
  {
    id: "actions",
    prop: "actions",
    header: m.common_je_col_actions?.() || "Actions",
    name: "Actions",
    width: 90,
    size: 90,
    type: "action",
    align: "center",
    readonly: true,
    meta: { items: userActions },
  },
];

// ── Handlers ───────────────────────────────────────────────────────────
async function handleAction(event: GridActionEvent) {
  const user = event.row as User;
  if (event.actionId === "edit" || event.actionId === "dblclick") {
    goto(localizeHref(`/${orgId}/settings/users/${user.id}`));
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
          queryKey: ["users", orgId],
        });
        await queryClient.invalidateQueries({
          queryKey: ["users_paginated", orgId],
        });
        queryClient.removeQueries({ queryKey: ["user", orgId, user.id] });
        toast.success("User deleted successfully");
        loadUsers();
      } catch (e) {
        toast.error((e as any)?.message || "Failed to delete user");
      }
    }
  }
}
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-4 scrollbar-hide font-sans">
  <!-- Header: Filters + Add Button -->
  <div class="flex items-center justify-between gap-4 flex-nowrap shrink-0">
    <div class="flex items-center gap-3 flex-1">
      <div class="relative flex-1 max-w-xs">
        <Search
          size={14}
          class="absolute inset-s-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none"
        />
        <Input
          type="text"
          placeholder={m.settings_users_search_placeholder()}
          bind:value={searchQuery}
          class="h-8 ps-8 text-xs bg-card border-border/60"
        />
      </div>

      <!-- Role Filter -->
      <div class="w-36 text-start shrink-0">
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
    </div>

    <div class="flex items-center gap-2 flex-nowrap shrink-0">
      <FilterTrigger
        bind:filter={activeFilter}
        fields={userFilterFields}
      />
      <Button variant="ghost" size="icon-sm" onclick={loadUsers} class="text-muted-foreground">
        <Refresh size={14} class={isLoading ? "animate-spin" : ""} />
      </Button>
      <Button variant="lime" class="text-xs h-8 px-4 font-semibold" href={localizeHref(`/${orgId}/settings/users/new`)}>
        <Plus size={14} class="me-1.5" />
        {m.settings_users_add()}
      </Button>
    </div>
  </div>

  <!-- OrbisGrid -->
  <div class="flex-1 min-h-0 flex flex-col">
    <OrbisGrid
      {columns}
      source={gridSource}
      loading={isLoading}
      clientQuery
      search={searchQuery}
      filter={effectiveFilter}
      emptyTitle={m.settings_users_empty()}
      emptyDescription={m.settings_users_empty_desc()}
      rtl={getLocale() === "ar"}
      readonly
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
</div>
