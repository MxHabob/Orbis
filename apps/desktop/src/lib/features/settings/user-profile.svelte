<!--
  @component UserProfile
  User profile management in Settings.
  Integrates demographic data, dynamic role assignment, user status toggles,
  and User Groups management via side Sheet.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { confirmStore } from "$lib/stores";
import { api, type GroupDetailDto, type User } from "$lib/api";
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

let { orgId, userId }: { orgId: string; userId: string } = $props();

const userQuery = createUserQuery(
  () => orgId,
  () => userId,
);
const queryClient = useQueryClient();
const user = $derived((userQuery.data as User | null) ?? null);
const isLoading = $derived(userQuery.isLoading);
let isSaving = $state(false);

// Groups query
const groupsQuery = createQuery<GroupDetailDto[]>(() => ({
  queryKey: ["groups", orgId],
  queryFn: () => api.listGroups(),
  staleTime: 5 * 60 * 1000,
}));

const allGroups = $derived(groupsQuery.data ?? []);

// Compute assigned groups for this user
const userAssignedGroups = $derived.by<GroupDetailDto[]>(() => {
  if (!allGroups.length || !userId) return [];
  return allGroups.filter((g) =>
    g.members?.some((m) => m.user_id === userId)
  );
});

const userAssignedGroupIds = $derived(userAssignedGroups.map((g) => g.id));

// Manage Groups Drawer Sheet State
let groupsSheetOpen = $state(false);

// Form fields
let fullName = $state("");
let nameAr = $state("");
let nameEn = $state("");
let email = $state("");
let phone = $state("");
let role = $state("staff");
let isActiveState = $state(true);

let initializedUserId = $state<string | null>(null);
$effect(() => {
  const fetched = user;
  if (!fetched || initializedUserId === userId) return;
  fullName = fetched.full_name || "";
  nameAr = fetched.name_ar || "";
  nameEn = fetched.name_en || "";
  email = fetched.email || "";
  phone = (fetched.phone as string) || "";
  role = fetched.roles?.[0] || "staff";
  isActiveState =
    fetched.is_active !== undefined ? Boolean(fetched.is_active) : true;
  initializedUserId = userId;
});

function openManageGroups() {
  groupsSheetOpen = true;
}

function resetForm() {
  if (!user) return;
  fullName = user.full_name || "";
  nameAr = user.name_ar || "";
  nameEn = user.name_en || "";
  email = user.email || "";
  phone = (user.phone as string) || "";
  role = user.roles?.[0] || "staff";
  isActiveState =
    user.is_active !== undefined ? Boolean(user.is_active) : true;
}

async function toggleAccountStatus() {
  if (!userId || !user) return;
  const nextState = !isActiveState;
  const confirmMsg = nextState
    ? m.settings_user_detail_activate_confirm
      ? m.settings_user_detail_activate_confirm()
      : l("هل أنت متأكد من تفعيل الحساب؟", "Activate account?")
    : m.settings_user_detail_deactivate_confirm
      ? m.settings_user_detail_deactivate_confirm()
      : l("هل أنت متأكد من تعطيل الحساب؟", "Deactivate account?");
  const confirmed = await confirmStore.prompt({
    title:
      (m.common_word_status ? m.common_word_status() : l("الحالة", "Status")) +
      "؟",
    description: confirmMsg,
    confirmText: m.common_word_status
      ? m.common_word_status()
      : l("تأكيد", "Confirm"),
    variant: nextState ? "default" : "destructive",
  });
  if (!confirmed) return;

  isSaving = true;
  try {
    const request = { ...user, is_active: nextState };
    await api.updateUser(userId, request as any);
    isActiveState = nextState;
    toast.success(
      nextState
        ? m.settings_user_detail_activate_success
          ? m.settings_user_detail_activate_success()
          : l("تم تفعيل الحساب بنجاح", "Account activated")
        : m.settings_user_detail_deactivate_success
          ? m.settings_user_detail_deactivate_success()
          : l("تم تعطيل الحساب بنجاح", "Account deactivated"),
    );
  } catch (e: unknown) {
    toast.error((e as any)?.message || l("فشل التحديث", "Update failed"));
  } finally {
    isSaving = false;
  }
}

async function saveProfile() {
  if (!userId) return;
  isSaving = true;
  try {
    const request = {
      ...user,
      full_name: fullName,
      name_ar: nameAr || null,
      name_en: nameEn || null,
      email: email || null,
      phone: phone || null,
      is_active: isActiveState,
      roles: role ? [role] : user?.roles,
    };
    await api.updateUser(userId, request as any);

    await queryClient.invalidateQueries({ queryKey: ["user", orgId, userId] });
    await queryClient.invalidateQueries({ queryKey: ["users", orgId] });
    await queryClient.invalidateQueries({
      queryKey: ["users_paginated", orgId],
    });

    toast.success(
      m.settings_user_detail_saved
        ? m.settings_user_detail_saved()
        : l("تم حفظ البيانات بنجاح", "Profile saved successfully"),
    );
    await userQuery.refetch();
  } catch (e: unknown) {
    toast.error((e as any)?.message || l("فشل الحفظ", "Save failed"));
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
        {l("مفعّل (نشط)", "Enabled")}
      </Label>
    </div>

    <!-- Main Profile Fields -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <!-- Full Name -->
      <div class="flex flex-col gap-1.5 text-start">
        <Label
          for="fullName"
          class="text-xs font-medium text-muted-foreground"
        >
          {m.common_word_fullname ? m.common_word_fullname() : l("الاسم الكامل", "Full Name")} *
        </Label>
        <Input id="fullName" type="text" bind:value={fullName} required />
      </div>

      <!-- Email -->
      <div class="flex flex-col gap-1.5 text-start">
        <Label for="email" class="text-xs font-medium text-muted-foreground">
          {m.common_word_email ? m.common_word_email() : l("البريد الإلكتروني", "Email")} *
        </Label>
        <Input id="email" type="email" bind:value={email} required />
      </div>

      <!-- Arabic Name -->
      <div class="flex flex-col gap-1.5 text-start">
        <Label for="nameAr" class="text-xs font-medium text-muted-foreground">
          {m.common_form_full_name_ar ? m.common_form_full_name_ar() : l("الاسم بالكامل (عربي)", "Full Name (Arabic)")}
        </Label>
        <Input id="nameAr" type="text" bind:value={nameAr} />
      </div>

      <!-- Dynamic Primary Role Select -->
      <div class="flex flex-col gap-1.5 text-start">
        <Label for="role" class="text-xs font-medium text-muted-foreground">
          {m.settings_user_detail_label_primary_role ? m.settings_user_detail_label_primary_role() : l("الدور الرئيسي", "Primary Role")}
        </Label>
        <RoleSelect bind:value={role} />
      </div>

      <!-- Phone -->
      <div class="flex flex-col gap-1.5 text-start">
        <Label for="phone" class="text-xs font-medium text-muted-foreground">
          {m.settings_user_detail_label_phone ? m.settings_user_detail_label_phone() : l("رقم الهاتف", "Phone Number")}
        </Label>
        <Input id="phone" type="text" bind:value={phone} />
      </div>

      <!-- English Name -->
      <div class="flex flex-col gap-1.5 text-start">
        <Label for="nameEn" class="text-xs font-medium text-muted-foreground">
          {m.settings_user_detail_label_fullname_en ? m.settings_user_detail_label_fullname_en() : l("الاسم بالكامل (إنجليزي)", "Full Name (English)")}
        </Label>
        <Input id="nameEn" type="text" bind:value={nameEn} />
      </div>
    </div>

    <!-- Assigned User Groups Card -->
    <div class="flex flex-col gap-3 rounded-xl border border-border/70 bg-card p-4 mt-2 shadow-2xs">
      <div class="flex items-center justify-between border-b border-border/50 pb-2.5">
        <div class="flex items-center gap-2">
          <UserGroup size={16} class="text-primary" />
          <h3 class="text-xs font-bold text-foreground uppercase tracking-wider">{l("مجموعات المستخدمين المسندة", "Assigned User Groups")}</h3>
        </div>
        <Button
          variant="outline"
          size="sm"
          class="text-xs h-7 gap-1.5 cursor-pointer"
          onclick={openManageGroups}
        >
          <Pencil size={12} />
          <span>{l("إدارة المجموعات", "Manage Groups")}</span>
        </Button>
      </div>

      {#if groupsQuery.isLoading}
        <div class="flex gap-2 py-2">
          <Skeleton class="h-6 w-24 rounded-full" />
          <Skeleton class="h-6 w-28 rounded-full" />
        </div>
      {:else if userAssignedGroups.length === 0}
        <span class="text-xs text-muted-foreground italic py-1">
          {l("لا توجد مجموعات مسندة لهذا المستخدم حالياً.", "No groups currently assigned to this user.")}
        </span>
      {:else}
        <div class="flex flex-wrap gap-2 py-1">
          {#each userAssignedGroups as grp}
            <Badge variant="secondary" class="text-xs py-1 px-2.5 font-medium flex items-center gap-1.5">
              <UserGroup size={12} />
              <span>{l(grp.name_ar, grp.name) || grp.name}</span>
            </Badge>
          {/each}
        </div>
      {/if}
    </div>

    <!-- Actions Toolbar -->
    <div class="flex justify-end gap-3 border-t border-border/40 pt-6 mt-2">
      <Button variant="ghost" size="sm" onclick={resetForm}>
        {m.common_switcher_reset ? m.common_switcher_reset() : l("إعادة تعيين", "Reset")}
      </Button>
      <Button
        variant="lime"
        size="sm"
        class="font-bold px-6 cursor-pointer"
        onclick={saveProfile}
        disabled={isSaving}
      >
        {isSaving ? "..." : (m.settings_user_detail_btn_save_details ? m.settings_user_detail_btn_save_details() : l("حفظ التعديلات", "Save Changes"))}
      </Button>
    </div>
  </div>

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
