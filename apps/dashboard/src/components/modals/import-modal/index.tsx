"use client";

import { StudentImportModal } from "./students/student-import-modal";
import { GradeImportModal } from "./grades/grade-import-modal";

export function ImportModal() {
  return (
    <>
      <StudentImportModal />
      <GradeImportModal />
    </>
  );
}
