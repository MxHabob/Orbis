<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
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
import { commands } from "$lib/core/ipc";
import { goto } from "$app/navigation";
import SearchField from "$lib/shared/components/search-field.svelte";
import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";
import {
  Search,
  Plus,
  Trash2,
  X,
  ChevronRight,
  UserCheck,
} from "@lucide/svelte";

let { data }: { data: { orgId: string } } = $props();

interface PermissionRule {
  role: string;
  document_type: string;
  level: number;
  only_if_creator: boolean;
  actions: string[];
}

interface PermissionMetadata {
  roles: string[];
  document_types: string[];
  actions_by_doc_type: Record<string, string[]>;
}

const metadata: PermissionMetadata = {
  roles: [
    "super_admin",
    "admin",
    "principal",
    "teacher",
    "staff",
    "registrar",
    "accountant",
    "student",
    "guardian",
  ],
  document_types: [
    "Organization",
    "User",
    "Period",
    "GradeEntry",
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
    "ImportExport",
    "Exam",
    "Template",
  ],
  actions_by_doc_type: {
    Organization: ["read", "write"],
    User: ["read", "write", "delete", "role_write", "password_write"],
    Period: ["read", "write", "activate", "lock", "unlock"],
    GradeEntry: ["manage"],
    OrgUnit: ["read", "write", "delete"],
    Class: ["read", "write", "delete"],
    Subject: ["read", "write", "delete"],
    Schedule: ["read", "write", "delete"],
    Assessment: ["read", "write", "delete"],
    Guardian: ["read", "write", "delete"],
    Document: ["read", "write", "delete"],
    Enrollment: ["read", "write", "promote", "transfer"],
    Grade: ["read", "write", "analytics", "report"],
    Attendance: ["read", "write", "analytics", "report"],
    Behavior: ["read", "write", "delete", "ai"],
    Notification: ["read", "write"],
    Dashboard: ["read"],
    Audit: ["read"],
    Finance: ["read", "write"],
    ImportExport: ["import", "export"],
    Exam: [
      "read",
      "write",
      "activate",
      "delete",
      "generate_seating",
      "read_seating",
      "generate_secret",
      "read_secret",
      "reveal_secret",
      "read_hall",
      "write_hall",
      "allocate_hall",
      "read_proctor",
      "assign_proctor",
      "read_order",
      "rebuild_order",
      "freeze_order",
      "read_schedule",
      "write_schedule",
    ],
    Template: [
      "read",
      "write",
      "publish",
      "delete",
      "import",
      "export",
      "generate_single",
      "generate_batch",
      "generate_download",
      "asset_upload",
      "asset_delete",
      "component_write",
      "component_delete",
    ],
  },
};

const standardRoles = metadata.roles;
const standardDocTypes = metadata.document_types;
const actionsByDocType = metadata.actions_by_doc_type;

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
let newRuleRole = $state("super_admin");
let newRuleLevel = $state(0);
let newRuleOnlyIfCreator = $state(false);
let newRuleActions = $state<string[]>([]);

function parseActions(actions: any): string[] {
  if (Array.isArray(actions)) return actions;
  if (typeof actions === "string") {
    try {
      const parsed = JSON.parse(actions);
      if (Array.isArray(parsed)) return parsed;
    } catch {
      if (actions.trim()) return actions.split(",").map((s: string) => s.trim()).filter(Boolean);
    }
  }
  return [];
}

const availableRoles = $derived(
  Array.from(new Set([...standardRoles, ...rules.map((r) => r.role)])),
);

// Lifeperiod
onMount(async () => {
  await loadPermissions();
  await loadUsers();
});

async function loadPermissions() {
  isLoading = true;
  try {
    const rawRules: any[] = await commands.iam.getPermissions(null);
    rules = (rawRules || []).map((r) => ({
      role: r.role,
      document_type: r.document_type,
      level: r.level ?? 0,
      only_if_creator: Boolean(r.only_if_creator),
      actions: parseActions(r.actions),
    }));
    initialRolesList = Array.from(new Set(rules.map((r) => r.role)));
    hasUnsavedChanges = false;
  } catch (e) {
    console.error("Failed to load permissions", e);
    toast.error(m.settings_permissions_load_error());
  } finally {
    isLoading = false;
  }
}

async function loadUsers() {
  try {
    if (data.orgId) {
      usersList = await commands.iam.listUsers({});
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
      await commands.iam.savePermissions(r, roleRules);
    }

    toast.success(m.settings_permissions_save_success());
    await loadPermissions();
  } catch (e) {
    console.error("Failed to save permissions", e);
    toast.error(m.settings_permissions_save_error());
  } finally {
    isLoading = false;
  }
}

function discardChanges() {
  loadPermissions();
  toast.info(m.settings_permissions_discarded());
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
  if (!docType) return [];
  const matchKey = Object.keys(actionsByDocType).find(
    (k) => k.toLowerCase() === docType.toLowerCase()
  );
  if (matchKey) {
    return actionsByDocType[matchKey];
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
    toast.error(m.settings_permissions_input_doctype_error());
    return;
  }
  if (!newRuleRole.trim()) {
    toast.error(m.settings_permissions_input_role_error());
    return;
  }

  // Check duplicate
  const exists = rules.some(
    (r) =>
      r.role.toLowerCase() === newRuleRole.toLowerCase() &&
      r.document_type.toLowerCase() === newRuleDocType.toLowerCase(),
  );
  if (exists) {
    toast.error(m.settings_permissions_rule_exists_error());
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
  toast.success(m.settings_permissions_rule_added_success());
}

function deleteRule(index: number) {
  const deleted = filteredRules[index];
  rules = rules.filter(
    (r) =>
      !(r.role === deleted.role && r.document_type === deleted.document_type),
  );
  hasUnsavedChanges = true;
  toast.success(m.settings_permissions_rule_deleted_success());
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
          <SearchField
            placeholder={m.settings_permissions_search_doc_type()}
            bind:value={docTypeFilter}
            class="flex-1 bg-background/50 max-w-sm"
          />

          <!-- Role Filter -->
          <div class="w-full sm:w-48 text-start">
            <Select type="single" value={roleFilter} onValueChange={(v) => (roleFilter = v ?? "")}>
              <SelectTrigger class="h-9 w-full text-xs bg-background/50">
                {roleFilter ? m.settings_permissions_filter_role({ role: roleFilter }) : m.settings_users_filter_role_all()}
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="">{m.settings_users_filter_role_all()}</SelectItem>
                {#each availableRoles as role}
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
            <UserCheck size={14} class="me-1.5" />
            {m.settings_permissions_btn_set_user()}
          </Button>

          <Button
            variant="lime"
            size="sm"
            class="h-9 text-xs font-semibold"
            onclick={() => showAddRuleModal = true}
          >
            <Plus size={14} class="me-1.5" />
            {m.settings_permissions_btn_add_rule()}
          </Button>
        </div>
      </div>

      <!-- Rules Table Content -->
      <div class="w-full">
        <div class="overflow-x-auto">
      <Table>
        <TableHeader class="bg-muted/40 select-none">
          <TableRow>
            <TableHead class="text-start font-bold text-xs w-[20%]">{m.settings_permissions_col_doc_type()}</TableHead>
            <TableHead class="text-start font-bold text-xs w-[20%]">{m.common_user_form_role()}</TableHead>
            <TableHead class="text-start font-bold text-xs w-[10%]">{m.academics_exams_level()}</TableHead>
            <TableHead class="text-start font-bold text-xs w-[45%]">{m.settings_permissions_col_permissions()}</TableHead>
            <TableHead class="text-center w-[5%]"></TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {#if isLoading}
            <TableRow>
              <TableCell colspan={5} class="py-12 text-center text-xs text-muted-foreground">
                {m.settings_permissions_loading()}
              </TableCell>
            </TableRow>
          {:else if filteredRules.length === 0}
            <TableRow>
              <TableCell colspan={5} class="py-12 text-center text-xs text-muted-foreground">
                {m.settings_permissions_empty_rules()}
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
                      {m.settings_permissions_only_creator()}
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
                    title={m.settings_permissions_delete_rule_title()}
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

  <SettingsBottomBar
    visible={hasUnsavedChanges}
    isDirty={hasUnsavedChanges}
    isSaving={isLoading}
    message={m.settings_permissions_unsaved_title()}
    onsave={saveChanges}
    onreset={discardChanges}
  />
</div>

<!-- Create Custom Rule Modal -->
<Dialog bind:open={showAddRuleModal}>
  <DialogContent class="max-w-lg text-start">
    <DialogHeader>
      <DialogTitle>{m.settings_permissions_dialog_add_title()}</DialogTitle>
    </DialogHeader>

    <div class="flex flex-col gap-4 py-2">
      <!-- Document Type Selector -->
      <div class="flex flex-col gap-1.5 text-start">
        <Label for="rule_doc_type" class="text-xs font-medium text-muted-foreground">
          {m.settings_permissions_col_doc_type()}
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
          {m.settings_permissions_label_role()}
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
            {m.academics_exams_level()}
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
              {m.settings_permissions_only_creator()}
            </span>
          </label>
        </div>
      </div>

      <!-- Actions Selector Checklist -->
      <div class="flex flex-col gap-2 border-t border-border/40 pt-4 mt-2 text-start">
        <span class="text-xs font-semibold text-muted-foreground mb-1">
          {m.settings_permissions_label_allowed_actions()}
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
        {m.common_action_cancel()}
      </Button>
      <Button variant="lime" size="sm" class="font-bold" onclick={addRule}>
        {m.settings_permissions_btn_add()}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>

<!-- Set User Permissions Modal Dialog -->
<Dialog bind:open={showSetUserPermissionsModal}>
  <DialogContent class="max-w-md text-start">
    <DialogHeader>
      <DialogTitle>{m.settings_permissions_dialog_user_title()}</DialogTitle>
      <DialogDescription>
        {m.settings_permissions_dialog_user_desc()}
      </DialogDescription>
    </DialogHeader>

    <div class="my-2">
      <SearchField
        placeholder={m.settings_permissions_search_user_placeholder()}
        bind:value={userSearchQuery}
        class="max-w-none w-full"
      />
    </div>

    <!-- Users Scrollable list -->
    <div class="flex flex-col gap-1.5 max-h-60 overflow-y-auto pr-1">
      {#if filteredUsers.length === 0}
        <span class="text-xs text-muted-foreground text-center py-6">
          {m.settings_users_empty()}
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
        {m.common_titlebar_close()}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
