<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { goto } from "$app/navigation";
  import { fade } from "svelte/transition";
  import { Button } from "@orbis/ui-svelte/button";
  import { Input } from "@orbis/ui-svelte/input";
  import { Skeleton } from "@orbis/ui-svelte/skeleton";
  import { Badge } from "@orbis/ui-svelte/badge";
  import { Checkbox } from "@orbis/ui-svelte/checkbox";
  import { Switch } from "@orbis/ui-svelte/switch";
  import { Tabs, TabsList, TabsTrigger, TabsContent } from "@orbis/ui-svelte/tabs";
  import { Dialog, DialogContent, DialogTitle, DialogDescription, DialogHeader, DialogFooter } from "@orbis/ui-svelte/dialog";
  import { Label } from "@orbis/ui-svelte/label";
  import { Select, SelectTrigger, SelectContent, SelectItem } from "@orbis/ui-svelte/select";
  import { toast } from "svelte-sonner";
  import { getLocale, localizeHref } from "$lib/paraglide/runtime";
  import * as m from "$lib/paraglide/messages";
  import { commands } from "$lib/services/ipc";
  import type { User } from "$lib/services/ipc";
  import {
    ArrowLeft,
    ArrowRight,
    Monitor,
    Smartphone,
    Trash2,
    Layout,
    Globe,
  } from "lucide-svelte";

  // Derive params
  let orgId = $derived(page.params.orgId);
  let userId = $derived(page.params.userId);

  // Tab state
  let activeTab = $state<"profile" | "roles" | "sessions" | "settings">("profile");

  // User data state
  let user = $state<User | null>(null);
  let isLoading = $state(true);
  let isSaving = $state(false);
  let isActiveState = $state(true);

  // Password Reset Dialog State
  let showResetModal = $state(false);
  let newPassword = $state("");
  let confirmPassword = $state("");
  let isResettingPassword = $state(false);

  // Form Fields
  let fullName = $state("");
  let nameAr = $state("");
  let nameEn = $state("");
  let email = $state("");
  let phone = $state("");
  let role = $state("teacher");

  // Multi-roles checklist state
  let checkedRoles = $state<string[]>([]);
  let roleSearchQuery = $state("");

  // Role Profile List
  let allAvailableRoles = $state<string[]>([]);
  let filteredRoles = $derived(allAvailableRoles.filter(r => r.toLowerCase().includes(roleSearchQuery.toLowerCase())));

  // User Preferences State
  let prefTheme = $state("dark");
  let prefLanguage = $state("ar");
  let notifyEmail = $state(true);
  let notifyPush = $state(true);

  // Available system roles select list
  const systemRoles = [
    { value: "super_admin", label: "Super Admin" },
    { value: "admin", label: "System Admin" },
    { value: "principal", label: "School Principal" },
    { value: "teacher", label: "Teacher" },
    { value: "staff", label: "Staff Member" },
    { value: "student", label: "Student" },
    { value: "parent", label: "Parent / Guardian" },
  ];

  // Mock Active Sessions
  const activeSessions = [
    {
      id: "sess_1",
      device: "Windows PC (Tauri Desktop App)",
      location: "Sana'a, Yemen (Current Device)",
      ip: "109.74.32.18",
      active: true,
      icon: Monitor,
      time: "Active now",
    },
    {
      id: "sess_2",
      device: "iPhone 15 Pro (Safari Mobile)",
      location: "Riyadh, Saudi Arabia",
      ip: "37.220.112.4",
      active: false,
      icon: Smartphone,
      time: "2 hours ago",
    }
  ];

  async function loadUser() {
    if (!userId) return;
    isLoading = true;
    try {
      const fetched = await commands.users.getUser(userId);
      user = fetched;

      // Bind form fields
      fullName = fetched.full_name || "";
      nameAr = fetched.name_ar || "";
      nameEn = fetched.name_en || "";
      email = fetched.email || "";
      phone = (fetched.phone as string) || "";
      role = fetched.primary_role || fetched.role || "teacher";
      isActiveState = fetched.is_active !== undefined ? Boolean(fetched.is_active) : true;

      // Fetch user specific overrides from database
      try {
        await commands.permissions.getUserPermissions(userId);
      } catch (err) {
        console.error("Failed to load user permissions overrides", err);
      }

      // Fetch dynamic user roles list
      try {
        checkedRoles = await commands.users.getUserRoles(userId);
      } catch (e) {
        console.error("Failed to load user roles list:", e);
      }

      // Fetch all available roles list dynamically from database
      try {
        allAvailableRoles = await commands.permissions.getAvailableRoles();
      } catch (e) {
        console.error("Failed to load available roles from db:", e);
      }

    } catch (e) {
      console.error("Failed to load user:", e);
      toast.error(m.settings_user_detail_load_error());
    } finally {
      isLoading = false;
    }
  }

  onMount(() => {
    loadUser();
  });

  // Global Unified Save changes
  async function saveAllChanges() {
    if (!userId) return;
    isSaving = true;
    try {
      const request = {
        full_name: fullName,
        name_ar: nameAr || null,
        name_en: nameEn || null,
        email: email || null,
        phone: phone || null,
        is_active: isActiveState,
      };
      await commands.users.updateUser(userId, request);

      // Only modify base role based on the primary system roles dropdown
      if (role) {
        await commands.users.changeUserRole(userId, { role });
      }

      // Save additional roles checklist values
      await commands.users.saveUserRoles(userId, checkedRoles);

      toast.success(m.settings_user_detail_saved());
      loadUser();
    } catch (e: unknown) {
      toast.error((e as any)?.message || "Save failed");
    } finally {
      isSaving = false;
    }
  }

  // Toggle Account Active / Inactive
  async function toggleAccountStatus() {
    if (!userId || !user) return;

    const nextState = !isActiveState;
    const confirmMsg = nextState
      ? m.settings_user_detail_activate_confirm()
      : m.settings_user_detail_deactivate_confirm();

    if (!confirm(confirmMsg)) return;

    isSaving = true;
    try {
      const request = {
        ...user,
        is_active: nextState,
      };
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

  // Reset Password
  async function handleResetPassword(e: Event) {
    e.preventDefault();
    if (newPassword !== confirmPassword) {
      toast.error(m.settings_user_detail_reset_mismatch());
      return;
    }

    if (!userId) return;
    isResettingPassword = true;
    try {
      await commands.users.changeUserPassword(userId as string, {
        new_password: newPassword,
      });
      toast.success(m.settings_user_detail_reset_success());
      showResetModal = false;
      newPassword = "";
      confirmPassword = "";
    } catch (e: unknown) {
      toast.error((e as any)?.message || "Failed");
    } finally {
      isResettingPassword = false;
    }
  }

  // Delete User Account
  async function handleDeleteUser() {
    if (!userId) return;
    if (!confirm(m.settings_user_detail_delete_confirm())) return;
    try {
      await commands.users.deleteUser(userId as string, "admin");
      toast.success(m.settings_user_detail_deleted());
      await goto(localizeHref(`/${orgId}/settings/users`));
    } catch (e: unknown) {
      toast.error((e as any)?.message || "Failed");
    }
  }

  function selectAllRoles() {
    checkedRoles = [...allAvailableRoles];
  }

  function unselectAllRoles() {
    checkedRoles = [];
  }

  function toggleRole(roleName: string, isChecked: boolean) {
    if (isChecked) {
      if (!checkedRoles.includes(roleName)) {
        checkedRoles = [...checkedRoles, roleName];
      }
    } else {
      checkedRoles = checkedRoles.filter((r) => r !== roleName);
    }
  }
</script>

<svelte:head>
  <title>{fullName || m.settings_user_detail_title()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto bg-background scrollbar-hide">
  <div class="max-w-3xl mx-auto p-8 flex flex-col gap-10">
    <!-- Header -->
    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 border-b border-border/40 pb-5">
      <div class="flex flex-col gap-1 text-start">
        <div class="flex items-center gap-3">
          <h1 class="text-xl font-bold tracking-tight text-foreground">
            {fullName || m.settings_user_detail_title()}
          </h1>
          {#if isActiveState}
            <Badge variant="lime">{m.settings_user_detail_badge_active()}</Badge>
          {:else}
            <Badge variant="destructive">{m.settings_user_detail_badge_inactive()}</Badge>
          {/if}
        </div>
        {#if email}
          <p class="text-xs text-muted-foreground">{email}</p>
        {/if}
      </div>

      <div class="flex items-center gap-2 flex-wrap sm:flex-nowrap shrink-0 justify-end">
        <Button
          variant="outline"
          size="sm"
          href={localizeHref(`/${orgId}/settings/users`)}
          class="gap-1.5"
        >
          {#if getLocale() === "ar"}
            <ArrowRight size={14} />
          {:else}
            <ArrowLeft size={14} />
          {/if}
          {m.settings_user_detail_back()}
        </Button>

        <Button
          variant="outline"
          size="sm"
          onclick={() => showResetModal = true}
        >
          {m.settings_user_detail_btn_reset_pass()}
        </Button>

        <Button
          variant="destructive"
          size="sm"
          onclick={handleDeleteUser}
        >
          <Trash2 size={14} />
          {m.settings_user_detail_btn_delete()}
        </Button>

        <Button
          variant="lime"
          size="sm"
          class="px-5 font-bold"
          onclick={saveAllChanges}
          disabled={isSaving}
        >
          {isSaving ? "..." : m.settings_user_detail_save()}
        </Button>
      </div>
    </div>

    {#if isLoading}
      <!-- Loading Skeletons -->
      <div class="flex flex-col gap-8">
        {@render SectionSkeleton()}
        {@render SectionSkeleton()}
      </div>
    {:else}
      <!-- Tabs -->
      <Tabs bind:value={activeTab}>
        <TabsList variant="line" class="h-10 gap-6">
          <TabsTrigger value="profile">
            {m.settings_user_detail_tab_profile()}
          </TabsTrigger>
          <TabsTrigger value="roles">
            {m.settings_user_detail_tab_roles()}
          </TabsTrigger>
          <TabsTrigger value="sessions">
            {m.settings_user_detail_tab_sessions()}
          </TabsTrigger>
          <TabsTrigger value="settings">
            {m.settings_user_detail_tab_settings()}
          </TabsTrigger>
        </TabsList>

        <!-- Tab: Profile -->
        <TabsContent value="profile" class="mt-8">
          <div in:fade={{ duration: 100 }} class="flex flex-col gap-6 text-start">
            <div class="border-b border-border/60 pb-3">
              <h2 class="text-sm font-semibold text-foreground">{m.settings_user_detail_profile_title()}</h2>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
              <!-- Full Name -->
              <div class="flex flex-col gap-2 text-start">
                <Label for="fullName" class="text-xs font-medium text-muted-foreground">
                  {m.settings_user_detail_label_fullname()}
                </Label>
                <Input id="fullName" type="text" bind:value={fullName} required />
              </div>

              <!-- Primary System Role -->
              <div class="flex flex-col gap-2 text-start">
                <Label for="role" class="text-xs font-medium text-muted-foreground">
                  {m.settings_user_detail_label_primary_role()}
                </Label>
                <Select type="single" value={role} onValueChange={(v) => (role = v ?? "")}>
                  <SelectTrigger class="h-9 w-full">
                    {systemRoles.find(r => r.value === role)?.label || role}
                  </SelectTrigger>
                  <SelectContent>
                    {#each systemRoles as r}
                      <SelectItem value={r.value}>{r.label}</SelectItem>
                    {/each}
                  </SelectContent>
                </Select>
              </div>

              <!-- Arabic Name -->
              <div class="flex flex-col gap-2 text-start">
                <Label for="nameAr" class="text-xs font-medium text-muted-foreground">
                  {m.settings_user_detail_label_fullname_ar()}
                </Label>
                <Input id="nameAr" type="text" bind:value={nameAr} />
              </div>

              <!-- English Name -->
              <div class="flex flex-col gap-2 text-start">
                <Label for="nameEn" class="text-xs font-medium text-muted-foreground">
                  {m.settings_user_detail_label_fullname_en()}
                </Label>
                <Input id="nameEn" type="text" bind:value={nameEn} />
              </div>

              <!-- Email -->
              <div class="flex flex-col gap-2 text-start">
                <Label for="email" class="text-xs font-medium text-muted-foreground">
                  {m.settings_user_detail_label_email()}
                </Label>
                <Input id="email" type="email" bind:value={email} />
              </div>

              <!-- Phone -->
              <div class="flex flex-col gap-2 text-start">
                <Label for="phone" class="text-xs font-medium text-muted-foreground">
                  {m.settings_user_detail_label_phone()}
                </Label>
                <Input id="phone" type="text" bind:value={phone} />
              </div>
            </div>

            <!-- Account Status Toggle -->
            <div class="flex items-center justify-between border-t border-border/40 pt-4 mt-2">
              <div class="flex flex-col gap-1 text-start">
                <span class="text-xs font-semibold text-foreground">
                  {m.settings_user_detail_status_active()}
                </span>
              </div>
              <Switch
                checked={isActiveState}
                onCheckedChange={toggleAccountStatus}
              />
            </div>

            <div class="flex justify-end gap-3 border-t border-border/40 pt-6 mt-4">
              <Button variant="ghost" size="sm" onclick={loadUser}>
                {m.settings_user_detail_btn_reset()}
              </Button>
              <Button variant="lime" size="sm" class="font-bold" onclick={saveAllChanges} disabled={isSaving}>
                {isSaving ? "..." : m.settings_user_detail_btn_save_details()}
              </Button>
            </div>
          </div>
        </TabsContent>

        <!-- Tab: Roles -->
        <TabsContent value="roles" class="mt-8">
          <div in:fade={{ duration: 100 }} class="flex flex-col gap-6 text-start">
            <div class="border-b border-border/60 pb-3">
              <h2 class="text-sm font-semibold text-foreground">{m.settings_user_detail_roles_title()}</h2>
            </div>

            <!-- Search & Actions -->
            <div class="flex flex-col sm:flex-row items-center gap-3 w-full justify-between">
              <div class="w-full sm:max-w-xs">
                <Input
                  type="text"
                  bind:value={roleSearchQuery}
                  placeholder={m.settings_user_detail_search_roles()}
                />
              </div>
              <div class="flex items-center gap-2 w-full sm:w-auto justify-end">
                <Button variant="outline" size="sm" class="text-xs" onclick={selectAllRoles}>
                  {m.settings_user_detail_select_all()}
                </Button>
                <Button variant="outline" size="sm" class="text-xs" onclick={unselectAllRoles}>
                  {m.settings_user_detail_unselect_all()}
                </Button>
              </div>
            </div>

            <!-- Roles Checklist Grid -->
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4 pt-2">
              {#each filteredRoles as roleName (roleName)}
                <div class="flex items-center justify-between p-3.5 rounded-xl border border-border bg-card/25 hover:bg-accent/5 transition-all select-none">
                  <span class="text-xs font-semibold text-foreground">{roleName}</span>
                  <Checkbox
                    checked={checkedRoles.includes(roleName)}
                    onCheckedChange={(v) => toggleRole(roleName, Boolean(v))}
                  />
                </div>
              {/each}
            </div>
          </div>
        </TabsContent>

        <!-- Tab: Sessions -->
        <TabsContent value="sessions" class="mt-8">
          <div in:fade={{ duration: 100 }} class="flex flex-col gap-6 text-start">
            <div class="border-b border-border/60 pb-3">
              <h2 class="text-sm font-semibold text-foreground">{m.settings_user_detail_sessions_title()}</h2>
            </div>

            <div class="flex flex-col gap-4">
              {#each activeSessions as session (session.id)}
                <div class="flex items-center justify-between py-3 border-b border-border/40 last:border-b-0">
                  <div class="flex items-center gap-3">
                    <div class="p-2.5 rounded-lg bg-muted text-foreground">
                      <session.icon size={18} />
                    </div>
                    <div class="flex flex-col gap-0.5">
                      <span class="text-xs font-bold text-foreground flex items-center gap-2">
                        <span>{session.device}</span>
                        {#if session.active}
                          <Badge variant="lime" class="text-[8px] font-semibold uppercase py-0 px-1.5 h-4.5 rounded">
                            {m.settings_user_detail_session_active_now()}
                          </Badge>
                        {/if}
                      </span>
                      <span class="text-[10px] text-muted-foreground font-mono">
                        {session.location} • {session.ip} • {session.time}
                      </span>
                    </div>
                  </div>
                </div>
              {/each}
            </div>
          </div>
        </TabsContent>

        <!-- Tab: Settings -->
        <TabsContent value="settings" class="mt-8">
          <div in:fade={{ duration: 100 }} class="flex flex-col gap-6 text-start">
            <div class="border-b border-border/60 pb-3">
              <h2 class="text-sm font-semibold text-foreground">{m.settings_user_detail_pref_title()}</h2>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
              <!-- Theme -->
              <div class="flex flex-col gap-2 text-start">
                <Label class="text-xs font-medium text-muted-foreground flex items-center gap-1.5">
                  <Layout size={13} />
                  {m.settings_user_detail_pref_theme()}
                </Label>
                <Select type="single" value={prefTheme} onValueChange={(v) => (prefTheme = v ?? "")}>
                  <SelectTrigger class="h-9 w-full">
                    {#if prefTheme === "system"}
                      {m.settings_user_detail_theme_system()}
                    {:else}
                      {prefTheme === "light" ? m.settings_user_detail_theme_light() : m.settings_user_detail_theme_dark()}
                    {/if}
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="system">{m.settings_user_detail_theme_system()}</SelectItem>
                    <SelectItem value="light">{m.settings_user_detail_theme_light()}</SelectItem>
                    <SelectItem value="dark">{m.settings_user_detail_theme_dark()}</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              <!-- Language -->
              <div class="flex flex-col gap-2 text-start">
                <Label class="text-xs font-medium text-muted-foreground flex items-center gap-1.5">
                  <Globe size={13} />
                  {m.settings_user_detail_pref_lang()}
                </Label>
                <Select type="single" value={prefLanguage} onValueChange={(v) => (prefLanguage = v ?? "")}>
                  <SelectTrigger class="h-9 w-full">
                    {prefLanguage === "ar" ? m.settings_user_detail_lang_ar() : m.settings_user_detail_lang_en()}
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="ar">{m.settings_user_detail_lang_ar()}</SelectItem>
                    <SelectItem value="en">{m.settings_user_detail_lang_en()}</SelectItem>
                  </SelectContent>
                </Select>
              </div>
            </div>

            <!-- Notification Channels -->
            <div class="flex flex-col gap-4 border-t border-border/40 pt-4 mt-2">
              <span class="text-xs font-medium text-muted-foreground">
                {m.settings_user_detail_pref_notifications()}
              </span>
              <div class="flex flex-col gap-3">
                <div class="flex items-center justify-between">
                  <label class="text-xs text-muted-foreground cursor-pointer select-none">
                    {m.settings_user_detail_notify_email()}
                  </label>
                  <Switch bind:checked={notifyEmail} />
                </div>
                <div class="flex items-center justify-between">
                  <label class="text-xs text-muted-foreground cursor-pointer select-none">
                    {m.settings_user_detail_notify_push()}
                  </label>
                  <Switch bind:checked={notifyPush} />
                </div>
              </div>
            </div>

            <div class="flex justify-end border-t border-border/40 pt-6 mt-4">
              <Button variant="lime" size="sm" class="font-bold" onclick={() => toast.success(m.settings_user_detail_settings_saved())}>
                {m.settings_user_detail_save()}
              </Button>
            </div>
          </div>
        </TabsContent>
      </Tabs>
    {/if}
  </div>
</div>

<!-- Reset Password Modal Dialog -->
<Dialog bind:open={showResetModal}>
  <DialogContent class="max-w-md text-start">
    <form onsubmit={handleResetPassword} class="flex flex-col gap-4">
      <DialogHeader>
        <DialogTitle>{m.settings_user_detail_reset_pass_title()}</DialogTitle>
        <DialogDescription>
          {m.settings_user_detail_reset_pass_desc({ name: fullName || "User" })}
        </DialogDescription>
      </DialogHeader>

      <!-- New Password Field -->
      <div class="flex flex-col gap-2 text-start">
        <Label for="new_password" class="text-xs font-medium text-muted-foreground">
          {m.settings_user_detail_new_password()}
        </Label>
        <Input
          id="new_password"
          type="password"
          bind:value={newPassword}
          placeholder="••••••••"
          required
          minlength={6}
          disabled={isResettingPassword}
        />
      </div>

      <!-- Confirm Password Field -->
      <div class="flex flex-col gap-2 text-start">
        <Label for="confirm_password" class="text-xs font-medium text-muted-foreground">
          {m.settings_user_detail_confirm_password()}
        </Label>
        <Input
          id="confirm_password"
          type="password"
          bind:value={confirmPassword}
          placeholder="••••••••"
          required
          disabled={isResettingPassword}
        />
      </div>

      <DialogFooter class="border-t border-border pt-4 mt-2">
        <Button
          type="button"
          variant="ghost"
          size="sm"
          onclick={() => { showResetModal = false; newPassword = ''; confirmPassword = ''; }}
          disabled={isResettingPassword}
        >
          {m.settings_user_detail_cancel()}
        </Button>
        <Button
          type="submit"
          variant="lime"
          size="sm"
          class="font-bold"
          disabled={isResettingPassword}
        >
          {isResettingPassword ? "..." : m.settings_user_detail_btn_reset_pass()}
        </Button>
      </DialogFooter>
    </form>
  </DialogContent>
</Dialog>

{#snippet SectionSkeleton()}
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
{/snippet}
