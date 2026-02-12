import { createContext, useContext } from "react";
import type { Control, UseFormSetValue, UseFormWatch } from "react-hook-form";
import { 
  mappableFields, 
  importSchema, 
  gradeImportSchema, 
  studentImportSchema, 
  type ImportCsvFormData,
  type GradeImportFormData,
  type StudentImportFormData
} from "@midday/import";

export { 
  mappableFields, 
  importSchema, 
  gradeImportSchema, 
  studentImportSchema, 
  type ImportCsvFormData,
  type GradeImportFormData,
  type StudentImportFormData
};

export type ImportCsvContextType<T extends ImportCsvFormData = any> = {
  fileColumns: string[] | null;
  setFileColumns: (columns: string[] | null) => void;
  firstRows: Record<string, string>[] | null;
  setFirstRows: (rows: Record<string, string>[] | null) => void;
  control: Control<T>;
  watch: UseFormWatch<T>;
  setValue: UseFormSetValue<T>;
  onFileProcessed?: () => void;
};

export const ImportCsvContext = createContext<ImportCsvContextType<any> | null>(null);

export function useCsvContext<T extends ImportCsvFormData = ImportCsvFormData>() {
  const context = useContext(ImportCsvContext);

  if (!context)
    throw new Error(
      "useCsvContext must be used within an ImportCsvContext.Provider",
    );

  return context as ImportCsvContextType<T>;
}
