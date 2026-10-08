<!--
  @component GroupsManager
  User Groups catalog and lifecycle manager for IAM subsystem.
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { goto } from "$app/navigation";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Textarea } from "@orbis/ui/textarea";
import { Badge } from "@orbis/ui/badge";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@orbis/ui/dialog";
import {
  ArrowPath,
  Keyframes,
  Pencil,
  Plus,
  Refresh,
  Search,
  Trash,
  UserBadgeCheck,
  UserGroup,
  Users,
} from "@orbis/icons";
import { toast } from "@orbis/ui";
import { localizeHref } from "$lib/paraglide/runtime";
import { formatNumber, formatDate, l } from "$lib/core/i18n";
import { extractErrorMessage } from "$lib/api";
import type { GroupDetailDto, CreateGroupRequest, UpdateGroupRequest } from "$lib/api/types";
import {
  createGroupsQuery,
  createCreateGroupMutation,
  createUpdateGroupMutation,
  createDeleteGroupMutation,
} from "./queries";
import { confirmStore } from "$lib/stores";

interface Props {
  orgId: string;
}

let { orgId }: Props = $props();

let searchQuery = $state("");

// Queries & Mutations
const groupsQuery = createGroupsQuery(() => orgId);
const createMutation = createCreateGroupMutation();
const updateMutation = createUpdateGroupMutation();
const deleteMutation = createDeleteGroupMutation();

const groups = $derived(
  (groupsQuery.data ?? []).filter((g) => {
    const q = searchQuery.toLowerCase().trim();
    if (!q) return true;
    const name = (g.name || "").toLowerCase();
    const nameAr = (g.name_ar || "").toLowerCase();
    const desc = (g.description || "").toLowerCase();
    return name.includes(q) || nameAr.includes(q) || desc.includes(q);
  })
);

// Create / Edit Dialog State
let isEditDialogOpen = $state(false);
let editingGroup = $state<GroupDetailDto | null>(null);
let formName = $state("");
let formNameAr = $state("");
let formDesc = $state("");

function openCreateDialog() {
  editingGroup = null;
  formName = "";
  formNameAr = "";
  formDesc = "";
  isEditDialogOpen = true;
}

function openEditDialog(group: GroupDetailDto) {
  editingGroup = group;
  formName = group.name;
  formNameAr = group.name_ar || "";
  formDesc = group.description || "";
  isEditDialogOpen = true;
}

async function handleSaveGroup() {
  if (!formName.trim() && !formNameAr.trim()) {
    toast.error(m.ui_copy_please_enter_group_name_74edc117());
    return;
  }

  try {
    if (editingGroup) {
      const payload: UpdateGroupRequest = {
        name: formName.trim() || undefined,
        name_ar: formNameAr.trim() || undefined,
        description: formDesc.trim() || undefined,
      };
      await updateMutation.mutateAsync({ id: editingGroup.id, req: payload });
      toast.success(m.groups_save_success());
    } else {
      const payload: CreateGroupRequest = {
        name: formName.trim() || formNameAr.trim(),
        name_ar: formNameAr.trim() || undefined,
        description: formDesc.trim() || undefined,
      };
      await createMutation.mutateAsync(payload);
      toast.success(m.groups_save_success());
    }
    isEditDialogOpen = false;
  } catch (err) {
    toast.error(extractErrorMessage(err));
  }
}

async function promptDeleteGroup(group: GroupDetailDto) {
  const confirmed = await confirmStore.prompt({
    title: m.ui_copy_delete_user_group_2da27c25(),
    description: m.groups_delete_confirm_description({
      group: l(group.name_ar || "", group.name),
    }),
    confirmText: m.ui_copy_delete_group_d873cd2b(),
    variant: "destructive",
  });

  if (!confirmed) return;

  try {
    await deleteMutation.mutateAsync(group.id);
    toast.success(m.groups_delete_success());
  } catch (err) {
    toast.error(extractErrorMessage(err));
  }
}
</script>

<div class="h-full w-full overflow-hidden flex flex-col p-6 gap-4 text-start">
  <!-- Top Settings Toolbar -->
  <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3 shrink-0">
    <div class="relative w-48 sm:w-64">
      <Search size={14} class="absolute start-2.5 top-2.5 text-muted-foreground pointer-events-none" />
      <Input
        type="search"
        placeholder={m.ui_copy_search_groups_d05b5ef5()}
        class="ps-8 h-9 text-xs bg-card"
        bind:value={searchQuery}
      />
    </div>

    <!-- Actions -->
    <div class="flex items-center gap-2">
      <Button
        variant="outline"
        size="icon"
        onclick={() => groupsQuery.refetch()}
        class="h-9 w-9 cursor-pointer"
        title={m.common_control_refresh()}
      >
        {#if groupsQuery.isLoading}<LoadingIndicator  size={13} />{:else}<Refresh size={13}   />{/if}
      </Button>

      <Button
        variant="lime"
        size="sm"
        onclick={openCreateDialog}
        class="h-9 text-xs flex items-center gap-1.5 font-bold cursor-pointer"
      >
        <Plus size={14} />
        <span>{m.groups_create_btn()}</span>
      </Button>
    </div>
  </div>

  <!-- Content Area -->
  <div class="flex-1 min-h-0 overflow-y-auto">
    {#if groupsQuery.isLoading}
      <div class="h-full w-full flex flex-col items-center justify-center p-8 gap-3 text-muted-foreground">
        <LoadingIndicator class="text-primary"  size={24} />
        <span class="text-xs">{m.ui_copy_loading_user_groups_ff96ce28()}</span>
      </div>
    {:else if groupsQuery.error}
      <div class="h-full w-full flex flex-col items-center justify-center p-6 text-center gap-3">
        <p class="text-xs text-destructive font-semibold">{extractErrorMessage(groupsQuery.error)}</p>
        <Button variant="outline" size="sm" class="text-xs" onclick={() => groupsQuery.refetch()}>
          {m.common_retry()}
        </Button>
      </div>
    {:else if groups.length === 0}
      <div class="h-full w-full flex flex-col items-center justify-center p-16 text-center gap-3 select-none border border-border/70 rounded-xl bg-card">
        <div class="size-14 rounded-full bg-primary/10 flex items-center justify-center text-primary mb-2">
          <UserGroup size={28} />
        </div>
        <h3 class="text-sm font-bold text-foreground">{m.ui_copy_no_groups_found_fd77c6df()}</h3>
        <p class="text-xs text-muted-foreground max-w-xs">{m.ui_copy_create_groups_to_organize_staff_4874538a()}</p>
        <Button variant="lime" size="sm" class="text-xs mt-2 font-bold cursor-pointer" onclick={openCreateDialog}>
          <Plus size={14} class="me-1.5" />
          {m.groups_create_btn()}
        </Button>
      </div>
    {:else}
      <!-- Groups Grid -->
      <div class="grid grid-cols-3 gap-4">
        {#each groups as group (group.id)}
          <div class="flex flex-col justify-between rounded-xl border border-border/80 bg-card p-5 shadow-xs hover:border-border transition-all">
            <div class="flex flex-col gap-3">
              <!-- Top Card Header -->
              <div class="flex items-start justify-between gap-2">
                <div class="flex flex-col min-w-0">
                  <h3 class="text-sm font-bold text-foreground truncate">
                    {l(group.name_ar || "", group.name)}
                  </h3>
                  {#if group.name_ar && group.name}
                    <span class="text-[11px] font-mono text-muted-foreground truncate">{group.name}</span>
                  {/if}
                </div>

                {#if group.is_system}
                  <Badge variant="secondary" class="text-[10px] shrink-0 font-medium">
                    {m.groups_system_badge()}
                  </Badge>
                {/if}
              </div>

              <!-- Description -->
              <p class="text-xs text-muted-foreground min-h-10 line-clamp-2 leading-relaxed">
                {group.description || m.ui_copy_no_description_provided_081f8206()}
              </p>

              <!-- Metrics -->
              <div class="flex items-center gap-3 pt-2 text-xs text-muted-foreground border-t border-border/50">
                <span class="flex items-center gap-1.5 font-medium text-foreground">
                  <Users size={13} class="text-primary" />
                  {formatNumber(group.member_count)} {m.groups_members_count()}
                </span>
                <span>•</span>
                <span class="flex items-center gap-1.5 font-mono">
                  <Keyframes size={13} />
                  {formatNumber(group.permissions.length)} {m.ui_copy_perms_dd19d499()}
                </span>
              </div>
            </div>

            <!-- Action Buttons Footer -->
            <div class="flex items-center justify-between pt-4 mt-3 border-t border-border/60">
              <Button
                variant="default"
                size="sm"
                class="text-xs h-8 px-3 cursor-pointer"
                onclick={() => goto(localizeHref(`/${orgId}/settings/groups/${group.id}`))}
              >
                <Pencil size={12} class="me-1.5" />
                {m.ui_copy_members_permissions_bd07beaa()}
              </Button>

              <div class="flex items-center gap-1">
                <Button
                  variant="ghost"
                  size="icon"
                  class="size-8 cursor-pointer"
                  onclick={() => openEditDialog(group)}
                >
                  <Pencil size={13} />
                </Button>

                {#if !group.is_system}
                  <Button
                    variant="ghost"
                    size="icon"
                    class="size-8 cursor-pointer text-destructive hover:text-destructive"
                    onclick={() => promptDeleteGroup(group)}
                  >
                    <Trash size={13} />
                  </Button>
                {/if}
              </div>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>

  <!-- Create / Edit Dialog -->
  <Dialog bind:open={isEditDialogOpen}>
    <DialogContent class="max-w-md p-6 text-start">
      <DialogHeader>
        <DialogTitle class="text-base font-bold flex items-center gap-2">
          <UserGroup size={18} class="text-primary" />
          {editingGroup ? (m.groups_edit_title()) : (m.groups_create_btn())}
        </DialogTitle>
      </DialogHeader>

      <div class="flex flex-col gap-4 py-3 text-xs">
        <div class="flex flex-col gap-1.5">
          <Label class="text-xs font-medium">{m.groups_name_ar()}</Label>
          <Input
            type="text"
            placeholder={m.groups_name_ar_placeholder()}
            class="text-xs"
            bind:value={formNameAr}
          />
        </div>

        <div class="flex flex-col gap-1.5">
          <Label class="text-xs font-medium">{m.groups_name()}</Label>
          <Input
            type="text"
            placeholder="e.g. Faculty & Supervisors"
            class="text-xs font-mono"
            bind:value={formName}
          />
        </div>

        <div class="flex flex-col gap-1.5">
          <Label class="text-xs font-medium">{m.groups_description()}</Label>
          <Textarea
            placeholder={m.groups_description_placeholder()}
            class="text-xs min-h-20"
            bind:value={formDesc}
          />
        </div>
      </div>

      <DialogFooter class="flex items-center justify-end gap-2 pt-3 border-t border-border">
        <Button variant="outline" size="sm" class="text-xs cursor-pointer" onclick={() => (isEditDialogOpen = false)}>
          {m.common_action_cancel()}
        </Button>
        <Button size="sm" class="text-xs cursor-pointer" disabled={createMutation.isPending || updateMutation.isPending} onclick={handleSaveGroup}>
          {#if createMutation.isPending || updateMutation.isPending}
            <LoadingIndicator class="me-1.5"  size={13} />
          {/if}
          {m.settings_action_save_changes()}
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</div>
