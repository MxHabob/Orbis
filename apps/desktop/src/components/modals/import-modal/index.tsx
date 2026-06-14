"use client";

import { InitializeAssessmentsModal } from "../grades/initialize-assessments-modal";
import { GradeImportModal } from "./grades/grade-import-modal";
import { StudentImportModal } from "./students/student-import-modal";

export function ImportModal() {
  return (
    <>
      <StudentImportModal />
      <GradeImportModal />
      <InitializeAssessmentsModal />
    </>
  );
}
