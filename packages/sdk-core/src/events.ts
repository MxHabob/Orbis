/**
 * Event type constants — mirrors all contracts/events/event_types modules.
 *
 * Used by: EventBus subscribers, Autopilot, AI Agents.
 */
export const EventTypes = {
  // Academic
  CLASS_CREATED: "class.created",
  CLASS_UPDATED: "class.updated",
  CLASS_DELETED: "class.deleted",
  SUBJECT_CREATED: "subject.created",
  ORG_UNIT_CREATED: "org_unit.created",
  ENROLLMENT_CREATED: "enrollment.created",
  ENROLLMENT_WITHDRAWN: "enrollment.withdrawn",
  ENROLLMENT_TRANSFERRED: "enrollment.transferred",

  // Assessment
  ASSESSMENT_CREATED: "assessment.created",
  ASSESSMENT_UPDATED: "assessment.updated",

  // Finance
  PAYMENT_RECORDED: "payment.recorded",
  PAYMENT_REVERSED: "payment.reversed",
  FEE_ASSIGNED: "fee.assigned",
  FEE_WAIVED: "fee.waived",
  PAYMENT_OVERDUE: "payment.overdue",
  TREASURY_DEPOSIT: "treasury.deposit",
  TREASURY_WITHDRAWAL: "treasury.withdrawal",

  // Scheduling
  SCHEDULING_CREATED: "scheduling.created",
  SCHEDULING_UPDATED: "scheduling.updated",

  // Exam
  EXAM_CREATED: "exam.created",
  EXAM_UPDATED: "exam.updated",

  // IAM
  USER_CREATED: "user.created",
  USER_UPDATED: "user.updated",
  USER_DELETED: "user.deleted",
  USER_LOGIN: "user.login",
  USER_LOGOUT: "user.logout",
  ROLE_CHANGED: "user.role_changed",
  PASSWORD_CHANGED: "user.password_changed",

  // Organization
  ORGANIZATION_CREATED: "organization.created",
  ORGANIZATION_UPDATED: "organization.updated",
  SETTINGS_CHANGED: "organization.settings_changed",
  POLICY_CHANGED: "organization.policy_changed",
} as const;

export type EventType = (typeof EventTypes)[keyof typeof EventTypes];
