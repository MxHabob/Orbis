"use client";

import { useTRPC } from "@/trpc/client";
import {
  useMutation,
  useQueryClient,
  useSuspenseQuery,
} from "@tanstack/react-query";

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
  return useSuspenseQuery(trpc.academicYear.get.queryOptions(params));
}

/**
 * Hook to fetch the current active academic year
 */
export function useCurrentAcademicYearQuery() {
  const trpc = useTRPC();
  return useSuspenseQuery(trpc.academicYear.getCurrent.queryOptions());
}

/**
 * Hook to fetch a specific academic year by its ID
 */
export function useAcademicYearQuery(id: number) {
  const trpc = useTRPC();
  return useSuspenseQuery(trpc.academicYear.getById.queryOptions({ id }));
}

/**
 * Hook to create a new academic year
 */
export function useCreateAcademicYearMutation() {
  const trpc = useTRPC();
  const queryClient = useQueryClient();

  return useMutation(
    trpc.academicYear.create.mutationOptions({
      onSuccess: () => {
        queryClient.invalidateQueries({
          queryKey: trpc.academicYear.get.queryKey(),
        });
        queryClient.invalidateQueries({
          queryKey: trpc.academicYear.getCurrent.queryKey(),
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
    trpc.academicYear.update.mutationOptions({
      onSuccess: (data, variables) => {
        queryClient.invalidateQueries({
          queryKey: trpc.academicYear.get.queryKey(),
        });
        queryClient.invalidateQueries({
          queryKey: trpc.academicYear.getById.queryKey({ id: variables.id }),
        });
        queryClient.invalidateQueries({
          queryKey: trpc.academicYear.getCurrent.queryKey(),
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
    trpc.academicYear.delete.mutationOptions({
      onSuccess: (data, variables) => {
        queryClient.invalidateQueries({
          queryKey: trpc.academicYear.get.queryKey(),
        });
        queryClient.invalidateQueries({
          queryKey: trpc.academicYear.getById.queryKey({ id: variables.id }),
        });
        queryClient.invalidateQueries({
          queryKey: trpc.academicYear.getCurrent.queryKey(),
        });
      },
    }),
  );
}

// --- Academic Terms Hooks ---

/**
 * Hook to fetch terms associated with a specific academic year
 */
export function useAcademicTermsQuery(academicYearId: number) {
  const trpc = useTRPC();
  return useSuspenseQuery(
    trpc.academicYear.getTerms.queryOptions({ academicYearId }),
  );
}

/**
 * Hook to create a new academic term
 */
export function useCreateAcademicTermMutation() {
  const trpc = useTRPC();
  const queryClient = useQueryClient();

  return useMutation(
    trpc.academicYear.createTerm.mutationOptions({
      onSuccess: (data, variables) => {
        queryClient.invalidateQueries({
          queryKey: trpc.academicYear.getTerms.queryKey({
            academicYearId: variables.academicYearId,
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
    trpc.academicYear.updateTerm.mutationOptions({
      onSuccess: () => {
        // Since we don't always have the academicYearId here, we invalidate all terms
        queryClient.invalidateQueries({
          queryKey: trpc.academicYear.getTerms.queryKey(),
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
    trpc.academicYear.deleteTerm.mutationOptions({
      onSuccess: () => {
        queryClient.invalidateQueries({
          queryKey: trpc.academicYear.getTerms.queryKey(),
        });
      },
    }),
  );
}
