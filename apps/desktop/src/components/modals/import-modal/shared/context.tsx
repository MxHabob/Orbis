import {
  type GradeImportFormData,
  gradeImportSchema,
  type ImportCsvFormData,
  importSchema,
  mappableFields,
  type StudentImportFormData,
  studentImportSchema,
} from "@orbis/import";
import { createContext, useContext } from "react";
import type {
  Control,
  FormState,
  UseFormSetValue,
  UseFormWatch,
} from "react-hook-form";

export {
  mappableFields,
  importSchema,
  gradeImportSchema,
  studentImportSchema,
  type ImportCsvFormData,
  type GradeImportFormData,
  type StudentImportFormData,
};

export type ImportCsvContextType<T extends ImportCsvFormData = any> = {
  fileColumns: string[] | null;
  setFileColumns: (columns: string[] | null) => void;
  firstRows: Record<string, string>[] | null;
  setFirstRows: (rows: Record<string, string>[] | null) => void;
  control: Control<T>;
  watch: UseFormWatch<T>;
  setValue: UseFormSetValue<T>;
  formState: FormState<T>;
  onFileProcessed?: () => void;
};

export const ImportCsvContext = createContext<ImportCsvContextType<any> | null>(
  null,
);

export function useCsvContext<
  T extends ImportCsvFormData = ImportCsvFormData,
>() {
  const context = useContext(ImportCsvContext);

  if (!context)
    throw new Error(
      "useCsvContext must be used within an ImportCsvContext.Provider",
    );

  return context as ImportCsvContextType<T>;
}
