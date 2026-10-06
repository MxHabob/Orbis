<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { goto } from "$app/navigation";
import { page } from "$app/state";
import { Dialog, DialogContent } from "@orbis/ui/dialog";
import { Kbd } from "@orbis/ui/kbd";
import { uiStore, authStore } from "$lib/stores";
import { api, queryKeys, type EnrollmentDto, type ClassDto, type SubjectDto } from "$lib/api";
import { queryClient } from "$lib/api";

import { matchStudent } from "$lib/shared/utils/search";
import { AdjustmentsHorizontal, AppNotification, ArrowPath, ArrowUturnLeft, Banknotes, Bolt, BookOpen, BuildingLibrary, BuildingOffice, Calendar, ChartBar, CreditCard, Document, DocumentChartBar, DocumentCheck, DocumentPage, GlobeAlt, Pin, Plus, ReceiptPercent, Search, Settings, ShieldCheck, UserBadgeCheck, Users } from "@orbis/icons";

let searchQuery = $state("");
let inputEl = $state<HTMLInputElement | null>(null);

// In-memory data sets
let students = $state<EnrollmentDto[]>([]);
let classes = $state<ClassDto[]>([]);
let subjects = $state<SubjectDto[]>([]);

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
  const cachedStudents = queryClient.getQueryData<EnrollmentDto[]>(
    queryKeys.student.enrollments(currentOrgId),
  );
  const cachedClasses = queryClient.getQueryData<ClassDto[]>(
    queryKeys.academic.classes(currentOrgId),
  );
  const cachedSubjects = queryClient.getQueryData<SubjectDto[]>(
    queryKeys.academic.subjects(currentOrgId),
  );

  if (cachedStudents && cachedClasses && cachedSubjects) {
    students = cachedStudents;
    classes = cachedClasses;
    subjects = cachedSubjects;
    return;
  }

  try {
    const [studentsData, classesData, subjectsData] = await Promise.all([
      cachedStudents ?? api.listEnrollments().catch(() => []),
      cachedClasses ?? api.listClasses().catch(() => []),
      cachedSubjects ?? api.listSubjects().catch(() => []),
    ]);
    students = studentsData;
    classes = classesData;
    subjects = subjectsData;
    // Populate cache for future use
    if (!cachedStudents)
      queryClient.setQueryData(queryKeys.student.enrollments(currentOrgId), studentsData);
    if (!cachedClasses)
      queryClient.setQueryData(queryKeys.academic.classes(currentOrgId), classesData);
    if (!cachedSubjects)
      queryClient.setQueryData(queryKeys.academic.subjects(currentOrgId), subjectsData);
  } catch (e) {
    console.error("Failed to load search data:", e);
  }
}

// Map static shortcuts
const shortcuts = $derived([
  {
    id: "sc-create-student",
    title: m.students_create(),
    icon: Plus,
    action: () => goto(`/${currentOrgId}/students`),
    group: m.common_search_group_shortcuts(),
  },
  {
    id: "sc-view-templates",
    title: m.settings_nav_templates(),
    icon: DocumentPage,
    action: () => goto(`/${currentOrgId}/templates`),
    group: m.common_search_group_shortcuts(),
  },
  {
    id: "sc-view-reports",
    title: m.reports_extract_report(),
    icon: ChartBar,
    action: () => goto(`/${currentOrgId}/reports`),
    group: m.common_search_group_shortcuts(),
  },
  {
    id: "sc-view-settings",
    title: m.common_word_settings(),
    icon: Settings,
    action: () => goto(`/${currentOrgId}/settings`),
    group: m.common_search_group_shortcuts(),
  },
]);

// Map System Pages
const systemPages = $derived([
  // Finance
  {
    title: m.finance_layout_nav_fees(),
    path: `/${currentOrgId}/finance/fees`,
    icon: DocumentChartBar,
  },
  {
    title: m.finance_layout_nav_payments(),
    path: `/${currentOrgId}/finance/payments`,
    icon: CreditCard,
  },
  {
    title: m.finance_layout_nav_cash_accounts(),
    path: `/${currentOrgId}/finance/accounts`,
    icon: BuildingLibrary,
  },
  {
    title: m.common_je_title(),
    path: `/${currentOrgId}/finance/journal-entries`,
    icon: BookOpen,
  },
  {
    title: m.finance_layout_nav_ledger(),
    path: `/${currentOrgId}/finance/statement`,
    icon: DocumentChartBar,
  },
  {
    title: m.finance_layout_nav_reports(),
    path: `/${currentOrgId}/finance/reports`,
    icon: ChartBar,
  },
  // Academics
  {
    title: m.common_search_group_classes(),
    path: `/${currentOrgId}/academics/classes`,
    icon: BookOpen,
  },
  {
    title: m.common_exams_detail_schedule(),
    path: `/${currentOrgId}/academics/schedule`,
    icon: Calendar,
  },
  {
    title: m.common_nav_exams(),
    path: `/${currentOrgId}/academics/exams`,
    icon: DocumentCheck,
  },
  {
    title: m.common_control_label(),
    path: `/${currentOrgId}/academics/control`,
    icon: AdjustmentsHorizontal,
  },
  // Settings
  {
    title: m.common_nav_notifications(),
    path: `/${currentOrgId}/settings/notifications`,
    icon: AppNotification,
  },
  {
    title: m.common_nav_security(),
    path: `/${currentOrgId}/settings/security`,
    icon: ShieldCheck,
  },
  {
    title: m.settings_nav_users(),
    path: `/${currentOrgId}/settings/users`,
    icon: Users,
  },
  {
    title: m.settings_nav_permissions(),
    path: `/${currentOrgId}/settings/permissions`,
    icon: ShieldCheck,
  },
  {
    title: m.settings_nav_cycles(),
    path: `/${currentOrgId}/settings/periods`,
    icon: Calendar,
  },
  {
    title: m.settings_nav_units(),
    path: `/${currentOrgId}/settings/units`,
    icon: BuildingOffice,
  },
  {
    title: m.common_nav_rooms(),
    path: `/${currentOrgId}/settings/rooms`,
    icon: Pin,
  },
  {
    title: m.common_exams_detail_subjects(),
    path: `/${currentOrgId}/settings/subjects`,
    icon: BookOpen,
  },
  {
    title: m.settings_nav_currencies(),
    path: `/${currentOrgId}/settings/currencies`,
    icon: GlobeAlt,
  },
  {
    title: m.common_breadcrumb_fees(),
    path: `/${currentOrgId}/settings/fee-templates`,
    icon: DocumentPage,
  },
  {
    title: m.common_form_summary_discount(),
    path: `/${currentOrgId}/settings/discount-templates`,
    icon: ReceiptPercent,
  },
  // User Account
  {
    title: m.common_account_settings_title(),
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
        group: m.common_search_group_system_pages(),
      }));
    list.push(...matchedPages);
  }

  // 3. Add matching students
  if (query.length >= 1) {
    const matchedStudents = students
      .filter((s) => matchStudent(s, query))
      .map((s) => ({
        id: `student-${s.id}`,
        title: s.full_name || s.name_ar || s.name_en || "Student",
        subtitle: s.student_number || "",
        icon: Users,
        action: () => goto(`/${currentOrgId}/students/${s.id}`),
        group: m.common_word_students(),
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
        icon: BookOpen,
        action: () => goto(`/${currentOrgId}/academics/classes`),
        group: m.common_search_group_classes(),
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
        group: m.common_exams_detail_subjects(),
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
    class="overflow-hidden p-0 max-w-full w-full md:max-w-185 h-133.75 m-0 select-none flex flex-col focus:outline-none"
  >
    <!-- Search Input Bar -->
    <div class="px-4 py-3  shrink-0">
      <div class="relative flex items-center">
        <Search class="absolute inset-s-3 size-4 text-muted-foreground pointer-events-none" />
        <input
          bind:this={inputEl}
          type="text"
          placeholder={m.common_header_search()}
          bind:value={searchQuery}
          class="input-sunken ps-10 w-full"
        />
      </div>
    </div>

    <!-- Search Results -->
    <div class="flex-1 overflow-y-auto p-2 scrollbar-hide ">
      {#if filteredItems.length === 0}
        <div class="py-6 text-center text-sm text-muted-foreground select-none">
          {m.common_no_data()}
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
                    <ArrowUturnLeft class="size-3 text-muted-foreground group-hover:text-accent-foreground" />
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
        <Bolt class="size-3.5 fill-primary text-primary animate-pulse" />
        <span class="font-bold text-foreground">{m.shared_ui_orbis_search()}</span>
      </div>
      <div class="flex items-center gap-3 font-medium">
        <div class="flex items-center gap-1">
          <Kbd>↑↓</Kbd>
          <span>{m.common_search_footer_navigate()}</span>
        </div>
        <div class="flex items-center gap-1">
          <Kbd>Enter</Kbd>
          <span>{m.common_search_footer_select()}</span>
        </div>
        <div class="flex items-center gap-1">
          <Kbd>Esc</Kbd>
          <span>{m.common_titlebar_close()}</span>
        </div>
      </div>
    </div>
  </DialogContent>
</Dialog>
