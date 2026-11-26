import * as m from "$lib/paraglide/messages";

interface RouteNode {
  segment: string;
  label: () => string;
  children?: RouteNode[];
}

const ROUTE_TREE: RouteNode[] = [
  { segment: "approvals", label: () => m.settings_nav_approvals() },
  { segment: "billing", label: () => m.license_nav_title() },
  { segment: "notifications", label: () => m.common_nav_notifications() },
  { segment: "reports", label: () => m.common_nav_reports() },
  { segment: "templates", label: () => m.settings_nav_templates() },
  { segment: "employees", label: () => m.operations_ui_report_category_hr() },
  {
    segment: "account",
    label: () => m.common_account_settings_title(),
    children: [
      { segment: "security", label: () => m.common_account_menu_security() },
      { segment: "appearance", label: () => m.appearance_nav_label() },
    ],
  },
  {
    segment: "ai-center",
    label: () => m.shared_ui_ask_orbis(),
    children: [
      { segment: "ai-logs", label: () => m.common_nav_audit() },
      { segment: "integrations", label: () => m.ai_center_nav_integrations() },
      { segment: "live", label: () => m.operations_ui_metrics_analytics() },
      { segment: "permissions", label: () => m.settings_nav_permissions() },
      { segment: "settings", label: () => m.common_word_settings() },
    ],
  },
  {
    segment: "students",
    label: () => m.common_word_students(),
    children: [{ segment: "new", label: () => m.common_new() }],
  },
  {
    segment: "academics",
    label: () => m.common_nav_academics(),
    children: [
      {
        segment: "sections",
        label: () => m.common_search_group_classes(),
        children: [
          { segment: "students", label: () => m.common_word_students() },
          { segment: "curriculum", label: () => m.common_search_group_subjects() },
        ],
      },
      { segment: "classes", label: () => m.common_search_group_classes() },
      { segment: "schedule", label: () => m.common_exams_detail_schedule() },
      {
        segment: "exams",
        label: () => m.common_nav_exams(),
        children: [
          { segment: "sessions", label: () => m.common_exams_sessions_title() },
          { segment: "reports", label: () => m.common_nav_reports() },
        ],
      },
      { segment: "grades", label: () => m.common_breadcrumb_grades() },
      { segment: "attendance", label: () => m.common_nav_attendance() },
      { segment: "calendar", label: () => m.settings_nav_academic_calendars() },
      { segment: "promotion", label: () => m.academics_overview_card_promotion_title() },
      { segment: "reports", label: () => m.ui_page_academics_reports() },
      { segment: "results", label: () => m.academics_nav_results_review() },
    ],
  },
  {
    segment: "finance",
    label: () => m.common_nav_finance(),
    children: [
      { segment: "fees", label: () => m.finance_layout_nav_fees() },
      { segment: "payments", label: () => m.finance_layout_nav_payments() },
      { segment: "accounts", label: () => m.finance_layout_nav_cash_accounts() },
      { segment: "disbursements", label: () => m.finance_layout_nav_disbursements() },
      { segment: "journal-entries", label: () => m.common_je_title() },
      { segment: "statement", label: () => m.finance_layout_nav_ledger() },
      { segment: "ledger", label: () => m.finance_layout_nav_ledger() },
      { segment: "reports", label: () => m.finance_layout_nav_reports() },
    ],
  },
  {
    segment: "settings",
    label: () => m.common_word_settings(),
    children: [
      { segment: "notifications", label: () => m.common_nav_notifications() },
      { segment: "security", label: () => m.common_nav_security() },
      { segment: "users", label: () => m.settings_nav_users() },
      { segment: "groups", label: () => m.groups_nav_title() },
      { segment: "permissions", label: () => m.settings_nav_permissions() },
      { segment: "periods", label: () => m.settings_nav_cycles() },
      { segment: "academic-calendars", label: () => m.settings_nav_academic_calendars() },
      { segment: "academic-policy", label: () => m.common_nav_academic_policy() },
      { segment: "assessment-schemes", label: () => m.settings_nav_assessment_schemes() },
      { segment: "units", label: () => m.settings_nav_units() },
      { segment: "branches", label: () => m.common_org_unit_branch() },
      { segment: "rooms", label: () => m.common_nav_rooms() },
      { segment: "subjects", label: () => m.common_exams_detail_subjects() },
      { segment: "audit", label: () => m.common_nav_audit() },
      {
        segment: "archive",
        label: () => m.settings_nav_archive(),
        children: [{ segment: "trash", label: () => m.ui_page_settings_archive_trash() }],
      },
      { segment: "currencies", label: () => m.settings_nav_currencies() },
      { segment: "ordering", label: () => m.common_nav_ordering() },
      { segment: "fee-templates", label: () => m.common_breadcrumb_fees() },
      { segment: "discount-templates", label: () => m.common_form_summary_discount() },
      { segment: "login-rules", label: () => m.settings_nav_login_rules() },
      { segment: "registration", label: () => m.student_registration_policy_title() },
      { segment: "mulverse", label: () => m.settings_nav_orbis_mobile() },
      { segment: "database", label: () => m.settings_nav_database() },
      { segment: "bulk-input", label: () => m.settings_nav_import_export() },
      { segment: "calculation-engine", label: () => m.settings_nav_calculation_engine() },
    ],
  },
];

const STUDENT_SUB_ROUTES: Record<string, () => string> = {
  edit: () => m.common_detail_edit_data(),
  grades: () => m.common_breadcrumb_grades(),
  fees: () => m.common_breadcrumb_fees(),
  attendance: () => m.common_promo_attendance(),
  behavior: () => m.common_breadcrumb_behavior(),
  documents: () => m.settings_nav_archive(),
  guardians: () => m.common_guardians_roles_title(),
};

export function getRouteTitle(params: {
  pathname: string;
  orgId: string;
  studentId?: string;
  fallback?: string;
}): string {
  const { pathname, orgId, studentId, fallback } = params;
  const cleanPath = pathname.replace(/\/+$/, "");
  const orgPrefix = `/${orgId}`;
  const relative = cleanPath.startsWith(orgPrefix)
    ? cleanPath.slice(orgPrefix.length)
    : cleanPath;
  const segments = relative.split("/").filter(Boolean);

  if (segments.length === 0) return m.common_nav_overview();

  if (segments[0] === "students" && studentId) {
    const sub = segments[2];
    return sub && STUDENT_SUB_ROUTES[sub]
      ? STUDENT_SUB_ROUTES[sub]()
      : m.common_breadcrumb_student_profile();
  }

  let currentNodes: RouteNode[] | undefined = ROUTE_TREE;
  let matched: RouteNode | undefined;

  for (const seg of segments) {
    const node: RouteNode | undefined = currentNodes?.find((n) => n.segment === seg);
    if (!node) continue;
    matched = node;
    currentNodes = node.children;
  }

  return matched ? matched.label() : (fallback ?? m.common_nav_overview());
}
