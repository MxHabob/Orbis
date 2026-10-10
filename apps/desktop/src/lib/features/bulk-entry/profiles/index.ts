import type { BulkImportProfile, BulkProfileId } from "../types";
import { studentImportProfile } from "./student-profile";
import { employeeImportProfile } from "./employee-profile";
import { gradeImportProfile } from "./grade-profile";
import { teacherSubjectImportProfile } from "./teacher-subject-profile";

export { studentImportProfile, studentColumnMetadata, studentGridColumns } from "./student-profile";
export { employeeImportProfile, employeeColumnMetadata, employeeGridColumns } from "./employee-profile";
export { gradeImportProfile, gradeColumnMetadata, gradeGridColumns } from "./grade-profile";
export {
  teacherSubjectImportProfile,
  teacherSubjectColumnMetadata,
  teacherSubjectGridColumns,
} from "./teacher-subject-profile";

export const BULK_PROFILES: Record<BulkProfileId, BulkImportProfile<any, any>> = {
  students: studentImportProfile,
  employees: employeeImportProfile,
  grades: gradeImportProfile,
  "teacher-subjects": teacherSubjectImportProfile,
};

export function getBulkProfile(id: BulkProfileId): BulkImportProfile<any, any> {
  const profile = BULK_PROFILES[id];
  if (!profile) {
    throw new Error(`Bulk profile "${id}" not found`);
  }
  return profile;
}
