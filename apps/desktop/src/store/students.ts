import type { Column } from "@tanstack/react-table";
import { create } from "zustand";

interface StudentsState {
  columns: Column<any, unknown>[];
  setColumns: (columns?: Column<any, unknown>[]) => void;
}

export const useStudentsStore = create<StudentsState>()((set) => ({
  columns: [],
  setColumns: (columns) => set({ columns: columns || [] }),
}));
