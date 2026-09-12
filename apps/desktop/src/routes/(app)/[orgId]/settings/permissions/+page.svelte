<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Badge } from "@orbis/ui/badge";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@orbis/ui/tabs";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@orbis/ui/table";
import { toast } from "@orbis/ui";
import { api } from "$lib/api";
import type { UserDto } from "$lib/api/types";
import { goto } from "$app/navigation";
import {
  Shield,
  ShieldAlert,
  ShieldCheck,
  Plus,
  Trash2,
  Search,
  KeyRound,
  Users,
  Sparkles,
  Lock,
  Layers,
  Edit3,
  UserCheck,
  Sliders,
  RefreshCw,
} from "@lucide/svelte";

import RoleCreationSheet from "$lib/features/settings/components/role-creation-sheet.svelte";
import PermissionRuleDialog from "$lib/features/settings/components/permission-rule-dialog.svelte";
import UserPermissionsDialog from "$lib/features/settings/components/user-permissions-dialog.svelte";

let { data }: { data: { orgId: string } } = $props();

interface PermissionRule {
  id?: string;
  role: string;
  document_type: string;
  level: number;
  only_if_creator: boolean;
  actions: string[];
}

interface RoleCardData {
  id: string;
  name: string;
  nameAr: string;
  description: string;
  isSystem: boolean;
  color: string;
  icon: any;
  permissions: Record<string, string[]>;
}

const STANDARD_DOC_TYPES = [
  "Organization",
  "User",
  "Period",
  "OrgUnit",
  "Class",
  "Subject",
  "Schedule",
  "Assessment",
  "Guardian",
  "Document",
  "Enrollment",
  "Grade",
  "Attendance",
  "Behavior",
  "Notification",
  "Dashboard",
  "Audit",
  "Finance",
  "Exam",
  "Template",
];

const ACTIONS_BY_DOC_TYPE: Record<string, string[]> = {
  Organization: ["read", "write", "manage"],
  User: ["read", "write", "delete", "role_write", "password_write"],
  Period: ["read", "write", "activate", "lock"],
  OrgUnit: ["read", "write", "delete"],
  Class: ["read", "write", "delete"],
  Subject: ["read", "write", "delete"],
  Schedule: ["read", "write"],
  Assessment: ["read", "write", "delete"],
  Guardian: ["read", "write", "delete"],
  Document: ["read", "write", "delete"],
  Enrollment: ["read", "write", "promote", "transfer"],
  Grade: ["read", "write", "analytics"],
  Attendance: ["read", "write"],
  Behavior: ["read", "write", "delete"],
  Notification: ["read", "write"],
  Dashboard: ["read"],
  Audit: ["read"],
  Finance: ["read", "write"],
  Exam: ["read", "write", "generate_seating", "generate_secret", "assign_proctor"],
  Template: ["read", "write", "publish", "delete"],
};

function getRoleVisuals(roleId: string) {
  switch (roleId) {
    case "super_admin":
      return {
        icon: ShieldAlert,
        color: "bg-red-500/10 text-red-600 dark:text-red-400 border-red-500/20",
      };
    case "admin":
      return {
        icon: ShieldCheck,
        color: "bg-amber-500/10 text-amber-600 dark:text-amber-400 border-amber-500/20",
      };
    case "principal":
      return {
        icon: Shield,
        color: "bg-blue-500/10 text-blue-600 dark:text-blue-400 border-blue-500/20",
      };
    case "teacher":
      return {
        icon: KeyRound,
        color: "bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 border-emerald-500/20",
      };
    case "accountant":
      return {
        icon: ShieldCheck,
        color: "bg-purple-500/10 text-purple-600 dark:text-purple-400 border-purple-500/20",
      };
    case "registrar":
      return {
        icon: Users,
        color: "bg-cyan-500/10 text-cyan-600 dark:text-cyan-400 border-cyan-500/20",
      };
    default:
      return {
        icon: Sparkles,
        color: "bg-primary/10 text-primary border-primary/20",
      };
  }
}

function getRoleDefaultDescription(id: string): string {
  switch (id) {
    case "super_admin":
      return "كامل الصلاحيات غير المقيدة على مستوى المنظمة وفروعها وكافة وحداتها.";
    case "admin":
      return "إدارة المستخدمين والشؤون الأكاديمية والمالية مع قيود أمان النظام الحساسة.";
    case "principal":
      return "الإشراف الكامل على الكادر التعليمي، الشعب، تقارير الحضور والدرجات.";
    case "teacher":
      return "رصد درجات المواد المسندة، تسجيل الحضور اليومي، وإدارة الاختبارات الفصلية.";
    case "accountant":
      return "إدارة قيود اليومية، تحصيل الرسوم وسندات القبض، ومتابعة مديونيات الطلاب.";
    case "registrar":
      return "تسجيل وقبول الطلاب، إدارة ملفات أولياء الأمور، والترحيل بين المراحل.";
    case "student":
      return "الاطلاع على الجدول الأكاديمي، كشف الدرجات، وسجل الحضور والواجبات.";
    case "parent":
      return "متابعة الأداء الأكاديمي والمالي وسجل الحضور للأبناء المرتبطين.";
    default:
      return "دور مخصص للمنظمة تم إنشاؤه وتخصيصه للتحكم في الصلاحيات.";
  }
}

let roles = $state<RoleCardData[]>([]);
let rules = $state<PermissionRule[]>([]);
let usersList = $state<UserDto[]>([]);
let activeTab = $state("roles");
let searchQuery = $state("");
let roleFilter = $state("all");
let docTypeFilter = $state("all");
let isLoading = $state(false);

// Modals / Sheets
let showRoleSheet = $state(false);
let selectedSheetRole = $state<RoleCardData | null>(null);

let showAddRuleDialog = $state(false);
let showUserPermissionsDialog = $state(false);

// Add Rule Form States
let newRuleDocType = $state("");
let newRuleRole = $state("super_admin");
let newRuleLevel = $state(0);
let newRuleOnlyIfCreator = $state(false);
let newRuleActions = $state<string[]>([]);

// Users List Search
let userSearchQuery = $state("");

// Derived
const allRoles = $derived(roles);
const customRolesCount = $derived(roles.filter((r) => !r.isSystem).length);

const filteredRules = $derived(
  rules.filter((rule) => {
    const matchesSearch =
      !searchQuery ||
      rule.role.toLowerCase().includes(searchQuery.toLowerCase()) ||
      rule.document_type.toLowerCase().includes(searchQuery.toLowerCase());
    const matchesRole = roleFilter === "all" || rule.role === roleFilter;
    const matchesDocType =
      docTypeFilter === "all" || rule.document_type === docTypeFilter;
    return matchesSearch && matchesRole && matchesDocType;
  })
);

const filteredUsers = $derived(
  usersList.filter((u) => {
    if (!userSearchQuery) return true;
    const query = userSearchQuery.toLowerCase();
    return (
      (u.name && u.name.toLowerCase().includes(query)) ||
      (u.email && u.email.toLowerCase().includes(query)) ||
      (u.role && u.role.toLowerCase().includes(query))
    );
  })
);

onMount(async () => {
  await loadData();
});

async function loadData() {
  isLoading = true;
  try {
    const [rolesData, permissionsData, usersData] = await Promise.all([
      api.listRoles(),
      api.listPermissions(),
      api.listUsers(),
    ]);

    roles = (rolesData || []).map((r) => {
      const visuals = getRoleVisuals(r.id);
      return {
        id: r.id,
        name: r.name,
        nameAr: r.name_ar || r.name,
        description: r.description || getRoleDefaultDescription(r.id),
        isSystem: r.is_system,
        color: visuals.color,
        icon: visuals.icon,
        permissions: r.permissions || {},
      };
    });

    rules = (permissionsData || []).map((p) => ({
      id: p.id,
      role: p.role,
      document_type: p.document_type,
      level: p.level ?? 10,
      only_if_creator: p.only_if_creator ?? false,
      actions: p.actions || [],
    }));

    usersList = usersData || [];
  } catch (e) {
    console.error("Failed to load permissions data from backend", e);
    toast.error(m.settings_permissions_load_error());
  } finally {
    isLoading = false;
  }
}

function openEditRoleSheet(role: RoleCardData) {
  selectedSheetRole = role;
  showRoleSheet = true;
}

function openCreateRoleSheet() {
  selectedSheetRole = {
    id: "",
    name: "",
    nameAr: "",
    description: "",
    isSystem: false,
    color: "bg-primary/10 text-primary border-primary/20",
    icon: Sparkles,
    permissions: {},
  };
  showRoleSheet = true;
}

async function handleSaveRole(formData: {
  id: string;
  name: string;
  nameAr: string;
  description: string;
  permissions: Record<string, string[]>;
}) {
  try {
    await api.saveRole({
      id: formData.id,
      name: formData.name,
      name_ar: formData.nameAr,
      description: formData.description,
      permissions: formData.permissions,
    });
    toast.success("تم حفظ صلاحيات الدور بنجاح في قاعدة البيانات");
    await loadData();
  } catch (e) {
    console.error("Failed to save role", e);
    toast.error("فشل حفظ الدور");
  }
}

async function handleAddSingleRule() {
  if (!newRuleDocType || newRuleActions.length === 0) return;

  const newRule: PermissionRule = {
    role: newRuleRole,
    document_type: newRuleDocType,
    level: newRuleLevel,
    only_if_creator: newRuleOnlyIfCreator,
    actions: [...newRuleActions],
  };

  try {
    await api.savePermissions([newRule]);
    toast.success(m.settings_permissions_save_success());
    newRuleDocType = "";
    newRuleActions = [];
    showAddRuleDialog = false;
    await loadData();
  } catch (e) {
    console.error("Failed to save rule to backend", e);
    toast.error("فشل حفظ القاعدة");
  }
}

async function handleDeleteRule(index: number) {
  const targetRule = rules[index];
  if (!targetRule) return;

  try {
    await api.savePermissions([
      {
        role: targetRule.role,
        document_type: targetRule.document_type,
        actions: [],
      },
    ]);
    toast.success("تم حذف القاعدة بنجاح");
    await loadData();
  } catch (e) {
    console.error("Failed to delete rule", e);
    toast.error("فشل حذف القاعدة");
  }
}

function getActionsForDocType(type: string): string[] {
  return ACTIONS_BY_DOC_TYPE[type] || ["read", "write", "delete"];
}

function capitalizeFirstLetter(str: string): string {
  return str.charAt(0).toUpperCase() + str.slice(1);
}

function handleSelectUserForPermissions(userId: string) {
  showUserPermissionsDialog = false;
  goto(`/${data.orgId}/settings/users/${userId}/profile`);
}
</script>

<svelte:head>
  <title>{m.settings_permissions_window_title?.() || "الصلاحيات"} | Orbis</title>
</svelte:head>

<div class="p-6 max-w-7xl mx-auto space-y-6">
  <!-- Hero Header -->
  <div class="relative overflow-hidden rounded-2xl border border-border/80 bg-linear-to-b from-card to-card/60 p-6 shadow-sm backdrop-blur-md">
    <div class="flex flex-col md:flex-row md:items-center justify-between gap-4">
      <div class="flex items-center gap-3.5">
        <div class="w-12 h-12 rounded-2xl bg-primary/10 border border-primary/20 flex items-center justify-center text-primary shadow-xs">
          <Shield class="w-6 h-6" />
        </div>
        <div>
          <h1 class="text-xl font-bold text-foreground tracking-tight">
            {m.settings_permissions_window_title?.() || "صلاحيات الأدوار"}
          </h1>
          <p class="text-xs text-muted-foreground mt-0.5">
            إدارة الأدوار المؤسسية، مصفوفة الصلاحيات الدقيقة، والتحكم بالوصول (IAM & Access Control)
          </p>
        </div>
      </div>

      <!-- Quick Action Buttons -->
      <div class="flex items-center gap-2">
        <Button
          variant="outline"
          size="sm"
          class="h-9 text-xs font-semibold gap-1.5 shadow-2xs"
          onclick={loadData}
          disabled={isLoading}
        >
          <RefreshCw class="w-3.5 h-3.5 {isLoading ? 'animate-spin' : ''}" />
          تحديث
        </Button>

        <Button
          variant="outline"
          size="sm"
          class="h-9 text-xs font-semibold gap-1.5 shadow-2xs"
          onclick={() => (showUserPermissionsDialog = true)}
        >
          <UserCheck class="w-4 h-4" />
          {m.settings_permissions_btn_set_user()}
        </Button>

        <Button
          variant="default"
          size="sm"
          class="h-9 text-xs font-bold gap-1.5 shadow-sm"
          onclick={openCreateRoleSheet}
        >
          <Plus class="w-4 h-4" />
          إنشاء دور مخصص جديد
        </Button>
      </div>
    </div>

    <!-- Stats Bar -->
    <div class="grid grid-cols-2 sm:grid-cols-4 gap-3 mt-6 pt-4 border-t border-border/60">
      <div class="flex items-center gap-3">
        <div class="w-8 h-8 rounded-lg bg-primary/10 flex items-center justify-center text-primary">
          <Layers class="w-4 h-4" />
        </div>
        <div>
          <div class="text-lg font-bold text-foreground font-mono leading-none">{allRoles.length}</div>
          <div class="text-[11px] text-muted-foreground mt-0.5">إجمالي الأدوار</div>
        </div>
      </div>

      <div class="flex items-center gap-3">
        <div class="w-8 h-8 rounded-lg bg-amber-500/10 flex items-center justify-center text-amber-600 dark:text-amber-400">
          <KeyRound class="w-4 h-4" />
        </div>
        <div>
          <div class="text-lg font-bold text-foreground font-mono leading-none">{rules.length}</div>
          <div class="text-[11px] text-muted-foreground mt-0.5">قواعد الصلاحيات المطبقة</div>
        </div>
      </div>

      <div class="flex items-center gap-3">
        <div class="w-8 h-8 rounded-lg bg-emerald-500/10 flex items-center justify-center text-emerald-600 dark:text-emerald-400">
          <Users class="w-4 h-4" />
        </div>
        <div>
          <div class="text-lg font-bold text-foreground font-mono leading-none">{usersList.length}</div>
          <div class="text-[11px] text-muted-foreground mt-0.5">المستخدمين المسجلين</div>
        </div>
      </div>

      <div class="flex items-center gap-3">
        <div class="w-8 h-8 rounded-lg bg-blue-500/10 flex items-center justify-center text-blue-600 dark:text-blue-400">
          <ShieldCheck class="w-4 h-4" />
        </div>
        <div>
          <div class="text-lg font-bold text-foreground font-mono leading-none">{customRolesCount}</div>
          <div class="text-[11px] text-muted-foreground mt-0.5">أدوار مخصصة</div>
        </div>
      </div>
    </div>
  </div>

  <!-- Main Tabs -->
  <Tabs value={activeTab} onValueChange={(v) => (activeTab = v)} class="space-y-4">
    <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3">
      <TabsList class="bg-muted/30 p-1 rounded-xl border border-border/60">
        <TabsTrigger value="roles" class="text-xs font-semibold gap-1.5 px-3.5 py-1.5 rounded-lg">
          <Shield class="w-3.5 h-3.5" />
          بطاقات الأدوار والصلاحيات ({roles.length})
        </TabsTrigger>
        <TabsTrigger value="matrix" class="text-xs font-semibold gap-1.5 px-3.5 py-1.5 rounded-lg">
          <Sliders class="w-3.5 h-3.5" />
          جدول القواعد التفصيلي ({rules.length})
        </TabsTrigger>
      </TabsList>

      {#if activeTab === "matrix"}
        <div class="flex items-center gap-2 w-full sm:w-auto">
          <div class="relative w-full sm:w-64">
            <Search class="absolute start-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-muted-foreground" />
            <Input
              type="search"
              placeholder="بحث في القواعد والمستندات..."
              bind:value={searchQuery}
              class="h-8 ps-8 text-xs w-full"
            />
          </div>

          <Button
            variant="outline"
            size="sm"
            class="h-8 text-xs font-semibold gap-1 shrink-0"
            onclick={() => (showAddRuleDialog = true)}
          >
            <Plus class="w-3.5 h-3.5" />
            إضافة قاعدة
          </Button>
        </div>
      {/if}
    </div>

    <!-- Tab 1: Roles Grid -->
    <TabsContent value="roles" class="space-y-4 m-0">
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {#each allRoles as role}
          {@const Icon = role.icon || Shield}
          {@const roleRulesCount = rules.filter((r) => r.role === role.id).length}

          <div class="group relative rounded-2xl border border-border/80 bg-card p-5 shadow-xs hover:shadow-md transition-all hover:border-primary/40 flex flex-col justify-between gap-4">
            <div class="space-y-3">
              <div class="flex items-start justify-between gap-2">
                <div class="flex items-center gap-2.5">
                  <div class="w-10 h-10 rounded-xl {role.color} border flex items-center justify-center shrink-0">
                    <Icon class="w-5 h-5" />
                  </div>
                  <div>
                    <h3 class="text-sm font-bold text-foreground group-hover:text-primary transition-colors">
                      {role.nameAr || role.name}
                    </h3>
                    <div class="text-[10px] text-muted-foreground font-mono">
                      {role.name} ({role.id})
                    </div>
                  </div>
                </div>

                {#if role.isSystem}
                  <Badge variant="outline" class="text-[9px] gap-1 bg-muted/40 font-medium">
                    <Lock class="w-2.5 h-2.5" />
                    أساسي
                  </Badge>
                {:else}
                  <Badge variant="secondary" class="text-[9px] gap-1 bg-primary/10 text-primary font-semibold">
                    <Sparkles class="w-2.5 h-2.5" />
                    مخصص
                  </Badge>
                {/if}
              </div>

              <p class="text-xs text-muted-foreground leading-relaxed line-clamp-2">
                {role.description || "لا يوجد وصف مخصص لهذا الدور."}
              </p>
            </div>

            <div class="space-y-3 pt-3 border-t border-border/50">
              <div class="flex items-center justify-between text-[11px] text-muted-foreground">
                <span>القواعد المعرفة في النظام:</span>
                <span class="font-bold text-foreground font-mono">{roleRulesCount}</span>
              </div>

              <Button
                variant="outline"
                size="sm"
                class="w-full h-8 text-xs font-semibold gap-1.5 group-hover:bg-primary group-hover:text-primary-foreground group-hover:border-primary transition-all"
                onclick={() => openEditRoleSheet(role)}
              >
                <Edit3 class="w-3.5 h-3.5" />
                تعديل وتخصيص الصلاحيات
              </Button>
            </div>
          </div>
        {/each}

        <!-- Add Role Card Button -->
        <button
          type="button"
          onclick={openCreateRoleSheet}
          class="rounded-2xl border-2 border-dashed border-border/80 hover:border-primary/60 bg-muted/10 hover:bg-primary/5 p-6 flex flex-col items-center justify-center gap-2 text-center transition-all cursor-pointer group min-h-[200px]"
        >
          <div class="w-10 h-10 rounded-full bg-primary/10 group-hover:bg-primary group-hover:text-primary-foreground flex items-center justify-center text-primary transition-all shadow-xs">
            <Plus class="w-5 h-5" />
          </div>
          <div class="text-xs font-bold text-foreground group-hover:text-primary transition-colors">
            إنشاء دور مخصص جديد
          </div>
          <div class="text-[10px] text-muted-foreground max-w-[200px]">
            حدد اسماً وصلاحيات وصول مخصصة للموظفين أو المشرفين
          </div>
        </button>
      </div>
    </TabsContent>

    <!-- Tab 2: Granular Rules Matrix -->
    <TabsContent value="matrix" class="space-y-4 m-0">
      <div class="rounded-2xl border border-border/80 bg-card shadow-xs overflow-hidden">
        <Table>
          <TableHeader class="bg-muted/40">
            <TableRow>
              <TableHead class="text-xs font-bold w-44">{m.settings_permissions_col_doc_type()}</TableHead>
              <TableHead class="text-xs font-bold w-36">{m.settings_permissions_label_role()}</TableHead>
              <TableHead class="text-xs font-bold w-24 text-center">المستوى</TableHead>
              <TableHead class="text-xs font-bold w-36">القيود</TableHead>
              <TableHead class="text-xs font-bold">{m.settings_permissions_col_permissions()}</TableHead>
              <TableHead class="w-16 text-end"></TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {#if filteredRules.length === 0}
              <TableRow>
                <TableCell colspan={6} class="text-center py-12 text-xs text-muted-foreground">
                  {#if isLoading}
                    جاري تحميل الصلاحيات من الخادم...
                  {:else}
                    لا توجد قواعد صلاحيات محفوظة حالياً
                  {/if}
                </TableCell>
              </TableRow>
            {:else}
              {#each filteredRules as rule, idx}
                <TableRow class="hover:bg-muted/20 transition-colors">
                  <TableCell class="font-mono text-xs font-bold text-foreground">
                    {rule.document_type}
                  </TableCell>
                  <TableCell>
                    <Badge variant="secondary" class="text-[10px] font-semibold">
                      {rule.role}
                    </Badge>
                  </TableCell>
                  <TableCell class="text-center font-mono text-xs">
                    {rule.level}
                  </TableCell>
                  <TableCell>
                    {#if rule.only_if_creator}
                      <Badge variant="outline" class="text-[9px] bg-amber-500/10 text-amber-600 dark:text-amber-400 border-amber-500/30">
                        {m.settings_permissions_only_creator()}
                      </Badge>
                    {:else}
                      <span class="text-[10px] text-muted-foreground">عام (بدون قيود)</span>
                    {/if}
                  </TableCell>
                  <TableCell>
                    <div class="flex flex-wrap gap-1">
                      {#each rule.actions as action}
                        <span class="px-1.5 py-0.5 rounded-md text-[10px] font-mono font-medium bg-primary/10 text-primary border border-primary/20">
                          {action}
                        </span>
                      {/each}
                    </div>
                  </TableCell>
                  <TableCell class="text-end">
                    <Button
                      variant="ghost"
                      size="sm"
                      class="h-7 w-7 p-0 text-muted-foreground hover:text-destructive transition-colors"
                      onclick={() => handleDeleteRule(idx)}
                    >
                      <Trash2 class="w-3.5 h-3.5" />
                    </Button>
                  </TableCell>
                </TableRow>
              {/each}
            {/if}
          </TableBody>
        </Table>
      </div>
    </TabsContent>
  </Tabs>
</div>

<!-- Slide-over Role Creation & Editing Sheet -->
{#if selectedSheetRole}
  <RoleCreationSheet
    bind:open={showRoleSheet}
    roleId={selectedSheetRole.id}
    roleName={selectedSheetRole.name}
    roleNameAr={selectedSheetRole.nameAr}
    roleDescription={selectedSheetRole.description}
    isSystem={selectedSheetRole.isSystem}
    selectedPermissions={selectedSheetRole.permissions}
    onsave={handleSaveRole}
  />
{/if}

<!-- Add Single Permission Rule Dialog -->
<PermissionRuleDialog
  bind:open={showAddRuleDialog}
  bind:docType={newRuleDocType}
  bind:role={newRuleRole}
  bind:level={newRuleLevel}
  bind:onlyIfCreator={newRuleOnlyIfCreator}
  bind:actions={newRuleActions}
  standardDocTypes={STANDARD_DOC_TYPES}
  standardRoles={allRoles.map((r) => r.id)}
  {getActionsForDocType}
  {capitalizeFirstLetter}
  onsubmit={handleAddSingleRule}
/>

<!-- User Permissions Picker Dialog -->
<UserPermissionsDialog
  bind:open={showUserPermissionsDialog}
  users={usersList}
  bind:searchQuery={userSearchQuery}
  {filteredUsers}
  onselect={handleSelectUserForPermissions}
/>
