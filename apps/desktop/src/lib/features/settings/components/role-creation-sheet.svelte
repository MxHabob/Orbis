<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Badge } from "@orbis/ui/badge";
import { Switch } from "@orbis/ui/switch";
import { ScrollArea } from "@orbis/ui/scroll-area";
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetDescription,
  SheetFooter,
} from "@orbis/ui/sheet";
import {
  Shield,
  KeyRound,
  Layers,
  GraduationCap,
  Users,
  FileCheck,
  Wallet,
  BarChart3,
  CheckCircle2,
  Lock,
  Sparkles,
  Search,
} from "@lucide/svelte";

interface PermissionDomain {
  id: string;
  name: string;
  nameAr: string;
  icon: any;
  resources: {
    key: string;
    label: string;
    labelAr: string;
    actions: { key: string; label: string; labelAr: string }[];
  }[];
}

const DOMAIN_DEFINITIONS: PermissionDomain[] = [
  {
    id: "iam",
    name: "Identity & Organization",
    nameAr: "الهوية والهيكل الإداري",
    icon: Shield,
    resources: [
      {
        key: "Organization",
        label: "Organization Settings",
        labelAr: "إعدادات المنظمة",
        actions: [
          { key: "read", label: "View", labelAr: "عرض" },
          { key: "write", label: "Edit", labelAr: "تعديل" },
          { key: "manage", label: "Full Admin", labelAr: "إدارة كاملة" },
        ],
      },
      {
        key: "OrgUnit",
        label: "Organizational Units / Branches",
        labelAr: "الوحدات والفروع",
        actions: [
          { key: "read", label: "View", labelAr: "عرض" },
          { key: "write", label: "Create/Edit", labelAr: "إنشاء وتعديل" },
          { key: "delete", label: "Delete", labelAr: "حذف" },
        ],
      },
      {
        key: "User",
        label: "Users & Staff",
        labelAr: "المستخدمين والموظفين",
        actions: [
          { key: "read", label: "View", labelAr: "عرض" },
          { key: "write", label: "Create/Edit", labelAr: "إنشاء وتعديل" },
          { key: "role_write", label: "Assign Roles", labelAr: "تعيين الصلاحيات" },
          { key: "password_write", label: "Reset Password", labelAr: "إعادة تعيين كلمة المرور" },
          { key: "delete", label: "Suspend/Delete", labelAr: "إيقاف وحذف" },
        ],
      },
    ],
  },
  {
    id: "academic",
    name: "Academics & Timetable",
    nameAr: "الشؤون الأكاديمية والجدول",
    icon: GraduationCap,
    resources: [
      {
        key: "Period",
        label: "Academic Periods",
        labelAr: "الفصول الأكاديمية",
        actions: [
          { key: "read", label: "View", labelAr: "عرض" },
          { key: "write", label: "Create/Edit", labelAr: "إنشاء وتعديل" },
          { key: "activate", label: "Open/Close", labelAr: "تفعيل وإغلاق" },
          { key: "lock", label: "Lock Grades", labelAr: "قفل الدرجات" },
        ],
      },
      {
        key: "Subject",
        label: "Subjects & Curriculum",
        labelAr: "المواد والمناهج",
        actions: [
          { key: "read", label: "View", labelAr: "عرض" },
          { key: "write", label: "Create/Edit", labelAr: "إنشاء وتعديل" },
          { key: "delete", label: "Delete", labelAr: "حذف" },
        ],
      },
      {
        key: "Class",
        label: "Classes & Sections",
        labelAr: "الفصول والشعب",
        actions: [
          { key: "read", label: "View", labelAr: "عرض" },
          { key: "write", label: "Create/Edit", labelAr: "إنشاء وتعديل" },
          { key: "delete", label: "Delete", labelAr: "حذف" },
        ],
      },
      {
        key: "Schedule",
        label: "Timetables & Slots",
        labelAr: "الجداول والحصص",
        actions: [
          { key: "read", label: "View", labelAr: "عرض" },
          { key: "write", label: "Modify Slots", labelAr: "تعديل الحصص" },
        ],
      },
    ],
  },
  {
    id: "students",
    name: "Student Lifecycle",
    nameAr: "شؤون الطلاب والتسجيل",
    icon: Users,
    resources: [
      {
        key: "Student",
        label: "Student Profiles",
        labelAr: "سجلات الطلاب",
        actions: [
          { key: "read", label: "View", labelAr: "عرض" },
          { key: "write", label: "Register/Edit", labelAr: "تسجيل وتعديل" },
          { key: "delete", label: "Archive", labelAr: "أرشفة" },
        ],
      },
      {
        key: "Enrollment",
        label: "Enrollments & Status",
        labelAr: "القيد وحالات الدراسة",
        actions: [
          { key: "read", label: "View", labelAr: "عرض" },
          { key: "write", label: "Update Status", labelAr: "تحديث الحالة" },
          { key: "promote", label: "Promote Cohort", labelAr: "ترحيل الطلاب" },
          { key: "transfer", label: "Transfer Class", labelAr: "نقل الشعب" },
        ],
      },
      {
        key: "Guardian",
        label: "Guardians & Contacts",
        labelAr: "أولياء الأمور",
        actions: [
          { key: "read", label: "View", labelAr: "عرض" },
          { key: "write", label: "Link/Edit", labelAr: "ربط وتعديل" },
        ],
      },
    ],
  },
  {
    id: "assessment",
    name: "Assessment & Exams",
    nameAr: "التقييم والاختبارات",
    icon: FileCheck,
    resources: [
      {
        key: "Grade",
        label: "Grades & Grading",
        labelAr: "رصد الدرجات",
        actions: [
          { key: "read", label: "View", labelAr: "عرض" },
          { key: "write", label: "Enter Grades", labelAr: "رصد الدرجات" },
          { key: "analytics", label: "Calculations & GPA", labelAr: "احتساب المعدلات" },
        ],
      },
      {
        key: "Attendance",
        label: "Daily Attendance",
        labelAr: "الحضور والغياب",
        actions: [
          { key: "read", label: "View", labelAr: "عرض" },
          { key: "write", label: "Record Attendance", labelAr: "تسجيل الحضور" },
        ],
      },
      {
        key: "Exam",
        label: "Exam Control & Seating",
        labelAr: "كنترول الامتحانات والجلوس",
        actions: [
          { key: "read", label: "View Sessions", labelAr: "عرض اللجان" },
          { key: "write", label: "Schedule Exams", labelAr: "جدولة الامتحانات" },
          { key: "generate_seating", label: "Generate Seating", labelAr: "أرقام الجلوس" },
          { key: "generate_secret", label: "Generate Secret #", labelAr: "الأرقام السرية" },
          { key: "assign_proctor", label: "Assign Proctors", labelAr: "توزيع المراقبين" },
        ],
      },
    ],
  },
  {
    id: "finance",
    name: "Finance & Accounting",
    nameAr: "المالية والمحاسبة",
    icon: Wallet,
    resources: [
      {
        key: "Finance",
        label: "Accounts & Ledger",
        labelAr: "شجرة الحسابات والقيود",
        actions: [
          { key: "read", label: "View Ledger", labelAr: "عرض القيود" },
          { key: "write", label: "Create Entries", labelAr: "إنشاء قيود" },
        ],
      },
      {
        key: "StudentFee",
        label: "Student Fees & Billing",
        labelAr: "رسوم وفواتير الطلاب",
        actions: [
          { key: "read", label: "View Invoices", labelAr: "عرض الرسوم" },
          { key: "write", label: "Issue Fees", labelAr: "إصدار الرسوم" },
          { key: "apply_discount", label: "Apply Discounts", labelAr: "تطبيق الخصومات" },
        ],
      },
      {
        key: "Payment",
        label: "Payments & Receipts",
        labelAr: "المدفوعات وسندات القبض",
        actions: [
          { key: "read", label: "View Receipts", labelAr: "عرض السندات" },
          { key: "write", label: "Process Payments", labelAr: "تسجيل دفعات" },
          { key: "void", label: "Cancel Receipt", labelAr: "إلغاء سند" },
        ],
      },
    ],
  },
  {
    id: "reporting",
    name: "Reporting & Automation",
    nameAr: "التقارير وسير العمل",
    icon: BarChart3,
    resources: [
      {
        key: "Document",
        label: "Report Generation",
        labelAr: "توليد وطباعة التقارير",
        actions: [
          { key: "read", label: "View & Download", labelAr: "عرض وتحميل" },
          { key: "write", label: "Generate Batch", labelAr: "طباعة مجمعة" },
        ],
      },
      {
        key: "Template",
        label: "Report Templates & Designer",
        labelAr: "تصميم قوالب التقارير",
        actions: [
          { key: "read", label: "View Templates", labelAr: "عرض القوالب" },
          { key: "write", label: "Design/Publish", labelAr: "تصميم ونشر" },
        ],
      },
      {
        key: "Audit",
        label: "Audit Logs & Security",
        labelAr: "سجل التدقيق والأمان",
        actions: [
          { key: "read", label: "View Audit Trail", labelAr: "عرض السجلات" },
        ],
      },
    ],
  },
];

let {
  open = $bindable(false),
  roleId = "",
  roleName = $bindable(""),
  roleNameAr = $bindable(""),
  roleDescription = $bindable(""),
  isSystem = false,
  selectedPermissions = $bindable<Record<string, string[]>>({}),
  onsave,
}: {
  open: boolean;
  roleId?: string;
  roleName?: string;
  roleNameAr?: string;
  roleDescription?: string;
  isSystem?: boolean;
  selectedPermissions?: Record<string, string[]>;
  onsave: (data: {
    id: string;
    name: string;
    nameAr: string;
    description: string;
    permissions: Record<string, string[]>;
  }) => Promise<void>;
} = $props();

let activeDomainId = $state("iam");
let searchQuery = $state("");
let isSaving = $state(false);

const isEditMode = $derived(Boolean(roleId));

// Count total enabled permissions
const totalSelectedPermissionsCount = $derived(
  Object.values(selectedPermissions).reduce(
    (acc, actions) => acc + (actions?.length || 0),
    0
  )
);

function isActionEnabled(resourceKey: string, actionKey: string): boolean {
  return selectedPermissions[resourceKey]?.includes(actionKey) ?? false;
}

function toggleAction(resourceKey: string, actionKey: string, enabled: boolean) {
  const current = selectedPermissions[resourceKey] || [];
  if (enabled) {
    if (!current.includes(actionKey)) {
      selectedPermissions = {
        ...selectedPermissions,
        [resourceKey]: [...current, actionKey],
      };
    }
  } else {
    selectedPermissions = {
      ...selectedPermissions,
      [resourceKey]: current.filter((a) => a !== actionKey),
    };
  }
}

function toggleResourceAll(resourceKey: string, allActions: string[], enable: boolean) {
  if (enable) {
    selectedPermissions = {
      ...selectedPermissions,
      [resourceKey]: Array.from(new Set([...(selectedPermissions[resourceKey] || []), ...allActions])),
    };
  } else {
    selectedPermissions = {
      ...selectedPermissions,
      [resourceKey]: [],
    };
  }
}

function toggleDomainAll(domain: PermissionDomain, enable: boolean) {
  const updated = { ...selectedPermissions };
  for (const resource of domain.resources) {
    const actionKeys = resource.actions.map((a) => a.key);
    if (enable) {
      updated[resource.key] = Array.from(new Set([...(updated[resource.key] || []), ...actionKeys]));
    } else {
      updated[resource.key] = [];
    }
  }
  selectedPermissions = updated;
}

async function handleSave() {
  if (!roleName.trim()) return;
  isSaving = true;
  try {
    await onsave({
      id: roleId || roleName.toLowerCase().replace(/\s+/g, "_"),
      name: roleName,
      nameAr: roleNameAr || roleName,
      description: roleDescription,
      permissions: selectedPermissions,
    });
    open = false;
  } finally {
    isSaving = false;
  }
}
</script>

<Sheet bind:open>
  <SheetContent
    side="right"
    class="w-full sm:max-w-3xl flex flex-col p-0 gap-0 bg-card border-s border-border shadow-2xl overflow-hidden"
  >
    <!-- Header -->
    <SheetHeader class="p-6 pb-4 border-b border-border/70 bg-muted/20">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2.5">
          <div class="w-9 h-9 rounded-xl bg-primary/10 border border-primary/20 flex items-center justify-center text-primary shadow-sm">
            <KeyRound class="w-5 h-5" />
          </div>
          <div>
            <SheetTitle class="text-base font-bold text-foreground">
              {isEditMode ? "تعديل صلاحيات الدور" : "إنشاء دور وصلاحيات جديدة"}
            </SheetTitle>
            <SheetDescription class="text-xs text-muted-foreground mt-0.5">
              {isEditMode
                ? `تخصيص الحزم والصلاحيات المسندة لـ "${roleName || roleId}"`
                : "تحديد اسم الدور وحزم الصلاحيات الدقيقة للوحدات والعمليات"}
            </SheetDescription>
          </div>
        </div>

        <div class="flex items-center gap-2">
          {#if isSystem}
            <Badge variant="outline" class="text-[10px] gap-1 bg-amber-500/10 text-amber-600 dark:text-amber-400 border-amber-500/30">
              <Lock class="w-3 h-3" />
              دور أساسي بالنظام
            </Badge>
          {:else}
            <Badge variant="secondary" class="text-[10px] gap-1 bg-primary/10 text-primary border-primary/20">
              <Sparkles class="w-3 h-3" />
              دور مخصص
            </Badge>
          {/if}
          <Badge variant="outline" class="text-[10px] font-mono">
            {totalSelectedPermissionsCount} صلاحية مفعلة
          </Badge>
        </div>
      </div>

      <!-- Basic Info Fields -->
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 mt-4 pt-3 border-t border-border/50">
        <div class="space-y-1 text-start">
          <Label for="role_name" class="text-xs font-semibold text-foreground">
            اسم الدور (English) <span class="text-destructive">*</span>
          </Label>
          <Input
            id="role_name"
            bind:value={roleName}
            placeholder="e.g. Academic Supervisor"
            disabled={isSystem}
            class="h-8 text-xs font-medium"
          />
        </div>

        <div class="space-y-1 text-start">
          <Label for="role_name_ar" class="text-xs font-semibold text-foreground">
            اسم الدور بالعربية
          </Label>
          <Input
            id="role_name_ar"
            bind:value={roleNameAr}
            placeholder="مثال: مشرف أكاديمي"
            disabled={isSystem}
            class="h-8 text-xs font-medium"
          />
        </div>
      </div>
    </SheetHeader>

    <!-- Content: Domain Tabs & Permissions Grid -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left / Start Sidebar: Categories -->
      <div class="w-48 border-e border-border/60 bg-muted/10 p-2 flex flex-col gap-1 overflow-y-auto">
        <span class="text-[10px] font-bold uppercase tracking-wider text-muted-foreground px-2 py-1.5">
          مجالات الصلاحيات
        </span>
        {#each DOMAIN_DEFINITIONS as domain}
          {@const Icon = domain.icon}
          {@const domainActiveCount = domain.resources.reduce((acc, res) => {
            return acc + (selectedPermissions[res.key]?.length || 0);
          }, 0)}
          <button
            type="button"
            onclick={() => (activeDomainId = domain.id)}
            class="w-full flex items-center justify-between px-2.5 py-2 rounded-lg text-xs font-medium transition-all text-start cursor-pointer {activeDomainId === domain.id
              ? 'bg-primary/10 text-primary border border-primary/20 shadow-xs font-semibold'
              : 'text-muted-foreground hover:bg-muted/40 hover:text-foreground'}"
          >
            <div class="flex items-center gap-2 truncate">
              <Icon class="w-3.5 h-3.5 shrink-0" />
              <span class="truncate">{domain.nameAr}</span>
            </div>
            {#if domainActiveCount > 0}
              <span class="px-1.5 py-0.2 rounded-full text-[9px] font-mono bg-primary text-primary-foreground">
                {domainActiveCount}
              </span>
            {/if}
          </button>
        {/each}
      </div>

      <!-- Right: Permissions Matrix for Active Domain -->
      <ScrollArea class="flex-1 p-5">
        {#each DOMAIN_DEFINITIONS.filter((d) => d.id === activeDomainId) as domain}
          <div class="space-y-4">
            <div class="flex items-center justify-between pb-2 border-b border-border/60">
              <div>
                <h3 class="text-sm font-bold text-foreground flex items-center gap-1.5">
                  <domain.icon class="w-4 h-4 text-primary" />
                  {domain.nameAr} ({domain.name})
                </h3>
                <p class="text-[11px] text-muted-foreground">
                  تحديد الصلاحيات الممنوحة لهذا الدور على مستوى {domain.nameAr}
                </p>
              </div>

              <div class="flex items-center gap-1.5">
                <Button
                  variant="outline"
                  size="sm"
                  class="h-7 text-[11px] font-semibold"
                  onclick={() => toggleDomainAll(domain, true)}
                >
                  منح الكل
                </Button>
                <Button
                  variant="ghost"
                  size="sm"
                  class="h-7 text-[11px] text-muted-foreground"
                  onclick={() => toggleDomainAll(domain, false)}
                >
                  إلغاء الكل
                </Button>
              </div>
            </div>

            <!-- Resources in this Domain -->
            <div class="space-y-3">
              {#each domain.resources as resource}
                {@const resourceActions = resource.actions.map((a) => a.key)}
                {@const isAllSelected = resourceActions.every((a) => isActionEnabled(resource.key, a))}
                {@const activeCount = resourceActions.filter((a) => isActionEnabled(resource.key, a)).length}

                <div class="rounded-xl border border-border/70 bg-card p-3.5 space-y-3 transition-colors hover:border-primary/30">
                  <div class="flex items-center justify-between">
                    <div>
                      <div class="flex items-center gap-2">
                        <span class="text-xs font-bold text-foreground">
                          {resource.labelAr}
                        </span>
                        <span class="text-[10px] font-mono text-muted-foreground">
                          ({resource.key})
                        </span>
                      </div>
                      <span class="text-[10px] text-muted-foreground">
                        {activeCount} من {resource.actions.length} إجراءات مفعلة
                      </span>
                    </div>

                    <label class="inline-flex items-center gap-2 cursor-pointer select-none">
                      <span class="text-[10px] text-muted-foreground font-medium">تحديد الكل</span>
                      <Switch
                        checked={isAllSelected}
                        onCheckedChange={(checked) => toggleResourceAll(resource.key, resourceActions, Boolean(checked))}
                      />
                    </label>
                  </div>

                  <!-- Actions List -->
                  <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2 pt-2 border-t border-border/40">
                    {#each resource.actions as action}
                      {@const enabled = isActionEnabled(resource.key, action.key)}
                      <button
                        type="button"
                        onclick={() => toggleAction(resource.key, action.key, !enabled)}
                        class="flex items-center justify-between p-2 rounded-lg border text-start transition-all cursor-pointer select-none {enabled
                          ? 'border-primary/40 bg-primary/5 text-foreground shadow-2xs font-medium'
                          : 'border-border/50 bg-background/50 text-muted-foreground hover:bg-muted/40 hover:text-foreground'}"
                      >
                        <div class="flex items-center gap-2">
                          <div class="w-4 h-4 rounded-md flex items-center justify-center {enabled ? 'bg-primary text-primary-foreground' : 'border border-muted-foreground/30'}">
                            {#if enabled}
                              <CheckCircle2 class="w-3 h-3" />
                            {/if}
                          </div>
                          <div>
                            <div class="text-xs leading-none font-semibold">
                              {action.labelAr}
                            </div>
                            <div class="text-[9px] text-muted-foreground font-mono mt-0.5">
                              {action.key}
                            </div>
                          </div>
                        </div>
                      </button>
                    {/each}
                  </div>
                </div>
              {/each}
            </div>
          </div>
        {/each}
      </ScrollArea>
    </div>

    <!-- Footer Actions -->
    <SheetFooter class="p-4 border-t border-border bg-muted/20 flex items-center justify-between">
      <div class="text-xs text-muted-foreground font-medium">
        إجمالي الصلاحيات المختارة: <span class="text-foreground font-bold font-mono">{totalSelectedPermissionsCount}</span>
      </div>

      <div class="flex items-center gap-2">
        <Button
          variant="outline"
          size="sm"
          class="h-8 text-xs font-medium"
          onclick={() => (open = false)}
        >
          {m.common_action_cancel()}
        </Button>
        <Button
          variant="default"
          size="sm"
          class="h-8 text-xs font-bold gap-1.5 shadow-sm"
          disabled={!roleName.trim() || isSaving}
          onclick={handleSave}
        >
          {#if isSaving}
            <span class="w-3.5 h-3.5 border-2 border-primary-foreground border-t-transparent rounded-full animate-spin"></span>
          {:else}
            <CheckCircle2 class="w-3.5 h-3.5" />
          {/if}
          حفظ الدور والصلاحيات
        </Button>
      </div>
    </SheetFooter>
  </SheetContent>
</Sheet>
