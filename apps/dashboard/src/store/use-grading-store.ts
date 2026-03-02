import { create } from "zustand";
import { persist } from "zustand/middleware";

interface GradingState {
  yearId: number | null;
  classId: number | null;
  subjectId: number | null;
  termId: number | null;
  month: number | null;
  
  // Actions
  setFilters: (filters: Partial<Omit<GradingState, 'setFilters' | 'reset'>>) => void;
  reset: () => void;
}

export const useGradingStore = create<GradingState>()(
  persist(
    (set) => ({
      yearId: null,
      classId: null,
      subjectId: null,
      termId: null,
      month: null,

      setFilters: (filters) => set((state) => ({ ...state, ...filters })),
      reset: () => set({
        yearId: null,
        classId: null,
        subjectId: null,
        termId: null,
        month: null,
      }),
    }),
    {
      name: "grading-storage-2026", // Key in localStorage
    }
  )
);
