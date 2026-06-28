<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { fade, slide } from "svelte/transition";
  import { Button } from "@orbis/ui-svelte/button";
  import { Input } from "@orbis/ui-svelte/input";
  import { Skeleton } from "@orbis/ui-svelte/skeleton";
  import { Avatar, AvatarImage, AvatarFallback } from "@orbis/ui-svelte/avatar";
  import { Badge } from "@orbis/ui-svelte/badge";
  import { Checkbox } from "@orbis/ui-svelte/checkbox";
  import { Switch } from "@orbis/ui-svelte/switch";
  import { Tabs, TabsList, TabsTrigger, TabsContent } from "@orbis/ui-svelte/tabs";
  import { Dialog, DialogContent, DialogTitle, DialogDescription, DialogHeader, DialogFooter } from "@orbis/ui-svelte/dialog";
  import { toast } from "svelte-sonner";
  import { getLocale, localizeHref } from "$lib/paraglide/runtime";
  import * as m from "$lib/paraglide/messages";
  import { commands } from "$lib/services/ipc";
  import type { User } from "$lib/services/ipc";
  import {
    ArrowLeft,
    ArrowRight,
    Shield,
    Key,
    Power,
    Trash2,
    Save,
    Layout,
    Globe,
    Monitor,
    Smartphone,
    UserCheck,
    Plus,
    X,
    Heart,
    MessageCircle,
    Printer,
    ChevronLeft,
    ChevronRight,
    MoreHorizontal
  } from "lucide-svelte";

  // Derive params
  let orgId = $derived(page.params.orgId);
  let userId = $derived(page.params.userId);

  // Tab state matching screenshot
  let activeTab = $state<"profile" | "roles" | "more" | "settings" | "connections">("roles");

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

  // Specific Override Permissions State (retained for backend overrides compatibility if needed)
  let overridePermissions = $state<Record<string, boolean>>({
    acad_grades_submit: false,
    fin_ledger_view: false,
    student_enroll: false,
    report_generate: false,
  });

  // User Preferences State
  let prefTheme = $state("dark");
  let prefLanguage = $state("ar");
  let prefSidebar = $state("wide");
  let notifyEmail = $state(true);
  let notifyPush = $state(true);
  let notifySms = $state(false);

  // Mock Active Sessions
  let activeSessions = $state([
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
  ]);

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
        const dbOverrides = await commands.permissions.getUserPermissions(userId);
        overridePermissions = {
          acad_grades_submit: dbOverrides["GradeWrite"] === "allow",
          fin_ledger_view: dbOverrides["FinanceRead"] === "allow",
          student_enroll: dbOverrides["EnrollmentWrite"] === "allow",
          report_generate: dbOverrides["GradeReport"] === "allow",
        };
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
      toast.error(
        getLocale() === "ar"
          ? "فشل تحميل بيانات المستخدم"
          : "Failed to load user details",
      );
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
      // 1. Update basic profile info
      const request = {
        full_name: fullName,
        name_ar: nameAr || null,
        name_en: nameEn || null,
        email: email || null,
        phone: phone || null,
        is_active: isActiveState,
      };
      await commands.users.updateUser(userId, request);

      // Save primary role based on checked roles or original select
      if (checkedRoles.length > 0) {
        await commands.users.changeUserRole(userId, { role: checkedRoles[0].toLowerCase().replace(/\s+/g, '_') });
      } else if (role) {
        await commands.users.changeUserRole(userId, { role });
      }

      // 2. Save user checklist roles to user_roles table
      await commands.users.saveUserRoles(userId, checkedRoles);

      toast.success(
        getLocale() === "ar"
          ? "تم حفظ البيانات والأدوار بنجاح!"
          : "User details and roles updated successfully!"
      );
      loadUser();
    } catch (e) {
      const errorMsg = e instanceof Error ? e.message : String(e);
      toast.error(errorMsg);
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
    } catch (e) {
      const errorMsg = e instanceof Error ? e.message : String(e);
      toast.error(errorMsg);
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
    } catch (e) {
      const errorMsg = e instanceof Error ? e.message : String(e);
      toast.error(errorMsg);
    } finally {
      isResettingPassword = false;
    }
  }

  // Delete User Account
  async function handleDeleteUser() {
    const confirmMsg =
      getLocale() === "ar"
        ? "هل أنت متأكد نهائياً من حذف هذا الحساب؟ هذا الإجراء لا يمكن التراجع عنه وسيتم مسح كافة ملفاته الشخصية!"
        : "Are you sure you want to permanently delete this user? This action is irreversible!";

    if (!userId) return;
    if (!confirm(confirmMsg)) return;
    try {
      await commands.users.deleteUser(userId as string, "admin");
      toast.success(
        getLocale() === "ar"
          ? "تم حذف حساب المستخدم نهائياً"
          : "User deleted permanently",
      );
      window.location.href = localizeHref(`/${orgId}/settings/users`);
    } catch (e) {
      const errorMsg = e instanceof Error ? e.message : String(e);
      toast.error(errorMsg);
    }
  }

  function getInitials(name: string): string {
    if (!name) return "Ma";
    const parts = name.trim().split(/\s+/);
    if (parts.length >= 2) {
      return (parts[0][0] + parts[1][0]).toUpperCase();
    }
    return name.slice(0, 2).toUpperCase();
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

<div class="h-full w-full overflow-hidden flex font-sans text-start relative">
  <!-- Left Column: User Profile Details Sidebar -->
  <div class="w-72 p-6 flex flex-col gap-6 select-none overflow-y-auto scrollbar-hide flex-shrink-0  text-center items-center">
    <!-- Big Initials Avatar -->
    <div class="w-full aspect-square bg-muted/50 border border-border rounded-xl flex items-center justify-center text-5xl font-semibold text-muted-foreground font-sans shadow-xs">
      {getInitials(fullName)}
    </div>

    <!-- User Quick Info -->
    <div class="flex flex-col gap-1 items-center">
      <h2 class="text-sm font-bold text-foreground">{fullName || m.settings_user_detail_title()}</h2>
      <span class="text-xs text-muted-foreground">{email}</span>
      {#if phone}
        <span class="text-[10px] text-muted-foreground/80 font-mono mt-0.5">{phone}</span>
      {/if}
    </div>
  </div>

  <!-- Right Column: Main Content & Header Controls -->
  <div class="flex-1 flex flex-col overflow-hidden">
    <!-- Top Action controls -->
    <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
      <!-- Title & Active Badge -->
      <div class="flex items-center gap-3 text-start">
        <h1 class="text-lg font-bold text-foreground tracking-tight">
          {fullName || m.settings_user_detail_title()}
        </h1>
        {#if isActiveState}
          <Badge variant="lime" class="text-[9px] font-bold uppercase py-0 px-1.5 h-4.5 rounded">
            {m.settings_user_detail_badge_active()}
          </Badge>
        {:else}
          <Badge variant="destructive" class="text-[9px] font-bold uppercase py-0 px-1.5 h-4.5 rounded">
            {m.settings_user_detail_badge_inactive()}
          </Badge>
        {/if}
      </div>

      <!-- Controls actions buttons -->
      <div class="flex flex-wrap items-center gap-2">
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
          <span>{m.settings_user_detail_back()}</span>
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
          {m.settings_user_detail_btn_delete()}
        </Button>

        <Button
          variant="lime"
          size="sm"
          class="px-5 font-bold"
          onclick={saveAllChanges}
          disabled={isSaving}
        >
          {isSaving ? "..." : (getLocale() === "ar" ? "حفظ" : "Save")}
        </Button>
      </div>
    </div>

    <!-- Tabs Wrapper -->
    <Tabs bind:value={activeTab} class="flex-1 flex flex-col overflow-hidden">
      <!-- Inner page tabs row -->
      <div class="px-6 bg-card border-b border-border flex items-center justify-between">
        <TabsList variant="line" class="h-10 gap-6">
          <TabsTrigger value="profile">
            {m.settings_user_detail_tab_profile()}
          </TabsTrigger>
          <TabsTrigger value="roles">
            {m.settings_nav_permissions()}
          </TabsTrigger>
          <TabsTrigger value="more">
            {m.settings_user_detail_tab_sessions()}
          </TabsTrigger>
          <TabsTrigger value="settings">
            {m.settings_user_detail_tab_settings()}
          </TabsTrigger>
          <TabsTrigger value="connections">
            {m.settings_user_detail_tab_connections()}
          </TabsTrigger>
        </TabsList>
      </div>

      <!-- Active Tab Content Area -->
      <div class="flex-1 overflow-y-auto p-8 scrollbar-hide">
        {#if isLoading}
          <div class="flex flex-col gap-6 w-full">
            <Skeleton class="h-6 w-48" />
            <Skeleton class="h-10 w-full" />
            <Skeleton class="h-64 w-full" />
          </div>
        {:else}
          <!-- Tab 1: Roles & Permissions Checklist -->
          <TabsContent value="roles" class="mt-0">
            <div in:fade={{ duration: 100 }} class="flex flex-col gap-6 text-start">
              <div class="flex flex-col">
                <h2 class="text-sm font-bold text-foreground uppercase tracking-wider">{m.settings_user_detail_roles_title()}</h2>
              </div>

              <!-- Search Profile field -->
              <div class="flex flex-col gap-1.5 w-72 text-start">
                <span class="text-xs font-semibold text-muted-foreground">{m.settings_user_detail_role_profile()}</span>
                <Input
                  type="text"
                  bind:value={roleSearchQuery}
                  placeholder={getLocale() === "ar" ? "ابحث بالأدوار..." : "Search roles..."}
                />
              </div>

              <!-- Action Pill Buttons -->
              <div class="flex items-center gap-2">
                <Button
                  variant="outline"
                  size="xs"
                  onclick={selectAllRoles}
                >
                  {m.settings_user_detail_select_all()}
                </Button>
                <Button
                  variant="outline"
                  size="xs"
                  onclick={unselectAllRoles}
                >
                  {m.settings_user_detail_unselect_all()}
                </Button>
              </div>

              <!-- Checklist Grid (3 columns) -->
              <div class="grid grid-cols-3 gap-y-4 gap-x-12 mt-4 max-h-[480px] overflow-y-auto pr-1">
                {#each filteredRoles as roleName}
                  <label class="flex items-center gap-3 text-xs text-muted-foreground hover:text-foreground cursor-pointer select-none font-medium transition-colors">
                    <Checkbox
                      checked={checkedRoles.includes(roleName)}
                      onCheckedChange={(checked) => toggleRole(roleName, !!checked)}
                    />
                    <span>{roleName}</span>
                  </label>
                {/each}
              </div>
            </div>
          </TabsContent>

          <!-- Tab 2: User Details Form -->
          <TabsContent value="profile" class="mt-0">
            <div in:fade={{ duration: 100 }} class="flex flex-col gap-6 text-start max-w-2xl bg-card border border-border p-6 rounded-xl shadow-xs">
              <h2 class="text-sm font-bold text-foreground border-b border-border pb-3">
                {m.settings_user_detail_profile_title()}
              </h2>

              <div class="grid grid-cols-2 gap-5">
                <!-- Full Name -->
                <div class="flex flex-col gap-1.5">
                  <label for="fullName" class="text-xs font-semibold text-muted-foreground">
                    {m.settings_user_detail_label_fullname()}
                  </label>
                  <Input
                    id="fullName"
                    type="text"
                    bind:value={fullName}
                    required
                  />
                </div>

                <!-- Primary System Role -->
                <div class="flex flex-col gap-1.5">
                  <label for="role" class="text-xs font-semibold text-muted-foreground">
                    {m.settings_user_detail_label_primary_role()}
                  </label>
                  <select
                    id="role"
                    bind:value={role}
                    class="bg-input/30 text-foreground border border-border rounded-md px-3 py-2 text-xs outline-none focus:border-primary cursor-pointer h-8"
                  >
                    <option value="super_admin">Super Admin</option>
                    <option value="admin">System Admin</option>
                    <option value="principal">School Principal</option>
                    <option value="teacher">Teacher</option>
                    <option value="staff">Staff Member</option>
                    <option value="student">Student</option>
                    <option value="parent">Parent / Guardian</option>
                  </select>
                </div>

                <!-- Arabic Name -->
                <div class="flex flex-col gap-1.5">
                  <label for="nameAr" class="text-xs font-semibold text-muted-foreground">
                    {m.settings_user_detail_label_fullname_ar()}
                  </label>
                  <Input
                    id="nameAr"
                    type="text"
                    bind:value={nameAr}
                  />
                </div>

                <!-- English Name -->
                <div class="flex flex-col gap-1.5">
                  <label for="nameEn" class="text-xs font-semibold text-muted-foreground">
                    {m.settings_user_detail_label_fullname_en()}
                  </label>
                  <Input
                    id="nameEn"
                    type="text"
                    bind:value={nameEn}
                  />
                </div>

                <!-- Email -->
                <div class="flex flex-col gap-1.5">
                  <label for="email" class="text-xs font-semibold text-muted-foreground">
                    {m.settings_user_detail_label_email()}
                  </label>
                  <Input
                    id="email"
                    type="email"
                    bind:value={email}
                  />
                </div>

                <!-- Phone -->
                <div class="flex flex-col gap-1.5">
                  <label for="phone" class="text-xs font-semibold text-muted-foreground">
                    {m.settings_user_detail_label_phone()}
                  </label>
                  <Input
                    id="phone"
                    type="text"
                    bind:value={phone}
                  />
                </div>
              </div>
              
              <div class="flex justify-end gap-2 border-t border-border pt-4 mt-2">
                <Button
                  variant="ghost"
                  size="sm"
                  class="text-muted-foreground hover:text-foreground"
                  onclick={loadUser}
                >
                  {m.settings_user_detail_btn_reset()}
                </Button>
                <Button
                  variant="lime"
                  size="sm"
                  class="font-bold"
                  onclick={saveAllChanges}
                >
                  {m.settings_user_detail_btn_save_details()}
                </Button>
              </div>
            </div>
          </TabsContent>

          <!-- Tab 3: More Information -->
          <TabsContent value="more" class="mt-0">
            <div in:fade={{ duration: 100 }} class="flex flex-col gap-6 text-start max-w-2xl bg-card border border-border p-6 rounded-xl shadow-xs">
              <h2 class="text-sm font-bold text-foreground border-b border-border pb-3">
                {m.settings_user_detail_sessions_title()}
              </h2>

              <div class="flex flex-col gap-4">
                {#each activeSessions as session (session.id)}
                  <div class="flex items-center justify-between p-4 rounded-xl border border-border bg-muted/30 hover:bg-muted/50 transition-colors">
                    <div class="flex items-center gap-3">
                      <div class="p-2.5 rounded-lg bg-muted text-foreground">
                        <session.icon size={18} />
                      </div>
                      <div class="flex flex-col gap-0.5">
                        <span class="text-xs font-bold text-foreground flex items-center gap-2">
                          <span>{session.device}</span>
                          {#if session.active}
                            <Badge variant="lime" class="text-[8px] font-semibold uppercase py-0 px-1.5 h-4.5 rounded">
                              {getLocale() === "ar" ? "نشط" : "active"}
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

          <!-- Tab 4: Settings -->
          <TabsContent value="settings" class="mt-0">
            <div in:fade={{ duration: 100 }} class="flex flex-col gap-6 text-start max-w-2xl bg-card border border-border p-6 rounded-xl shadow-xs">
              <h2 class="text-sm font-bold text-foreground border-b border-border pb-3">
                {getLocale() === "ar" ? "تفضيلات حساب المستخدم" : "System Settings & Preferences"}
              </h2>

              <div class="grid grid-cols-2 gap-5">
                <!-- Theme -->
                <div class="flex flex-col gap-1.5 text-start">
                  <span class="text-xs font-semibold text-muted-foreground flex items-center gap-1.5">
                    <Layout size={13} />
                    {m.settings_user_detail_pref_theme()}
                  </span>
                  <select
                    bind:value={prefTheme}
                    class="bg-input/30 text-foreground border border-border rounded-md px-3 py-2 text-xs outline-none focus:border-primary cursor-pointer h-8"
                  >
                    <option value="system">{m.settings_user_detail_theme_system()}</option>
                    <option value="light">{m.settings_user_detail_theme_light()}</option>
                    <option value="dark">{m.settings_user_detail_theme_dark()}</option>
                  </select>
                </div>

                <!-- Language -->
                <div class="flex flex-col gap-1.5 text-start">
                  <span class="text-xs font-semibold text-muted-foreground flex items-center gap-1.5">
                    <Globe size={13} />
                    {m.settings_user_detail_pref_lang()}
                  </span>
                  <select
                    bind:value={prefLanguage}
                    class="bg-input/30 text-foreground border border-border rounded-md px-3 py-2 text-xs outline-none focus:border-primary cursor-pointer h-8"
                  >
                    <option value="ar">{m.header_language_ar()}</option>
                    <option value="en">{m.header_language_en()}</option>
                  </select>
                </div>

                <!-- Notification Channels -->
                <div class="flex flex-col gap-3 col-span-2 mt-2 border-t border-border pt-4 text-start">
                  <span class="text-xs font-semibold text-muted-foreground">
                    {m.settings_user_detail_pref_notifications()}
                  </span>
                  <div class="flex flex-col gap-2">
                    <label class="flex items-center justify-between text-xs text-muted-foreground cursor-pointer select-none">
                      <span>{m.settings_user_detail_notify_email()}</span>
                      <Switch bind:checked={notifyEmail} />
                    </label>
                    <label class="flex items-center justify-between text-xs text-muted-foreground cursor-pointer select-none">
                      <span>{m.settings_user_detail_notify_push()}</span>
                      <Switch bind:checked={notifyPush} />
                    </label>
                  </div>
                </div>
              </div>

              <div class="flex justify-end border-t border-border pt-4 mt-2">
                <Button variant="lime" size="sm" class="font-bold" onclick={() => toast.success(m.settings_user_detail_settings_saved())}>
                  {m.settings_permissions_save_changes()}
                </Button>
              </div>
            </div>
          </TabsContent>

          <!-- Tab 5: Connections -->
          <TabsContent value="connections" class="mt-0">
            <div in:fade={{ duration: 100 }} class="flex flex-col gap-6 text-start max-w-2xl bg-card border border-border p-6 rounded-xl shadow-xs">
              <h2 class="text-sm font-bold text-foreground border-b border-border pb-3">
                {m.settings_user_detail_tab_connections()}
              </h2>
              <p class="text-xs text-muted-foreground">
                {m.settings_user_detail_connections_empty()}
              </p>
            </div>
          </TabsContent>
        {/if}
      </div>
    </Tabs>
  </div>
</div>

<!-- Reset Password Modal Dialog -->
<Dialog bind:open={showResetModal}>
  <DialogContent class="sm:max-w-md">
    <DialogHeader class="border-b border-border pb-3 text-start">
      <DialogTitle class="flex items-center gap-1.5 text-foreground">
        <Key size={16} class="text-primary" />
        <span>{m.settings_user_detail_reset_pass_title()}</span>
      </DialogTitle>
      <DialogDescription class="text-[10px] text-muted-foreground mt-0.5">
        {getLocale() === "ar" 
          ? `قم بإدخال كلمة مرور جديدة وتأكيدها للمستخدم: ${fullName}` 
          : `Set and confirm a new login password for ${fullName}.`}
      </DialogDescription>
    </DialogHeader>

    <form onsubmit={handleResetPassword} class="flex flex-col gap-4 text-start">
      <!-- Password Field -->
      <div class="flex flex-col gap-1.5">
        <label for="new_password" class="text-xs font-semibold text-muted-foreground">
          {m.settings_user_detail_new_password()}
        </label>
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
      <div class="flex flex-col gap-1.5">
        <label for="confirm_password" class="text-xs font-semibold text-muted-foreground">
          {m.settings_user_detail_confirm_password()}
        </label>
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
          {getLocale() === "ar" ? "إلغاء" : "Cancel"}
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
