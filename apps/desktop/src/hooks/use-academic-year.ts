import {
  useCreateAcademicTerm,
  useCreateAcademicYear,
  useCurrentYear,
  useAcademicTerms,
  useAcademicYears,
  useAcademicYear,
  useDeleteAcademicTerm,
  useDeleteAcademicYear,
  useUpdateAcademicTerm,
  useUpdateAcademicYear,
} from "@/hooks/api/use-academics";

interface GetAcademicYearsParams {
  cursor?: string | null;
  pageSize?: number;
  sort?: string[];
  activeOnly?: boolean;
}

/** @deprecated Use useAcademicYears from @/hooks/api/use-academics */
export function useAcademicYearsQuery(_params: GetAcademicYearsParams = {}) {
  return useAcademicYears();
}

/** @deprecated Use useCurrentYear from @/hooks/api/use-academics */
export function useCurrentAcademicYearQuery() {
  return useCurrentYear();
}

/** @deprecated Use useAcademicYear from @/hooks/api/use-academics */
export function useAcademicYearQuery(id: number) {
  return useAcademicYear(String(id));
}

/** @deprecated Use useCreateAcademicYear from @/hooks/api/use-academics */
export function useCreateAcademicYearMutation() {
  return useCreateAcademicYear();
}

/** @deprecated Use useUpdateAcademicYear from @/hooks/api/use-academics */
export function useUpdateAcademicYearMutation() {
  return useUpdateAcademicYear();
}

/** @deprecated Use useDeleteAcademicYear from @/hooks/api/use-academics */
export function useDeleteAcademicYearMutation() {
  return useDeleteAcademicYear();
}

/** @deprecated Use useAcademicTerms from @/hooks/api/use-academics */
export function useAcademicTermsQuery(yearId: number) {
  return useAcademicTerms(String(yearId));
}

/** @deprecated Use useCreateAcademicTerm from @/hooks/api/use-academics */
export function useCreateAcademicTermMutation() {
  return useCreateAcademicTerm();
}

/** @deprecated Use useUpdateAcademicTerm from @/hooks/api/use-academics */
export function useUpdateAcademicTermMutation() {
  return useUpdateAcademicTerm();
}

/** @deprecated Use useDeleteAcademicTerm from @/hooks/api/use-academics */
export function useDeleteAcademicTermMutation() {
  return useDeleteAcademicTerm();
}
