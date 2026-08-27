export type { LoginInput } from "./iam.js";
export { loginInputSchema } from "./iam.js";
export {
	feeAssignmentInputSchema,
	feeTemplateInputSchema,
} from "./finance.js";
export type { FeeAssignmentInput, FeeTemplateInput } from "./finance.js";
export {
	createAssessmentInputSchema,
	createRoomInputSchema,
	createSubjectInputSchema,
	createTimeSlotInputSchema,
} from "./academic.js";
export type {
	CreateAssessmentInput,
	CreateRoomInput,
	CreateSubjectInput,
	CreateTimeSlotInput,
} from "./academic.js";
