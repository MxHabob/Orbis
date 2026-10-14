<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import { Dialog, DialogContent } from "@orbis/ui/dialog";
  import { Button } from "@orbis/ui/button";
  import { uiStore, authStore } from "$lib/stores";
  import { createQuery } from "@tanstack/svelte-query";
  import {
    api,
    queryKeys,
  } from "$lib/api";
  import {
    AdjustmentsHorizontal,
    AppNotification,
    Banknotes,
    BookOpen,
    Bolt,
    BuildingLibrary,
    BuildingOffice,
    Calendar,
    ChartBar,
    CreditCard,
    DocumentCheck,
    DocumentChartBar,
    DocumentPage,
    EmployeeAffairs,
    GlobeAlt,
    Pin,
    Plus,
    ReceiptPercent,
    Search,
    Settings,
    Sparkles,
    ShieldCheck,
    UserBadgeCheck,
    Users,
  } from "@orbis/icons";
  import { matchStudent, normalizeArabic, normalizeDigits } from "$lib/shared/utils/search";
  import SearchModalResults from "./search-modal-results.svelte";
  import SearchModalFooter from "./search-modal-footer.svelte";
  import type { Component } from "svelte";
  import { onMount } from "svelte";
  import type { SearchItem, SearchItemGroup } from "./search-modal.types";

  type PageEntry = {
    title: string;
    path: string;
    icon: Component<{ class?: string; size?: number }>;
    keywords?: string[];
  };

  let searchQuery = $state("");
  let debouncedQuery = $state("");
  let inputEl = $state<HTMLInputElement | null>(null);
  let selectedIndex = $state(0);
  let debounceTimer: ReturnType<typeof setTimeout> | undefined;

  onMount(() => {
    requestAnimationFrame(() => inputEl?.focus());
  });

  const currentOrgId = $derived(
    String(page.params.orgId || authStore.orgId || ""),
  );

  const currentQuery = $derived(searchQuery.trim());
  const queryIsSettled = $derived(debouncedQuery === currentQuery);
  const searchEnabled = $derived(
    uiStore.showSearch &&
      currentOrgId.length > 0 &&
      queryIsSettled &&
      debouncedQuery.length >= 2,
  );
  const referenceSearchEnabled = $derived(
    uiStore.showSearch &&
      currentOrgId.length > 0 &&
      queryIsSettled &&
      debouncedQuery.length >= 2,
  );

  const studentsQuery = createQuery(() => ({
    queryKey: queryKeys.student.list(currentOrgId, {
      search: debouncedQuery,
      page: 1,
      page_size: 10,
    }),
    queryFn: () =>
      api.listStudents({
        search: debouncedQuery,
        page: 1,
        page_size: 10,
      }),
    enabled: searchEnabled,
    staleTime: 30_000,
    gcTime: 2 * 60_000,
    retry: 1,
  }));

  const employeesQuery = createQuery(() => ({
    queryKey: queryKeys.employee.list(currentOrgId, {
      query: debouncedQuery,
      page: 1,
      page_size: 10,
    }),
    queryFn: () =>
      api.listEmployees({
        query: debouncedQuery,
        page: 1,
        page_size: 10,
      }),
    enabled: searchEnabled,
    staleTime: 30_000,
    gcTime: 2 * 60_000,
    retry: 1,
  }));

  const sectionsQuery = createQuery(() => ({
    queryKey: queryKeys.academic.classes(currentOrgId),
    queryFn: () => api.listSections(),
    enabled: referenceSearchEnabled,
    staleTime: 5 * 60_000,
    retry: 1,
  }));

  const subjectsQuery = createQuery(() => ({
    queryKey: queryKeys.academic.subjects(currentOrgId),
    queryFn: () => api.listSubjects(),
    enabled: referenceSearchEnabled,
    staleTime: 5 * 60_000,
    retry: 1,
  }));

  const roomsQuery = createQuery(() => ({
    queryKey: queryKeys.scheduling.rooms(currentOrgId),
    queryFn: () => api.listRooms(),
    enabled: referenceSearchEnabled,
    staleTime: 5 * 60_000,
    retry: 1,
  }));

  const orgUnitsQuery = createQuery(() => ({
    queryKey: queryKeys.org.units(currentOrgId),
    queryFn: () => api.listOrgUnits(),
    enabled: referenceSearchEnabled,
    staleTime: 5 * 60_000,
    retry: 1,
  }));

  const groupsQuery = createQuery(() => ({
    queryKey: queryKeys.group.list(currentOrgId),
    queryFn: () => api.listGroups(),
    enabled: referenceSearchEnabled,
    staleTime: 5 * 60_000,
    retry: 1,
  }));

  const usersQuery = createQuery(() => ({
    queryKey: queryKeys.auth.users(currentOrgId),
    queryFn: () => api.listUsers(),
    enabled: referenceSearchEnabled,
    staleTime: 5 * 60_000,
    gcTime: 10 * 60_000,
    retry: 1,
  }));

  const periodsQuery = createQuery(() => ({
    queryKey: queryKeys.calendar.periods(currentOrgId),
    queryFn: () => api.listAcademicPeriods(),
    enabled: referenceSearchEnabled,
    staleTime: 5 * 60_000,
    gcTime: 10 * 60_000,
    retry: 1,
  }));

  const currenciesQuery = createQuery(() => ({
    queryKey: queryKeys.finance.currencies(),
    queryFn: () => api.listCurrencies(),
    enabled: referenceSearchEnabled,
    staleTime: 10 * 60_000,
    gcTime: 10 * 60_000,
    retry: 1,
  }));

  const accountsQuery = createQuery(() => ({
    queryKey: queryKeys.finance.accounts(currentOrgId),
    queryFn: () => api.listAccounts(),
    enabled: referenceSearchEnabled,
    staleTime: 5 * 60_000,
    gcTime: 10 * 60_000,
    retry: 1,
  }));

  const feeTemplatesQuery = createQuery(() => ({
    queryKey: queryKeys.finance.feeTemplates(currentOrgId),
    queryFn: () => api.listFeeTemplates(),
    enabled: referenceSearchEnabled,
    staleTime: 5 * 60_000,
    gcTime: 10 * 60_000,
    retry: 1,
  }));

  const discountTemplatesQuery = createQuery(() => ({
    queryKey: queryKeys.finance.discountTemplates(currentOrgId),
    queryFn: () => api.listDiscountTemplates(),
    enabled: referenceSearchEnabled,
    staleTime: 5 * 60_000,
    gcTime: 10 * 60_000,
    retry: 1,
  }));

  const sections = $derived(sectionsQuery.data ?? []);
  const subjects = $derived(subjectsQuery.data ?? []);
  const rooms = $derived(roomsQuery.data ?? []);
  const orgUnits = $derived(orgUnitsQuery.data ?? []);
  const groups = $derived(groupsQuery.data ?? []);
  const users = $derived(usersQuery.data ?? []);
  const periods = $derived(periodsQuery.data ?? []);
  const currencies = $derived(currenciesQuery.data ?? []);
  const accounts = $derived(accountsQuery.data ?? []);
  const feeTemplates = $derived(feeTemplatesQuery.data ?? []);
  const discountTemplates = $derived(discountTemplatesQuery.data ?? []);
  const indexedSections = $derived(
    sections.map((item) => ({
      item,
      text: normalizeSearchText(
        [item.name, item.name_ar, item.code, item.level, item.track].join(" "),
      ),
    })),
  );
  const indexedSubjects = $derived(
    subjects.map((item) => ({
      item,
      text: normalizeSearchText(
        [item.name, item.name_ar, item.code, item.description].join(" "),
      ),
    })),
  );
  const indexedRooms = $derived(
    rooms.map((item) => ({
      item,
      text: normalizeSearchText(
        [item.name, item.name_ar, item.room_type].join(" "),
      ),
    })),
  );
  const indexedOrgUnits = $derived(
    orgUnits.map((item) => ({
      item,
      text: normalizeSearchText(
        [
          item.name,
          item.name_ar,
          item.code,
          item.unit_type,
          item.building,
          item.room_number,
        ].join(" "),
      ),
    })),
  );
  const indexedGroups = $derived(
    groups.map((item) => ({
      item,
      text: normalizeSearchText(
        [item.name, item.name_ar, item.description].join(" "),
      ),
    })),
  );
  const indexedUsers = $derived(
    users.map((item) => ({
      item,
      text: normalizeSearchText(
        [
          item.full_name,
          item.full_name_ar,
          item.email,
          item.phone,
          item.national_id,
          item.status,
          ...(item.roles ?? []),
        ].join(" "),
      ),
    })),
  );
  const indexedPeriods = $derived(
    periods.map((item) => ({
      item,
      text: normalizeSearchText(
        [
          item.name,
          item.name_ar,
          item.code,
          item.period_kind,
          item.period_type,
          item.status,
          item.start_date,
          item.end_date,
        ].join(" "),
      ),
    })),
  );
  const indexedCurrencies = $derived(
    currencies.map((item) => ({
      item,
      text: normalizeSearchText([item.name, item.code, item.symbol].join(" ")),
    })),
  );
  const indexedAccounts = $derived(
    accounts.map((item) => ({
      item,
      text: normalizeSearchText(
        [
          item.name,
          item.name_ar,
          item.code,
          item.account_type,
          item.currency,
          item.sub_type,
        ].join(" "),
      ),
    })),
  );
  const indexedFeeTemplates = $derived(
    feeTemplates.map((item) => ({
      item,
      text: normalizeSearchText(
        [
          item.name,
          item.name_ar,
          item.code,
          item.category,
          item.currency,
          item.frequency,
        ].join(" "),
      ),
    })),
  );
  const indexedDiscountTemplates = $derived(
    discountTemplates.map((item) => ({
      item,
      text: normalizeSearchText(
        [
          item.name,
          item.name_ar,
          item.code,
          item.discount_type,
          item.value,
        ].join(" "),
      ),
    })),
  );
  const normalizedQuery = $derived(normalizeSearchText(debouncedQuery));
  const searchTerms = $derived(normalizedQuery.split(/\s+/).filter(Boolean));
  const queryPending = $derived(currentQuery.length > 0 && !queryIsSettled);
  const studentResults = $derived(
    queryIsSettled && searchEnabled
      ? (studentsQuery.data?.items ?? []).filter((student) =>
          matchStudent(student, debouncedQuery),
        )
      : [],
  );
  const employeeResults = $derived(
    queryIsSettled && searchEnabled
      ? (employeesQuery.data?.items ?? [])
      : [],
  );
  const isSearching = $derived(
    queryPending ||
      (searchEnabled &&
    (studentsQuery.isFetching || employeesQuery.isFetching)) ||
    (referenceSearchEnabled &&
      (sectionsQuery.isFetching ||
        subjectsQuery.isFetching ||
        roomsQuery.isFetching ||
        orgUnitsQuery.isFetching ||
        groupsQuery.isFetching ||
        usersQuery.isFetching ||
        periodsQuery.isFetching ||
        currenciesQuery.isFetching ||
        accountsQuery.isFetching ||
        feeTemplatesQuery.isFetching ||
        discountTemplatesQuery.isFetching)),
  );
  const failedSearchSources = $derived([
    ...(searchEnabled && studentsQuery.isError
      ? [m.common_word_students()]
      : []),
    ...(searchEnabled && employeesQuery.isError
      ? [m.employees_nav_title()]
      : []),
    ...(referenceSearchEnabled && sectionsQuery.isError
      ? [m.common_search_group_classes()]
      : []),
    ...(referenceSearchEnabled && subjectsQuery.isError
      ? [m.common_search_group_subjects()]
      : []),
    ...(referenceSearchEnabled && roomsQuery.isError
      ? [m.common_search_group_rooms()]
      : []),
    ...(referenceSearchEnabled && orgUnitsQuery.isError
      ? [m.common_search_group_org_units()]
      : []),
    ...(referenceSearchEnabled && groupsQuery.isError
      ? [m.common_search_group_groups()]
      : []),
    ...(referenceSearchEnabled && usersQuery.isError
      ? [m.common_search_group_users()]
      : []),
    ...(referenceSearchEnabled && periodsQuery.isError
      ? [m.common_search_group_periods()]
      : []),
    ...(referenceSearchEnabled && currenciesQuery.isError
      ? [m.common_search_group_currencies()]
      : []),
    ...(referenceSearchEnabled && accountsQuery.isError
      ? [m.common_search_group_accounts()]
      : []),
    ...(referenceSearchEnabled && feeTemplatesQuery.isError
      ? [m.common_search_group_fee_templates()]
      : []),
    ...(referenceSearchEnabled && discountTemplatesQuery.isError
      ? [m.common_search_group_discount_templates()]
      : []),
  ]);
  const searchError = $derived(
    failedSearchSources.length
      ? `${m.common_search_load_failed()} (${failedSearchSources.join(" · ")})`
      : "",
  );

  const shortcuts = $derived([
    {
      id: "sc-create-student",
      title: m.students_create(),
      icon: Plus,
      action: () => goto(`/${currentOrgId}/students/new`),
      group: m.common_search_group_shortcuts(),
    },
    {
      id: "sc-employees",
      title: m.employees_nav_title(),
      icon: EmployeeAffairs,
      action: () => goto(`/${currentOrgId}/employees`),
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

  const knownSystemPages = $derived<PageEntry[]>([
    { title: m.common_nav_overview(), path: `/${currentOrgId}`, icon: ChartBar },
    { title: m.common_word_students(), path: `/${currentOrgId}/students`, icon: Users, keywords: ["طلاب", "طالب", "التلاميذ"] },
    { title: m.employees_nav_title(), path: `/${currentOrgId}/employees`, icon: EmployeeAffairs, keywords: ["موظفين", "معلمين", "طاقم المدرسة"] },
    { title: m.common_nav_reports(), path: `/${currentOrgId}/reports`, icon: ChartBar, keywords: ["تقارير", "تقرير", "تحليلات"] },
    { title: m.common_nav_academics(), path: `/${currentOrgId}/academics`, icon: BookOpen },
    { title: m.common_search_group_classes(), path: `/${currentOrgId}/academics/sections`, icon: BookOpen, keywords: ["شعب", "الشُعب", "الشعب الدراسية"] },
    { title: m.common_promo_attendance(), path: `/${currentOrgId}/academics/attendance`, icon: UserBadgeCheck },
    { title: m.common_exams_detail_schedule(), path: `/${currentOrgId}/academics/schedule`, icon: Calendar },
    { title: m.common_nav_exams(), path: `/${currentOrgId}/academics/exams`, icon: DocumentCheck },
    { title: m.common_control_label(), path: `/${currentOrgId}/academics/exams/control`, icon: AdjustmentsHorizontal },
    { title: m.common_nav_finance(), path: `/${currentOrgId}/finance`, icon: Banknotes },
    { title: m.finance_layout_nav_fees(), path: `/${currentOrgId}/finance/fees`, icon: DocumentChartBar },
    { title: m.finance_layout_nav_payments(), path: `/${currentOrgId}/finance/payments`, icon: CreditCard },
    { title: m.finance_layout_nav_cash_accounts(), path: `/${currentOrgId}/finance/accounts`, icon: BuildingLibrary },
    { title: m.common_je_title(), path: `/${currentOrgId}/finance/journal-entries`, icon: BookOpen },
    { title: m.finance_layout_nav_ledger(), path: `/${currentOrgId}/finance/statement`, icon: DocumentChartBar },
    { title: m.finance_layout_nav_reports(), path: `/${currentOrgId}/finance/reports`, icon: ChartBar },
    { title: m.common_autopilot_popover_ai_center(), path: `/${currentOrgId}/ai-center`, icon: Bolt },
    { title: m.settings_nav_templates(), path: `/${currentOrgId}/templates`, icon: DocumentPage },
    { title: m.common_word_settings(), path: `/${currentOrgId}/settings`, icon: Settings },
    { title: m.settings_nav_users(), path: `/${currentOrgId}/settings/users`, icon: Users, keywords: ["مستخدمين", "حسابات المستخدمين"] },
    { title: m.settings_nav_permissions(), path: `/${currentOrgId}/settings/permissions`, icon: ShieldCheck },
    { title: m.common_search_group_groups(), path: `/${currentOrgId}/settings/groups`, icon: Users },
    { title: m.settings_nav_units(), path: `/${currentOrgId}/settings/units`, icon: BuildingOffice },
    { title: m.settings_sidebar_branches(), path: `/${currentOrgId}/settings/branches`, icon: BuildingOffice },
    { title: m.settings_nav_periods(), path: `/${currentOrgId}/settings/periods`, icon: Calendar },
    { title: m.common_nav_rooms(), path: `/${currentOrgId}/settings/rooms`, icon: Pin },
    { title: m.common_exams_detail_subjects(), path: `/${currentOrgId}/settings/subjects`, icon: BookOpen },
    { title: m.common_nav_notifications(), path: `/${currentOrgId}/settings/notifications`, icon: AppNotification },
    { title: m.common_nav_security(), path: `/${currentOrgId}/settings/security`, icon: ShieldCheck },
    { title: m.settings_nav_currencies(), path: `/${currentOrgId}/settings/currencies`, icon: GlobeAlt, keywords: ["عملات", "عملة", "الصرف"] },
    { title: m.common_breadcrumb_fees(), path: `/${currentOrgId}/settings/fee-templates`, icon: DocumentPage },
    { title: m.common_form_summary_discount(), path: `/${currentOrgId}/settings/discount-templates`, icon: ReceiptPercent },
    { title: m.common_account_settings_title(), path: `/${currentOrgId}/account`, icon: Settings },
  ]);

  const routeAliases: Record<string, string[]> = {
    students: ["طلاب", "طالب", "التلاميذ"],
    employees: ["موظفون", "موظفين", "معلمون", "معلمين"],
    reports: ["تقارير", "تقرير", "تحليلات"],
    academics: ["أكاديميات", "دراسة", "تعليم"],
    classes: ["فصول", "الفصول", "صفوف", "الصفوف", "شعب"],
    attendance: ["حضور", "غياب", "الحضور والغياب"],
    exams: ["اختبارات", "امتحانات", "اختبار"],
    schedule: ["جدول", "الجدول الدراسي", "مواعيد"],
    finance: ["مالية", "المالية", "محاسبة"],
    fees: ["رسوم", "الرسوم"],
    payments: ["مدفوعات", "المدفوعات", "سداد"],
    accounts: ["حسابات", "الحسابات", "حسابات مالية"],
    "journal-entries": ["قيود", "قيود يومية", "القيود اليومية"],
    statement: ["كشف حساب", "كشف الحساب", "أستاذ عام"],
    settings: ["إعدادات", "الاعدادات", "الضبط"],
    periods: ["فترات", "الفترات الدراسية", "الفصل الدراسي"],
    subjects: ["مواد", "المواد", "مقررات", "المقررات"],
    rooms: ["قاعات", "القاعات", "غرف"],
    branches: ["فروع", "الفروع"],
    units: ["وحدات", "الوحدات التنظيمية"],
    users: ["مستخدمون", "مستخدمين", "المستخدمون"],
    groups: ["مجموعات", "المجموعات"],
    permissions: ["صلاحيات", "الصلاحيات"],
    currencies: ["عملات", "العملات", "عملة", "الصرف"],
    templates: ["قوالب", "النماذج"],
    audit: ["تدقيق", "سجل التدقيق", "سجل العمليات"],
    notifications: ["إشعارات", "الإشعارات", "تنبيهات"],
    security: ["أمان", "الأمان", "حماية"],
    billing: ["فوترة", "الفوترة", "اشتراك"],
    onboarding: ["تهيئة", "الإعداد الأولي"],
  };

  const routePageModules = import.meta.glob(
    "../../routes/(app)/[orgId]/**/+page.svelte",
  );
  const systemPages = $derived.by((): PageEntry[] => {
    const pages = [...knownSystemPages];
    const knownPaths = new Set(pages.map((item) => item.path));

    for (const filePath of Object.keys(routePageModules)) {
      const route = filePath.match(/\/\[orgId\](\/.*)\/\+page\.svelte$/);
      if (!route) continue;

      const routePath = route[1] === "/" ? "" : route[1];
      if (/\[[^/]+\]/.test(routePath)) continue;

      const path = `/${currentOrgId}${routePath}`;
      if (knownPaths.has(path)) continue;

      const segment = routePath.split("/").filter(Boolean).at(-1);
      if (!segment) continue;

      const title = segment
        .replace(/[-_]/g, " ")
        .replace(/\b\w/g, (character) => character.toUpperCase());
      const keywords = routePath
        .split("/")
        .filter(Boolean)
        .flatMap((part) => routeAliases[part] ?? []);
      const icon = routePath.includes("/finance/")
        ? Banknotes
        : routePath.includes("/settings/")
          ? Settings
          : routePath.includes("/academics/")
            ? BookOpen
            : ChartBar;

      pages.push({ title, path, icon, keywords });
      knownPaths.add(path);
    }

    return pages.map((item) => ({
      ...item,
      keywords: [
        ...new Set([
          ...(item.keywords ?? []),
          ...item.path
            .split("/")
            .filter((part) => part && part !== currentOrgId)
            .flatMap((part) => routeAliases[part] ?? []),
        ]),
      ],
    }));
  });

  function normalizeSearchText(value: unknown): string {
    return normalizeArabic(normalizeDigits(String(value ?? "").toLowerCase()));
  }

  function matchesQuery(query: string, ...values: unknown[]): boolean {
    const terms = normalizeSearchText(query).split(/\s+/).filter(Boolean);
    const searchableText = values.map(normalizeSearchText).join(" ");
    return terms.every((term) => searchableText.includes(term));
  }

  function matchesIndexedText(text: string): boolean {
    return searchTerms.every((term) => text.includes(term));
  }

  const filteredItems = $derived.by((): SearchItem[] => {
    const query = debouncedQuery;
    const list: SearchItem[] = [];

    if (!currentQuery) {
      list.push(
        ...shortcuts
          .map((shortcut) => ({ ...shortcut, subtitle: undefined })),
      );
      return list;
    }

    if (!queryIsSettled || !query) return list;

    list.push(
      ...shortcuts
        .filter((shortcut) => matchesQuery(query, shortcut.title))
        .map((shortcut) => ({ ...shortcut, subtitle: undefined })),
    );

    list.push(
      ...systemPages
        .filter((item) =>
          matchesQuery(query, item.title, item.path, ...(item.keywords ?? [])),
        )
        .map((item) => ({
          id: `page-${item.path}`,
          title: item.title,
          icon: item.icon,
          action: () => goto(item.path),
          group: m.common_search_group_system_pages(),
        })),
    );

    if (query.length >= 2) {
      list.push(
        ...studentResults.slice(0, 8).map((result) => {
          const student = result.student;
          const person = result.person;
          return {
            id: `student-${student.id}`,
            title:
              person.display_name ||
              person.full_name ||
              student.full_name ||
              student.name_ar ||
              m.common_word_students(),
            subtitle: student.student_number,
            icon: Users,
            action: () => goto(`/${currentOrgId}/students/${student.id}`),
            group: m.common_word_students(),
          };
        }),
      );

      list.push(
        ...employeeResults.slice(0, 8).map((employee) => ({
          id: `employee-${employee.id}`,
          title: employee.display_name || employee.full_name_ar || employee.full_name,
          subtitle: [employee.employee_number, employee.job_title]
            .filter(Boolean)
            .join(" · "),
          icon: EmployeeAffairs,
          action: () => goto(`/${currentOrgId}/employees`),
          group: m.employees_nav_title(),
        })),
      );
    }

    list.push(
      ...indexedSections
        .filter(({ text }) => matchesIndexedText(text))
        .slice(0, 8)
        .map(({ item: section }) => ({
          id: `section-${section.id}`,
          title: section.name_ar || section.name,
          subtitle: [section.code, section.level, section.track]
            .filter(Boolean)
            .join(" · "),
          icon: BookOpen,
          action: () =>
            goto(`/${currentOrgId}/academics/sections/${section.id}`),
          group: m.common_search_group_classes(),
        })),
    );

    list.push(
      ...indexedSubjects
        .filter(({ text }) => matchesIndexedText(text))
        .slice(0, 8)
        .map(({ item: subject }) => ({
          id: `subject-${subject.id}`,
          title: subject.name_ar || subject.name,
          subtitle: subject.code,
          icon: BookOpen,
          action: () => goto(`/${currentOrgId}/settings/subjects`),
          group: m.common_search_group_subjects(),
        })),
    );

    list.push(
      ...indexedRooms
        .filter(({ text }) => matchesIndexedText(text))
        .slice(0, 8)
        .map(({ item: room }) => ({
          id: `room-${room.id}`,
          title: room.name_ar || room.name,
          subtitle: room.room_type,
          icon: Pin,
          action: () => goto(`/${currentOrgId}/settings/rooms`),
          group: m.common_search_group_rooms(),
        })),
    );

    list.push(
      ...indexedOrgUnits
        .filter(({ text }) => matchesIndexedText(text))
        .slice(0, 8)
        .map(({ item: unit }) => ({
          id: `unit-${unit.id}`,
          title: unit.name_ar || unit.name,
          subtitle: [unit.code, unit.unit_type].filter(Boolean).join(" · "),
          icon: BuildingOffice,
          action: () => goto(`/${currentOrgId}/settings/units`),
          group: m.common_search_group_org_units(),
        })),
    );

    list.push(
      ...indexedGroups
        .filter(({ text }) => matchesIndexedText(text))
        .slice(0, 8)
        .map(({ item: group }) => ({
          id: `group-${group.id}`,
          title: group.name_ar || group.name,
          subtitle: group.description || undefined,
          icon: Users,
          action: () => goto(`/${currentOrgId}/settings/groups/${group.id}`),
          group: m.common_search_group_groups(),
        })),
    );

    list.push(
      ...indexedUsers
        .filter(({ text }) => matchesIndexedText(text))
        .slice(0, 8)
        .map(({ item: user }) => ({
          id: `user-${user.id}`,
          title: user.full_name_ar || user.full_name,
          subtitle: [user.email, user.phone, ...(user.roles ?? [])]
            .filter(Boolean)
            .join(" · "),
          icon: Users,
          action: () => goto(`/${currentOrgId}/settings/users/${user.id}`),
          group: m.common_search_group_users(),
        })),
    );

    list.push(
      ...indexedPeriods
        .filter(({ text }) => matchesIndexedText(text))
        .slice(0, 8)
        .map(({ item: period }) => ({
          id: `period-${period.id}`,
          title: period.name_ar || period.name,
          subtitle: [period.code, period.period_type, period.status]
            .filter(Boolean)
            .join(" · "),
          icon: Calendar,
          action: () => goto(`/${currentOrgId}/settings/periods`),
          group: m.common_search_group_periods(),
        })),
    );

    list.push(
      ...indexedCurrencies
        .filter(({ text }) => matchesIndexedText(text))
        .slice(0, 8)
        .map(({ item: currency }) => ({
          id: `currency-${currency.code}`,
          title: currency.name,
          subtitle: [currency.code, currency.symbol].filter(Boolean).join(" · "),
          icon: GlobeAlt,
          action: () => goto(`/${currentOrgId}/settings/currencies`),
          group: m.common_search_group_currencies(),
        })),
    );

    list.push(
      ...indexedAccounts
        .filter(({ text }) => matchesIndexedText(text))
        .slice(0, 8)
        .map(({ item: account }) => ({
          id: `account-${account.id}`,
          title: account.name_ar || account.name,
          subtitle: [account.code, account.account_type, account.currency]
            .filter(Boolean)
            .join(" · "),
          icon: BuildingLibrary,
          action: () => goto(`/${currentOrgId}/finance/accounts`),
          group: m.common_search_group_accounts(),
        })),
    );

    list.push(
      ...indexedFeeTemplates
        .filter(({ text }) => matchesIndexedText(text))
        .slice(0, 8)
        .map(({ item: template }) => ({
          id: `fee-template-${template.id}`,
          title: template.name_ar || template.name,
          subtitle: [template.code, template.category, template.currency]
            .filter(Boolean)
            .join(" · "),
          icon: DocumentPage,
          action: () => goto(`/${currentOrgId}/settings/fee-templates`),
          group: m.common_search_group_fee_templates(),
        })),
    );

    list.push(
      ...indexedDiscountTemplates
        .filter(({ text }) => matchesIndexedText(text))
        .slice(0, 8)
        .map(({ item: template }) => ({
          id: `discount-template-${template.id}`,
          title: template.name_ar || template.name,
          subtitle: [template.code, template.discount_type, template.value]
            .filter(Boolean)
            .join(" · "),
          icon: ReceiptPercent,
          action: () =>
            goto(`/${currentOrgId}/settings/discount-templates`),
          group: m.common_search_group_discount_templates(),
        })),
    );

    return list;
  });

  const groupedItems = $derived.by((): SearchItemGroup[] => {
    const grouped = new Map<string, SearchItem[]>();
    for (const item of filteredItems) {
      const items = grouped.get(item.group) ?? [];
      items.push(item);
      grouped.set(item.group, items);
    }
    return [...grouped].map(([label, items]) => ({ label, items }));
  });

  const effectiveSelectedIndex = $derived(
    Math.min(selectedIndex, Math.max(filteredItems.length - 1, 0)),
  );
  const selectedItem = $derived(
    filteredItems[effectiveSelectedIndex] ?? null,
  );

  function triggerAction(item: SearchItem | null) {
    if (!item) return;
    void item.action();
    uiStore.setSearch(false);
  }

  function handleKeydown(event: KeyboardEvent) {
    if (!uiStore.showSearch) return;

    if (event.key === "ArrowDown" && filteredItems.length > 0) {
      event.preventDefault();
      moveSelection((effectiveSelectedIndex + 1) % filteredItems.length);
    } else if (event.key === "ArrowUp" && filteredItems.length > 0) {
      event.preventDefault();
      moveSelection(
        (effectiveSelectedIndex - 1 + filteredItems.length) %
          filteredItems.length,
      );
    } else if (event.key === "Enter") {
      event.preventDefault();
      triggerAction(selectedItem);
    } else if (event.key === "Escape") {
      event.preventDefault();
      uiStore.setSearch(false);
    }
  }

  function moveSelection(index: number) {
    selectedIndex = index;
    const selectedId = filteredItems[index]?.id;
    if (!selectedId) return;

    requestAnimationFrame(() => {
      document
        .getElementById(`search-result-${selectedId}`)
        ?.scrollIntoView({ block: "nearest" });
    });
  }

  function handleSearchInput(event: Event) {
    const target = event.currentTarget;
    if (!(target instanceof HTMLInputElement)) return;

    searchQuery = target.value;
    selectedIndex = 0;
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      debouncedQuery = searchQuery.trim();
    }, 180);
  }

  function handleOpenChange(open: boolean) {
    if (open) {
      if (debounceTimer) {
        clearTimeout(debounceTimer);
        debounceTimer = undefined;
      }
      searchQuery = "";
      debouncedQuery = "";
      selectedIndex = 0;
      requestAnimationFrame(() => inputEl?.focus());
    } else if (debounceTimer) {
      clearTimeout(debounceTimer);
      debounceTimer = undefined;
    }

    uiStore.setSearch(open);
  }

  function askOrbisAboutSearch() {
    const query = searchQuery.trim();
    if (!query) return;

    const newUrl = new URL(page.url.href);
    newUrl.searchParams.set("ask_orbis", query);
    void goto(newUrl.pathname + newUrl.search, {
      replaceState: true,
      keepFocus: true,
      noScroll: true,
    });
    uiStore.setSearch(false);
  }
</script>

<Dialog
  open={uiStore.showSearch}
  onOpenChange={handleOpenChange}
>
  <DialogContent
    showCloseButton={false}
    class="overflow-hidden p-0 max-w-full w-full md:max-w-185 h-133.75 m-0 select-none flex flex-col focus:outline-none"
  >
    <div class="px-4 py-3 shrink-0">
      <div
        class="flex items-center gap-2 rounded-lg border border-input bg-background p-1.5 shadow-sm transition-colors focus-within:border-ring focus-within:ring-2 focus-within:ring-ring/20"
      >
        <Search class="ms-2 size-4 shrink-0 text-muted-foreground" />
        <input
          bind:this={inputEl}
          type="text"
          dir="auto"
          placeholder={m.common_header_search()}
          aria-label={m.common_header_search()}
          value={searchQuery}
          oninput={handleSearchInput}
          onkeydown={handleKeydown}
          class="min-w-0 flex-1 bg-transparent px-1 py-2 text-sm text-foreground outline-none placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-0 focus-visible:ring-offset-0"
        />
        <Button
          variant="outline"
          size="sm"
          disabled={!searchQuery.trim()}
          onclick={askOrbisAboutSearch}
          class="shrink-0 gap-1.5 border-primary/20 bg-primary/5 px-3 font-semibold text-primary shadow-sm transition-colors hover:bg-primary/10 hover:text-primary disabled:opacity-50"
          aria-label={m.common_search_ask_orbis_hint()}
          title={m.common_search_ask_orbis_hint()}
        >
          <Sparkles class="size-3.5" />
          <span>{m.shared_ui_ask_orbis()}</span>
        </Button>
      </div>
    </div>

    <SearchModalResults
      groups={groupedItems}
      selectedItemId={selectedItem?.id ?? null}
      {isSearching}
      error={searchError}
      onSelect={triggerAction}
    />
    <SearchModalFooter />
  </DialogContent>
</Dialog>
