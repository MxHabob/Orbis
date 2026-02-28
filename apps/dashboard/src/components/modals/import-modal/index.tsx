"use client";

import { StudentImportModal } from "./students/student-import-modal";
import { GradeImportModal } from "./grades/grade-import-modal";
import { InitializeAssessmentsModal } from "../grades/initialize-assessments-modal";

export function ImportModal() {
  return (
    <>
      <StudentImportModal />
      <GradeImportModal />
      <InitializeAssessmentsModal />
    </>
  );
}
