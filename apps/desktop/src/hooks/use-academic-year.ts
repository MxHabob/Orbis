"use client";

import {
  useMutation,
  useQueryClient,
  useSuspenseQuery,
} from "@tanstack/react-query";
import { useTRPC } from "@/ipc/query-adapter";

interface GetAcademicYearsParams {
  cursor?: string | null;
  pageSize?: number;
  sort?: string[];
  activeOnly?: boolean;
}

/**
 * Hook to fetch all academic years with pagination and sorting
 */
export function useAcademicYearsQuery(params: GetAcademicYearsParams = {}) {
  const trpc = useTRPC();
  return useSuspenseQuery(trpc.academic.getYears.queryOptions(params));
}

/**
 * Hook to fetch the current active academic year
 */
export function useCurrentAcademicYearQuery() {
  const trpc = useTRPC();
  return useSuspenseQuery(trpc.academic.getCurrentYear.queryOptions());
}

/**
 * Hook to fetch a specific academic year by its ID
 */
export function useAcademicYearQuery(id: number) {
  const trpc = useTRPC();
  return useSuspenseQuery(trpc.academic.getYearById.queryOptions({ id }));
}

/**
 * Hook to create a new academic year
 */
export function useCreateAcademicYearMutation() {
  const trpc = useTRPC();
  const queryClient = useQueryClient();

  return useMutation(
    trpc.academic.createYear.mutationOptions({
      onSuccess: () => {
        queryClient.invalidateQueries({
          queryKey: trpc.academic.getYears.queryKey(),
        });
        queryClient.invalidateQueries({
          queryKey: trpc.academic.getCurrentYear.queryKey(),
        });
      },
    }),
  );
}

/**
 * Hook to update an existing academic year
 */
export function useUpdateAcademicYearMutation() {
  const trpc = useTRPC();
  const queryClient = useQueryClient();

  return useMutation(
    trpc.academic.updateYear.mutationOptions({
      onSuccess: (data, variables) => {
        queryClient.invalidateQueries({
          queryKey: trpc.academic.getYears.queryKey(),
        });
        queryClient.invalidateQueries({
          queryKey: trpc.academic.getYearById.queryKey({ id: variables.id }),
        });
        queryClient.invalidateQueries({
          queryKey: trpc.academic.getCurrentYear.queryKey(),
        });
      },
    }),
  );
}

/**
 * Hook to delete an academic year
 */
export function useDeleteAcademicYearMutation() {
  const trpc = useTRPC();
  const queryClient = useQueryClient();

  return useMutation(
    trpc.academic.deleteYear.mutationOptions({
      onSuccess: (data, variables) => {
        queryClient.invalidateQueries({
          queryKey: trpc.academic.getYears.queryKey(),
        });
        queryClient.invalidateQueries({
          queryKey: trpc.academic.getYearById.queryKey({ id: variables.id }),
        });
        queryClient.invalidateQueries({
          queryKey: trpc.academic.getCurrentYear.queryKey(),
        });
      },
    }),
  );
}

// --- Academic Terms Hooks ---

/**
 * Hook to fetch terms associated with a specific academic year
 */
export function useAcademicTermsQuery(yearId: number) {
  const trpc = useTRPC();
  return useSuspenseQuery(trpc.academic.getTerms.queryOptions({ yearId }));
}

/**
 * Hook to create a new academic term
 */
export function useCreateAcademicTermMutation() {
  const trpc = useTRPC();
  const queryClient = useQueryClient();

  return useMutation(
    trpc.academic.createTerm.mutationOptions({
      onSuccess: (data, variables) => {
        queryClient.invalidateQueries({
          queryKey: trpc.academic.getTerms.queryKey({
            yearId: variables.academicYearId as number,
          }),
        });
      },
    }),
  );
}

/**
 * Hook to update an existing academic term
 */
export function useUpdateAcademicTermMutation() {
  const trpc = useTRPC();
  const queryClient = useQueryClient();

  return useMutation(
    trpc.academic.updateTerm.mutationOptions({
      onSuccess: () => {
        // Since we don't always have the yearId here, we invalidate all terms
        queryClient.invalidateQueries({
          queryKey: trpc.academic.getTerms.queryKey(),
        });
      },
    }),
  );
}

/**
 * Hook to delete an academic term
 */
export function useDeleteAcademicTermMutation() {
  const trpc = useTRPC();
  const queryClient = useQueryClient();

  return useMutation(
    trpc.academic.deleteTerm.mutationOptions({
      onSuccess: () => {
        queryClient.invalidateQueries({
          queryKey: trpc.academic.getTerms.queryKey(),
        });
      },
    }),
  );
}
