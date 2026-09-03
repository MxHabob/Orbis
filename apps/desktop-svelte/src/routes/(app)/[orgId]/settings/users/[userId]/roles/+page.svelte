<!--
@component UserRolesPage
Roles tab for user detail — checklist of available system roles using official S.C.R.A.P.S. design tokens and UI components.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Checkbox } from "@orbis/ui-svelte/checkbox";
import { Card } from "@orbis/ui-svelte/card";
import { Badge } from "@orbis/ui-svelte/badge";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { toast } from "@orbis/ui-svelte";
import { fade } from "svelte/transition";
import { l } from "$lib/core/i18n";
import SearchField from "$lib/shared/components/search-field.svelte";
import {
  Crown,
  Shield,
  GraduationCap,
  BookOpen,
  UserCog,
  ClipboardCheck,
  Landmark,
  User,
  Users,
} from "@lucide/svelte";

const userId = $derived(page.params.userId);

let isLoading = $state(true);
let isSaving = $state(false);
let checkedRoles = $state<string[]>([]);
let roleSearchQuery = $state("");

interface SystemRoleInfo {
  id: string;
  nameAr: string;
  nameEn: string;
  descAr: string;
  descEn: string;
  icon: any;
}

const SYSTEM_ROLES: SystemRoleInfo[] = [
  {
    id: "super_admin",
    nameAr: "مدير عام النظام",
    nameEn: "Super Administrator",
    descAr: "صلاحيات وتحكم كامل وشامل بجميع إعدادات النظام، والمنظمات، وقواعد الأمان.",
    descEn: "Full, unrestricted access to all system settings, organizations, and security rules.",
    icon: Crown,
  },
  {
    id: "admin",
    nameAr: "مدير النظام",
    nameEn: "System Administrator",
    descAr: "إدارة المستخدمين، الأدوار، الصلاحيات، والتهيئة الإدارية للمؤسسة.",
    descEn: "Manages users, roles, system permissions, and institutional configurations.",
    icon: Shield,
  },
  {
    id: "principal",
    nameAr: "مدير المدرسة / المنشأة",
    nameEn: "School Principal",
    descAr: "الإشراف والاعتماد الأكاديمي والإداري الشامل ومتابعة كافة الأقسام.",
    descEn: "Comprehensive academic and administrative oversight across all school departments.",
    icon: GraduationCap,
  },
  {
    id: "teacher",
    nameAr: "معلم / هيئة تدريس",
    nameEn: "Teacher / Faculty",
    descAr: "إدارة الصفوف، والمقررات، وتسجيل الحضور، والتقييمات ورصد الدرجات.",
    descEn: "Manages assigned classes, subjects, attendance, assessments, and gradebooks.",
    icon: BookOpen,
  },
  {
    id: "registrar",
    nameAr: "مسجل أكاديمي",
    nameEn: "Academic Registrar",
    descAr: "إدارة قبول وتسجيل الطلاب، النقل والترقية، وتوثيق الفترات الأكاديمية.",
    descEn: "Handles student admissions, enrollments, promotions, transfers, and academic periods.",
    icon: ClipboardCheck,
  },
  {
    id: "accountant",
    nameAr: "محاسب مالي",
    nameEn: "Financial Accountant",
    descAr: "إدارة الرسوم الدراسية، والفواتير، والمدفوعات، وإصدار التقارير المالية.",
    descEn: "Manages tuition fees, invoicing, payment tracking, and financial ledgers.",
    icon: Landmark,
  },
  {
    id: "staff",
    nameAr: "كادر إداري",
    nameEn: "Administrative Staff",
    descAr: "متابعة المهام التشغيلية اليومية، المراسلات، والعمليات الإدارية المساندة.",
    descEn: "Performs day-to-day administrative duties, messaging, and support workflows.",
    icon: UserCog,
  },
  {
    id: "student",
    nameAr: "طالب",
    nameEn: "Student",
    descAr: "الاطلاع على الجدول الأكاديمي، نتائج التقييمات، والواجبات المدرسية.",
    descEn: "Views personal class timetable, assessment results, and course materials.",
    icon: User,
  },
  {
    id: "guardian",
    nameAr: "ولي أمر",
    nameEn: "Parent / Guardian",
    descAr: "متابعة السجل الأكاديمي وسلوك الأبناء، وسداد الرسوم والتواصل.",
    descEn: "Monitors children's academic progress, attendance records, and fee payments.",
    icon: Users,
  },
];

let filteredRoles = $derived(
  SYSTEM_ROLES.filter((r) => {
    const q = roleSearchQuery.toLowerCase().trim();
    if (!q) return true;
    return (
      r.id.toLowerCase().includes(q) ||
      r.nameAr.toLowerCase().includes(q) ||
      r.nameEn.toLowerCase().includes(q) ||
      r.descAr.toLowerCase().includes(q) ||
      r.descEn.toLowerCase().includes(q)
    );
  })
);

async function loadRoles() {
  if (!userId) return;
  isLoading = true;
  try {
    const userRoles = await commands.iam.listRolesByUser(userId);
    const rolesFromDb = (userRoles || []).map((r: any) => r.role);

    // Fetch user details to get primary_role as fallback
    const userDetails = (await commands.iam.getUser(userId)) as any;
    if (userDetails && userDetails.primary_role) {
      if (!rolesFromDb.includes(userDetails.primary_role)) {
        rolesFromDb.push(userDetails.primary_role);
      }
    }

    checkedRoles = rolesFromDb;
  } catch (e) {
    console.error("Failed to load user roles:", e);
  } finally {
    isLoading = false;
  }
}

onMount(loadRoles);

function selectAllRoles() {
  checkedRoles = SYSTEM_ROLES.map((r) => r.id);
}

function unselectAllRoles() {
  checkedRoles = [];
}

function toggleRole(roleName: string, active: boolean) {
  if (active) {
    if (!checkedRoles.includes(roleName)) {
      checkedRoles = [...checkedRoles, roleName];
    }
  } else {
    checkedRoles = checkedRoles.filter((r) => r !== roleName);
  }
}

async function saveRoles() {
  if (!userId) return;
  isSaving = true;
  try {
    await commands.iam.saveForUser(userId, checkedRoles);

    // Also update primary_role in users table if needed
    if (checkedRoles.length > 0) {
      const userDetails = (await commands.iam.getUser(userId)) as any;
      const currentPrimary = userDetails?.primary_role;
      if (!currentPrimary || !checkedRoles.includes(currentPrimary)) {
        await commands.iam.changeRole(userId, { role: checkedRoles[0] });
      }
    }

    toast.success(m.settings_user_detail_saved ? m.settings_user_detail_saved() : l("تم حفظ الأدوار بنجاح", "Roles saved successfully"));
  } catch (e: unknown) {
    toast.error((e as any)?.message || l("فشل الحفظ", "Save failed"));
  } finally {
    isSaving = false;
  }
}
</script>

{#if isLoading}
  <div class="flex flex-col gap-6 text-start">
    <div class="flex items-center justify-between gap-3">
      <Skeleton class="h-9 w-full max-w-xs" />
      <div class="flex gap-2">
        <Skeleton class="h-8 w-20" />
        <Skeleton class="h-8 w-24" />
      </div>
    </div>
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each Array(6) as _}
        <Skeleton class="h-28 w-full rounded-lg" />
      {/each}
    </div>
  </div>
{:else}
  <div in:fade={{ duration: 100 }} class="flex flex-col gap-5 text-start">
    <!-- Toolbar using Orbis Design Tokens -->
    <div class="flex flex-col sm:flex-row items-center gap-3 w-full justify-between">
      <div class="w-full sm:max-w-xs">
        <SearchField
          placeholder={m.settings_user_detail_search_roles ? m.settings_user_detail_search_roles() : l("البحث في الأدوار...", "Search roles...")}
          bind:value={roleSearchQuery}
          class="sm:max-w-xs"
        />
      </div>

      <div class="flex items-center gap-2.5 w-full sm:w-auto justify-between sm:justify-end">
        <Badge variant="secondary" class="text-xs font-medium">
          {l(
            `تم تحديد ${checkedRoles.length} من أصل ${SYSTEM_ROLES.length}`,
            `${checkedRoles.length} of ${SYSTEM_ROLES.length} selected`
          )}
        </Badge>
        <div class="flex items-center gap-1.5">
          <Button
            variant="outline"
            size="sm"
            class="text-xs h-8"
            onclick={selectAllRoles}
          >
            {m.common_td_ctx_select_all ? m.common_td_ctx_select_all() : l("تحديد الكل", "Select all")}
          </Button>
          <Button
            variant="outline"
            size="sm"
            class="text-xs h-8"
            onclick={unselectAllRoles}
          >
            {m.settings_user_detail_unselect_all ? m.settings_user_detail_unselect_all() : l("إلغاء التحديد", "Clear")}
          </Button>
        </div>
      </div>
    </div>

    <!-- Roles Grid using official @orbis/ui-svelte Card tokens -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3.5 pt-1">
      {#each filteredRoles as role (role.id)}
        {@const isChecked = checkedRoles.includes(role.id)}
        {@const IconComp = role.icon}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <Card
          interactive={true}
          onclick={() => toggleRole(role.id, !isChecked)}
          class="p-4 cursor-pointer select-none transition-all justify-between
                 {isChecked ? 'border-primary/60 bg-primary/5 ring-1 ring-primary/20' : 'border-border/60 bg-card hover:border-border'}"
        >
          <div>
            <!-- Top row: Icon + Title/Slug + Checkbox -->
            <div class="flex items-start justify-between gap-3 mb-2.5">
              <div class="flex items-center gap-2.5 min-w-0">
                <div class="w-8 h-8 rounded-lg flex items-center justify-center shrink-0 bg-muted text-foreground">
                  <IconComp size={16} />
                </div>
                <div class="flex flex-col min-w-0">
                  <span class="text-xs font-semibold text-foreground truncate">
                    {l(role.nameAr, role.nameEn)}
                  </span>
                  <span class="text-[10px] text-muted-foreground font-mono">
                    {role.id}
                  </span>
                </div>
              </div>

              <div class="pt-0.5 shrink-0" onclick={(e) => e.stopPropagation()}>
                <Checkbox
                  id={`role_${role.id}`}
                  checked={isChecked}
                  onCheckedChange={(v) => toggleRole(role.id, Boolean(v))}
                />
              </div>
            </div>

            <!-- Role Description -->
            <p class="text-xs text-muted-foreground leading-relaxed line-clamp-2">
              {l(role.descAr, role.descEn)}
            </p>
          </div>
        </Card>
      {/each}
    </div>

    <!-- Save Button Container using Orbis Design Tokens -->
    <div class="flex justify-end border-t border-border/40 pt-5 mt-2">
      <Button
        variant="lime"
        size="sm"
        class="font-bold px-6"
        onclick={saveRoles}
        disabled={isSaving}
      >
        {isSaving ? "..." : (m.settings_action_save_changes ? m.settings_action_save_changes() : l("حفظ التغييرات", "Save Changes"))}
      </Button>
    </div>
  </div>
{/if}
