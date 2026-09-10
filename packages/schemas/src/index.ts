export type {
  ClassFormInput,
  ClassSubjectFormInput,
  CreateAssessmentInput,
  CreateRoomInput,
  CreateSubjectInput,
  CreateTimeSlotInput,
  LessonFormInput,
} from "./academic.js";
export {
  classFormInputSchema,
  classSubjectFormInputSchema,
  createAssessmentInputSchema,
  createRoomInputSchema,
  createSubjectInputSchema,
  createTimeSlotInputSchema,
  lessonFormInputSchema,
} from "./academic.js";
export type { CreatePeriodInput } from "./calendar.js";
export { createPeriodInputSchema } from "./calendar.js";
export type { ExamSessionInput, ExamSubjectInput } from "./exams.js";
export { examSessionInputSchema, examSubjectInputSchema } from "./exams.js";
export type {
  CashAccountInput,
  CurrencyInput,
  DiscountTemplateInput,
  EntityAccountInput,
  FeeAssignmentInput,
  FeeTemplateInput,
  GeneralLedgerAccountInput,
  PaymentRecordInput,
} from "./finance.js";
export {
  cashAccountInputSchema,
  currencyInputSchema,
  discountTemplateInputSchema,
  entityAccountInputSchema,
  feeAssignmentInputSchema,
  feeTemplateInputSchema,
  generalLedgerAccountInputSchema,
  paymentRecordInputSchema,
} from "./finance.js";
export type { LoginInput } from "./iam.js";
export { loginInputSchema } from "./iam.js";
export type { LedgerAccountInput } from "./ledger.js";
export { ledgerAccountInputSchema } from "./ledger.js";
export type { OrgUnitInput } from "./organization.js";
export { orgUnitInputSchema } from "./organization.js";
export type { RegistrationSettingsInput } from "./registration.js";
export { registrationSettingsSchema } from "./registration.js";
export type { GeneralSettingsInput } from "./settings.js";
export {
  createAppSettingsSchema,
  generalSettingsInputSchema,
} from "./settings.js";
export type {
  GuardianFormInput,
  StudentRegistrationInput,
} from "./students.js";
export {
  guardianFormInputSchema,
  studentRegistrationSchema,
} from "./students.js";
export type { BuildingInput, TemplateInput } from "./templates.js";
export { buildingInputSchema, templateInputSchema } from "./templates.js";
export type { UserCreateInput } from "./users.js";
export { userCreateInputSchema } from "./users.js";
