<!--
@component UserProfilePage
Profile tab for user detail — personal information, role, and status.
Extracted from the original monolith TabsContent "profile".
-->
<script lang="ts">
  import { page } from "$app/state";
  import { onMount } from "svelte";
  import { commands } from "$lib/services/ipc";
  import type { User } from "$lib/services/ipc";
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
  import { toast } from "svelte-sonner";
  import { fade } from "svelte/transition";
  import * as m from "$lib/paraglide/messages";

  const orgId = $derived(page.params.orgId);
  const userId = $derived(page.params.userId);

  let user = $state<User | null>(null);
  let isLoading = $state(true);
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
    { value: "super_admin", label: "Super Admin" },
    { value: "admin", label: "System Admin" },
    { value: "principal", label: "School Principal" },
    { value: "teacher", label: "Teacher" },
    { value: "staff", label: "Staff Member" },
    { value: "student", label: "Student" },
    { value: "parent", label: "Parent / Guardian" },
  ];

  async function loadUser() {
    if (!userId) return;
    isLoading = true;
    try {
      const fetched = await commands.users.getUser(userId) as any;
      user = fetched;
      fullName = fetched.full_name || "";
      nameAr = fetched.name_ar || "";
      nameEn = fetched.name_en || "";
      email = fetched.email || "";
      phone = (fetched.phone as string) || "";
      role = fetched.primary_role || fetched.role || "teacher";
      isActiveState =
        fetched.is_active !== undefined ? Boolean(fetched.is_active) : true;
    } catch (e) {
      console.error("Failed to load user:", e);
      toast.error(m.settings_user_detail_load_error());
    } finally {
      isLoading = false;
    }
  }

  onMount(loadUser);

  async function toggleAccountStatus() {
    if (!userId || !user) return;
    const nextState = !isActiveState;
    const confirmMsg = nextState
      ? m.settings_user_detail_activate_confirm()
      : m.settings_user_detail_deactivate_confirm();
    if (!confirm(confirmMsg)) return;

    isSaving = true;
    try {
      const request = { ...user, is_active: nextState };
      await commands.users.updateUser(userId, request);
      isActiveState = nextState;
      toast.success(
        nextState
          ? m.settings_user_detail_activate_success()
          : m.settings_user_detail_deactivate_success(),
      );
    } catch (e: unknown) {
      toast.error((e as any)?.message || "Failed");
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
      await commands.users.updateUser(userId, request);

      if (role) {
        await commands.users.changeUserRole(userId, { role });
      }

      toast.success(m.settings_user_detail_saved());
      loadUser();
    } catch (e: unknown) {
      toast.error((e as any)?.message || "Save failed");
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
        class="text-xs font-bold text-foreground cursor-pointer select-none"
      >
        Enabled
      </Label>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <!-- Email -->
      <div class="flex flex-col gap-1.5 text-start">
        <Label for="email" class="text-xs font-medium text-muted-foreground">
          {m.settings_user_detail_label_email()} *
        </Label>
        <Input id="email" type="email" bind:value={email} required />
      </div>

      <!-- Full Name -->
      <div class="flex flex-col gap-1.5 text-start">
        <Label
          for="fullName"
          class="text-xs font-medium text-muted-foreground"
        >
          {m.settings_user_detail_label_fullname()}
        </Label>
        <Input id="fullName" type="text" bind:value={fullName} />
      </div>

      <!-- Primary Role -->
      <div class="flex flex-col gap-1.5 text-start">
        <Label for="role" class="text-xs font-medium text-muted-foreground">
          {m.settings_user_detail_label_primary_role()}
        </Label>
        <Select
          type="single"
          value={role}
          onValueChange={(v) => (role = v ?? "")}
        >
          <SelectTrigger class="h-9 w-full">
            {systemRoles.find((r) => r.value === role)?.label || role}
          </SelectTrigger>
          <SelectContent>
            {#each systemRoles as r}
              <SelectItem value={r.value}>{r.label}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>

      <!-- Arabic Name -->
      <div class="flex flex-col gap-1.5 text-start">
        <Label for="nameAr" class="text-xs font-medium text-muted-foreground">
          {m.settings_user_detail_label_fullname_ar()}
        </Label>
        <Input id="nameAr" type="text" bind:value={nameAr} />
      </div>

      <!-- English Name -->
      <div class="flex flex-col gap-1.5 text-start">
        <Label for="nameEn" class="text-xs font-medium text-muted-foreground">
          {m.settings_user_detail_label_fullname_en()}
        </Label>
        <Input id="nameEn" type="text" bind:value={nameEn} />
      </div>

      <!-- Phone -->
      <div class="flex flex-col gap-1.5 text-start">
        <Label for="phone" class="text-xs font-medium text-muted-foreground">
          {m.settings_user_detail_label_phone()}
        </Label>
        <Input id="phone" type="text" bind:value={phone} />
      </div>
    </div>

    <div class="flex justify-end gap-3 border-t border-border/20 pt-6 mt-4">
      <Button variant="ghost" size="sm" onclick={loadUser}>
        {m.settings_user_detail_btn_reset()}
      </Button>
      <Button
        variant="lime"
        size="sm"
        class="font-bold"
        onclick={saveProfile}
        disabled={isSaving}
      >
        {isSaving ? "..." : m.settings_user_detail_btn_save_details()}
      </Button>
    </div>
  </div>
{/if}
