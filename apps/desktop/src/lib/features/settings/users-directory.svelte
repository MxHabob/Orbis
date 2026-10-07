<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { confirmStore } from "$lib/stores";
import { api } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { toast } from "@orbis/ui";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import { formatDate, l } from "$lib/core/i18n";
import { Search, Refresh, ShieldCheck, UserPlus } from "@orbis/icons";

import type { User, AccountProvisioningResultDto } from "$lib/api";
import { OrbisGrid, type OrbisColumnDef, type GridActionEvent, type ActionItem } from "@orbis/svelte-grid";
import { createUsersPaginatedQuery } from "$lib/features/settings/queries";
import { FilterTrigger, type UniversalFilter } from "@orbis/ui/filter-builder";
import { userFilterFields } from "$lib/core/filter-configs";
import { useQueryClient } from "@tanstack/svelte-query";
import { goto } from "$app/navigation";
import RoleSelect from "$lib/shared/components/role-select.svelte";
import CredentialDeliveryModal from "$lib/shared/components/credential-delivery-modal.svelte";
import CreateUserSheet from "$lib/features/settings/components/create-user-sheet.svelte";

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

// Provisioning & Credential modal state
let createUserSheetOpen = $state(false);
let credentialModalOpen = $state(false);
let activeProvisioningResult = $state<AccountProvisioningResultDto | null>(null);
let activeUserName = $state("");

function handleUserProvisioned(result: AccountProvisioningResultDto, name: string) {
  activeProvisioningResult = result;
  activeUserName = name;
  credentialModalOpen = true;
  loadUsers();
}

const userActions: ActionItem[] = [
  { id: "edit", label: m.common_action_edit(), icon: "Pencil" },
  { id: "reset_password", label: m.settings_user_detail_btn_reset_pass(), icon: "LockClosed" },
  { id: "delete", label: m.common_action_delete(), icon: "Trash2", variant: "destructive" },
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
    created_at_display: u.created_at ? formatDate(u.created_at, "datetime") : "—",
    updated_at_display: u.updated_at ? formatDate(u.updated_at, "datetime") : "—",
  })),
);

// ── Columns ─────────────────────────────────────────────────────────────
const columns: OrbisColumnDef[] = [
  { id: "__idx", prop: "__idx", header: "#", name: "#", width: 48, minWidth: 48, readonly: true, pin: "left", align: "center", flex: 0 },
  { id: "full_name", prop: "full_name", header: m.common_col_name(), name: m.common_col_name(), width: 220, minWidth: 180, type: "avatar", readonly: true, pin: "left", flex: 2 },
  { id: "email", prop: "email", header: m.common_word_email(), name: m.common_word_email(), width: 200, minWidth: 160, readonly: true, flex: 1.5 },
  { id: "phone", prop: "phone", header: m.common_bulk_col_phone(), name: m.common_bulk_col_phone(), width: 140, minWidth: 120, readonly: true, cellClass: "font-mono", flex: 1 },
  { id: "role", prop: "role", header: m.common_user_form_role(), name: m.common_user_form_role(), width: 110, minWidth: 100, readonly: true, align: "center", flex: 1 },
  { id: "status", prop: "status", header: m.common_word_status(), name: m.common_word_status(), width: 100, minWidth: 90, type: "badge", align: "center", readonly: true, flex: 0 },
  { id: "created_at_display", prop: "created_at_display", header: m.ui_copy_created_at_01f6ea5f(), name: m.ui_copy_created_at_01f6ea5f(), width: 130, minWidth: 120, readonly: true, cellClass: "font-mono text-xs text-muted-foreground", flex: 1 },
  { id: "updated_at_display", prop: "updated_at_display", header: m.ui_copy_updated_at_ba0d0a9f(), name: m.ui_copy_updated_at_ba0d0a9f(), width: 130, minWidth: 120, readonly: true, cellClass: "font-mono text-xs text-muted-foreground", flex: 1 },
  {
    id: "actions",
    prop: "actions",
    header: m.common_je_col_actions(),
    name: m.ui_copy_actions_c48127a5(),
    width: 120,
    minWidth: 110,
    type: "action",
    align: "center",
    readonly: true,
    flex: 0,
    meta: { items: userActions },
  },
];

// ── Handlers ───────────────────────────────────────────────────────────
async function handleAction(event: GridActionEvent) {
  const user = event.row as User;
  if (event.actionId === "edit" || event.actionId === "dblclick") {
    goto(localizeHref(`/${orgId}/settings/users/${user.id}`));
  } else if (event.actionId === "reset_password") {
    const confirmed = await confirmStore.prompt({
      title: m.settings_user_detail_btn_reset_pass(),
      description: l(
        m.settings_ui_user_reset_password_confirm({ user: user.full_name || user.email }),
        `Are you sure you want to generate a new temporary password for ${user.full_name || user.email}?`
      ),
      confirmText: m.settings_user_detail_btn_reset_pass(),
    });
    if (confirmed) {
      try {
        const result = await api.resetUserPassword(user.id);
        activeProvisioningResult = result;
        activeUserName = user.full_name || user.email;
        credentialModalOpen = true;
        toast.success(m.employees_password_reset_success());
      } catch (e) {
        toast.error((e as any)?.message || m.settings_ui_user_reset_password_error());
      }
    }
  } else if (event.actionId === "delete") {
    const confirmed = await confirmStore.prompt({
      title: m.common_action_delete(),
      description: m.settings_ui_user_delete_confirm({ user: user.full_name }),
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
        toast.success(m.settings_ui_user_deleted());
        loadUsers();
      } catch (e) {
        toast.error((e as any)?.message || m.settings_ui_user_delete_error());
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
      <div class="w-44 text-start shrink-0">
        <RoleSelect
          bind:value={selectedRole}
          allowAll
          size="sm"
        />
      </div>
    </div>

    <div class="flex items-center gap-2 flex-nowrap shrink-0">
      <FilterTrigger
        bind:filter={activeFilter}
        fields={userFilterFields}
      />
      <Button variant="ghost" size="icon-sm" onclick={loadUsers} class="text-muted-foreground">
        {#if isLoading}<LoadingIndicator  size={14} />{:else}<Refresh size={14}   />{/if}
      </Button>
      <Button
        variant="outline"
        class="text-xs h-8 px-3 font-medium gap-1.5 cursor-pointer"
        onclick={() => goto(localizeHref(`/${orgId}/employees`))}
      >
        <ShieldCheck size={14} class="text-primary" />
        <span>{m.employees_nav_title()}</span>
      </Button>
      <Button
        variant="default"
        class="text-xs h-8 px-3.5 font-semibold gap-1.5 cursor-pointer shadow-xs"
        onclick={() => (createUserSheetOpen = true)}
      >
        <UserPlus size={14} />
        <span>{m.ui_copy_provision_user_7ee34dd9()}</span>
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

  <!-- Create User Slide-over Sheet -->
  <CreateUserSheet
    bind:open={createUserSheetOpen}
    {orgId}
    onsuccess={handleUserProvisioned}
  />

  <!-- Temporary Credentials Modal -->
  <CredentialDeliveryModal
    bind:open={credentialModalOpen}
    provisioning={activeProvisioningResult}
    userName={activeUserName}
  />
</div>
