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
  progress: number;
  status: "idle" | "processing" | "completed" | "failed";
  exportingTransactionIds: string[];
  setExportData: (exportData?: ExportData) => void;
  setIsExporting: (isExporting: boolean) => void;
  setProgress: (progress: number) => void;
  setStatus: (status: "idle" | "processing" | "completed" | "failed") => void;
  setExportingTransactionIds: (ids: string[]) => void;
}

export const useExportStore = create<ExportState>()((set) => ({
  exportData: undefined,
  isExporting: false,
  progress: 0,
  status: "idle",
  exportingTransactionIds: [],
  setExportData: (exportData) => set({ exportData }),
  setIsExporting: (isExporting) => set({ isExporting }),
  setProgress: (progress) => set({ progress }),
  setStatus: (status) => set({ status }),
  setExportingTransactionIds: (exportingTransactionIds) =>
    set({ exportingTransactionIds }),
}));
