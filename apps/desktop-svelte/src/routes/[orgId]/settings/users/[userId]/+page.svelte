<script lang="ts">
import { onMount } from "svelte";
import { page } from "$app/state";
import { fade, slide } from "svelte/transition";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { Avatar, AvatarImage, AvatarFallback } from "@orbis/ui-svelte/avatar";
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
  User as UserIcon, 
  Monitor, 
  Smartphone, 
  Globe, 
  ShieldAlert, 
  Save, 
  Layout, 
  Lock, 
  Activity,
  CheckCircle,
  AlertTriangle,
  UserCheck
} from "lucide-svelte";

// Derive params
let orgId = $derived(page.params.orgId);
let userId = $derived(page.params.userId);

// Tab state
let activeTab = $state<"profile" | "sessions" | "permissions" | "settings">("profile");

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

function getRoleLabel(roleStr: string): string {
  const r = String(roleStr).toLowerCase();
  if (getLocale() === "ar") {
    switch (r) {
      case "super_admin": return "مدير النظام الرئيسي";
      case "admin": return "إداري النظام";
      case "principal": return "مدير المدرسة";
      case "teacher": return "معلم";
      case "staff": return "موظف كادر مساند";
      case "student": return "طالب";
      case "parent": return "ولي أمر";
      default: return r;
    }
  } else {
    switch (r) {
      case "super_admin": return "Super Admin";
      case "admin": return "System Admin";
      case "principal": return "School Principal";
      case "teacher": return "Teacher";
      case "staff": return "Staff Member";
      case "student": return "Student";
      case "parent": return "Parent / Guardian";
      default: return r;
    }
  }
}

// Form Fields
let fullName = $state("");
let nameAr = $state("");
let nameEn = $state("");
let email = $state("");
let phone = $state("");
let role = $state("teacher");

// Specific Override Permissions State
let overridePermissions = $state<Record<string, boolean>>({
  "acad_grades_submit": false,
  "fin_ledger_view": false,
  "student_enroll": false,
  "report_generate": false
});

// User Preferences State
let prefTheme = $state("system");
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
    time: "Active now"
  },
  {
    id: "sess_2",
    device: "iPhone 15 Pro (Safari Mobile)",
    location: "Riyadh, Saudi Arabia",
    ip: "37.220.112.4",
    active: false,
    icon: Smartphone,
    time: "2 hours ago"
  },
  {
    id: "sess_3",
    device: "macOS Sonoma (Chrome Browser)",
    location: "Sana'a, Yemen",
    ip: "109.74.32.90",
    active: false,
    icon: Monitor,
    time: "3 days ago"
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
        "acad_grades_submit": dbOverrides["GradeWrite"] === "allow",
        "fin_ledger_view": dbOverrides["FinanceRead"] === "allow",
        "student_enroll": dbOverrides["EnrollmentWrite"] === "allow",
        "report_generate": dbOverrides["GradeReport"] === "allow",
      };
    } catch (err) {
      console.error("Failed to load user permissions overrides", err);
    }
  } catch (e) {
    console.error("Failed to load user:", e);
    toast.error(getLocale() === "ar" ? "فشل تحميل بيانات المستخدم" : "Failed to load user details");
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  loadUser();
});

// Update Profile
async function handleUpdateProfile(e: Event) {
  e.preventDefault();
  if (!userId) return;
  isSaving = true;
  try {
    const request = {
      full_name: fullName,
      name_ar: nameAr || null,
      name_en: nameEn || null,
      email: email || null,
      phone: phone || null,
      is_active: isActiveState
    };
    await commands.users.updateUser(userId, request);
    if (role) {
      await commands.users.changeUserRole(userId, { role });
    }
    toast.success(m.settings_user_detail_save_success());
    loadUser();
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(errorMsg);
  } finally {
    isSaving = false;
  }
}

async function handleSaveOverrides() {
  if (!userId) return;
  isSaving = true;
  try {
    const permissionsMap: Record<string, string> = {
      "GradeWrite": overridePermissions.acad_grades_submit ? "allow" : "deny",
      "FinanceRead": overridePermissions.fin_ledger_view ? "allow" : "deny",
      "EnrollmentWrite": overridePermissions.student_enroll ? "allow" : "deny",
      "GradeReport": overridePermissions.report_generate ? "allow" : "deny",
    };
    await commands.permissions.saveUserPermissions(userId, permissionsMap);
    toast.success(m.settings_permissions_save_success());
  } catch (err) {
    const errorMsg = err instanceof Error ? err.message : String(err);
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
      is_active: nextState
    };
    await commands.users.updateUser(userId, request);
    isActiveState = nextState;
    
    toast.success(nextState
      ? m.settings_user_detail_activate_success()
      : m.settings_user_detail_deactivate_success()
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
      new_password: newPassword
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

// Revoke Session Simulation
function revokeSession(sessionId: string) {
  const confirmMsg = m.settings_user_detail_session_revoke_confirm();
  if (!confirm(confirmMsg)) return;

  activeSessions = activeSessions.filter(s => s.id !== sessionId);
  toast.success(m.settings_user_detail_session_revoked());
}

// Delete User Account
async function handleDeleteUser() {
  const confirmMsg = getLocale() === "ar"
    ? "هل أنت متأكد نهائياً من حذف هذا الحساب؟ هذا الإجراء لا يمكن التراجع عنه وسيتم مسح كافة ملفاته الشخصية!"
    : "Are you sure you want to permanently delete this user? This action is irreversible!";
    
  if (!userId) return;
  try {
    await commands.users.deleteUser(userId as string, "admin");
    toast.success(getLocale() === "ar" ? "تم حذف حساب المستخدم نهائياً" : "User deleted permanently");
    // Redirect to users directory
    window.location.href = localizeHref(`/${orgId}/settings/users`);
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(errorMsg);
  }
}

</script>

<div class="h-full w-full overflow-y-auto bg-background/50 scrollbar-hide flex flex-col p-6 gap-6 font-sans">
  <!-- Back Button & Breadcrumbs -->
  <div class="flex items-center gap-2">
    <Button 
      variant="ghost" 
      size="sm" 
      class="text-xs flex items-center gap-1.5 hover:bg-card/50" 
      href={localizeHref(`/${orgId}/settings/users`)}
    >
      {#if getLocale() === "ar"}
        <ArrowRight size={14} />
      {:else}
        <ArrowLeft size={14} />
      {/if}
      <span>{m.settings_user_detail_back()}</span>
    </Button>
  </div>

  {#if isLoading}
    <div class="flex flex-col gap-6 max-w-4xl mx-auto w-full">
      <div class="flex items-center gap-4">
        <Skeleton class="w-16 h-16 rounded-full" />
        <div class="flex flex-col gap-2 flex-1">
          <Skeleton class="h-6 w-48" />
          <Skeleton class="h-4 w-32" />
        </div>
      </div>
      <Skeleton class="h-10 w-full" />
      <Skeleton class="h-96 w-full" />
    </div>
  {:else if !user}
    <div class="max-w-md mx-auto text-center p-8 border border-border bg-card rounded-2xl flex flex-col items-center gap-4">
      <ShieldAlert size={48} class="text-destructive animate-bounce" />
      <h2 class="text-sm font-bold text-foreground">User Not Found</h2>
      <p class="text-xs text-muted-foreground">The specified user ID does not exist or has been deleted.</p>
      <Button variant="lime" href={localizeHref(`/${orgId}/settings/users`)}>
        {m.settings_user_detail_back()}
      </Button>
    </div>
  {:else}
    <div class="max-w-4xl mx-auto w-full flex flex-col gap-6 pb-12">
      <!-- Profile Overview Header Card -->
      <div class="bg-card border border-border rounded-2xl p-6 flex flex-col md:flex-row md:items-center justify-between gap-6 relative overflow-hidden">
        <div class="flex items-center gap-4 text-start z-10">
          <Avatar class="w-16 h-16 border-2 border-primary/20">
            <AvatarImage src={user.avatar ? String(user.avatar) : undefined} alt={fullName} class="object-cover" />
            <AvatarFallback class="text-lg font-bold bg-muted text-muted-foreground">
              {(fullName || "U").charAt(0).toUpperCase()}
            </AvatarFallback>
          </Avatar>
          <div class="flex flex-col">
            <h1 class="text-lg font-bold text-foreground flex items-center gap-2">
              <span>{getLocale() === "ar" && nameAr ? nameAr : fullName}</span>
              <span class="px-2 py-0.5 rounded border text-[9px] font-bold tracking-wide uppercase bg-primary/10 text-primary border-primary/20">
                {getRoleLabel(role)}
              </span>
            </h1>
            <span class="text-xs text-muted-foreground font-mono leading-none mt-1">{email || "No Email"}</span>
            
            <div class="flex items-center gap-2 mt-2">
              {#if isActiveState}
                <span class="text-[9px] px-2 py-0.5 rounded-full font-bold bg-emerald-500/10 text-emerald-500 border border-emerald-500/20 flex items-center gap-1">
                  <span class="w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse"></span>
                  {m.settings_user_detail_status_active()}
                </span>
              {:else}
                <span class="text-[9px] px-2 py-0.5 rounded-full font-bold bg-rose-500/10 text-rose-500 border border-rose-500/20 flex items-center gap-1">
                  <span class="w-1.5 h-1.5 rounded-full bg-rose-500"></span>
                  {m.settings_user_detail_status_inactive()}
                </span>
              {/if}
            </div>
          </div>
        </div>

        <!-- Quick actions buttons in header -->
        <div class="flex flex-wrap items-center gap-2 z-10">
          <!-- Toggle Status -->
          <Button 
            variant={isActiveState ? "outline" : "lime"} 
            class="text-xs h-9 px-3.5 font-semibold"
            onclick={toggleAccountStatus}
            disabled={isSaving}
          >
            <Power size={13} class={getLocale() === "ar" ? 'ml-1.5' : 'mr-1.5'} />
            {isActiveState ? m.settings_user_detail_btn_deactivate() : m.settings_user_detail_btn_activate()}
          </Button>

          <!-- Reset Password -->
          <Button 
            variant="outline" 
            class="text-xs h-9 px-3.5 font-semibold"
            onclick={() => showResetModal = true}
          >
            <Key size={13} class={getLocale() === "ar" ? 'ml-1.5' : 'mr-1.5'} />
            {m.settings_user_detail_btn_reset_pass()}
          </Button>

          <!-- Delete User -->
          <Button 
            variant="ghost" 
            class="text-xs h-9 px-3 text-destructive hover:bg-destructive/10 font-semibold"
            disabled={user.id === page.params.userId && false /* disable deleting oneself logic if needed */}
            onclick={handleDeleteUser}
          >
            <Trash2 size={13} class={getLocale() === "ar" ? 'ml-1.5' : 'mr-1.5'} />
            {m.settings_user_detail_btn_delete()}
          </Button>
        </div>
      </div>

      <!-- Segment Tabs Navigation -->
      <div class="flex border-b border-border/40 gap-1 overflow-x-auto select-none scrollbar-hide">
        <button
          class="px-4 py-2.5 text-xs font-semibold border-b-2 transition-all cursor-pointer whitespace-nowrap
            {activeTab === 'profile' ? 'border-primary text-primary font-bold' : 'border-transparent text-muted-foreground hover:text-foreground'}"
          onclick={() => activeTab = "profile"}
        >
          {m.settings_user_detail_tab_profile()}
        </button>
        <button
          class="px-4 py-2.5 text-xs font-semibold border-b-2 transition-all cursor-pointer whitespace-nowrap
            {activeTab === 'sessions' ? 'border-primary text-primary font-bold' : 'border-transparent text-muted-foreground hover:text-foreground'}"
          onclick={() => activeTab = "sessions"}
        >
          {m.settings_user_detail_tab_sessions()}
        </button>
        <button
          class="px-4 py-2.5 text-xs font-semibold border-b-2 transition-all cursor-pointer whitespace-nowrap
            {activeTab === 'permissions' ? 'border-primary text-primary font-bold' : 'border-transparent text-muted-foreground hover:text-foreground'}"
          onclick={() => activeTab = "permissions"}
        >
          {m.settings_user_detail_tab_permissions()}
        </button>
        <button
          class="px-4 py-2.5 text-xs font-semibold border-b-2 transition-all cursor-pointer whitespace-nowrap
            {activeTab === 'settings' ? 'border-primary text-primary font-bold' : 'border-transparent text-muted-foreground hover:text-foreground'}"
          onclick={() => activeTab = "settings"}
        >
          {m.settings_user_detail_tab_settings()}
        </button>
      </div>

      <!-- Tabs Views -->
      {#if activeTab === 'profile'}
        <div in:fade={{ duration: 100 }} class="bg-card border border-border rounded-xl p-6">
          <form onsubmit={handleUpdateProfile} class="flex flex-col gap-6 text-start">
            <h2 class="text-sm font-bold text-foreground border-b border-border/60 pb-3">
              {getLocale() === "ar" ? "تعديل البيانات الأساسية للمستخدم" : "Basic User Profile Information"}
            </h2>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
              <!-- Full Name -->
              <div class="flex flex-col gap-2">
                <label for="fullName" class="text-xs font-semibold text-muted-foreground">
                  {getLocale() === "ar" ? "الاسم الكامل (المطابق للهوية)" : "Full Name"}
                </label>
                <Input
                  id="fullName"
                  type="text"
                  bind:value={fullName}
                  placeholder="e.g., Jane Doe"
                  required
                  disabled={isSaving}
                />
              </div>

              <!-- Role Selector -->
              <div class="flex flex-col gap-2">
                <label for="role" class="text-xs font-semibold text-muted-foreground">
                  {getLocale() === "ar" ? "الدور الوظيفي في النظام" : "System Authorization Role"}
                </label>
                <select
                  id="role"
                  bind:value={role}
                  disabled={isSaving}
                  class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-sm focus:ring-1 focus:ring-ring outline-none h-9 w-full cursor-pointer"
                >
                  <option value="super_admin">{getLocale() === "ar" ? "مدير النظام الرئيسي" : "Super Admin"}</option>
                  <option value="admin">{getLocale() === "ar" ? "إداري النظام" : "System Admin"}</option>
                  <option value="principal">{getLocale() === "ar" ? "مدير المدرسة" : "School Principal"}</option>
                  <option value="teacher">{getLocale() === "ar" ? "المعلم / المدرس" : "Teacher"}</option>
                  <option value="staff">{getLocale() === "ar" ? "موظف / كادر مساند" : "Staff Member"}</option>
                  <option value="student">{getLocale() === "ar" ? "الطالب" : "Student"}</option>
                  <option value="parent">{getLocale() === "ar" ? "ولي الأمر" : "Parent / Guardian"}</option>
                </select>
              </div>

              <!-- Arabic Name -->
              <div class="flex flex-col gap-2">
                <label for="nameAr" class="text-xs font-semibold text-muted-foreground">
                  {getLocale() === "ar" ? "الاسم بالكامل (باللغة العربية)" : "Full Name (Arabic)"}
                </label>
                <Input
                  id="nameAr"
                  type="text"
                  bind:value={nameAr}
                  placeholder="الاسم الثلاثي أو الرباعي..."
                  disabled={isSaving}
                />
              </div>

              <!-- English Name -->
              <div class="flex flex-col gap-2">
                <label for="nameEn" class="text-xs font-semibold text-muted-foreground">
                  {getLocale() === "ar" ? "الاسم بالكامل (باللغة الإنجليزية)" : "Full Name (English)"}
                </label>
                <Input
                  id="nameEn"
                  type="text"
                  bind:value={nameEn}
                  placeholder="Full name in English..."
                  disabled={isSaving}
                />
              </div>

              <!-- Email -->
              <div class="flex flex-col gap-2">
                <label for="email" class="text-xs font-semibold text-muted-foreground">
                  {getLocale() === "ar" ? "البريد الإلكتروني" : "Email Address"}
                </label>
                <Input
                  id="email"
                  type="email"
                  bind:value={email}
                  placeholder="user@example.com"
                  disabled={isSaving}
                />
              </div>

              <!-- Phone -->
              <div class="flex flex-col gap-2">
                <label for="phone" class="text-xs font-semibold text-muted-foreground">
                  {getLocale() === "ar" ? "رقم الهاتف / الجوال" : "Phone Number"}
                </label>
                <Input
                  id="phone"
                  type="text"
                  bind:value={phone}
                  placeholder="+96777xxxxxxx"
                  disabled={isSaving}
                />
              </div>
            </div>

            <div class="flex justify-end gap-2 border-t border-border/40 pt-4 mt-2">
              <Button 
                type="submit" 
                variant="lime" 
                class="px-6 font-bold h-10 text-xs" 
                disabled={isSaving}
              >
                <Save size={14} class={getLocale() === "ar" ? 'ml-1.5' : 'mr-1.5'} />
                {isSaving ? (getLocale() === "ar" ? "جاري الحفظ..." : "Saving...") : m.settings_user_detail_save_profile()}
              </Button>
            </div>
          </form>
        </div>
      {/if}

      {#if activeTab === 'sessions'}
        <div in:fade={{ duration: 100 }} class="bg-card border border-border rounded-xl p-6 flex flex-col gap-6 text-start">
          <div class="flex flex-col gap-1 border-b border-border/60 pb-3">
            <h2 class="text-sm font-bold text-foreground">
              {getLocale() === "ar" ? "إدارة الأجهزة والجلسات النشطة للمستخدم" : "Device Authorization & Sessions"}
            </h2>
            <p class="text-[10px] text-muted-foreground">
              {getLocale() === "ar" 
                ? "قائمة بالمتصفحات وتطبيقات الديسك توب التي سجلت دخولها مؤخراً بهذا الحساب. يمكنك إلغاء أي جلسة لفرض الخروج" 
                : "Active sessions for this account. Revoke any session to force sign out from that device."}
            </p>
          </div>

          <div class="flex flex-col gap-4">
            {#each activeSessions as session (session.id)}
              <div class="flex items-center justify-between p-4 rounded-xl border border-border bg-background/40 hover:bg-background/80 transition-colors">
                <div class="flex items-center gap-3">
                  <div class="p-2.5 rounded-lg bg-muted text-foreground">
                    <session.icon size={18} />
                  </div>
                  <div class="flex flex-col gap-0.5">
                    <span class="text-xs font-bold text-foreground flex items-center gap-2">
                      <span>{session.device}</span>
                      {#if session.active}
                        <span class="text-[8px] px-1.5 py-0.2 rounded-full font-semibold bg-emerald-500/10 text-emerald-500 border border-emerald-500/20">
                          {m.settings_user_detail_session_active_now()}
                        </span>
                      {/if}
                    </span>
                    <span class="text-[10px] text-muted-foreground">
                      {session.location} • {session.ip} • <span class="font-mono">{session.time}</span>
                    </span>
                  </div>
                </div>

                {#if !session.active}
                  <Button 
                    variant="outline" 
                    size="xs" 
                    class="text-[10px] text-destructive hover:bg-destructive/10 border-destructive/20"
                    onclick={() => revokeSession(session.id)}
                  >
                    {m.settings_user_detail_session_revoke()}
                  </Button>
                {/if}
              </div>
            {/each}
          </div>
        </div>
      {/if}

      {#if activeTab === 'permissions'}
        <div in:fade={{ duration: 100 }} class="bg-card border border-border rounded-xl p-6 flex flex-col gap-6 text-start">
          <div class="flex flex-col gap-1 border-b border-border/60 pb-3">
            <h2 class="text-sm font-bold text-foreground">
              {getLocale() === "ar" ? "تجاوز صلاحيات الدور (صلاحيات استثنائية)" : "Role Permissions Override"}
            </h2>
            <p class="text-[10px] text-muted-foreground">
              {getLocale() === "ar" 
                ? `يرث هذا الحساب كافة صلاحيات دور: ${getRoleLabel(role)}. يمكنك أدناه منح أو حظر ميزات استثنائية خارج نطاق الدور المعتاد.`
                : `This account inherits the default permissions of ${getRoleLabel(role)}. You can set individual overrides below.`}
            </p>
          </div>

          <div class="bg-primary/5 border border-primary/20 rounded-xl p-4 flex gap-3 text-start items-start">
            <Shield size={18} class="text-primary mt-0.5 flex-shrink-0" />
            <div class="flex flex-col gap-1">
              <span class="text-xs font-bold text-foreground">
                {getLocale() === "ar" ? "الصلاحيات الموروثة مفعلة تلقائياً" : "Inheritance Mode Active"}
              </span>
              <p class="text-[10px] text-muted-foreground leading-normal">
                {getLocale() === "ar"
                  ? "لا تحتاج لإعادة إسناد الصلاحيات الموجودة مسبقاً في دور المستخدم الأصلي، فقط قم بإضافة المزايا الحصرية التي تريد توسيع نطاقها له."
                  : "Inherited permissions don't need configuration. Switch overrides here if this specific user needs individual privileges."}
              </p>
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <!-- Submit grades override -->
            <div class="flex items-center justify-between p-4 rounded-xl border border-border bg-background/20">
              <div class="flex flex-col gap-1">
                <span class="text-xs font-bold text-foreground">
                  {getLocale() === "ar" ? "رصد وتعديل الدرجات مباشرة" : "Submit Grades Directly"}
                </span>
                <span class="text-[9px] text-muted-foreground max-w-xs">
                  {getLocale() === "ar" ? "السماح بالرصد حتى لو لم يكن معلماً للصف" : "Allow inputting scores across all classes"}
                </span>
              </div>
              <button 
                class="w-10 h-6 rounded-full p-0.5 transition-colors duration-200 cursor-pointer focus:outline-none flex items-center
                  {overridePermissions.acad_grades_submit ? 'bg-primary' : 'bg-muted'}"
                disabled={isSaving}
                onclick={() => overridePermissions.acad_grades_submit = !overridePermissions.acad_grades_submit}
                aria-label="Toggle Override"
                type="button"
              >
                <div class="w-5 h-5 rounded-full bg-white shadow-md transform transition-transform duration-200
                  {overridePermissions.acad_grades_submit ? (getLocale() === 'ar' ? '-translate-x-4' : 'translate-x-4') : 'translate-x-0'}">
                </div>
              </button>
            </div>
 
            <!-- View Ledgers override -->
            <div class="flex items-center justify-between p-4 rounded-xl border border-border bg-background/20">
              <div class="flex flex-col gap-1">
                <span class="text-xs font-bold text-foreground">
                  {getLocale() === "ar" ? "الاطلاع على القيود المالية اليومية" : "Access Financial Ledgers"}
                </span>
                <span class="text-[9px] text-muted-foreground max-w-xs">
                  {getLocale() === "ar" ? "الولوج للميزانيات والحسابات الختامية" : "View bookkeeping ledgers and balance sheet"}
                </span>
              </div>
              <button 
                class="w-10 h-6 rounded-full p-0.5 transition-colors duration-200 cursor-pointer focus:outline-none flex items-center
                  {overridePermissions.fin_ledger_view ? 'bg-primary' : 'bg-muted'}"
                disabled={isSaving}
                onclick={() => overridePermissions.fin_ledger_view = !overridePermissions.fin_ledger_view}
                aria-label="Toggle Override"
                type="button"
              >
                <div class="w-5 h-5 rounded-full bg-white shadow-md transform transition-transform duration-200
                  {overridePermissions.fin_ledger_view ? (getLocale() === 'ar' ? '-translate-x-4' : 'translate-x-4') : 'translate-x-0'}">
                </div>
              </button>
            </div>
 
            <!-- Enroll student override -->
            <div class="flex items-center justify-between p-4 rounded-xl border border-border bg-background/20">
              <div class="flex flex-col gap-1">
                <span class="text-xs font-bold text-foreground">
                  {getLocale() === "ar" ? "تسجيل وقبول ملفات الطلاب" : "Enroll Students"}
                </span>
                <span class="text-[9px] text-muted-foreground max-w-xs">
                  {getLocale() === "ar" ? "حق قبول الطلاب وإصدار الأرقام الأكاديمية" : "Register new students and assign IDs"}
                </span>
              </div>
              <button 
                class="w-10 h-6 rounded-full p-0.5 transition-colors duration-200 cursor-pointer focus:outline-none flex items-center
                  {overridePermissions.student_enroll ? 'bg-primary' : 'bg-muted'}"
                disabled={isSaving}
                onclick={() => overridePermissions.student_enroll = !overridePermissions.student_enroll}
                aria-label="Toggle Override"
                type="button"
              >
                <div class="w-5 h-5 rounded-full bg-white shadow-md transform transition-transform duration-200
                  {overridePermissions.student_enroll ? (getLocale() === 'ar' ? '-translate-x-4' : 'translate-x-4') : 'translate-x-0'}">
                </div>
              </button>
            </div>
 
            <!-- Generate reports override -->
            <div class="flex items-center justify-between p-4 rounded-xl border border-border bg-background/20">
              <div class="flex flex-col gap-1">
                <span class="text-xs font-bold text-foreground">
                  {getLocale() === "ar" ? "استخراج الشهادات والتقارير العامة" : "Generate Academic Reports"}
                </span>
                <span class="text-[9px] text-muted-foreground max-w-xs">
                  {getLocale() === "ar" ? "الطباعة المباشرة لكشوفات الفصل الأول والثاني" : "Export reports and school certifications"}
                </span>
              </div>
              <button 
                class="w-10 h-6 rounded-full p-0.5 transition-colors duration-200 cursor-pointer focus:outline-none flex items-center
                  {overridePermissions.report_generate ? 'bg-primary' : 'bg-muted'}"
                disabled={isSaving}
                onclick={() => overridePermissions.report_generate = !overridePermissions.report_generate}
                aria-label="Toggle Override"
                type="button"
              >
                <div class="w-5 h-5 rounded-full bg-white shadow-md transform transition-transform duration-200
                  {overridePermissions.report_generate ? (getLocale() === 'ar' ? '-translate-x-4' : 'translate-x-4') : 'translate-x-0'}">
                </div>
              </button>
            </div>
          </div>

          <div class="flex justify-end border-t border-border/40 pt-4 mt-2">
            <Button variant="lime" class="text-xs h-9" onclick={handleSaveOverrides}>
              {getLocale() === "ar" ? "حفظ صلاحيات التجاوز" : "Save Overrides"}
            </Button>
          </div>
        </div>
      {/if}

      {#if activeTab === 'settings'}
        <div in:fade={{ duration: 100 }} class="bg-card border border-border rounded-xl p-6 flex flex-col gap-6 text-start">
          <h2 class="text-sm font-bold text-foreground border-b border-border/60 pb-3">
            {getLocale() === "ar" ? "الخيارات وتفضيلات النظام المخصصة للمستخدم" : "Custom Settings & Application Preferences"}
          </h2>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Theme Preference -->
            <div class="flex flex-col gap-2">
              <span class="text-xs font-semibold text-muted-foreground flex items-center gap-1.5">
                <Layout size={13} />
                {m.settings_user_detail_pref_theme()}
              </span>
              <select
                bind:value={prefTheme}
                class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-xs focus:ring-1 focus:ring-ring outline-none h-9 w-full cursor-pointer"
              >
                <option value="system">{getLocale() === "ar" ? "تلقائي (حسب النظام)" : "System Default"}</option>
                <option value="light">{getLocale() === "ar" ? "مظهر مضيء" : "Light Mode"}</option>
                <option value="dark">{getLocale() === "ar" ? "مظهر داكن" : "Dark Mode"}</option>
              </select>
            </div>

            <!-- Language Preference -->
            <div class="flex flex-col gap-2">
              <span class="text-xs font-semibold text-muted-foreground flex items-center gap-1.5">
                <Globe size={13} />
                {m.settings_user_detail_pref_lang()}
              </span>
              <select
                bind:value={prefLanguage}
                class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-xs focus:ring-1 focus:ring-ring outline-none h-9 w-full cursor-pointer"
              >
                <option value="ar">العربية (Arabic)</option>
                <option value="en">English (الإنجليزية)</option>
              </select>
            </div>

            <!-- Sidebar Preference -->
            <div class="flex flex-col gap-2">
              <span class="text-xs font-semibold text-muted-foreground flex items-center gap-1.5">
                <Layout size={13} />
                {m.settings_user_detail_pref_sidebar()}
              </span>
              <select
                bind:value={prefSidebar}
                class="bg-background text-foreground border border-border rounded-md px-3 py-2 text-xs focus:ring-1 focus:ring-ring outline-none h-9 w-full cursor-pointer"
              >
                <option value="wide">{m.settings_user_detail_pref_sidebar_wide()}</option>
                <option value="compact">{m.settings_user_detail_pref_sidebar_compact()}</option>
              </select>
            </div>

            <!-- Notification Channels -->
            <div class="flex flex-col gap-3">
              <span class="text-xs font-semibold text-muted-foreground">
                {m.settings_user_detail_pref_notifications()}
              </span>
              <div class="flex flex-col gap-2">
                 <label class="flex items-center justify-between text-xs text-foreground cursor-pointer select-none">
                  <span>{getLocale() === "ar" ? "إرسال الإشعارات عبر البريد الإلكتروني" : "Email Notifications"}</span>
                  <button 
                    class="w-10 h-6 rounded-full p-0.5 transition-colors duration-200 cursor-pointer focus:outline-none flex items-center
                      {notifyEmail ? 'bg-primary' : 'bg-muted'}"
                    disabled={isSaving}
                    onclick={() => notifyEmail = !notifyEmail}
                    aria-label="Toggle Email Notification"
                    type="button"
                  >
                    <div class="w-5 h-5 rounded-full bg-white shadow-md transform transition-transform duration-200
                      {notifyEmail ? (getLocale() === 'ar' ? '-translate-x-4' : 'translate-x-4') : 'translate-x-0'}">
                    </div>
                  </button>
                </label>
                <label class="flex items-center justify-between text-xs text-foreground cursor-pointer select-none">
                  <span>{getLocale() === "ar" ? "تنبيهات فورية (Push Notifications)" : "Push System Alerts"}</span>
                  <button 
                    class="w-10 h-6 rounded-full p-0.5 transition-colors duration-200 cursor-pointer focus:outline-none flex items-center
                      {notifyPush ? 'bg-primary' : 'bg-muted'}"
                    disabled={isSaving}
                    onclick={() => notifyPush = !notifyPush}
                    aria-label="Toggle Push Notification"
                    type="button"
                  >
                    <div class="w-5 h-5 rounded-full bg-white shadow-md transform transition-transform duration-200
                      {notifyPush ? (getLocale() === 'ar' ? '-translate-x-4' : 'translate-x-4') : 'translate-x-0'}">
                    </div>
                  </button>
                </label>
                <label class="flex items-center justify-between text-xs text-foreground cursor-pointer select-none">
                  <span>{getLocale() === "ar" ? "رسائل جوال SMS (للحالات العاجلة)" : "SMS Notifications"}</span>
                  <button 
                    class="w-10 h-6 rounded-full p-0.5 transition-colors duration-200 cursor-pointer focus:outline-none flex items-center
                      {notifySms ? 'bg-primary' : 'bg-muted'}"
                    disabled={isSaving}
                    onclick={() => notifySms = !notifySms}
                    aria-label="Toggle SMS Notification"
                    type="button"
                  >
                    <div class="w-5 h-5 rounded-full bg-white shadow-md transform transition-transform duration-200
                      {notifySms ? (getLocale() === 'ar' ? '-translate-x-4' : 'translate-x-4') : 'translate-x-0'}">
                    </div>
                  </button>
                </label>
              </div>
            </div>
          </div>

          <div class="flex justify-end border-t border-border/40 pt-4 mt-2">
            <Button variant="lime" class="text-xs h-9 px-6 font-bold" onclick={() => toast.success(getLocale() === "ar" ? "تم حفظ تفضيلات المستخدم" : "Preferences saved successfully")}>
              {getLocale() === "ar" ? "حفظ التفضيلات" : "Save Preferences"}
            </Button>
          </div>
        </div>
      {/if}
    </div>
  {/if}
</div>

<!-- Reset Password Modal Dialog -->
{#if showResetModal}
  <div 
    in:fade={{ duration: 150 }} 
    class="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4"
  >
    <div 
      in:slide={{ duration: 200 }} 
      class="bg-card border border-border rounded-2xl w-full max-w-md p-6 flex flex-col gap-5 text-start shadow-2xl font-sans"
    >
      <div class="flex flex-col gap-1">
        <h2 class="text-sm font-bold text-foreground flex items-center gap-1.5">
          <Key size={16} class="text-primary" />
          <span>{m.settings_user_detail_reset_pass_title()}</span>
        </h2>
        <p class="text-[10px] text-muted-foreground">
          {getLocale() === "ar" 
            ? `قم بإدخال كلمة مرور جديدة وتأكيدها للمستخدم: ${fullName}` 
            : `Set and confirm a new login password for ${fullName}.`}
        </p>
      </div>

      <form onsubmit={handleResetPassword} class="flex flex-col gap-4">
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

        <div class="flex items-center justify-end gap-2 border-t border-border/40 pt-4 mt-2">
          <Button 
            type="button" 
            variant="ghost" 
            class="text-xs h-8" 
            onclick={() => { showResetModal = false; newPassword = ''; confirmPassword = ''; }}
            disabled={isResettingPassword}
          >
            {getLocale() === "ar" ? "إلغاء" : "Cancel"}
          </Button>
          <Button 
            type="submit" 
            variant="lime" 
            class="text-xs h-8 font-bold" 
            disabled={isResettingPassword}
          >
            {isResettingPassword ? (getLocale() === "ar" ? "جاري الحفظ..." : "Resetting...") : m.settings_user_detail_btn_reset_pass()}
          </Button>
        </div>
      </form>
    </div>
  </div>
{/if}
