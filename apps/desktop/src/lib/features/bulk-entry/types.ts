import type { OrbisColumnDef } from "@orbis/svelte-grid";
import type { BulkValidationContext } from "./engine/duplicate-index";

export type BulkProfileId = "students" | "employees" | "grades" | "teacher-subjects";

export type ValidationSeverity = "BLOCKING_ERROR" | "WARNING" | "INFO";

export interface BulkCellIssue {
  severity: ValidationSeverity;
  columnId: string;
  message: string;
  value?: unknown;
}

export interface BulkRowState {
  rowNumber: number; // 1-indexed Excel row
  data: Record<string, unknown>;
  errors: Map<string, string>; // columnId -> error message for OrbisGrid .is-error
  warnings: Map<string, string>; // columnId -> warning message
  info: Map<string, string>; // columnId -> info message
  status: "pending" | "valid" | "error" | "warning" | "committed" | "failed";
  commitError?: string;
}

export interface ReferenceItem {
  id: string;
  code?: string;
  name: string;
  nameAr?: string;
  meta?: Record<string, unknown>;
}

export interface BulkReferenceData {
  classes: ReferenceItem[];
  teachers: ReferenceItem[];
  sections: ReferenceItem[];
  levels: ReferenceItem[];
  subjects: ReferenceItem[];
  feeTemplates: ReferenceItem[];
  discountTemplates: ReferenceItem[];
  userGroups: ReferenceItem[];
  assessmentSchemes: ReferenceItem[];
  academicPeriods: ReferenceItem[];
  enums: {
    genders: Array<{ value: string; labelAr: string; labelEn: string }>;
    studyModes: Array<{ value: string; labelAr: string; labelEn: string }>;
    employeeTypes: Array<{ value: string; labelAr: string; labelEn: string }>;
    employmentTypes: Array<{ value: string; labelAr: string; labelEn: string }>;
    bloodTypes: string[];
    relationships: Array<{ value: string; labelAr: string; labelEn: string }>;
  };
  registrationFields?: Record<string, {
    active: boolean;
    required: boolean;
    preventDuplicate: boolean;
  }>;
  existingSectionSubjectAssignments: Array<{
    classId: string;
    subjectId: string;
    periodId: string;
    teacherId: string;
  }>;
}

export interface ColumnDefinitionMetadata {
  id: string;
  headerAr: string;
  headerEn: string;
  required: boolean;
  type: "text" | "number" | "integer" | "decimal" | "date" | "boolean" | "enum" | "reference";
  format?: string;
  descriptionAr?: string;
  descriptionEn?: string;
  headerAliases?: string[];
  example?: string;
  referenceSource?: keyof BulkReferenceData;
  allowedValues?: string[];
  options?: Array<{ value: string | number | boolean; label: string }>;
}

export interface BulkPreviewSummary<T = Record<string, unknown>> {
  totalRows: number;
  validRowsCount: number;
  errorRowsCount: number;
  warningRowsCount: number;
  details: T;
}

export interface StudentPreviewDetails {
  studentsToCreate: number;
  studentsToUpdate: number;
  personsToCreate: number;
  personsToLink: number;
  userAccountsToProvision: number;
  guardiansToLink: number;
  newParentAccounts: number;
  mandatoryFeesAssignedCount: number;
  totalEstimatedMandatoryFees: number;
  currency: string;
  capacityOverridesRequired: Array<{ studentName: string; className: string; reason: string }>;
  classPlacementsSummary: Record<string, number>;
}

export interface EmployeePreviewDetails {
  employeesToCreate: number;
  personsToCreate: number;
  personsToLink: number;
  userAccountsToProvision: number;
  departmentsSummary: Record<string, number>;
  userGroupsSummary: Record<string, number>;
}

export interface GradePreviewDetails {
  gradesToRecord: number;
  gradesToUpdate: number;
  subjectsSummary: Record<string, number>;
  assessmentType: string;
  academicPeriodName: string;
  maxScore: number;
}

export interface ChunkProgress {
  totalChunks: number;
  currentChunk: number;
  chunkSize: number;
  totalRows: number;
  processedRows: number;
  committedRows: number;
  failedRows: number;
  isFinished: boolean;
  hasErrors: boolean;
}

export interface BulkCommitResult {
  batchId: string;
  profileId: BulkProfileId;
  totalRows: number;
  committedCount: number;
  failedCount: number;
  skippedCount: number;
  licenseRejected: boolean;
  createdUsersCount?: number;
  assignedFeesCount?: number;
  provisionedCredentials?: Array<{
    name: string;
    email: string;
    username?: string;
    tempPassword?: string;
    role: string;
  }>;
  failedRowNumbers: number[];
  errors: Array<{ rowNumber: number; reason: string }>;
  warnings: Array<{ rowNumber: number; reason: string }>;
}

export interface BulkChunkCommitResult {
  success: boolean;
  errors?: string[];
  rowErrors?: Array<{ rowNumber: number; reason: string }>;
  warnings?: Array<{ rowNumber: number; reason: string }>;
}

export interface BulkImportProfile<TRow = Record<string, unknown>, TSummary = Record<string, unknown>> {
  id: BulkProfileId;
  displayName: { ar: string; en: string };
  description: { ar: string; en: string };
  templateVersion: string;
  columnMetadata: ColumnDefinitionMetadata[];
  gridColumns: OrbisColumnDef[];
  buildReferenceData: (orgId: string) => Promise<BulkReferenceData>;
  validateCell: (colId: string, value: unknown, row: TRow, refData: BulkReferenceData) => BulkCellIssue | null;
  validateRow: (
    row: TRow,
    allRows: TRow[],
    rowIndex: number,
    refData: BulkReferenceData,
    context?: BulkValidationContext,
  ) => BulkCellIssue[];
  generatePreview: (rows: TRow[], refData: BulkReferenceData) => Promise<BulkPreviewSummary<TSummary>>;
  commitChunk: (rows: TRow[], orgId: string, refData: BulkReferenceData) => Promise<BulkChunkCommitResult>;
}
