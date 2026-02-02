import type { Column, RowSelectionState, Updater } from "@tanstack/react-table";
import { create } from "zustand";

export type GradeTab = "monthly" | "final";

interface RowSelectionByTab {
  monthly: Record<string, boolean>;
  final: Record<string, boolean>;
}

interface GradesState {
  canDelete?: boolean;
  columns: Column<any, unknown>[];
  setColumns: (columns?: Column<any, unknown>[]) => void;
  setCanDelete: (canDelete?: boolean) => void;
  // Per-tab row selection
  rowSelectionByTab: RowSelectionByTab;
  setRowSelection: (
    tab: GradeTab,
    updater: Updater<RowSelectionState>,
  ) => void;
  // Helper to get row selection for a specific tab
  getRowSelection: (tab: GradeTab) => Record<string, boolean>;
  // Clear selection for a specific tab
  clearRowSelection: (tab: GradeTab) => void;
  lastClickedIndex: number | null;
  setLastClickedIndex: (index: number | null) => void;
}

export const useGradesStore = create<GradesState>()((set, get) => ({
  columns: [],
  canDelete: false,
  rowSelectionByTab: {
    monthly: {},
    final: {},
  },
  lastClickedIndex: null,
  setCanDelete: (canDelete) => set({ canDelete }),
  setColumns: (columns) => set({ columns: columns || [] }),
  setRowSelection: (tab: GradeTab, updater: Updater<RowSelectionState>) =>
    set((state) => {
      const currentSelection = state.rowSelectionByTab[tab];
      const newSelection =
        typeof updater === "function" ? updater(currentSelection) : updater;
      return {
        rowSelectionByTab: {
          ...state.rowSelectionByTab,
          [tab]: newSelection,
        },
      };
    }),
  getRowSelection: (tab: GradeTab) => get().rowSelectionByTab[tab],
  clearRowSelection: (tab: GradeTab) =>
    set((state) => ({
      rowSelectionByTab: {
        ...state.rowSelectionByTab,
        [tab]: {},
      },
    })),
  setLastClickedIndex: (index) => set({ lastClickedIndex: index }),
}));
