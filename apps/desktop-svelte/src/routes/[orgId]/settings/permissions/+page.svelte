<script lang="ts">
import { onMount } from "svelte";
import { fade, slide } from "svelte/transition";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import { Checkbox } from "@orbis/ui-svelte/checkbox";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from "@orbis/ui-svelte/dialog";
import { Badge } from "@orbis/ui-svelte/badge";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@orbis/ui-svelte/table";
import { toast } from "svelte-sonner";
import { getLocale } from "$lib/paraglide/runtime";
import { commands } from "$lib/services/ipc";
import { goto } from "$app/navigation";
import {
  Shield,
  Search,
  Plus,
  Trash2,
  X,
  ChevronRight,
  UserCheck,
} from "lucide-svelte";

let { data }: { data: { orgId: string } } = $props();

interface PermissionRule {
  role: string;
  document_type: string;
  level: number;
  only_if_creator: boolean;
  actions: string[];
}

// Predefined lists for inputs
const standardDocTypes = [
  "Department",
  "Assessment Result",
  "Grade",
  "User",
  "Class",
  "Subject",
  "Schedule",
  "Attendance",
  "Behavior",
  "Finance",
  "Document",
  "Organization",
  "Enrollment",
  "Exam",
  "Notification",
  "Dashboard",
  "Audit",
  "ImportExport",
];

const standardRoles = [
  "Academics User",
  "principal",
  "teacher",
  "student",
  "parent",
  "academic_supervisor",
  "registrar",
  "finance_officer",
];

// UI States
let rules = $state<PermissionRule[]>([]);
let isLoading = $state(false);
let hasUnsavedChanges = $state(false);
let initialRolesList = $state<string[]>([]);

// Filters
let docTypeFilter = $state("");
let roleFilter = $state("");

// Modals
let showAddRuleModal = $state(false);
let showSetUserPermissionsModal = $state(false);

// Users List (for Set User Permissions)
let usersList = $state<any[]>([]);
let userSearchQuery = $state("");

// Add Rule Form States
let newRuleDocType = $state("");
let newRuleRole = $state("Academics User");
let newRuleLevel = $state(0);
let newRuleOnlyIfCreator = $state(false);
let newRuleActions = $state<string[]>([]);

// Lifecycle
onMount(async () => {
  await loadPermissions();
  await loadUsers();
});

async function loadPermissions() {
  isLoading = true;
  try {
    const rawRules = await commands.permissions.getRolePermissions();
    rules = rawRules.map((r: any) => ({
      role: r.role,
      document_type: r.document_type,
      level: r.level,
      only_if_creator: r.only_if_creator,
      actions: r.actions || [],
    }));
    initialRolesList = Array.from(new Set(rules.map((r) => r.role)));
    hasUnsavedChanges = false;
  } catch (e) {
    console.error("Failed to load permissions", e);
    toast.error(
      getLocale() === "ar"
        ? "فشل تحميل الصلاحيات من قاعدة البيانات"
        : "Failed to load permissions from database",
    );
  } finally {
    isLoading = false;
  }
}

async function loadUsers() {
  try {
    if (data.orgId) {
      usersList = await commands.users.getUsers(data.orgId);
    }
  } catch (e) {
    console.error("Failed to load users list", e);
  }
}

// Group and save rules
async function saveChanges() {
  isLoading = true;
  try {
    const rulesByRole: Record<string, PermissionRule[]> = {};
    for (const rule of rules) {
      if (!rulesByRole[rule.role]) {
        rulesByRole[rule.role] = [];
      }
      rulesByRole[rule.role].push(rule);
    }

    // Save for all roles that had rules or currently have rules
    const rolesToSave = new Set([
      ...initialRolesList,
      ...rules.map((r) => r.role),
    ]);
    for (const r of rolesToSave) {
      const roleRules = rulesByRole[r] || [];
      await commands.permissions.saveRolePermissions(r, roleRules);
    }

    toast.success(
      getLocale() === "ar"
        ? "تم حفظ التغييرات بنجاح!"
        : "Permissions updated and saved successfully!",
    );
    await loadPermissions();
  } catch (e) {
    console.error("Failed to save permissions", e);
    toast.error(
      getLocale() === "ar"
        ? "فشل في حفظ التحديثات في قاعدة البيانات"
        : "Failed to save permissions to database",
    );
  } finally {
    isLoading = false;
  }
}

function discardChanges() {
  loadPermissions();
  toast.info(
    getLocale() === "ar" ? "تم التراجع عن التغييرات" : "Changes discarded",
  );
}

// Filtered Rules
const filteredRules = $derived(
  rules.filter((r) => {
    const docMatch =
      !docTypeFilter ||
      r.document_type.toLowerCase().includes(docTypeFilter.toLowerCase());
    const roleMatch =
      !roleFilter || r.role.toLowerCase().includes(roleFilter.toLowerCase());
    return docMatch && roleMatch;
  }),
);

// Filtered Users (for user permissions list)
const filteredUsers = $derived(
  usersList.filter(
    (u) =>
      u.full_name?.toLowerCase().includes(userSearchQuery.toLowerCase()) ||
      u.email?.toLowerCase().includes(userSearchQuery.toLowerCase()),
  ),
);

// Actions per Doc Type
function getActionsForDocType(docType: string): string[] {
  const dt = docType.toLowerCase();
  if (dt === "assessment result" || dt === "assessment") {
    return [
      "select",
      "read",
      "write",
      "create",
      "delete",
      "submit",
      "cancel",
      "amend",
      "print",
      "email",
      "report",
      "import",
      "export",
      "share",
    ];
  }
  return [
    "select",
    "read",
    "write",
    "create",
    "delete",
    "print",
    "email",
    "report",
    "import",
    "export",
    "share",
  ];
}

function capitalizeFirstLetter(str: string): string {
  if (!str) return "";
  return str.charAt(0).toUpperCase() + str.slice(1);
}

// Manage Rules
function addRule() {
  if (!newRuleDocType.trim()) {
    toast.error(
      getLocale() === "ar"
        ? "يرجى إدخال نوع المستند"
        : "Please input a document type",
    );
    return;
  }
  if (!newRuleRole.trim()) {
    toast.error(
      getLocale() === "ar" ? "يرجى إدخال الدور" : "Please input a role",
    );
    return;
  }

  // Check duplicate
  const exists = rules.some(
    (r) =>
      r.role.toLowerCase() === newRuleRole.toLowerCase() &&
      r.document_type.toLowerCase() === newRuleDocType.toLowerCase(),
  );
  if (exists) {
    toast.error(
      getLocale() === "ar"
        ? "هذه القاعدة موجودة بالفعل!"
        : "This rule already exists!",
    );
    return;
  }

  rules = [
    ...rules,
    {
      role: newRuleRole,
      document_type: newRuleDocType,
      level: newRuleLevel,
      only_if_creator: newRuleOnlyIfCreator,
      actions: newRuleActions,
    },
  ];

  hasUnsavedChanges = true;
  showAddRuleModal = false;

  // Reset Form
  newRuleDocType = "";
  newRuleOnlyIfCreator = false;
  newRuleActions = [];
  toast.success(
    getLocale() === "ar"
      ? "تمت إضافة القاعدة بنجاح"
      : "Rule added successfully",
  );
}

function deleteRule(index: number) {
  const deleted = filteredRules[index];
  rules = rules.filter(
    (r) =>
      !(r.role === deleted.role && r.document_type === deleted.document_type),
  );
  hasUnsavedChanges = true;
  toast.success(getLocale() === "ar" ? "تم حذف القاعدة" : "Rule deleted");
}

function toggleAction(ruleIndex: number, action: string) {
  const rule = filteredRules[ruleIndex];
  if (rule.actions.includes(action)) {
    rule.actions = rule.actions.filter((a) => a !== action);
  } else {
    rule.actions = [...rule.actions, action];
  }
  hasUnsavedChanges = true;
}
</script>

<svelte:head>
  <title>
    {m.settings_permissions_window_title()} | Orbis
  </title>
</svelte:head>

<div class="h-full w-full overflow-hidden flex flex-col font-sans text-start bg-background relative">
  <div class="flex-1 overflow-y-auto scrollbar-hide">
    <div class="p-6 pb-28 flex flex-col gap-6">
      <!-- Header Area (Search, Filters, and Buttons) -->
      <div class="flex flex-col lg:flex-row lg:items-center justify-between gap-4 font-sans border-b border-border/40 pb-5 select-none">
        <div class="flex flex-1 flex-col sm:flex-row gap-3 items-center w-full">
          <!-- Document Type Search -->
          <div class="relative flex-1 w-full max-w-sm">
            <Search size={14} class="absolute top-1/2 -translate-y-1/2 text-muted-foreground ltr:left-3 rtl:right-3" />
            <Input
              type="text"
              bind:value={docTypeFilter}
              placeholder={m.settings_permissions_search_doc_type()}
              class="w-full ltr:pl-9 rtl:pr-9 h-9 text-xs bg-background/50"
            />
          </div>

          <!-- Role Filter -->
          <div class="w-full sm:w-48 text-start">
            <Select type="single" value={roleFilter} onValueChange={(v) => (roleFilter = v ?? "")}>
              <SelectTrigger class="h-9 w-full text-xs bg-background/50">
                {roleFilter ? m.settings_permissions_filter_role({ role: roleFilter }) : m.settings_permissions_filter_role_all()}
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="">{m.settings_permissions_filter_role_all()}</SelectItem>
                {#each standardRoles as role}
                  <SelectItem value={role}>{role}</SelectItem>
                {/each}
              </SelectContent>
            </Select>
          </div>
        </div>

        <div class="flex items-center gap-3 shrink-0">
          <Button
            variant="outline"
            size="sm"
            class="h-9 text-xs"
            onclick={() => showSetUserPermissionsModal = true}
          >
            <UserCheck size={14} class={getLocale() === "ar" ? 'ml-1.5' : 'mr-1.5'} />
            {getLocale() === "ar" ? "تعيين صلاحيات المستخدمين" : "Set User Permissions"}
          </Button>

          <Button
            variant="lime"
            size="sm"
            class="h-9 text-xs font-semibold"
            onclick={() => showAddRuleModal = true}
          >
            <Plus size={14} class={getLocale() === "ar" ? 'ml-1.5' : 'mr-1.5'} />
            {getLocale() === "ar" ? "إضافة قاعدة جديدة" : "+ Add A New Rule"}
          </Button>
        </div>
      </div>

      <!-- Rules Table Content -->
      <div class="w-full">
        <div class="overflow-x-auto">
      <Table>
        <TableHeader class="bg-muted/40 select-none">
          <TableRow>
            <TableHead class="text-start font-bold text-xs w-[20%]">{getLocale() === "ar" ? "نوع المستند" : "Document Type"}</TableHead>
            <TableHead class="text-start font-bold text-xs w-[20%]">{getLocale() === "ar" ? "الدور" : "Role"}</TableHead>
            <TableHead class="text-start font-bold text-xs w-[10%]">{getLocale() === "ar" ? "المستوى" : "Level"}</TableHead>
            <TableHead class="text-start font-bold text-xs w-[45%]">{getLocale() === "ar" ? "الصلاحيات" : "Permissions"}</TableHead>
            <TableHead class="text-center w-[5%]"></TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {#if isLoading}
            <TableRow>
              <TableCell colspan="5" class="py-12 text-center text-xs text-muted-foreground">
                {getLocale() === "ar" ? "جاري تحميل القواعد..." : "Loading rules..."}
              </TableCell>
            </TableRow>
          {:else if filteredRules.length === 0}
            <TableRow>
              <TableCell colspan="5" class="py-12 text-center text-xs text-muted-foreground">
                {getLocale() === "ar" ? "لا توجد قواعد مطابقة للفلاتر" : "No rules match the filters."}
              </TableCell>
            </TableRow>
          {:else}
            {#each filteredRules as rule, idx}
              <TableRow class="align-top hover:bg-accent/5">
                <!-- Document Type -->
                <TableCell class="text-sm font-bold text-foreground text-start">
                  {rule.document_type}
                </TableCell>
                
                <!-- Role & Conditions -->
                <TableCell class="text-start">
                  <span class="text-xs text-foreground font-semibold block">{rule.role}</span>
                  <label class="inline-flex items-center gap-2 mt-2.5 cursor-pointer group select-none">
                    <Checkbox
                      checked={rule.only_if_creator}
                      onCheckedChange={(checked) => {
                        rule.only_if_creator = Boolean(checked);
                        hasUnsavedChanges = true;
                      }}
                    />
                    <span class="text-[10px] text-muted-foreground group-hover:text-foreground transition-colors">
                      {getLocale() === "ar" ? "فقط إذا كان المنشئ" : "Only If Creator"}
                    </span>
                  </label>
                </TableCell>

                <!-- Level -->
                <TableCell class="text-xs text-muted-foreground font-mono text-start">
                  {rule.level}
                </TableCell>

                <!-- Permissions Actions Grid -->
                <TableCell>
                  <div class="grid grid-cols-3 gap-x-8 gap-y-3.5">
                    {#each getActionsForDocType(rule.document_type) as action}
                      <label class="flex items-center gap-2.5 text-xs text-muted-foreground hover:text-foreground cursor-pointer select-none font-medium transition-colors">
                        <Checkbox
                          checked={rule.actions.includes(action)}
                          onCheckedChange={() => toggleAction(idx, action)}
                        />
                        <span>{capitalizeFirstLetter(action)}</span>
                      </label>
                    {/each}
                  </div>
                </TableCell>

                <!-- Delete Button -->
                <TableCell class="text-center">
                  <Button
                    variant="ghost"
                    size="icon"
                    onclick={() => deleteRule(idx)}
                    title={getLocale() === "ar" ? "حذف القاعدة" : "Delete Rule"}
                  >
                    <Trash2 size={14} class="text-muted-foreground hover:text-destructive" />
                  </Button>
                </TableCell>
              </TableRow>
            {/each}
          {/if}
        </TableBody>
      </Table>
    </div>
  </div>
  </div>
  </div>

  <!-- Floating Save Changes Banner -->
  {#if hasUnsavedChanges}
    <div 
      in:slide={{ duration: 250 }} 
      out:slide={{ duration: 200 }} 
      class="absolute bottom-6 left-1/2 -translate-x-1/2 w-11/12 max-w-2xl z-50 rounded-2xl border border-border bg-card/95 backdrop-blur-xl shadow-2xl p-4 flex items-center justify-between gap-4"
    >
      <div class="flex items-center gap-3 text-start">
        <div class="w-8 h-8 rounded-lg bg-primary/10 flex items-center justify-center text-primary">
          <Shield size={16} />
        </div>
        <div class="flex flex-col">
          <span class="text-xs font-bold text-foreground">
            {getLocale() === "ar" ? "تغييرات غير محفوظة" : "Unsaved Rule Updates"}
          </span>
          <span class="text-[10px] text-muted-foreground mt-0.5">
            {getLocale() === "ar" 
              ? "لقد قمت بتعديل مصفوفة قواعد الصلاحيات." 
              : "You modified the permission rule configurations."}
          </span>
        </div>
      </div>

      <div class="flex items-center gap-2">
        <Button variant="ghost" size="sm" class="text-xs" onclick={discardChanges} disabled={isLoading}>
          {getLocale() === "ar" ? "تراجع" : "Discard"}
        </Button>
        <Button variant="lime" size="sm" class="text-xs font-bold" onclick={saveChanges} disabled={isLoading}>
          {isLoading ? (getLocale() === "ar" ? "جاري الحفظ..." : "Saving...") : (getLocale() === "ar" ? "حفظ التغييرات" : "Save Changes")}
        </Button>
      </div>
    </div>
  {/if}
</div>

<!-- Create Custom Rule Modal -->
<Dialog bind:open={showAddRuleModal}>
  <DialogContent class="max-w-lg text-start">
    <DialogHeader>
      <DialogTitle>{getLocale() === "ar" ? "إضافة قاعدة صلاحيات جديدة" : "Add A New Permission Rule"}</DialogTitle>
    </DialogHeader>

    <div class="flex flex-col gap-4 py-2">
      <!-- Document Type Selector -->
      <div class="flex flex-col gap-1.5 text-start">
        <Label for="rule_doc_type" class="text-xs font-medium text-muted-foreground">
          {getLocale() === "ar" ? "نوع المستند" : "Document Type"}
        </Label>
        <Select type="single" value={newRuleDocType} onValueChange={(v) => (newRuleDocType = v ?? "")}>
          <SelectTrigger class="h-9 w-full text-xs">
            {newRuleDocType || "Select Document Type"}
          </SelectTrigger>
          <SelectContent>
            {#each standardDocTypes as type}
              <SelectItem value={type}>{type}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>

      <!-- Role Selector -->
      <div class="flex flex-col gap-1.5 text-start">
        <Label for="rule_role" class="text-xs font-medium text-muted-foreground">
          {getLocale() === "ar" ? "الدور الوظيفي" : "Job Role"}
        </Label>
        <Select type="single" value={newRuleRole} onValueChange={(v) => (newRuleRole = v ?? "Academics User")}>
          <SelectTrigger class="h-9 w-full text-xs">
            {newRuleRole}
          </SelectTrigger>
          <SelectContent>
            {#each standardRoles as role}
              <SelectItem value={role}>{role}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>

      <div class="grid grid-cols-2 gap-4 items-end">
        <!-- Level -->
        <div class="flex flex-col gap-1.5 text-start">
          <Label for="rule_level" class="text-xs font-medium text-muted-foreground">
            {getLocale() === "ar" ? "المستوى" : "Level"}
          </Label>
          <Input
            id="rule_level"
            type="number"
            bind:value={newRuleLevel}
            class="h-9 text-xs"
          />
        </div>

        <!-- Creator Condition -->
        <div class="pb-2">
          <label class="inline-flex items-center gap-2 cursor-pointer select-none">
            <Checkbox
              checked={newRuleOnlyIfCreator}
              onCheckedChange={(checked) => (newRuleOnlyIfCreator = Boolean(checked))}
            />
            <span class="text-xs text-muted-foreground">
              {getLocale() === "ar" ? "فقط إذا كان المنشئ" : "Only If Creator"}
            </span>
          </label>
        </div>
      </div>

      <!-- Actions Selector Checklist -->
      <div class="flex flex-col gap-2 border-t border-border/40 pt-4 mt-2 text-start">
        <span class="text-xs font-semibold text-muted-foreground mb-1">
          {getLocale() === "ar" ? "الإجراءات المسموحة" : "Allowed Actions"}
        </span>
        <div class="grid grid-cols-3 gap-2.5 max-h-40 overflow-y-auto pr-1">
          {#each getActionsForDocType(newRuleDocType || "Default") as action}
            <label class="inline-flex items-center gap-2 text-xs text-muted-foreground hover:text-foreground cursor-pointer select-none font-medium">
              <Checkbox
                checked={newRuleActions.includes(action)}
                onCheckedChange={(checked) => {
                  if (checked) {
                    newRuleActions = [...newRuleActions, action];
                  } else {
                    newRuleActions = newRuleActions.filter(a => a !== action);
                  }
                }}
              />
              <span>{capitalizeFirstLetter(action)}</span>
            </label>
          {/each}
        </div>
      </div>
    </div>

    <DialogFooter class="border-t border-border pt-4 mt-2">
      <Button variant="ghost" size="sm" onclick={() => showAddRuleModal = false}>
        {getLocale() === "ar" ? "إلغاء" : "Cancel"}
      </Button>
      <Button variant="lime" size="sm" class="font-bold" onclick={addRule}>
        {getLocale() === "ar" ? "إضافة القاعدة" : "Add Rule"}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>

<!-- Set User Permissions Modal Dialog -->
<Dialog bind:open={showSetUserPermissionsModal}>
  <DialogContent class="max-w-md text-start">
    <DialogHeader>
      <DialogTitle>{getLocale() === "ar" ? "صلاحيات مستخدم محدد" : "User Permissions Settings"}</DialogTitle>
      <DialogDescription>
        {getLocale() === "ar"
          ? "اختر مستخدماً من القائمة لتعديل صلاحيات الاستثناء الفردية المخصصة له."
          : "Select a user from the directory below to configure custom individual override rules."}
      </DialogDescription>
    </DialogHeader>

    <div class="relative w-full my-2">
      <Search size={14} class="absolute top-1/2 -translate-y-1/2 text-muted-foreground rtl:right-3 ltr:left-3" />
      <Input
        type="text"
        bind:value={userSearchQuery}
        placeholder={getLocale() === "ar" ? "ابحث عن موظف أو مستخدم..." : "Search user directory..."}
        class="w-full rtl:pr-9 ltr:pl-9 h-9 text-xs"
      />
    </div>

    <!-- Users Scrollable list -->
    <div class="flex flex-col gap-1.5 max-h-60 overflow-y-auto pr-1">
      {#if filteredUsers.length === 0}
        <span class="text-xs text-muted-foreground text-center py-6">
          {getLocale() === "ar" ? "لا يوجد مستخدمون" : "No users found"}
        </span>
      {:else}
        {#each filteredUsers as u}
          <button
            onclick={() => {
              showSetUserPermissionsModal = false;
              goto(`/${data.orgId}/settings/users/${u.id}`);
            }}
            class="w-full text-start p-2.5 rounded-lg border border-border bg-background hover:bg-accent transition-all flex items-center justify-between group cursor-pointer"
          >
            <div class="flex flex-col gap-0.5">
              <span class="text-xs font-semibold text-foreground group-hover:text-primary transition-colors">{u.full_name}</span>
              <span class="text-[9px] text-muted-foreground font-mono">{u.email}</span>
            </div>
            <div class="flex items-center gap-1.5">
              <Badge variant="secondary" class="text-[9px] font-semibold">
                {u.primary_role}
              </Badge>
              <ChevronRight size={12} class="text-muted-foreground group-hover:text-foreground transition-colors" />
            </div>
          </button>
        {/each}
      {/if}
    </div>

    <DialogFooter class="border-t border-border pt-4">
      <Button variant="ghost" size="sm" onclick={() => showSetUserPermissionsModal = false}>
        {getLocale() === "ar" ? "إغلاق" : "Close"}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
