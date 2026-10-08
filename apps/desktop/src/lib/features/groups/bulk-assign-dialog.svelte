<!--
  @component BulkAssignDialog
  Searchable multi-user selection dialog for bulk assigning users to a user group.
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@orbis/ui/dialog";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Badge } from "@orbis/ui/badge";
import {
  ArrowPath,
  CheckCircle,
  Search,
  User,
  UserGroup,
  Users,
} from "@orbis/icons";
import { toast } from "@orbis/ui";
import { l, formatNumber } from "$lib/core/i18n";
import { extractErrorMessage } from "$lib/api";
import { createEmployeesQuery } from "$lib/features/employees/queries";
import { createBulkAssignGroupMutation } from "./queries";
import UserAvatar from "$lib/shared/components/user-avatar.svelte";

interface Props {
  open: boolean;
  orgId: string;
  groupId: string;
  groupName: string;
  existingMemberUserIds?: string[];
  oncomplete?: () => void;
}

let {
  open = $bindable(false),
  orgId,
  groupId,
  groupName,
  existingMemberUserIds = [],
  oncomplete,
}: Props = $props();

let searchQuery = $state("");
let selectedUserIds = $state<Set<string>>(new Set());

// Query all employees with accounts
const employeesQuery = createEmployeesQuery(() => orgId, () => ({ limit: 300 }));
const bulkAssignMutation = createBulkAssignGroupMutation();

// Derived candidates (employees that have a linked user account)
const candidates = $derived(
  (employeesQuery.data?.items ?? []).filter((e) => {
    if (!e.username) return false;
    const q = searchQuery.toLowerCase().trim();
    if (!q) return true;
    const name = (e.full_name || e.full_name_ar || e.first_name).toLowerCase();
    const uname = (e.username || "").toLowerCase();
    const empNum = (e.employee_number || "").toLowerCase();
    return name.includes(q) || uname.includes(q) || empNum.includes(q);
  })
);

function toggleUser(userId: string) {
  const next = new Set(selectedUserIds);
  if (next.has(userId)) {
    next.delete(userId);
  } else {
    next.add(userId);
  }
  selectedUserIds = next;
}

function handleSelectAll() {
  const next = new Set<string>();
  for (const c of candidates) {
    if (c.username && !existingMemberUserIds.includes(c.id)) {
      next.add(c.id);
    }
  }
  selectedUserIds = next;
}

function handleDeselectAll() {
  selectedUserIds = new Set();
}

async function handleAssign() {
  if (selectedUserIds.size === 0) return;
  try {
    await bulkAssignMutation.mutateAsync({
      group_id: groupId,
      user_ids: Array.from(selectedUserIds),
    });
    toast.success(m.ui_group_assign_users_success({ count: selectedUserIds.size }));
    open = false;
    selectedUserIds = new Set();
    oncomplete?.();
  } catch (err) {
    toast.error(extractErrorMessage(err));
  }
}
</script>

<Dialog bind:open>
  <DialogContent class="max-w-lg p-6 text-start flex flex-col max-h-[85vh]">
    <DialogHeader>
      <div class="flex items-center gap-3">
        <div class="flex size-10 shrink-0 items-center justify-center rounded-xl bg-primary/10 text-primary">
          <UserGroup size={20} />
        </div>
        <div class="flex flex-col">
          <DialogTitle class="text-base font-bold text-foreground">
            {m.groups_bulk_assign()}
          </DialogTitle>
          <span class="text-xs text-muted-foreground">
            {groupName}
          </span>
        </div>
      </div>
    </DialogHeader>

    <div class="flex flex-col gap-3 py-3 flex-1 min-h-0">
      <!-- Search Box -->
      <div class="relative w-full shrink-0">
        <Search class="absolute left-3 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-muted-foreground" />
        <Input
          type="text"
          placeholder={m.ui_copy_search_staff_e19aa622()}
          class="input-sunken ps-9 text-xs py-2"
          bind:value={searchQuery}
        />
      </div>

      <!-- Quick Selection Bar -->
      <div class="flex items-center justify-between text-xs text-muted-foreground px-1 shrink-0">
        <span>{formatNumber(selectedUserIds.size)} {m.ui_copy_selected_d24fa4a6()}</span>
        <div class="flex items-center gap-2">
          <button
            type="button"
            class="text-primary hover:underline text-[11px] cursor-pointer"
            onclick={handleSelectAll}
          >
            {m.ui_copy_select_all_99dda350()}
          </button>
          <span>•</span>
          <button
            type="button"
            class="text-muted-foreground hover:underline text-[11px] cursor-pointer"
            onclick={handleDeselectAll}
          >
            {m.ui_copy_deselect_all_9afdefeb()}
          </button>
        </div>
      </div>

      <!-- Candidates List -->
      <div class="flex-1 min-h-0 overflow-y-auto border border-border/70 rounded-lg divide-y divide-border/40 bg-card">
        {#if employeesQuery.isLoading}
          <div class="p-6 text-center text-xs text-muted-foreground flex items-center justify-center gap-2">
            <LoadingIndicator class="text-primary"  size={14} />
            <span>{m.ui_copy_loading_users_8a49770d()}</span>
          </div>
        {:else if candidates.length === 0}
          <div class="p-6 text-center text-xs text-muted-foreground">
            {m.ui_copy_no_staff_with_active_accounts_found_9fb1c918()}
          </div>
        {:else}
          {#each candidates as emp (emp.id)}
            {@const isAlreadyMember = existingMemberUserIds.includes(emp.id)}
            {@const isSelected = selectedUserIds.has(emp.id)}
            <button
              type="button"
              disabled={isAlreadyMember}
              class="w-full flex items-center justify-between p-3 text-start transition-colors {isAlreadyMember ? 'opacity-50 cursor-not-allowed bg-muted/20' : 'cursor-pointer hover:bg-muted/30'} {isSelected ? 'bg-primary/5' : ''}"
              onclick={() => toggleUser(emp.id)}
            >
              <div class="flex items-center gap-3 min-w-0">
                <UserAvatar
                  name={emp.full_name || emp.first_name}
                  avatarUrl={emp.avatar_url ?? undefined}
                  size="sm"
                />
                <div class="flex flex-col min-w-0">
                  <span class="text-xs font-semibold text-foreground truncate">
                    {emp.full_name || emp.full_name_ar || `${emp.first_name} ${emp.last_name}`}
                  </span>
                  <span class="text-[10px] font-mono text-muted-foreground truncate">
                    @{emp.username} • {emp.job_title || emp.employee_type}
                  </span>
                </div>
              </div>

              <div class="shrink-0 flex items-center gap-2">
                {#if isAlreadyMember}
                  <Badge variant="outline" class="text-[9px]">
                    {m.ui_copy_member_df6cf0b2()}
                  </Badge>
                {:else}
                  <div class="flex size-5 items-center justify-center rounded-sm {isSelected ? 'bg-primary text-primary-foreground' : 'border border-border'}">
                    {#if isSelected}
                      <CheckCircle size={12} />
                    {/if}
                  </div>
                {/if}
              </div>
            </button>
          {/each}
        {/if}
      </div>
    </div>

    <DialogFooter class="flex items-center justify-between w-full pt-3 border-t border-border shrink-0">
      <Button
        type="button"
        variant="outline"
        size="sm"
        class="text-xs cursor-pointer"
        onclick={() => (open = false)}
      >
        {m.common_action_cancel()}
      </Button>

      <Button
        type="button"
        variant="default"
        size="sm"
        class="text-xs px-4 cursor-pointer gap-1.5"
        disabled={selectedUserIds.size === 0 || bulkAssignMutation.isPending}
        onclick={handleAssign}
      >
        {#if bulkAssignMutation.isPending}
          <LoadingIndicator class="me-1"  size={13} />
        {/if}
        {m.groups_bulk_assign()} ({selectedUserIds.size})
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
