<script lang="ts">
  import { onMount } from "svelte";
  import { fade, slide } from "svelte/transition";
  import { Button } from "@orbis/ui-svelte/button";
  import { Input } from "@orbis/ui-svelte/input";
  import { toast } from "svelte-sonner";
  import { getLocale } from "$lib/paraglide/runtime";
  import { commands } from "$lib/services/ipc";
  import { goto } from "$app/navigation";
  import {
    Shield,
    Search,
    Plus,
    Users,
    Check,
    Trash2,
    X,
    ChevronRight,
    HelpCircle,
    UserCheck
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
    "ImportExport"
  ];

  const standardRoles = [
    "Academics User",
    "principal",
    "teacher",
    "student",
    "parent",
    "academic_supervisor",
    "registrar",
    "finance_officer"
  ];

  // UI States
  let rules = $state<PermissionRule[]>([]);
  let isLoading = $state(false);
  let hasUnsavedChanges = $state(false);
  let initialRolesList = $state<string[]>([]);

  // Filters
  let docTypeFilter = $state("");
  let roleFilter = $state("Academics User");

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
      initialRolesList = Array.from(new Set(rules.map(r => r.role)));
      hasUnsavedChanges = false;
    } catch (e) {
      console.error("Failed to load permissions", e);
      toast.error(
        getLocale() === "ar"
          ? "فشل تحميل الصلاحيات من قاعدة البيانات"
          : "Failed to load permissions from database"
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
      const rolesToSave = new Set([...initialRolesList, ...rules.map(r => r.role)]);
      for (const r of rolesToSave) {
        const roleRules = rulesByRole[r] || [];
        await commands.permissions.saveRolePermissions(r, roleRules);
      }

      toast.success(
        getLocale() === "ar"
          ? "تم حفظ التغييرات بنجاح!"
          : "Permissions updated and saved successfully!"
      );
      await loadPermissions();
    } catch (e) {
      console.error("Failed to save permissions", e);
      toast.error(
        getLocale() === "ar"
          ? "فشل في حفظ التحديثات في قاعدة البيانات"
          : "Failed to save permissions to database"
      );
    } finally {
      isLoading = false;
    }
  }

  function discardChanges() {
    loadPermissions();
    toast.info(
      getLocale() === "ar" ? "تم التراجع عن التغييرات" : "Changes discarded"
    );
  }

  // Filtered Rules
  const filteredRules = $derived(
    rules.filter(r => {
      const docMatch = !docTypeFilter || r.document_type.toLowerCase().includes(docTypeFilter.toLowerCase());
      const roleMatch = !roleFilter || r.role.toLowerCase().includes(roleFilter.toLowerCase());
      return docMatch && roleMatch;
    })
  );

  // Filtered Users (for user permissions list)
  const filteredUsers = $derived(
    usersList.filter(u => 
      u.full_name?.toLowerCase().includes(userSearchQuery.toLowerCase()) || 
      u.email?.toLowerCase().includes(userSearchQuery.toLowerCase())
    )
  );

  // Actions per Doc Type
  function getActionsForDocType(docType: string): string[] {
    const dt = docType.toLowerCase();
    if (dt === "assessment result" || dt === "assessment") {
      return [
        "select", "read", "write",
        "create", "delete", "submit",
        "cancel", "amend", "print",
        "email", "report", "import",
        "export", "share"
      ];
    }
    return [
      "select", "read", "write",
      "create", "delete", "print",
      "email", "report", "import",
      "export", "share"
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
        getLocale() === "ar" ? "يرجى إدخال نوع المستند" : "Please input a document type"
      );
      return;
    }
    if (!newRuleRole.trim()) {
      toast.error(
        getLocale() === "ar" ? "يرجى إدخال الدور" : "Please input a role"
      );
      return;
    }

    // Check duplicate
    const exists = rules.some(
      r => r.role.toLowerCase() === newRuleRole.toLowerCase() && 
           r.document_type.toLowerCase() === newRuleDocType.toLowerCase()
    );
    if (exists) {
      toast.error(
        getLocale() === "ar" ? "هذه القاعدة موجودة بالفعل!" : "This rule already exists!"
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
      }
    ];

    hasUnsavedChanges = true;
    showAddRuleModal = false;

    // Reset Form
    newRuleDocType = "";
    newRuleOnlyIfCreator = false;
    newRuleActions = [];
    toast.success(
      getLocale() === "ar" ? "تمت إضافة القاعدة بنجاح" : "Rule added successfully"
    );
  }

  function deleteRule(index: number) {
    const deleted = filteredRules[index];
    rules = rules.filter(r => !(r.role === deleted.role && r.document_type === deleted.document_type));
    hasUnsavedChanges = true;
    toast.success(
      getLocale() === "ar" ? "تم حذف القاعدة" : "Rule deleted"
    );
  }

  function toggleAction(ruleIndex: number, action: string) {
    const rule = filteredRules[ruleIndex];
    if (rule.actions.includes(action)) {
      rule.actions = rule.actions.filter(a => a !== action);
    } else {
      rule.actions = [...rule.actions, action];
    }
    hasUnsavedChanges = true;
  }
</script>

<svelte:head>
  <title>
    {getLocale() === "ar" ? "مدير صلاحيات الأدوار" : "Role Permissions Manager"} | Orbis
  </title>
</svelte:head>

<div class="h-full w-full overflow-hidden flex flex-col font-sans text-start bg-[#0f0f11] text-foreground relative">
  <!-- Header Area -->
  <div class="p-6 border-b border-border/20 bg-[#131316] flex items-center justify-between gap-4">
    <div class="flex flex-col gap-1 text-start">
      <h1 class="text-xl font-bold tracking-tight text-white flex items-center gap-2">
        <Shield size={20} class="text-[#a3e635]" />
        <span>{getLocale() === "ar" ? "مدير صلاحيات الأدوار" : "Role Permissions Manager"}</span>
      </h1>
    </div>

    <div class="flex items-center gap-3">
      <Button
        variant="ghost"
        class="text-xs h-9 px-4 font-semibold text-gray-300 hover:text-white bg-[#1a1a1f] border border-border/20"
        onclick={() => showSetUserPermissionsModal = true}
      >
        <UserCheck size={14} class={getLocale() === "ar" ? 'ml-1.5' : 'mr-1.5'} />
        {getLocale() === "ar" ? "تعيين صلاحيات المستخدمين" : "Set User Permissions"}
      </Button>

      <Button
        variant="lime"
        class="text-xs h-9 px-4 font-semibold bg-[#a3e635] text-black hover:bg-[#bef264]"
        onclick={() => showAddRuleModal = true}
      >
        <Plus size={14} class={getLocale() === "ar" ? 'ml-1.5' : 'mr-1.5'} />
        {getLocale() === "ar" ? "إضافة قاعدة جديدة" : "+ Add A New Rule"}
      </Button>
    </div>
  </div>

  <!-- Filters Row -->
  <div class="px-6 py-4 bg-[#131316]/50 border-b border-border/10 flex items-center gap-4">
    <div class="flex items-center gap-2 bg-[#17171c] px-3 py-1.5 rounded-lg border border-border/20 w-72">
      <span class="text-[10px] text-gray-400 font-bold uppercase tracking-wider min-w-[90px] text-start border-r border-border/10 pr-2">
        {getLocale() === "ar" ? "نوع المستند" : "Document Type"}
      </span>
      <input
        type="text"
        bind:value={docTypeFilter}
        placeholder={getLocale() === "ar" ? "ابحث بالنوع..." : "Search Type..."}
        class="bg-transparent border-none outline-none text-xs text-white placeholder-gray-500 w-full"
      />
    </div>

    <div class="flex items-center gap-2 bg-[#17171c] px-3 py-1.5 rounded-lg border border-border/20 w-72">
      <span class="text-[10px] text-gray-400 font-bold uppercase tracking-wider min-w-[90px] text-start border-r border-border/10 pr-2">
        {getLocale() === "ar" ? "الدور" : "Role"}
      </span>
      <input
        type="text"
        bind:value={roleFilter}
        placeholder={getLocale() === "ar" ? "ابحث بالدور..." : "Search Role..."}
        class="bg-transparent border-none outline-none text-xs text-white placeholder-gray-500 w-full"
      />
    </div>
  </div>

  <!-- Rules Table Content -->
  <div class="flex-1 overflow-y-auto p-6 scrollbar-hide pb-28">
    <div class="bg-[#131316] border border-border/20 rounded-2xl overflow-hidden shadow-xl">
      <div class="overflow-x-auto">
        <table class="w-full border-collapse">
          <thead>
            <tr class="border-b border-border/10 bg-[#17171c]/80 text-[10px] uppercase tracking-wider text-gray-400 font-bold">
              <th class="py-4 px-6 text-start w-[20%]">{getLocale() === "ar" ? "نوع المستند" : "Document Type"}</th>
              <th class="py-4 px-6 text-start w-[20%]">{getLocale() === "ar" ? "الدور" : "Role"}</th>
              <th class="py-4 px-6 text-start w-[10%]">{getLocale() === "ar" ? "المستوى" : "Level"}</th>
              <th class="py-4 px-6 text-start w-[45%]">{getLocale() === "ar" ? "الصلاحيات" : "Permissions"}</th>
              <th class="py-4 px-6 text-center w-[5%]"></th>
            </tr>
          </thead>
          <tbody class="divide-y divide-border/10">
            {#if isLoading}
              <tr>
                <td colspan="5" class="py-12 text-center text-xs text-gray-400">
                  {getLocale() === "ar" ? "جاري تحميل القواعد..." : "Loading rules..."}
                </td>
              </tr>
            {:else if filteredRules.length === 0}
              <tr>
                <td colspan="5" class="py-12 text-center text-xs text-gray-400">
                  {getLocale() === "ar" ? "لا توجد قواعد مطابقة للفلاتر" : "No rules match the filters."}
                </td>
              </tr>
            {:else}
              {#each filteredRules as rule, idx}
                <tr class="hover:bg-[#17171c]/30 transition-colors align-top">
                  <!-- Document Type -->
                  <td class="py-6 px-6 text-sm font-bold text-white text-start">
                    {rule.document_type}
                  </td>
                  
                  <!-- Role & Conditions -->
                  <td class="py-6 px-6 text-start">
                    <span class="text-xs text-gray-300 font-semibold block">{rule.role}</span>
                    <label class="inline-flex items-center gap-2 mt-2.5 cursor-pointer group">
                      <input
                        type="checkbox"
                        checked={rule.only_if_creator}
                        onchange={(e) => {
                          rule.only_if_creator = e.currentTarget.checked;
                          hasUnsavedChanges = true;
                        }}
                        class="h-3.5 w-3.5 rounded border-gray-600 bg-card text-[#a3e635] focus:ring-0 focus:ring-offset-0"
                      />
                      <span class="text-[10px] text-gray-400 group-hover:text-white transition-colors">
                        {getLocale() === "ar" ? "فقط إذا كان المنشئ" : "Only If Creator"}
                      </span>
                    </label>
                  </td>

                  <!-- Level -->
                  <td class="py-6 px-6 text-xs text-gray-300 font-mono text-start">
                    {rule.level}
                  </td>

                  <!-- Permissions Actions Grid -->
                  <td class="py-6 px-6">
                    <div class="grid grid-cols-3 gap-x-8 gap-y-3.5">
                      {#each getActionsForDocType(rule.document_type) as action}
                        <label class="flex items-center gap-2.5 text-xs text-gray-300 hover:text-white cursor-pointer select-none font-medium transition-colors">
                          <input
                            type="checkbox"
                            checked={rule.actions.includes(action)}
                            onchange={() => toggleAction(idx, action)}
                            class="h-3.5 w-3.5 rounded border-gray-600 bg-card text-[#a3e635] focus:ring-0 focus:ring-offset-0"
                          />
                          <span>{capitalizeFirstLetter(action)}</span>
                        </label>
                      {/each}
                    </div>
                  </td>

                  <!-- Delete Button -->
                  <td class="py-6 px-6 text-center">
                    <button
                      onclick={() => deleteRule(idx)}
                      class="p-2 rounded-lg text-gray-500 hover:text-red-500 hover:bg-red-500/10 transition-all cursor-pointer"
                      title={getLocale() === "ar" ? "حذف القاعدة" : "Delete Rule"}
                    >
                      <Trash2 size={14} />
                    </button>
                  </td>
                </tr>
              {/each}
            {/if}
          </tbody>
        </table>
      </div>
    </div>
  </div>

  <!-- Floating Save Changes Glassmorphic Banner -->
  {#if hasUnsavedChanges}
    <div 
      in:slide={{ duration: 250 }} 
      out:slide={{ duration: 200 }} 
      class="absolute bottom-6 left-1/2 -translate-x-1/2 w-11/12 max-w-2xl z-50 rounded-2xl border border-primary/30 bg-[#131316]/90 backdrop-blur-xl shadow-2xl p-4 flex items-center justify-between gap-4"
    >
      <div class="flex items-center gap-3 text-start">
        <div class="w-8 h-8 rounded-lg bg-[#a3e635]/20 flex items-center justify-center text-[#a3e635]">
          <Shield size={16} />
        </div>
        <div class="flex flex-col">
          <span class="text-xs font-bold text-white">
            {getLocale() === "ar" ? "تغييرات غير محفوظة" : "Unsaved Rule Updates"}
          </span>
          <span class="text-[10px] text-gray-400 mt-0.5">
            {getLocale() === "ar" 
              ? "لقد قمت بتعديل مصفوفة قواعد الصلاحيات." 
              : "You modified the permission rule configurations."}
          </span>
        </div>
      </div>

      <div class="flex items-center gap-2">
        <Button variant="ghost" class="text-xs h-8 px-4 text-gray-400 hover:text-white" onclick={discardChanges} disabled={isLoading}>
          {getLocale() === "ar" ? "تراجع" : "Discard"}
        </Button>
        <Button variant="lime" class="text-xs h-8 px-4 font-bold bg-[#a3e635] text-black hover:bg-[#bef264]" onclick={saveChanges} disabled={isLoading}>
          {isLoading ? (getLocale() === "ar" ? "جاري الحفظ..." : "Saving...") : (getLocale() === "ar" ? "حفظ التغييرات" : "Save Changes")}
        </Button>
      </div>
    </div>
  {/if}
</div>

<!-- Create Custom Rule Modal -->
{#if showAddRuleModal}
  <div 
    in:fade={{ duration: 150 }} 
    class="fixed inset-0 z-50 bg-black/70 backdrop-blur-sm flex items-center justify-center p-4"
  >
    <div 
      in:slide={{ duration: 200 }} 
      class="bg-[#131316] border border-border/20 rounded-2xl w-full max-w-lg p-6 flex flex-col gap-5 text-start shadow-2xl"
    >
      <div class="flex items-center justify-between border-b border-border/10 pb-3">
        <h2 class="text-sm font-bold text-white flex items-center gap-1.5">
          <Plus size={16} class="text-[#a3e635]" />
          <span>{getLocale() === "ar" ? "إضافة قاعدة صلاحيات جديدة" : "Add A New Permission Rule"}</span>
        </h2>
        <button onclick={() => showAddRuleModal = false} class="text-gray-400 hover:text-white cursor-pointer">
          <X size={16} />
        </button>
      </div>

      <div class="flex flex-col gap-4">
        <!-- Document Type Selector -->
        <div class="flex flex-col gap-1.5">
          <label for="rule_doc_type" class="text-xs font-semibold text-gray-400">
            {getLocale() === "ar" ? "نوع المستند" : "Document Type"}
          </label>
          <div class="relative">
            <select
              id="rule_doc_type"
              bind:value={newRuleDocType}
              class="w-full bg-[#17171c] text-white border border-border/20 rounded-lg px-3 py-2 text-xs outline-none focus:border-[#a3e635] cursor-pointer"
            >
              <option value="" disabled selected>Select Document Type</option>
              {#each standardDocTypes as type}
                <option value={type}>{type}</option>
              {/each}
            </select>
          </div>
        </div>

        <!-- Role Selector -->
        <div class="flex flex-col gap-1.5">
          <label for="rule_role" class="text-xs font-semibold text-gray-400">
            {getLocale() === "ar" ? "الدور الوظيفي" : "Job Role"}
          </label>
          <div class="relative">
            <select
              id="rule_role"
              bind:value={newRuleRole}
              class="w-full bg-[#17171c] text-white border border-border/20 rounded-lg px-3 py-2 text-xs outline-none focus:border-[#a3e635] cursor-pointer"
            >
              {#each standardRoles as role}
                <option value={role}>{role}</option>
              {/each}
            </select>
          </div>
        </div>

        <div class="grid grid-cols-2 gap-4">
          <!-- Level -->
          <div class="flex flex-col gap-1.5">
            <label for="rule_level" class="text-xs font-semibold text-gray-400">
              {getLocale() === "ar" ? "المستوى" : "Level"}
            </label>
            <Input
              id="rule_level"
              type="number"
              bind:value={newRuleLevel}
              class="bg-[#17171c] border-border/20 text-xs text-white h-9"
            />
          </div>

          <!-- Creator Condition -->
          <div class="flex flex-col justify-end pb-2">
            <label class="inline-flex items-center gap-2 cursor-pointer select-none">
              <input
                type="checkbox"
                bind:checked={newRuleOnlyIfCreator}
                class="h-4 w-4 rounded border-gray-600 bg-card text-[#a3e635] focus:ring-0 focus:ring-offset-0"
              />
              <span class="text-xs text-gray-300">
                {getLocale() === "ar" ? "فقط إذا كان المنشئ" : "Only If Creator"}
              </span>
            </label>
          </div>
        </div>

        <!-- Actions Selector Checklist -->
        <div class="flex flex-col gap-2 border-t border-border/10 pt-3">
          <span class="text-xs font-semibold text-gray-400 mb-1">
            {getLocale() === "ar" ? "الإجراءات المسموحة" : "Allowed Actions"}
          </span>
          <div class="grid grid-cols-3 gap-2.5 max-h-40 overflow-y-auto pr-1">
            {#each getActionsForDocType(newRuleDocType || "Default") as action}
              <label class="inline-flex items-center gap-2 text-xs text-gray-300 hover:text-white cursor-pointer select-none font-medium">
                <input
                  type="checkbox"
                  checked={newRuleActions.includes(action)}
                  onchange={(e) => {
                    if (e.currentTarget.checked) {
                      newRuleActions = [...newRuleActions, action];
                    } else {
                      newRuleActions = newRuleActions.filter(a => a !== action);
                    }
                  }}
                  class="h-3.5 w-3.5 rounded border-gray-600 bg-card text-[#a3e635] focus:ring-0 focus:ring-offset-0"
                />
                <span>{capitalizeFirstLetter(action)}</span>
              </label>
            {/each}
          </div>
        </div>
      </div>

      <div class="flex items-center justify-end gap-2 border-t border-border/10 pt-4 mt-2">
        <Button variant="ghost" class="text-xs h-8 text-gray-400 hover:text-white" onclick={() => showAddRuleModal = false}>
          {getLocale() === "ar" ? "إلغاء" : "Cancel"}
        </Button>
        <Button variant="lime" class="text-xs h-8 font-bold bg-[#a3e635] text-black hover:bg-[#bef264]" onclick={addRule}>
          {getLocale() === "ar" ? "إضافة القاعدة" : "Add Rule"}
        </Button>
      </div>
    </div>
  </div>
{/if}

<!-- Set User Permissions Modal -->
{#if showSetUserPermissionsModal}
  <div 
    in:fade={{ duration: 150 }} 
    class="fixed inset-0 z-50 bg-black/70 backdrop-blur-sm flex items-center justify-center p-4"
  >
    <div 
      in:slide={{ duration: 200 }} 
      class="bg-[#131316] border border-border/20 rounded-2xl w-full max-w-md p-6 flex flex-col gap-4 text-start shadow-2xl"
    >
      <div class="flex items-center justify-between border-b border-border/10 pb-3">
        <h2 class="text-sm font-bold text-white flex items-center gap-1.5">
          <UserCheck size={16} class="text-[#a3e635]" />
          <span>{getLocale() === "ar" ? "صلاحيات مستخدم محدد" : "User Permissions Settings"}</span>
        </h2>
        <button onclick={() => showSetUserPermissionsModal = false} class="text-gray-400 hover:text-white cursor-pointer">
          <X size={16} />
        </button>
      </div>

      <p class="text-xs text-gray-400">
        {getLocale() === "ar"
          ? "اختر مستخدماً من القائمة لتعديل صلاحيات الاستثناء الفردية المخصصة له."
          : "Select a user from the directory below to configure custom individual override rules."}
      </p>

      <div class="relative w-full my-2">
        <Search size={14} class="absolute top-1/2 -translate-y-1/2 text-gray-500 rtl:right-3 ltr:left-3" />
        <Input
          type="text"
          bind:value={userSearchQuery}
          placeholder={getLocale() === "ar" ? "ابحث عن موظف أو مستخدم..." : "Search user directory..."}
          class="w-full rtl:pr-9 ltr:pl-9 h-9 text-xs bg-[#17171c] border-border/20 text-white"
        />
      </div>

      <!-- Users Scrollable list -->
      <div class="flex flex-col gap-1.5 max-h-60 overflow-y-auto pr-1">
        {#if filteredUsers.length === 0}
          <span class="text-xs text-gray-500 text-center py-6">
            {getLocale() === "ar" ? "لا يوجد مستخدمون" : "No users found"}
          </span>
        {:else}
          {#each filteredUsers as u}
            <button
              onclick={() => {
                showSetUserPermissionsModal = false;
                goto(`/${data.orgId}/settings/users/${u.id}`);
              }}
              class="w-full text-start p-2.5 rounded-lg border border-border/10 bg-[#17171c]/50 hover:bg-[#1a1a24] hover:border-primary/20 transition-all flex items-center justify-between group cursor-pointer"
            >
              <div class="flex flex-col gap-0.5">
                <span class="text-xs font-semibold text-white group-hover:text-[#a3e635] transition-colors">{u.full_name}</span>
                <span class="text-[9px] text-gray-500 font-mono">{u.email}</span>
              </div>
              <div class="flex items-center gap-1.5">
                <span class="text-[9px] px-1.5 py-0.5 rounded bg-[#1f1f26] text-gray-400 font-semibold border border-border/10">
                  {u.primary_role}
                </span>
                <ChevronRight size={12} class="text-gray-500 group-hover:text-white transition-colors" />
              </div>
            </button>
          {/each}
        {/if}
      </div>

      <div class="flex items-center justify-end border-t border-border/10 pt-3 mt-1">
        <Button variant="ghost" class="text-xs h-8 text-gray-400 hover:text-white" onclick={() => showSetUserPermissionsModal = false}>
          {getLocale() === "ar" ? "إغلاق" : "Close"}
        </Button>
      </div>
    </div>
  </div>
{/if}
