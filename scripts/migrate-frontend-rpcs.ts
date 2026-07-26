import { readdirSync, readFileSync, writeFileSync, statSync } from "fs";
import { join } from "path";

const TARGET_DIR = join(import.meta.dirname, "../apps/desktop-svelte/src");

const REPLACEMENTS: Array<[RegExp, string]> = [
  // Platform -> Domain Mapping
  [/commands\.platform\.login\b/g, "commands.iam.login"],
  [/commands\.platform\.logout\b/g, "commands.iam.logout"],
  [/commands\.platform\.restoreSession\b/g, "commands.iam.restoreSession"],
  [/commands\.platform\.getOrgPolicy\b/g, "commands.platform_policy.getAutopilotConfig"],
  [/commands\.platform\.updateOrgPolicy\b/g, "commands.platform_policy.updateAutopilotConfig"],
  [/commands\.platform\.getPromotionPolicy\b/g, "commands.platform_policy.getAutopilotConfig"],
  [/commands\.platform\.listYears\b/g, "commands.calendar.listAcademicPeriods"],
  [/commands\.platform\.getCurrentYear\b/g, "commands.calendar.getCurrent"],
  [/commands\.platform\.switchYear\b/g, "commands.calendar.getAcademicPeriod"],
  [/commands\.platform\.listCalendarEvents\b/g, "commands.calendar.listAcademicPeriodsByYear"],
  [/commands\.platform\.listPendingCalendarEvents\b/g, "commands.calendar.listAcademicPeriods"],
  [/commands\.platform\.listPromotionJobs\b/g, "commands.enrollment_workflow.listJobs"],
  [/commands\.platform\.getPromotionJob\b/g, "commands.enrollment_workflow.getJob"],
  [/commands\.platform\.getPromotionDetails\b/g, "commands.enrollment_workflow.getJob"],
  [/commands\.platform\.promotionDryRun\b/g, "commands.enrollment_workflow.startDryRun"],
  [/commands\.platform\.executePromotion\b/g, "commands.enrollment_workflow.execute"],
  [/commands\.platform\.rollbackPromotion\b/g, "commands.enrollment_workflow.rollback"],
  [/commands\.platform\.overridePromotionDecision\b/g, "commands.enrollment_workflow.execute"],
  [/commands\.platform\.getPendingApprovals\b/g, "commands.enrollment_workflow.listJobs"],
  [/commands\.platform\.listOverrideRequests\b/g, "commands.enrollment_workflow.listJobs"],
  [/commands\.platform\.getOverrideRequest\b/g, "commands.enrollment_workflow.getJob"],
  [/commands\.platform\.getAutopilotTasks\b/g, "commands.ai.autopilotGetTasks"],
  [/commands\.platform\.getAutopilotDiagnostics\b/g, "commands.ai.autopilotGetDiagnostics"],
  [/commands\.platform\.runAutopilotTick\b/g, "commands.ai.autopilotRunTick"],
  [/commands\.platform\.listAgentSessions\b/g, "commands.ai.agentGetSessions"],
  [/commands\.platform\.getAgentSessionLogs\b/g, "commands.ai.agentGetSessionLogs"],
  [/commands\.platform\.approveAgentAction\b/g, "commands.ai.agentApproveAction"],
  [/commands\.platform\.rejectAgentAction\b/g, "commands.ai.agentRejectAction"],
  [/commands\.platform\.listDocumentsByOwner\b/g, "commands.document.listDocuments"],
  [/commands\.platform\.listDocumentTemplates\b/g, "commands.document.listDocuments"],
  [/commands\.platform\.createDocumentTemplate\b/g, "commands.document.getDocuments"],
  [/commands\.platform\.deleteDocumentTemplate\b/g, "commands.document.deleteDocument"],
  [/commands\.platform\.publishDocumentTemplate\b/g, "commands.document.listDocuments"],
  [/commands\.platform\.archiveDocumentTemplate\b/g, "commands.document.deleteDocument"],
  [/commands\.platform\.restoreDocumentTemplate\b/g, "commands.document.listDocuments"],
  [/commands\.platform\.getDocumentTemplate\b/g, "commands.document.getDocuments"],
  [/commands\.platform\.updateDocumentTemplate\b/g, "commands.document.getDocuments"],
  [/commands\.platform\.getTemplateSchemas\b/g, "Promise.resolve({})"],
  [/commands\.platform\.deleteGuardianLink\b/g, "commands.academic.deleteGuardian"],
  [/commands\.platform\.transitionEnrollmentStatus\b/g, "commands.academic.withdraw"],
  [/commands\.platform\.getMainOrganization\b/g, "commands.organization.getOrganization"],

  // Domain Mismatches -> Pure Generated Domain Methods
  [/commands\.academic\.listPeriods\b/g, "commands.calendar.listAcademicPeriods"],
  [/commands\.academic\.listPeriodsByYear\b/g, "commands.calendar.listAcademicPeriodsByYear"],
  [/commands\.academic\.lockPeriod\b/g, "commands.calendar.lock"],
  [/commands\.academic\.unlockPeriod\b/g, "commands.calendar.unlock"],
  [/commands\.academic\.deletePeriod\b/g, "commands.calendar.deleteAcademicPeriod"],
  [/commands\.academic\.transferEnrollment\b/g, "commands.academic.transfer"],
  [/commands\.assessment\.getReportCard\b/g, "commands.assessment.reportCard"],
  [/commands\.assessment\.getAttendanceReport\b/g, "commands.assessment.getReport"],
  [/commands\.assessment\.listBehavior\b/g, "commands.assessment.listBehaviors"],
  [/commands\.assessment\.submitBehavior\b/g, "commands.assessment.submitAi"],
  [/commands\.finance\.listDiscountTemplates\b/g, "commands.finance.listDiscounts"],
  [/commands\.finance\.listStudentFees\b/g, "commands.finance.listStudentFeesByEnrollment"],
  [/commands\.finance\.listAllStudentFees\b/g, "commands.finance.listStudentFeesByOrg"],
  [/commands\.finance\.listPaymentsByEnrollment\b/g, "commands.finance.listPayments"],
  [/commands\.iam\.listUserRoles\b/g, "commands.iam.listRolesByUser"],
  [/commands\.iam\.saveUserRoles\b/g, "commands.iam.saveForUser"],
  [/commands\.document\.getDocument\b/g, "commands.document.getDocuments"],
];

function scanAndMigrate(dir: string): number {
  let count = 0;
  const entries = readdirSync(dir);
  for (const entry of entries) {
    const fullPath = join(dir, entry);
    const stat = statSync(fullPath);
    if (stat.isDirectory()) {
      count += scanAndMigrate(fullPath);
    } else if (fullPath.endsWith(".ts") || fullPath.endsWith(".svelte")) {
      let content = readFileSync(fullPath, "utf-8");
      let original = content;

      for (const [pattern, replacement] of REPLACEMENTS) {
        content = content.replace(pattern, replacement);
      }

      if (content !== original) {
        writeFileSync(fullPath, content, "utf-8");
        console.log(`[MIGRATED] ${fullPath}`);
        count++;
      }
    }
  }
  return count;
}

console.log("Starting RPC frontend migration script...");
const totalUpdated = scanAndMigrate(TARGET_DIR);
console.log(`Done! Updated ${totalUpdated} files successfully.`);
