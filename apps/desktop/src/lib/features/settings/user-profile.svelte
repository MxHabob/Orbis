<!--
  @component UserProfile
  User profile management in Settings.
  Integrates demographic data, dynamic role assignment, user status toggles,
  and User Groups management via side Sheet.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { confirmStore } from "$lib/stores";
import { api, queryKeys, type GroupDetailDto, type User } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Checkbox } from "@orbis/ui/checkbox";
import { Badge } from "@orbis/ui/badge";
import { Skeleton } from "@orbis/ui/skeleton";
import { toast } from "@orbis/ui";
import { fade } from "svelte/transition";
import { l } from "$lib/core/i18n";
import { Pencil, UserGroup } from "@orbis/icons";
import { createUserQuery } from "$lib/features/settings/queries";
import { useQueryClient, createQuery } from "@tanstack/svelte-query";
import RoleSelect from "$lib/shared/components/role-select.svelte";
import ManageUserGroupsSheet from "./components/manage-user-groups-sheet.svelte";
import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";
import { BilingualNameFields } from "@orbis/ui/bilingual-name-fields";
import { suggestLocalTranslation } from "$lib/shared/translation/local-engine";
import { userAvatarDrafts } from "./user-avatar-draft.svelte";

let { orgId, userId }: { orgId: string; userId: string } = $props();

const userQuery = createUserQuery(
  () => orgId,
  () => userId,
);
const queryClient = useQueryClient();
const user = $derived((userQuery.data as User | null) ?? null);
const isLoading = $derived(userQuery.isLoading);
let isSaving = $state(false);

// Groups query & User Group memberships
const groupsQuery = createQuery<GroupDetailDto[]>(() => ({
  queryKey: ["groups", orgId],
  queryFn: () => api.listGroups(),
  staleTime: 5 * 60 * 1000,
}));

const allGroups = $derived(groupsQuery.data ?? []);

const userGroupsQuery = createQuery<GroupDetailDto[]>(() => ({
  queryKey: ["user_assigned_groups", orgId, userId],
  queryFn: async () => {
    const groups = await api.listGroups();
    const memberships = await Promise.all(
      groups.map(async (g) => {
        try {
          const members = await api.getGroupMembers(g.id);
          const isMember = members.some((m) => m.user_id === userId);
          return isMember ? g : null;
        } catch {
          return null;
        }
      }),
    );
    return memberships.filter(Boolean) as GroupDetailDto[];
  },
  enabled: !!userId,
  staleTime: 60 * 1000,
}));

const userAssignedGroups = $derived(userGroupsQuery.data ?? []);
const userAssignedGroupIds = $derived(userAssignedGroups.map((g) => g.id));

// Manage Groups Drawer Sheet State
let groupsSheetOpen = $state(false);

// Form fields
let fullName = $state("");
let nameAr = $state("");
let email = $state("");
let phone = $state("");
let role = $state("staff");
let isActiveState = $state(true);

let initializedUserId = $state<string | null>(null);
$effect(() => {
  const fetched = user;
  if (!fetched || initializedUserId === userId) return;
  fullName = fetched.full_name || "";
  nameAr = fetched.full_name_ar || "";
  email = fetched.email || "";
  phone = (fetched.phone as string) || "";
  role = fetched.roles?.[0] || "staff";
  isActiveState =
    fetched.is_active !== undefined ? Boolean(fetched.is_active) : true;
  initializedUserId = userId;
});

const isDirty = $derived.by(() => {
  if (!user) return false;
  return (
    fullName !== (user.full_name || "") ||
    nameAr !== (user.full_name_ar || "") ||
    email !== (user.email || "") ||
    phone !== ((user.phone as string) || "") ||
    role !== (user.roles?.[0] || "staff") ||
    isActiveState !== (user.is_active !== undefined ? Boolean(user.is_active) : true) ||
    !!userAvatarDrafts[userId]
  );
});

function openManageGroups() {
  groupsSheetOpen = true;
}

function resetForm() {
  if (!user) return;
  fullName = user.full_name || "";
  nameAr = user.full_name_ar || "";
  email = user.email || "";
  phone = (user.phone as string) || "";
  role = user.roles?.[0] || "staff";
  isActiveState =
    user.is_active !== undefined ? Boolean(user.is_active) : true;
  userAvatarDrafts[userId] = null;
}

async function toggleAccountStatus() {
  if (!userId || !user) return;
  const nextState = !isActiveState;
  const confirmMsg = nextState
    ? m.settings_user_detail_activate_confirm
      ? m.settings_user_detail_activate_confirm()
      : m.ui_copy_activate_account_89c1b85d()
    : m.settings_user_detail_deactivate_confirm
      ? m.settings_user_detail_deactivate_confirm()
      : m.ui_copy_deactivate_account_084a7a7b();
  const confirmed = await confirmStore.prompt({
    title: m.common_word_status ? m.common_word_status() : m.common_status(),
    description: confirmMsg,
    confirmText: m.common_word_status
      ? m.common_word_status()
      : m.ui_copy_confirm_0100f51c(),
    variant: nextState ? "default" : "destructive",
  });
  if (!confirmed) return;

  isSaving = true;
  try {
    await api.updateUser(userId, { is_active: nextState });
    isActiveState = nextState;
    toast.success(
      nextState
        ? m.settings_user_detail_activate_success
          ? m.settings_user_detail_activate_success()
          : m.ui_copy_account_activated_ecaf5563()
        : m.settings_user_detail_deactivate_success
          ? m.settings_user_detail_deactivate_success()
          : m.ui_copy_account_deactivated_37ae84cf(),
    );
  } catch (e: unknown) {
    toast.error((e as any)?.message || m.ui_copy_update_failed_19a589b0());
  } finally {
    isSaving = false;
  }
}

async function saveProfile() {
  if (!userId) return;
  isSaving = true;
  try {
    const avatarFile = userAvatarDrafts[userId];
    let avatarUrl: string | undefined;
    if (avatarFile) {
      const uploaded = await api.uploadFile({
        file: avatarFile,
        name: avatarFile.name,
        entity_type: "user",
        entity_id: userId,
        category: "avatar",
        asset_type: "image",
        visibility: "public",
      });
      avatarUrl = uploaded.url || uploaded.file_path;
    }

    await api.updateUser(userId, {
      full_name: fullName || undefined,
      full_name_ar: nameAr || undefined,
      phone: phone || undefined,
      is_active: isActiveState,
      roles: role ? [role] : user?.roles,
      ...(avatarUrl ? { avatar_url: avatarUrl } : {}),
    });
    userAvatarDrafts[userId] = null;

    await queryClient.invalidateQueries({ queryKey: ["user", orgId, userId] });
    await queryClient.invalidateQueries({ queryKey: ["users", orgId] });
    await queryClient.invalidateQueries({
      queryKey: ["users_paginated", orgId],
    });
    if (avatarUrl) {
      await queryClient.invalidateQueries({ queryKey: queryKeys.document.all });
    }

    toast.success(
      m.settings_user_detail_saved
        ? m.settings_user_detail_saved()
        : m.ui_copy_profile_saved_successfully_42b4e478(),
    );
    await userQuery.refetch();
  } catch (e: unknown) {
    toast.error((e as any)?.message || m.ui_copy_save_failed_38ef99fb());
  } finally {
    isSaving = false;
  }
}
</script>

{#if isLoading}
  <div class="flex flex-col gap-6 text-start">
    <div class="border-b border-border/60 pb-3">
      <Skeleton class="h-5 w-32" />
    </div>
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <div class="flex flex-col gap-2">
        <Skeleton class="h-3 w-16" />
        <Skeleton class="h-9 w-full" />
      </div>
      <div class="flex flex-col gap-2">
        <Skeleton class="h-3 w-16" />
        <Skeleton class="h-9 w-full" />
      </div>
    </div>
  </div>
{:else}
  <div in:fade={{ duration: 100 }} class="flex flex-col gap-6 text-start">
    <!-- Status Switch -->
    <div class="flex items-center gap-3">
      <Checkbox
        id="user_enabled"
        checked={isActiveState}
        onCheckedChange={toggleAccountStatus}
      />
      <Label
        for="user_enabled"
        class="text-xs font-semibold text-foreground cursor-pointer select-none"
      >
        {m.ui_copy_enabled_aceb3f59()}
      </Label>
    </div>

    <!-- Main Profile Fields -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <div class="md:col-span-2">
        <BilingualNameFields
          arId="nameAr"
          enId="fullName"
          arLabel={m.ui_copy_full_name_arabic_4871e6bf()}
          enLabel={m.common_word_fullname()}
          bind:arValue={nameAr}
          bind:enValue={fullName}
          kind="person-name"
          requestSuggestion={suggestLocalTranslation}
          enRequired
          disabled={isSaving}
          loadingMessage={m.bilingual_entry_loading()}
          failureMessage={m.bilingual_entry_failure()}
        />
      </div>

      <!-- Email -->
      <div class="flex flex-col gap-1.5 text-start">
        <Label for="email" class="text-xs font-medium text-muted-foreground">
          {m.common_word_email()} *
        </Label>
        <Input id="email" type="email" bind:value={email} required />
      </div>

      <!-- Dynamic Primary Role Select -->
      <div class="flex flex-col gap-1.5 text-start">
        <Label for="role" class="text-xs font-medium text-muted-foreground">
          {m.ui_copy_primary_role_b4890578()}
        </Label>
        <RoleSelect bind:value={role} />
      </div>

      <!-- Phone -->
      <div class="flex flex-col gap-1.5 text-start">
        <Label for="phone" class="text-xs font-medium text-muted-foreground">
          {m.employees_phone()}
        </Label>
        <Input id="phone" type="text" bind:value={phone} />
      </div>
    </div>

    <!-- Assigned User Groups Card -->
    <div class="flex flex-col gap-3 rounded-xl border border-border/70 bg-card p-4 mt-2 shadow-2xs">
      <div class="flex items-center justify-between border-b border-border/50 pb-2.5">
        <div class="flex items-center gap-2">
          <UserGroup size={16} class="text-primary" />
          <h3 class="text-xs font-bold text-foreground uppercase tracking-wider">{m.employees_user_groups()}</h3>
        </div>
        <Button
          variant="outline"
          size="sm"
          class="text-xs h-7 gap-1.5 cursor-pointer"
          onclick={openManageGroups}
        >
          <Pencil size={12} />
          <span>{m.ui_copy_manage_groups_bb35a396()}</span>
        </Button>
      </div>

      {#if groupsQuery.isLoading}
        <div class="flex gap-2 py-2">
          <Skeleton class="h-6 w-24 rounded-full" />
          <Skeleton class="h-6 w-28 rounded-full" />
        </div>
      {:else if userAssignedGroups.length === 0}
        <span class="text-xs text-muted-foreground italic py-1">
          {m.ui_copy_no_groups_currently_assigned_to_this_user_2a949232()}
        </span>
      {:else}
        <div class="flex flex-wrap gap-2 py-1">
          {#each userAssignedGroups as grp (grp.id)}
            <Badge variant="secondary" class="text-xs py-1 px-2.5 font-medium flex items-center gap-1.5">
              <UserGroup size={12} />
              <span>{l(grp.name_ar, grp.name) || grp.name}</span>
            </Badge>
          {/each}
        </div>
      {/if}
    </div>

  </div>

  <SettingsBottomBar
    visible={isDirty}
    {isDirty}
    {isSaving}
    onsave={saveProfile}
    onreset={resetForm}
  />

  <!-- Manage User Groups Drawer Sheet Component -->
  <ManageUserGroupsSheet
    bind:open={groupsSheetOpen}
    {orgId}
    {userId}
    userName={user?.full_name || fullName}
    assignedGroupIds={userAssignedGroupIds}
    onsaved={() => groupsQuery.refetch()}
  />
{/if}
