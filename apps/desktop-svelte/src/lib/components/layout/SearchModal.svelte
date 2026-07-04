<script lang="ts">
import { onMount } from "svelte";
import { goto } from "$app/navigation";
import { page } from "$app/state";
import { Dialog, DialogContent } from "@orbis/ui-svelte/dialog";
import { uiStore, authStore } from "$lib/stores";
import { commands } from "$lib/services/ipc";
import { queryClient } from "$lib/services/query-client";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
import {
  Search,
  Zap,
  CornerDownLeft,
  Users,
  GraduationCap,
  BookOpen,
  FileText,
  Settings,
  Plus,
  BarChart3,
  Calendar,
  Shield,
  ArrowRightLeft,
  Wallet,
} from "lucide-svelte";

let searchQuery = $state("");
let inputEl = $state<HTMLInputElement | null>(null);

// In-memory data sets
let students = $state<any[]>([]);
let classes = $state<any[]>([]);
let subjects = $state<any[]>([]);

let selectedIndex = $state(0);

// Load data when opened
$effect(() => {
  if (uiStore.showSearch) {
    searchQuery = "";
    selectedIndex = 0;
    loadSearchData();
    setTimeout(() => {
      inputEl?.focus();
    }, 50);
  }
});

const currentOrgId = $derived(page.params.orgId || authStore.orgId || "");

async function loadSearchData() {
  if (!currentOrgId) return;

  // Use TanStack cache if available — avoids redundant IPC calls
  const cachedStudents = queryClient.getQueryData<any[]>(["enrollments", currentOrgId]);
  const cachedClasses = queryClient.getQueryData<any[]>(["classes", currentOrgId]);
  const cachedSubjects = queryClient.getQueryData<any[]>(["subjects", currentOrgId]);

  if (cachedStudents && cachedClasses && cachedSubjects) {
    students = cachedStudents;
    classes = cachedClasses;
    subjects = cachedSubjects;
    return;
  }

  try {
    const [studentsData, classesData, subjectsData] = await Promise.all([
      cachedStudents ?? commands.enrollments.getAll(currentOrgId).catch(() => []),
      cachedClasses ?? commands.classes.getAll(currentOrgId).catch(() => []),
      cachedSubjects ?? commands.subjects.getAll(currentOrgId).catch(() => []),
    ]);
    students = studentsData;
    classes = classesData;
    subjects = subjectsData;
    // Populate cache for future use
    if (!cachedStudents) queryClient.setQueryData(["enrollments", currentOrgId], studentsData);
    if (!cachedClasses) queryClient.setQueryData(["classes", currentOrgId], classesData);
    if (!cachedSubjects) queryClient.setQueryData(["subjects", currentOrgId], subjectsData);
  } catch (e) {
    console.error("Failed to load search data:", e);
  }
}

// Map static shortcuts
const shortcuts = $derived([
  {
    id: "sc-create-student",
    title: m.students_create ? m.students_create() : "إضافة طالب جديد",
    icon: Plus,
    action: () => goto(`/${currentOrgId}/students`),
    group: m.search_group_shortcuts ? m.search_group_shortcuts() : "Shortcuts",
  },
  {
    id: "sc-view-templates",
    title: m.settings_nav_templates
      ? m.settings_nav_templates()
      : "قوالب المستندات",
    icon: FileText,
    action: () => goto(`/${currentOrgId}/templates`),
    group: m.search_group_shortcuts ? m.search_group_shortcuts() : "Shortcuts",
  },
  {
    id: "sc-view-reports",
    title: m.reports_extract_report
      ? m.reports_extract_report()
      : "استخراج التقارير",
    icon: BarChart3,
    action: () => goto(`/${currentOrgId}/reports`),
    group: m.search_group_shortcuts ? m.search_group_shortcuts() : "Shortcuts",
  },
  {
    id: "sc-view-settings",
    title: m.breadcrumb_settings ? m.breadcrumb_settings() : "الإعدادات العامة",
    icon: Settings,
    action: () => goto(`/${currentOrgId}/settings`),
    group: m.search_group_shortcuts ? m.search_group_shortcuts() : "Shortcuts",
  },
]);

// Map System Pages
const systemPages = $derived([
  // Finance
  {
    title: m.finance_layout_nav_fees
      ? m.finance_layout_nav_fees()
      : "Student Fees",
    path: `/${currentOrgId}/finance/fees`,
    icon: FileText,
  },
  {
    title: m.finance_layout_nav_payments
      ? m.finance_layout_nav_payments()
      : "Receipts",
    path: `/${currentOrgId}/finance/payments`,
    icon: Wallet,
  },
  {
    title: m.finance_layout_nav_treasuries
      ? m.finance_layout_nav_treasuries()
      : "Disbursals",
    path: `/${currentOrgId}/finance/treasuries`,
    icon: Wallet,
  },
  {
    title: m.finance_layout_nav_journal
      ? m.finance_layout_nav_journal()
      : "Journal Entries",
    path: `/${currentOrgId}/finance/journal-entries`,
    icon: FileText,
  },
  {
    title: m.finance_layout_nav_ledger
      ? m.finance_layout_nav_ledger()
      : "Ledger",
    path: `/${currentOrgId}/finance/ledger`,
    icon: FileText,
  },
  {
    title: m.finance_layout_nav_reports
      ? m.finance_layout_nav_reports()
      : "Financial Reports",
    path: `/${currentOrgId}/finance/reports`,
    icon: BarChart3,
  },
  // Academics
  {
    title: m.academics_layout_nav_classes
      ? m.academics_layout_nav_classes()
      : "Classes",
    path: `/${currentOrgId}/academics/classes`,
    icon: GraduationCap,
  },
  {
    title: m.academics_layout_nav_schedule
      ? m.academics_layout_nav_schedule()
      : "Schedule",
    path: `/${currentOrgId}/academics/schedule`,
    icon: Calendar,
  },
  {
    title: m.nav_exams ? m.nav_exams() : "Exams",
    path: `/${currentOrgId}/academics/exams`,
    icon: FileText,
  },
  {
    title: m.academics_control_label
      ? m.academics_control_label()
      : "Control Board",
    path: `/${currentOrgId}/academics/control`,
    icon: Shield,
  },
  // Settings
  {
    title: m.settings_nav_notifications
      ? m.settings_nav_notifications()
      : "Notifications",
    path: `/${currentOrgId}/settings/notifications`,
    icon: Settings,
  },
  {
    title: m.settings_nav_security ? m.settings_nav_security() : "Security",
    path: `/${currentOrgId}/settings/security`,
    icon: Settings,
  },
  {
    title: m.settings_nav_users ? m.settings_nav_users() : "Users & Staff",
    path: `/${currentOrgId}/settings/users`,
    icon: Users,
  },
  {
    title: m.settings_nav_permissions
      ? m.settings_nav_permissions()
      : "Permissions",
    path: `/${currentOrgId}/settings/permissions`,
    icon: Shield,
  },
  {
    title: m.settings_nav_cycles ? m.settings_nav_cycles() : "Academic Cycles",
    path: `/${currentOrgId}/settings/cycles`,
    icon: Calendar,
  },
  {
    title: m.settings_nav_units
      ? m.settings_nav_units()
      : "Organizational Units",
    path: `/${currentOrgId}/settings/units`,
    icon: Settings,
  },
  {
    title: m.settings_nav_rooms ? m.settings_nav_rooms() : "Rooms & Halls",
    path: `/${currentOrgId}/settings/rooms`,
    icon: Settings,
  },
  {
    title: m.settings_nav_subjects ? m.settings_nav_subjects() : "Subjects",
    path: `/${currentOrgId}/settings/subjects`,
    icon: BookOpen,
  },
  {
    title: m.settings_nav_currencies
      ? m.settings_nav_currencies()
      : "Currencies",
    path: `/${currentOrgId}/settings/currencies`,
    icon: ArrowRightLeft,
  },
  {
    title: m.settings_nav_fee_templates
      ? m.settings_nav_fee_templates()
      : "Fee Templates",
    path: `/${currentOrgId}/settings/fee-templates`,
    icon: FileText,
  },
  {
    title: m.settings_nav_discount_templates
      ? m.settings_nav_discount_templates()
      : "Discount Templates",
    path: `/${currentOrgId}/settings/discount-templates`,
    icon: FileText,
  },
  // User Account
  {
    title: m.account_settings_title
      ? m.account_settings_title()
      : "Account Settings",
    path: `/${currentOrgId}/account`,
    icon: Settings,
  },
]);

// Filtered and Grouped Results
const filteredItems = $derived.by(() => {
  const query = searchQuery.trim().toLowerCase();
  const list: any[] = [];

  // 1. Add matching shortcuts
  const matchedShortcuts = shortcuts.filter((s) =>
    s.title.toLowerCase().includes(query),
  );
  list.push(...matchedShortcuts);

  // 2. Add matching system pages
  if (query.length >= 1) {
    const matchedPages = systemPages
      .filter((p) => p.title.toLowerCase().includes(query))
      .map((p) => ({
        id: `page-${p.path}`,
        title: p.title,
        icon: p.icon,
        action: () => goto(p.path),
        group: m.search_group_system_pages
          ? m.search_group_system_pages()
          : "System Pages",
      }));
    list.push(...matchedPages);
  }

  // 3. Add matching students
  if (query.length >= 1) {
    const matchedStudents = students
      .filter(
        (s) =>
          String(s.full_name || "")
            .toLowerCase()
            .includes(query) ||
          String(s.student_number || "")
            .toLowerCase()
            .includes(query) ||
          String(s.email || "")
            .toLowerCase()
            .includes(query),
      )
      .map((s) => ({
        id: `student-${s.id}`,
        title: s.full_name || s.name_ar || s.name_en || "Student",
        subtitle: s.student_number || "",
        icon: Users,
        action: () => goto(`/${currentOrgId}/students/${s.id}/grades`),
        group: m.search_group_students ? m.search_group_students() : "Students",
      }));
    list.push(...matchedStudents.slice(0, 10)); // limit to 10
  }

  // 4. Add matching classes
  if (query.length >= 1) {
    const matchedClasses = classes
      .filter((c) =>
        String(c.name || "")
          .toLowerCase()
          .includes(query),
      )
      .map((c) => ({
        id: `class-${c.id}`,
        title: c.name || "Class",
        icon: GraduationCap,
        action: () => goto(`/${currentOrgId}/academics/classes`),
        group: m.search_group_classes ? m.search_group_classes() : "Classes",
      }));
    list.push(...matchedClasses);
  }

  // 5. Add matching subjects
  if (query.length >= 1) {
    const matchedSubjects = subjects
      .filter((s) =>
        String(s.name || "")
          .toLowerCase()
          .includes(query),
      )
      .map((s) => ({
        id: `subject-${s.id}`,
        title: s.name || "Subject",
        icon: BookOpen,
        action: () => goto(`/${currentOrgId}/settings/subjects`),
        group: m.search_group_subjects ? m.search_group_subjects() : "Subjects",
      }));
    list.push(...matchedSubjects);
  }

  return list;
});

// Grouped Items for rendering
const groupedItems = $derived.by(() => {
  const groups: Record<string, any[]> = {};
  for (const item of filteredItems) {
    if (!groups[item.group]) {
      groups[item.group] = [];
    }
    groups[item.group].push(item);
  }
  return groups;
});

const selectedItem = $derived(filteredItems[selectedIndex] || null);

function triggerAction(item: any) {
  if (!item) return;
  item.action();
  uiStore.setSearch(false);
}

function handleKeydown(e: KeyboardEvent) {
  // Toggle search on Cmd+K or Ctrl+K
  if ((e.metaKey || e.ctrlKey) && e.key === "k") {
    e.preventDefault();
    uiStore.toggleSearch();
    return;
  }

  if (!uiStore.showSearch) return;

  if (e.key === "ArrowDown") {
    e.preventDefault();
    selectedIndex = (selectedIndex + 1) % filteredItems.length;
  } else if (e.key === "ArrowUp") {
    e.preventDefault();
    selectedIndex =
      (selectedIndex - 1 + filteredItems.length) % filteredItems.length;
  } else if (e.key === "Enter") {
    e.preventDefault();
    if (selectedItem) {
      triggerAction(selectedItem);
    }
  } else if (e.key === "Escape") {
    e.preventDefault();
    uiStore.setSearch(false);
  }
}
</script>

<svelte:window onkeydown={handleKeydown} />

<Dialog open={uiStore.showSearch} onOpenChange={(open) => uiStore.setSearch(open)}>
  <DialogContent
    showCloseButton={false}
    class="overflow-hidden p-0 max-w-full w-full md:max-w-[740px] h-[535px] m-0 select-none flex flex-col focus:outline-none"
  >
    <!-- Search Input Bar -->
    <div class="px-4 py-3 border-b border-border bg-popover shrink-0">
      <div class="relative flex items-center">
        <Search class="absolute left-3 size-4 text-muted-foreground pointer-events-none" />
        <input
          bind:this={inputEl}
          type="text"
          placeholder={m.header_search ? m.header_search() : "البحث عن أي شيء..."}
          bind:value={searchQuery}
          class="input-sunken ps-10 w-full"
        />
      </div>
    </div>

    <!-- Search Results -->
    <div class="flex-1 overflow-y-auto p-2 scrollbar-hide bg-popover">
      {#if filteredItems.length === 0}
        <div class="py-6 text-center text-sm text-muted-foreground select-none">
          {m.common_no_data ? m.common_no_data() : "لا توجد نتائج مطابقة"}
        </div>
      {:else}
        {#each Object.entries(groupedItems) as [groupName, groupItems]}
          <div class="mb-4 last:mb-0">
            <div class="px-3 py-1 text-[10px] font-bold text-muted-foreground tracking-wider uppercase select-none text-start">
              {groupName}
            </div>
            <div class="mt-1 flex flex-col gap-0.5">
              {#each groupItems as item}
                {@const Icon = item.icon}
                <button
                  type="button"
                  onclick={() => triggerAction(item)}
                  class="w-full text-start flex items-center justify-between px-3 py-2 text-sm rounded-md transition-colors group
                    {item === selectedItem ? 'bg-accent text-accent-foreground font-medium' : 'text-foreground hover:bg-accent/40 hover:text-accent-foreground'}"
                >
                  <div class="flex items-center gap-2.5 truncate">
                    <Icon class="size-4 text-muted-foreground group-hover:text-accent-foreground shrink-0" />
                    <span class="truncate">{item.title}</span>
                    {#if item.subtitle}
                      <span class="text-xs text-muted-foreground font-mono truncate">{item.subtitle}</span>
                    {/if}
                  </div>
                  <div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
                    <CornerDownLeft class="size-3 text-muted-foreground group-hover:text-accent-foreground" />
                  </div>
                </button>
              {/each}
            </div>
          </div>
        {/each}
      {/if}
    </div>

    <!-- Search Footer -->
    <div class="h-10 px-4 border-t border-border bg-muted/30 flex items-center justify-between text-xs text-muted-foreground select-none shrink-0 rounded-b-xl">
      <div class="flex items-center gap-1.5">
        <Zap class="size-3.5 fill-primary text-primary animate-pulse" />
        <span class="font-bold text-foreground">Orbis Search</span>
      </div>
      <div class="flex items-center gap-3 font-medium">
        <div class="flex items-center gap-1">
          <kbd class="border border-border bg-background px-1.5 py-0.5 rounded text-[9px] font-mono text-muted-foreground shadow-[inset_0_2px_4px_rgba(0,0,0,0.06),_inset_0_1px_2px_rgba(0,0,0,0.04)]">↑↓</kbd>
          <span>{m.search_footer_navigate ? m.search_footer_navigate() : "Navigate"}</span>
        </div>
        <div class="flex items-center gap-1">
          <kbd class="border border-border bg-background px-1.5 py-0.5 rounded text-[9px] font-mono text-muted-foreground shadow-[inset_0_2px_4px_rgba(0,0,0,0.06),_inset_0_1px_2px_rgba(0,0,0,0.04)]">Enter</kbd>
          <span>{m.search_footer_select ? m.search_footer_select() : "Select"}</span>
        </div>
        <div class="flex items-center gap-1">
          <kbd class="border border-border bg-background px-1.5 py-0.5 rounded text-[9px] font-mono text-muted-foreground shadow-[inset_0_2px_4px_rgba(0,0,0,0.06),_inset_0_1px_2px_rgba(0,0,0,0.04)]">Esc</kbd>
          <span>{m.search_footer_close ? m.search_footer_close() : "Close"}</span>
        </div>
      </div>
    </div>
  </DialogContent>
</Dialog>
