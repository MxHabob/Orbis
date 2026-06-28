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
  import type { User, UserSession } from "$lib/services/ipc";
  import { formatDate } from "$lib/i18n-utils";
  import AvatarUpload from "$lib/components/shared/avatar-upload.svelte";
  import MainViewport from "$lib/components/layout/MainViewport.svelte";
  import {
    ArrowLeft,
    ArrowRight,
    Monitor,
    Smartphone,
    Trash2,
    Layout,
    Globe,
  } from "lucide-svelte";

  let orgId = $derived(page.params.orgId);
  let userId = $derived(page.params.userId);

  let activeTab = $state<"profile" | "roles" | "sessions" | "settings">("profile");

  let user = $state<User | null>(null);
  let isLoading = $state(true);
  let isSaving = $state(false);
  let isActiveState = $state(true);
  let avatarUrl = $state<string | null>(null);

  let showResetModal = $state(false);
  let newPassword = $state("");
  let confirmPassword = $state("");
  let isResettingPassword = $state(false);

  let fullName = $state("");
  let nameAr = $state("");
  let nameEn = $state("");
  let email = $state("");
  let phone = $state("");
  let role = $state("teacher");

  let checkedRoles = $state<string[]>([]);
  let roleSearchQuery = $state("");

  let allAvailableRoles = $state<string[]>([]);
  let filteredRoles = $derived(allAvailableRoles.filter(r => r.toLowerCase().includes(roleSearchQuery.toLowerCase())));

  let prefTheme = $state("dark");
  let prefLanguage = $state("ar");
  let notifyEmail = $state(true);
  let notifyPush = $state(true);

  const systemRoles = [
    { value: "super_admin", label: "Super Admin" },
    { value: "admin", label: "System Admin" },
    { value: "principal", label: "School Principal" },
    { value: "teacher", label: "Teacher" },
    { value: "staff", label: "Staff Member" },
    { value: "student", label: "Student" },
    { value: "parent", label: "Parent / Guardian" },
  ];

  let activeSessions = $state<any[]>([]);

  async function loadUser() {
    if (!userId) return;
    isLoading = true;
    try {
      const fetched = await commands.users.getUser(userId);
      user = fetched;

      fullName = fetched.full_name || "";
      nameAr = fetched.name_ar || "";
      nameEn = fetched.name_en || "";
      email = fetched.email || "";
      phone = (fetched.phone as string) || "";
      role = fetched.primary_role || fetched.role || "teacher";
      isActiveState = fetched.is_active !== undefined ? Boolean(fetched.is_active) : true;
      avatarUrl = fetched.avatar_url || fetched.avatar || null;

      try {
        await commands.permissions.getUserPermissions(userId);
      } catch (err) {
        console.error("Failed to load user permissions overrides", err);
      }

      try {
        checkedRoles = await commands.users.getUserRoles(userId);
      } catch (e) {
        console.error("Failed to load user roles list:", e);
      }

      try {
        allAvailableRoles = await commands.permissions.getAvailableRoles();
      } catch (e) {
        console.error("Failed to load available roles from db:", e);
      }

      try {
        const rawSessions = await commands.users.getUserSessions(userId);
        activeSessions = rawSessions.map((s: UserSession) => {
          const isMobile = s.device_type === "mobile" || (s.user_agent && /mobile|android|iphone|ipad/i.test(s.user_agent));
          const icon = isMobile ? Smartphone : Monitor;

          let device = "Unknown Device";
          if (s.device_name) {
            device = s.device_name;
            if (s.browser) device += ` (${s.browser})`;
          } else if (s.os) {
            device = s.os;
            if (s.browser) device += ` (${s.browser})`;
          }

          const location = s.is_current ? (getLocale() === "ar" ? "هذا الجهاز" : "Current Device") : (getLocale() === "ar" ? "جهاز نشط" : "Active Device");
          const time = s.is_current ? (getLocale() === "ar" ? "نشط الآن" : "Active now") : (s.last_active_at ? formatDate(s.last_active_at) : "");

          return {
            id: s.id,
            device,
            location,
            ip: s.ip_address || "127.0.0.1",
            active: s.is_current === 1,
            icon,
            time,
          };
        });
      } catch (err) {
        console.error("Failed to load user sessions:", err);
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

  async function saveAllChanges() {
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
        avatar_url: avatarUrl,
      };
      await commands.users.updateUser(userId, request);

      if (role) {
        await commands.users.changeUserRole(userId, { role });
      }

      await commands.users.saveUserRoles(userId, checkedRoles);

      toast.success(m.settings_user_detail_saved());
      loadUser();
    } catch (e: unknown) {
      toast.error((e as any)?.message || "Save failed");
    } finally {
      isSaving = false;
    }
  }

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

<div class="h-full w-full bg-background overflow-hidden flex flex-col">
  <!-- Full-Width Header (Name + Status Badge + Actions) -->
  <div class="px-6 py-4 flex items-center justify-between gap-4 select-none shrink-0">
    <div class="flex items-center gap-3 text-start shrink">
      <h1 class="text-xl font-bold tracking-tight text-foreground truncate shrink">
        {fullName || m.settings_user_detail_title()}
      </h1>
      {#if isActiveState}
        <Badge variant="lime" class="text-[9px] py-0 px-1.5 h-4.5 rounded shrink-0">
          {m.settings_user_detail_badge_active()}
        </Badge>
      {:else}
        <Badge variant="destructive" class="text-[9px] py-0 px-1.5 h-4.5 rounded shrink-0">
          {m.settings_user_detail_badge_inactive()}
        </Badge>
      {/if}
    </div>

    <!-- Action Buttons Row -->
    <div class="flex items-center gap-2 flex-wrap sm:flex-nowrap shrink-0 justify-end">
      <Button
        variant="outline"
        size="sm"
        onclick={() => showResetModal = true}
      >
        {m.settings_user_detail_btn_reset_pass()}
      </Button>

      <Button
        variant="outline"
        size="sm"
        class="text-destructive border-destructive/20 hover:bg-destructive/5"
        onclick={handleDeleteUser}
      >
        <Trash2 size={13} class="me-1" />
        {m.settings_user_detail_btn_delete()}
      </Button>

      <Button
        variant="lime"
        size="sm"
        class="px-6 font-bold"
        onclick={saveAllChanges}
        disabled={isSaving}
      >
        {isSaving ? "..." : m.settings_user_detail_save()}
      </Button>
    </div>
  </div>

  <!-- Resizable split view below the header -->
  <div class="flex-1 w-full overflow-hidden">
    <MainViewport
      name="user-detail-viewport"
      leftWidth={{ default: 220, min: 180 }}
    >
      <!-- Left Sidebar (Avatar and basic role info only) -->
      {#snippet left()}
        <div class="h-full p-4 flex flex-col gap-5 overflow-y-auto scrollbar-hide text-start bg-transparent select-none">
          <!-- Squared Avatar Uploader -->
          <div class="w-full flex justify-start">
            <AvatarUpload
              bind:avatarUrl={avatarUrl}
              size={150}
              fallbackText={fullName}
              class="rounded-xl border border-border"
            />
          </div>

          <div class="flex flex-col gap-1 w-full pt-1 text-start">
            <span class="text-xs text-muted-foreground font-mono truncate max-w-full">{email || "No email"}</span>
            <div class="pt-2">
              <Badge variant="secondary" class="text-[10px] font-semibold px-2 py-0.5 uppercase tracking-wider">
                {role.replace('_', ' ')}
              </Badge>
            </div>
          </div>
        </div>
      {/snippet}

      <!-- Right Workspace Area -->
      {#snippet middle()}
        <div class="h-full w-full overflow-y-auto p-6 scrollbar-hide">
          <div class="max-w-3xl mx-auto flex flex-col gap-6">
            {#if isLoading}
              <div class="flex flex-col gap-8">
                {@render SectionSkeleton()}
                {@render SectionSkeleton()}
              </div>
            {:else}
              <div class="border border-border/60 rounded-xl p-6 bg-card/5 shadow-xs">
                <Tabs bind:value={activeTab}>
                <TabsList variant="line" class="h-10 gap-6 border-b border-border/20">
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

                <!-- Profile Details -->
                <TabsContent value="profile" class="mt-6">
                  <div in:fade={{ duration: 100 }} class="flex flex-col gap-6 text-start">
                    
                    <div class="flex items-center gap-3">
                      <Checkbox
                        id="user_enabled"
                        checked={isActiveState}
                        onCheckedChange={toggleAccountStatus}
                      />
                      <Label for="user_enabled" class="text-xs font-bold text-foreground cursor-pointer select-none">
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
                        <Label for="fullName" class="text-xs font-medium text-muted-foreground">
                          {m.settings_user_detail_label_fullname()}
                        </Label>
                        <Input id="fullName" type="text" bind:value={fullName} />
                      </div>

                      <!-- Primary Role -->
                      <div class="flex flex-col gap-1.5 text-start">
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
                      <Button variant="lime" size="sm" class="font-bold" onclick={saveAllChanges} disabled={isSaving}>
                        {isSaving ? "..." : m.settings_user_detail_btn_save_details()}
                      </Button>
                    </div>
                  </div>
                </TabsContent>

                <!-- Roles checklist -->
                <TabsContent value="roles" class="mt-6">
                  <div in:fade={{ duration: 100 }} class="flex flex-col gap-6 text-start">
                    
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

                    <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4 pt-2">
                      {#each filteredRoles as roleName (roleName)}
                        <div class="flex items-center justify-between p-3.5 select-none">
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

                <!-- Sessions -->
                <TabsContent value="sessions" class="mt-6">
                  <div in:fade={{ duration: 100 }} class="flex flex-col gap-6 text-start">
                    
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

                <!-- Settings -->
                <TabsContent value="settings" class="mt-6">
                  <div in:fade={{ duration: 100 }} class="flex flex-col gap-6 text-start">
                    
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
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

                    <div class="flex flex-col gap-4 border-t border-border/40 pt-4 mt-2">
                      <span class="text-xs font-semibold text-muted-foreground">
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
              </div>
            {/if}
          </div>
        </div>
      {/snippet}
    </MainViewport>
  </div>
</div>

<!-- Reset Password Dialog -->
<Dialog bind:open={showResetModal}>
  <DialogContent class="max-w-md text-start">
    <form onsubmit={handleResetPassword} class="flex flex-col gap-4">
      <DialogHeader>
        <DialogTitle>{m.settings_user_detail_reset_pass_title()}</DialogTitle>
        <DialogDescription>
          {m.settings_user_detail_reset_pass_desc({ name: fullName || "User" })}
        </DialogDescription>
      </DialogHeader>

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
