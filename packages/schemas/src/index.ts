export type { LoginInput } from "./iam.js";
export { loginInputSchema } from "./iam.js";
export { userCreateInputSchema } from "./users.js";
export type { UserCreateInput } from "./users.js";

export {
  discountTemplateInputSchema,
  currencyInputSchema,
  cashAccountInputSchema,
  feeAssignmentInputSchema,
  feeTemplateInputSchema,
  entityAccountInputSchema,
  generalLedgerAccountInputSchema,
  paymentRecordInputSchema,
} from "./finance.js";
export type {
  DiscountTemplateInput,
  CurrencyInput,
  CashAccountInput,
  FeeAssignmentInput,
  FeeTemplateInput,
  EntityAccountInput,
  GeneralLedgerAccountInput,
  PaymentRecordInput,
} from "./finance.js";

export {
  createAssessmentInputSchema,
  classFormInputSchema,
  classSubjectFormInputSchema,
  lessonFormInputSchema,
  createRoomInputSchema,
  createSubjectInputSchema,
  createTimeSlotInputSchema,
} from "./academic.js";
export type {
  CreateAssessmentInput,
  ClassFormInput,
  ClassSubjectFormInput,
  LessonFormInput,
  CreateRoomInput,
  CreateSubjectInput,
  CreateTimeSlotInput,
} from "./academic.js";

export { createPeriodInputSchema } from "./calendar.js";
export {
  createAppSettingsSchema,
  generalSettingsInputSchema,
} from "./settings.js";
export type { GeneralSettingsInput } from "./settings.js";
export { guardianFormInputSchema, studentRegistrationSchema } from "./students.js";
export type { GuardianFormInput, StudentRegistrationInput } from "./students.js";
export { examSessionInputSchema, examSubjectInputSchema } from "./exams.js";
export type { ExamSessionInput, ExamSubjectInput } from "./exams.js";
export { buildingInputSchema, templateInputSchema } from "./templates.js";
export type { BuildingInput, TemplateInput } from "./templates.js";
export type { CreatePeriodInput } from "./calendar.js";

export { orgUnitInputSchema } from "./organization.js";
export type { OrgUnitInput } from "./organization.js";
export { registrationSettingsSchema } from "./registration.js";
export type { RegistrationSettingsInput } from "./registration.js";

export { ledgerAccountInputSchema } from "./ledger.js";
export type { LedgerAccountInput } from "./ledger.js";
