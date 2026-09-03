<!--
@component UserProfilePage
Profile tab for user detail — personal information, role, and status.
Follows S.C.R.A.P.S. design tokens, strict localization, and backend schema synchronization.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { confirmStore } from "$lib/stores";
import { commands } from "$lib/core/ipc";
import type { User } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { Checkbox } from "@orbis/ui-svelte/checkbox";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { toast } from "@orbis/ui-svelte";
import { fade } from "svelte/transition";
import { l } from "$lib/core/i18n";
import { createUserQuery } from "$lib/features/settings/queries";
import { useQueryClient } from "@tanstack/svelte-query";

const orgId = $derived(page.params.orgId);
const userId = $derived(page.params.userId);

const userQuery = createUserQuery(() => orgId, () => userId);
const queryClient = useQueryClient();
const user = $derived((userQuery.data as User | null) ?? null);
const isLoading = $derived(userQuery.isLoading);
let isSaving = $state(false);

// Form fields
let fullName = $state("");
let nameAr = $state("");
let nameEn = $state("");
let email = $state("");
let phone = $state("");
let role = $state("teacher");
let isActiveState = $state(true);

const systemRoles = [
  { value: "super_admin", labelAr: "مدير عام النظام", labelEn: "Super Administrator" },
  { value: "admin", labelAr: "مدير النظام", labelEn: "System Administrator" },
  { value: "principal", labelAr: "مدير المدرسة / المنشأة", labelEn: "School Principal" },
  { value: "teacher", labelAr: "معلم / هيئة تدريس", labelEn: "Teacher / Faculty" },
  { value: "registrar", labelAr: "مسجل أكاديمي", labelEn: "Academic Registrar" },
  { value: "accountant", labelAr: "محاسب مالي", labelEn: "Financial Accountant" },
  { value: "staff", labelAr: "كادر إداري", labelEn: "Administrative Staff" },
  { value: "student", labelAr: "طالب", labelEn: "Student" },
  { value: "guardian", labelAr: "ولي أمر", labelEn: "Parent / Guardian" },
];

let initializedUserId = $state<string | null>(null);
$effect(() => {
  const fetched = user;
  if (!fetched || initializedUserId === userId) return;
  fullName = fetched.full_name || "";
  nameAr = fetched.name_ar || "";
  nameEn = fetched.name_en || "";
  email = fetched.email || "";
  phone = (fetched.phone as string) || "";
  role = fetched.primary_role || fetched.role || "teacher";
  isActiveState = fetched.is_active !== undefined ? Boolean(fetched.is_active) : true;
  initializedUserId = userId;
});

async function toggleAccountStatus() {
  if (!userId || !user) return;
  const nextState = !isActiveState;
  const confirmMsg = nextState
    ? (m.settings_user_detail_activate_confirm ? m.settings_user_detail_activate_confirm() : l("هل أنت متأكد من تفعيل الحساب؟", "Activate account?"))
    : (m.settings_user_detail_deactivate_confirm ? m.settings_user_detail_deactivate_confirm() : l("هل أنت متأكد من تعطيل الحساب؟", "Deactivate account?"));
  const confirmed = await confirmStore.prompt({
    title: (m.common_word_status ? m.common_word_status() : l("الحالة", "Status")) + "؟",
    description: confirmMsg,
    confirmText: m.common_word_status ? m.common_word_status() : l("تأكيد", "Confirm"),
    variant: nextState ? "default" : "destructive",
  });
  if (!confirmed) return;

  isSaving = true;
  try {
    const request = { ...user, is_active: nextState };
    await commands.iam.updateUser(userId, request);
    isActiveState = nextState;
    toast.success(
      nextState
        ? (m.settings_user_detail_activate_success ? m.settings_user_detail_activate_success() : l("تم تفعيل الحساب بنجاح", "Account activated"))
        : (m.settings_user_detail_deactivate_success ? m.settings_user_detail_deactivate_success() : l("تم تعطيل الحساب بنجاح", "Account deactivated")),
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
    };
    await commands.iam.updateUser(userId, request);

    if (role) {
      await commands.iam.changeRole(userId, { role });
      const existingRoles = await commands.iam.listRolesByUser(userId);
      const roleList = (existingRoles || []).map((r: any) => r.role);
      if (!roleList.includes(role)) {
        roleList.unshift(role);
      }
      await commands.iam.saveForUser(userId, roleList.length > 0 ? roleList : [role]);
    }

    await queryClient.invalidateQueries({ queryKey: ["user", orgId, userId] });
    await queryClient.invalidateQueries({ queryKey: ["users", orgId] });
    await queryClient.invalidateQueries({ queryKey: ["users_paginated", orgId] });

    toast.success(m.settings_user_detail_saved ? m.settings_user_detail_saved() : l("تم حفظ البيانات بنجاح", "Profile saved successfully"));
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

      <!-- Primary Role -->
      <div class="flex flex-col gap-1.5 text-start">
        <Label for="role" class="text-xs font-medium text-muted-foreground">
          {m.settings_user_detail_label_primary_role ? m.settings_user_detail_label_primary_role() : l("الدور الرئيسي", "Primary Role")}
        </Label>
        <Select
          type="single"
          value={role}
          onValueChange={(v) => (role = v ?? "")}
        >
          <SelectTrigger class="h-9 w-full text-xs">
            {@const selected = systemRoles.find((r) => r.value === role)}
            {selected ? l(selected.labelAr, selected.labelEn) : role}
          </SelectTrigger>
          <SelectContent>
            {#each systemRoles as r}
              <SelectItem value={r.value}>
                {l(r.labelAr, r.labelEn)}
              </SelectItem>
            {/each}
          </SelectContent>
        </Select>
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

    <!-- Actions Toolbar -->
    <div class="flex justify-end gap-3 border-t border-border/40 pt-6 mt-4">
      <Button variant="ghost" size="sm" onclick={loadUser}>
        {m.common_switcher_reset ? m.common_switcher_reset() : l("إعادة تعيين", "Reset")}
      </Button>
      <Button
        variant="lime"
        size="sm"
        class="font-bold px-6"
        onclick={saveProfile}
        disabled={isSaving}
      >
        {isSaving ? "..." : (m.settings_user_detail_btn_save_details ? m.settings_user_detail_btn_save_details() : l("حفظ التعديلات", "Save Changes"))}
      </Button>
    </div>
  </div>
{/if}
