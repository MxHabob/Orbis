import { create } from "zustand";

interface ExportData {
  runId?: string;
  accessToken?: string;
  /** Type of export: "file" for certificate/report exports, "accounting" for legacy */
  exportType?: "file" | "accounting";
  /** Provider name (legacy, kept for compatibility) */
  providerName?: string;
}

interface ExportState {
  exportData?: ExportData;
  isExporting: boolean;
  exportingTransactionIds: string[];
  setExportData: (exportData?: ExportData) => void;
  setIsExporting: (isExporting: boolean) => void;
  setExportingTransactionIds: (ids: string[]) => void;
}

export const useExportStore = create<ExportState>()((set) => ({
  exportData: undefined,
  isExporting: false,
  exportingTransactionIds: [],
  setExportData: (exportData) => set({ exportData }),
  setIsExporting: (isExporting) => set({ isExporting }),
  setExportingTransactionIds: (exportingTransactionIds) =>
    set({ exportingTransactionIds }),
}));
