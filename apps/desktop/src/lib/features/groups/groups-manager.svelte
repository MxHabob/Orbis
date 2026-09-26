<!--
  @component GroupsManager
  User Groups catalog and lifecycle manager for IAM subsystem.
-->
<script lang="ts">
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
  CheckCircle,
  Keyframes,
  LockClosed,
  Pencil,
  Plus,
  Search,
  ShieldCheck,
  Trash,
  User,
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
import ConfirmDialog from "$lib/shared/components/confirm-dialog.svelte";

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

// Delete Dialog State
let isDeleteDialogOpen = $state(false);
let deletingGroup = $state<GroupDetailDto | null>(null);

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
    toast.error(l("يرجى إدخال اسم المجموعة", "Please enter group name"));
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
      toast.success(m.groups_save_success?.() || "تم حفظ بيانات المجموعة بنجاح");
    } else {
      const payload: CreateGroupRequest = {
        name: formName.trim() || formNameAr.trim(),
        name_ar: formNameAr.trim() || undefined,
        description: formDesc.trim() || undefined,
      };
      await createMutation.mutateAsync(payload);
      toast.success(m.groups_save_success?.() || "تم إنشاء المجموعة بنجاح");
    }
    isEditDialogOpen = false;
  } catch (err) {
    toast.error(extractErrorMessage(err));
  }
}

function promptDeleteGroup(group: GroupDetailDto) {
  deletingGroup = group;
  isDeleteDialogOpen = true;
}

async function confirmDeleteGroup() {
  if (!deletingGroup) return;
  try {
    await deleteMutation.mutateAsync(deletingGroup.id);
    toast.success(m.groups_delete_success?.() || "تم حذف المجموعة بنجاح");
  } catch (err) {
    toast.error(extractErrorMessage(err));
  } finally {
    isDeleteDialogOpen = false;
    deletingGroup = null;
  }
}
</script>

<div class="h-full w-full flex flex-col overflow-hidden bg-background">
  <!-- Header Bar -->
  <div class="border-b border-border/80 bg-card px-6 py-4 flex items-center justify-between shrink-0 shadow-2xs">
    <div class="flex items-center gap-3">
      <div class="flex size-10 shrink-0 items-center justify-center rounded-xl bg-primary/10 text-primary">
        <UserGroup size={20} />
      </div>
      <div class="flex flex-col">
        <h1 class="text-base font-bold text-foreground">
          {m.groups_title?.() || "مجموعات المستخدمين والأدوار"}
        </h1>
        <span class="text-xs text-muted-foreground">
          {m.groups_desc?.() || "تنظيم المستخدمين في مجموعات وظيفية وإدارة صلاحياتهم المشتركة."}
        </span>
      </div>
    </div>

    <div class="flex items-center gap-2">
      <Button
        variant="default"
        class="text-xs h-9 px-3 cursor-pointer"
        onclick={openCreateDialog}
      >
        <Plus size={14} class="me-1.5" />
        {m.groups_create_btn?.() || "إنشاء مجموعة جديدة"}
      </Button>
    </div>
  </div>

  <!-- Search & Content Area -->
  <div class="flex-1 min-h-0 overflow-y-auto p-6 flex flex-col gap-4">
    <div class="relative w-full max-w-sm shrink-0">
      <Search class="absolute left-3 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-muted-foreground" />
      <Input
        type="text"
        placeholder={l("بحث عن مجموعة...", "Search groups...")}
        class="input-sunken ps-9 text-xs py-2"
        bind:value={searchQuery}
      />
    </div>

    {#if groupsQuery.isLoading}
      <div class="h-full w-full flex flex-col items-center justify-center p-8 gap-3 text-muted-foreground">
        <ArrowPath size={24} class="animate-spin text-primary" />
        <span class="text-xs">{l("جاري تحميل المجموعات...", "Loading user groups...")}</span>
      </div>
    {:else if groupsQuery.error}
      <div class="h-full w-full flex flex-col items-center justify-center p-6 text-center gap-3">
        <p class="text-xs text-destructive font-semibold">{extractErrorMessage(groupsQuery.error)}</p>
        <Button variant="outline" size="sm" class="text-xs" onclick={() => groupsQuery.refetch()}>
          {m.common_retry?.() || "إعادة المحاولة"}
        </Button>
      </div>
    {:else if groups.length === 0}
      <div class="h-full w-full flex flex-col items-center justify-center p-8 text-center gap-3 bg-card border border-border rounded-xl">
        <div class="flex size-12 items-center justify-center rounded-full bg-muted text-muted-foreground">
          <UserGroup size={24} />
        </div>
        <div class="flex flex-col gap-1">
          <h3 class="text-sm font-semibold text-foreground">{l("لا توجد مجموعات معرفة", "No groups found")}</h3>
          <p class="text-xs text-muted-foreground max-w-xs">{l("قم بإنشاء مجموعات مستخدمين لتنظيم الكادر وتوزيع الصلاحيات.", "Create groups to organize staff.")}</p>
        </div>
        <Button variant="default" size="sm" class="text-xs mt-2" onclick={openCreateDialog}>
          <Plus size={14} class="me-1.5" />
          {m.groups_create_btn?.() || "إنشاء مجموعة"}
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
                    {group.name_ar || group.name}
                  </h3>
                  {#if group.name_ar && group.name}
                    <span class="text-[11px] font-mono text-muted-foreground truncate">{group.name}</span>
                  {/if}
                </div>

                {#if group.is_system}
                  <Badge variant="secondary" class="text-[10px] shrink-0 font-medium">
                    {m.groups_system_badge?.() || "نظام"}
                  </Badge>
                {/if}
              </div>

              <!-- Description -->
              <p class="text-xs text-muted-foreground min-h-10 line-clamp-2 leading-relaxed">
                {group.description || l("لا يوجد وصف للمجموعة.", "No description provided.")}
              </p>

              <!-- Metrics -->
              <div class="flex items-center gap-3 pt-2 text-xs text-muted-foreground border-t border-border/50">
                <span class="flex items-center gap-1.5 font-medium text-foreground">
                  <Users size={13} class="text-primary" />
                  {formatNumber(group.member_count)} {m.groups_members_count?.() || "عضو"}
                </span>
                <span>•</span>
                <span class="flex items-center gap-1.5 font-mono">
                  <Keyframes size={13} />
                  {formatNumber(group.permissions.length)} {l("صلاحية", "perms")}
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
                {l("الأعضاء والصلاحيات", "Members & Permissions")}
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
          {editingGroup ? (m.groups_edit_title?.() || "تعديل المجموعة") : (m.groups_create_btn?.() || "إنشاء مجموعة جديدة")}
        </DialogTitle>
      </DialogHeader>

      <div class="flex flex-col gap-4 py-3 text-xs">
        <div class="flex flex-col gap-1.5">
          <Label class="text-xs font-medium">{m.groups_name_ar?.() || "اسم المجموعة (بالعربية)"}</Label>
          <Input
            type="text"
            placeholder="مثال: هيئة التدريس والمشرفين"
            class="text-xs"
            bind:value={formNameAr}
          />
        </div>

        <div class="flex flex-col gap-1.5">
          <Label class="text-xs font-medium">{m.groups_name?.() || "Group Name (Latin)"}</Label>
          <Input
            type="text"
            placeholder="e.g. Faculty & Supervisors"
            class="text-xs font-mono"
            bind:value={formName}
          />
        </div>

        <div class="flex flex-col gap-1.5">
          <Label class="text-xs font-medium">{m.groups_description?.() || "الوصف"}</Label>
          <Textarea
            placeholder="وصف مهام وصلاحيات هذه المجموعة..."
            class="text-xs min-h-20"
            bind:value={formDesc}
          />
        </div>
      </div>

      <DialogFooter class="flex items-center justify-end gap-2 pt-3 border-t border-border">
        <Button variant="outline" size="sm" class="text-xs cursor-pointer" onclick={() => (isEditDialogOpen = false)}>
          {m.common_action_cancel?.() || "إلغاء"}
        </Button>
        <Button size="sm" class="text-xs cursor-pointer" disabled={createMutation.isPending || updateMutation.isPending} onclick={handleSaveGroup}>
          {#if createMutation.isPending || updateMutation.isPending}
            <ArrowPath size={13} class="animate-spin me-1.5" />
          {/if}
          {m.settings_action_save_changes?.() || "حفظ"}
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>

  <!-- Delete Confirm Dialog -->
  <ConfirmDialog
    bind:open={isDeleteDialogOpen}
    title={l("تأكيد حذف مجموعة المستخدمين", "Delete User Group")}
    description={m.groups_delete_confirm?.() || "هل أنت متأكد من حذف هذه المجموعة؟ سيفقد الأعضاء الصلاحيات الممنوحة من خلالها."}
    confirmText={l("حذف المجموعة", "Delete Group")}
    variant="destructive"
    onConfirm={confirmDeleteGroup}
  />
</div>
