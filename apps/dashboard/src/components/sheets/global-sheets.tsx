"use client";

import { ImportModal } from "@/components/modals/import-modal";
import { SearchModal } from "@/components/search/search-modal";
import { DocumentSheet } from "@/components/sheets/document-sheet";
import { EditRecurringSheet } from "@/components/sheets/edit-recurring-sheet";
import { TrackerCreateSheet } from "@/components/sheets/tracker-create-sheet";
import { TrackerScheduleSheet } from "@/components/sheets/tracker-schedule-sheet";
import { TrackerUpdateSheet } from "@/components/sheets/tracker-update-sheet";
import { StudentCreateSheet } from "@/components/sheets/student-create-sheet";
import { StudentEditSheet } from "@/components/sheets/student-edit-sheet";
import { StudentDetailsSheet } from "@/components/sheets/student-details-sheet";
import { ClassCreateSheet } from "@/components/sheets/class-create-sheet";
import { ClassEditSheet } from "@/components/sheets/class-edit-sheet";
import { SubjectCreateSheet } from "@/components/sheets/subject-create-sheet";
import { SubjectEditSheet } from "@/components/sheets/subject-edit-sheet";
import { CertificateSheet } from "@/components/sheets/certificate-sheet";
import { StudentCertificateSheet } from "@/components/sheets/student-certificate-sheet";
import { LessonCreateSheet } from "@/components/sheets/lesson-create-sheet";
import { LessonEditSheet } from "@/components/sheets/lesson-edit-sheet";
import { LessonScheduleSheet } from "@/components/sheets/lesson-schedule-sheet";
import { AcademicSheets } from "@/components/sheets/academic-sheets";

export function GlobalSheets() {
  return (
    <>
      {/* <TrackerUpdateSheet />
      <TrackerCreateSheet />
      <TrackerScheduleSheet /> */}

      {/* <CategoryCreateSheet />
      <CategoryEditSheet /> */}

      <StudentCreateSheet />
      <StudentEditSheet />
      <StudentDetailsSheet />
      <StudentCertificateSheet />
      <ClassCreateSheet />
      <ClassEditSheet />
      <SubjectCreateSheet />
      <SubjectEditSheet />
      {/* <CustomerDetailsSheet /> */}
      {/* <CustomerEditSheet /> */}

      {/* <ProductCreateSheet />
      <ProductEditSheet /> */}

      {/* <TransactionSheet />
      <TransactionCreateSheet />
      <TransactionEditSheet /> */}

      {/* <SelectBankAccountsModal /> */}

      {/* <SearchModal /> */}

      <DocumentSheet />
      {/* <InboxDetailsSheet /> */}

      <ImportModal />
      {/* <ConnectTransactionsModal /> */}

      {/* <InvoiceDetailsSheet /> */}
      {/* <InvoiceSheet /> */}
      {/* <EditRecurringSheet /> */}
      <CertificateSheet />
      <LessonCreateSheet />
      <LessonEditSheet />
      <LessonScheduleSheet />
      <AcademicSheets />
    </>
  );
}
