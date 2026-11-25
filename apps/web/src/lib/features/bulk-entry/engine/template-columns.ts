import { normalizeRegistrationFieldKey } from "$lib/features/students/registration-fields";
import type { BulkImportProfile, BulkProfileId, BulkReferenceData, ColumnDefinitionMetadata } from "../types";

type RegistrationFields = BulkReferenceData["registrationFields"];

export function getActiveImportColumns(profile: BulkImportProfile<any, any>, fields: RegistrationFields) {
  return profile.columnMetadata.filter((column) =>
    profile.id !== "students" || column.id === "full_name_ar" || column.id === "class_name" ||
    fields?.[normalizeRegistrationFieldKey(column.id)]?.active !== false,
  );
}

export function isImportColumnRequired(profile: BulkProfileId, column: ColumnDefinitionMetadata, fields: RegistrationFields) {
  return profile === "students"
    ? fields?.[normalizeRegistrationFieldKey(column.id)]?.required ?? column.required
    : column.required;
}
