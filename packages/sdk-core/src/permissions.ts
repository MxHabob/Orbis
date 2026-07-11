/**
 * Permission constants — mirrors shared-kernel Permission enum.
 *
 * Single source: shared-kernel/src/traits.rs → Permission enum
 */
export const Permissions = {
  // Academic
  ClassRead: "class.read",
  ClassWrite: "class.write",
  ClassDelete: "class.delete",
  SubjectRead: "subject.read",
  SubjectWrite: "subject.write",
  SubjectDelete: "subject.delete",
  OrgUnitRead: "org_unit.read",
  OrgUnitWrite: "org_unit.write",
  OrgUnitDelete: "org_unit.delete",
  EnrollmentRead: "enrollment.read",
  EnrollmentWrite: "enrollment.write",
  EnrollmentTransfer: "enrollment.transfer",
  GuardianRead: "guardian.read",
  GuardianWrite: "guardian.write",
  GuardianDelete: "guardian.delete",
  DocumentRead: "document.read",
  DocumentWrite: "document.write",
  DocumentDelete: "document.delete",
  NotificationRead: "notification.read",

  // Assessment
  GradeRead: "grade.read",
  GradeWrite: "grade.write",
  GradeReport: "grade.report",
  GradeAnalytics: "grade.analytics",
  AttendanceRead: "attendance.read",
  AttendanceWrite: "attendance.write",
  AttendanceReport: "attendance.report",
  AttendanceAnalytics: "attendance.analytics",
  BehaviorRead: "behavior.read",
  BehaviorWrite: "behavior.write",
  BehaviorDelete: "behavior.delete",
  BehaviorAI: "behavior.ai",
  AssessmentRead: "assessment.read",
  AssessmentWrite: "assessment.write",
  AssessmentDelete: "assessment.delete",

  // Finance
  FinanceRead: "finance.read",
  FinanceWrite: "finance.write",

  // Scheduling
  ScheduleRead: "schedule.read",
  ScheduleWrite: "schedule.write",
  ScheduleDelete: "schedule.delete",

  // Exam
  ExamSessionRead: "exam_session.read",
  ExamSessionWrite: "exam_session.write",

  // IAM
  UserRead: "user.read",
  UserWrite: "user.write",
  UserDelete: "user.delete",
  RoleWrite: "role.write",

  // Organization
  CycleRead: "cycle.read",
  CycleWrite: "cycle.write",
  CycleLock: "cycle.lock",
  PromotionRead: "promotion.read",
  PromotionWrite: "promotion.write",
  PromotionExecute: "promotion.execute",
} as const;

export type PermissionCode = (typeof Permissions)[keyof typeof Permissions];
