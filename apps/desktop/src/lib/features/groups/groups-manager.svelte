<!--
  @component GroupsManager
  User Groups catalog and lifecycle manager for IAM subsystem.
-->
<script lang="ts">
  import { Skeleton } from "@orbis/ui/skeleton";
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { goto } from "$app/navigation";
import { resolve } from "$app/paths";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Textarea } from "@orbis/ui/textarea";
import { Badge } from "@orbis/ui/badge";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@orbis/ui/card";
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

function displayGroupName(name: string): string {
  return name
    .trim()
    .replace(/[_-]+/g, " ")
    .toLocaleLowerCase()
    .replace(/\b\p{L}/gu, (letter) => letter.toLocaleUpperCase());
}

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
  <div class="flex items-start justify-between gap-4 shrink-0">
    <div class="flex min-w-0 items-start gap-3">
      <div class="min-w-0">
        <h1 class="text-lg font-bold tracking-tight text-foreground">{m.groups_nav_title()}</h1>
        <p class="mt-0.5 text-xs text-muted-foreground">{m.iam_resource_group_description()}</p>
      </div>
    </div>
    <Button
      variant="lime"
      size="sm"
      onclick={openCreateDialog}
      class="h-9 shrink-0 text-xs font-semibold cursor-pointer"
    >
      <Plus size={14} class="me-1.5" />
      {m.groups_create_btn()}
    </Button>
  </div>

  <div class="flex items-center justify-between gap-3 shrink-0">
    <div class="relative w-full max-w-sm">
      <Search size={14} class="absolute start-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
      <Input
        type="search"
        placeholder={m.ui_copy_search_groups_d05b5ef5()}
        class="ps-8 h-9 text-xs bg-card border-border/60"
        bind:value={searchQuery}
      />
    </div>
    <Button
      variant="ghost"
      size="icon-sm"
      onclick={() => groupsQuery.refetch()}
      class="text-muted-foreground"
      title={m.common_control_refresh()}
      aria-label={m.common_control_refresh()}
    >
      {#if groupsQuery.isLoading}<LoadingIndicator size={14} />{:else}<Refresh size={14} />{/if}
    </Button>
  </div>

  <!-- Content Area -->
  <div class="flex-1 min-h-0 overflow-y-auto">
    {#if groupsQuery.isLoading}
      <div class="grid grid-cols-[repeat(auto-fill,minmax(280px,1fr))] gap-4" aria-busy="true">
        <span class="sr-only">{m.ui_copy_loading_user_groups_ff96ce28()}</span>
        {#each Array.from({ length: 6 }) as _, index (index)}
          <Card class="flex h-full min-w-0 flex-col">
            <CardHeader class="p-4 pb-2">
              <div class="flex items-start justify-between gap-3">
                <Skeleton class="h-5 w-2/3 rounded" />
                <Skeleton class="h-5 w-16 rounded" />
              </div>
            </CardHeader>
            <CardContent class="flex grow flex-col gap-3 px-4 pb-4">
              <Skeleton class="mt-1 h-4 w-full rounded" />
              <Skeleton class="h-4 w-3/4 rounded" />
              <div class="mt-auto flex gap-2 pt-1">
                <Skeleton class="h-5 w-20 rounded" />
                <Skeleton class="h-5 w-24 rounded" />
              </div>
            </CardContent>
            <CardFooter class="flex items-center justify-between gap-2 border-t border-border/40 bg-muted/10 p-3">
              <Skeleton class="h-7 w-24 rounded-md" />
              <div class="flex gap-1.5">
                <Skeleton class="h-7 w-7 rounded-md" />
                <Skeleton class="h-7 w-7 rounded-md" />
              </div>
            </CardFooter>
          </Card>
        {/each}
      </div>
    {:else if groupsQuery.error}
      <div class="h-full w-full flex flex-col items-center justify-center p-6 text-center gap-3">
        <p class="text-xs text-destructive font-semibold">{extractErrorMessage(groupsQuery.error)}</p>
        <Button variant="outline" size="sm" class="text-xs" onclick={() => groupsQuery.refetch()}>
          {m.common_retry()}
        </Button>
      </div>
    {:else if groups.length === 0}
      <div class="h-full w-full flex flex-col items-center justify-center p-8 text-center gap-3 select-none border border-border/70 rounded-xl bg-card">
        <h3 class="text-sm font-bold text-foreground">{m.ui_copy_no_groups_found_fd77c6df()}</h3>
        <p class="text-xs text-muted-foreground max-w-xs">{m.ui_copy_create_groups_to_organize_staff_4874538a()}</p>
        <Button variant="lime" size="sm" class="text-xs mt-2 font-bold cursor-pointer" onclick={openCreateDialog}>
          <Plus size={14} class="me-1.5" />
          {m.groups_create_btn()}
        </Button>
      </div>
    {:else}
      <div class="grid grid-cols-[repeat(auto-fill,minmax(280px,1fr))] gap-4 text-start">
        {#each groups as group (group.id)}
          <Card class="flex h-full min-w-0 flex-col transition-colors hover:border-border">
            <CardHeader class="border-0 bg-transparent p-4 pb-3 text-foreground normal-case tracking-normal">
              <div class="flex min-w-0 items-start gap-3">
                <div class="min-w-0 flex-1">
                  <div class="flex flex-wrap items-start gap-x-2 gap-y-1">
                    <CardTitle
                      class="min-w-0 flex-1 whitespace-normal break-words text-sm font-bold leading-5 text-foreground"
                      title={l(group.name_ar || "", group.name)}
                    >
                      {l(group.name_ar || "", group.name)}
                    </CardTitle>
                    {#if group.is_system}
                      <Badge variant="secondary" class="h-5 shrink-0 px-1.5 text-[10px] leading-none">
                        {m.groups_system_badge()}
                      </Badge>
                    {/if}
                  </div>
                  {#if group.name_ar && group.name}
                    <p dir="ltr" class="mt-1 break-words text-end text-xs leading-4 text-muted-foreground">
                      {displayGroupName(group.name)}
                    </p>
                  {/if}
                </div>
              </div>
            </CardHeader>

            <CardContent class="flex flex-col gap-3 px-4 pb-4">
              <p class="line-clamp-2 min-h-10 break-words text-xs leading-5 text-muted-foreground">
                {group.description || m.ui_copy_no_description_provided_081f8206()}
              </p>

              <div class="flex flex-wrap items-center gap-2">
                <Badge variant="secondary" class="h-6 max-w-full gap-1 px-2 text-[11px] font-normal">
                  <Users size={11} />
                  <span>{formatNumber(group.member_count)} {m.groups_members_count()}</span>
                </Badge>
                <Badge variant="secondary" class="h-6 max-w-full gap-1 px-2 text-[11px] font-normal">
                  <Keyframes size={11} />
                  <span>{formatNumber(group.permissions.length)} {m.ui_copy_perms_dd19d499()}</span>
                </Badge>
              </div>
            </CardContent>

            <CardFooter class="flex flex-wrap items-center justify-between gap-2 border-t border-border/40 bg-muted/10 p-3">
              <Button
                variant="outline"
                size="sm"
                class="h-7 shrink-0 cursor-pointer rounded-md px-2.5 text-xs"
                onclick={() => goto(resolve(localizeHref(`/${orgId}/settings/groups/${group.id}`)))}
              >
                <Pencil size={12} class="me-1.5" />
                {m.ui_copy_members_permissions_bd07beaa()}
              </Button>

              <div class="flex items-center gap-1.5">
                <Button
                  variant="outline"
                  size="icon-xs"
                  class="h-7 w-7 shrink-0 cursor-pointer rounded-md"
                  onclick={() => openEditDialog(group)}
                  title={m.common_action_edit()}
                  aria-label={m.common_action_edit()}
                >
                  <Pencil size={13} />
                </Button>
                {#if !group.is_system}
                  <Button
                    variant="outline"
                    size="icon-xs"
                    class="h-7 w-7 shrink-0 cursor-pointer rounded-md text-destructive hover:text-destructive"
                    onclick={() => promptDeleteGroup(group)}
                    title={m.common_action_delete()}
                    aria-label={m.common_action_delete()}
                  >
                    <Trash size={13} />
                  </Button>
                {/if}
              </div>
            </CardFooter>
          </Card>
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
