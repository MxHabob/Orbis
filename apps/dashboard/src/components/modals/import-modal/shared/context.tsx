import { createContext, useContext } from "react";
import type { Control, UseFormSetValue, UseFormWatch } from "react-hook-form";
import { mappableFields, importSchema, type ImportCsvFormData } from "@midday/import";

export { mappableFields, importSchema, type ImportCsvFormData };

export type ImportCsvContextType = {
  fileColumns: string[] | null;
  setFileColumns: (columns: string[] | null) => void;
  firstRows: Record<string, string>[] | null;
  setFirstRows: (rows: Record<string, string>[] | null) => void;
  control: Control<ImportCsvFormData>;
  watch: UseFormWatch<ImportCsvFormData>;
  setValue: UseFormSetValue<ImportCsvFormData>;
};

export const ImportCsvContext = createContext<ImportCsvContextType | null>(null);

export function useCsvContext() {
  const context = useContext(ImportCsvContext);

  if (!context)
    throw new Error(
      "useCsvContext must be used within an ImportCsvContext.Provider",
    );

  return context;
}
