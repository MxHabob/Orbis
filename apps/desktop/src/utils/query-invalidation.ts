import type { QueryClient } from "@tanstack/react-query";
import { queryKeys } from "@/lib/query-keys";

type GradeGridInput = {
  classId: number;
  subjectId: number;
  termId: number;
  month: number;
  academicYearId: number;
};

export async function invalidateStudentDomain(
  queryClient: QueryClient,
  _trpc?: any,
) {
  await Promise.all([
    queryClient.invalidateQueries({
      queryKey: ["students"],
    }),
    queryClient.invalidateQueries({
      queryKey: ["classes"],
    }),
    queryClient.invalidateQueries({
      queryKey: ["grades"],
    }),
  ]);
}

export async function invalidateGradesDomain(
  queryClient: QueryClient,
  _trpc: any,
  input: GradeGridInput | null,
) {
  if (!input) {
    await queryClient.invalidateQueries({
      queryKey: ["grades"],
    });
    return;
  }

  await Promise.all([
    queryClient.invalidateQueries({
      queryKey: queryKeys.grades.grid(input as any),
    }),
    queryClient.invalidateQueries({
      queryKey: queryKeys.grades.classStatistics(String(input.classId), {
        academicYearId: input.academicYearId,
      }),
    }),
  ]);
}

export async function invalidateClassDomain(
  queryClient: QueryClient,
  _trpc: any,
  classId?: number | null,
) {
  await Promise.all([
    queryClient.invalidateQueries({
      queryKey: ["classes"],
    }),
    queryClient.invalidateQueries({
      queryKey: ["students"],
    }),
    queryClient.invalidateQueries({
      queryKey: ["grades"],
    }),
    queryClient.invalidateQueries({
      queryKey: ["assessments"],
    }),
  ]);
}

export async function invalidateSubjectDomain(
  queryClient: QueryClient,
  _trpc: any,
  subjectId?: number | null,
) {
  await Promise.all([
    queryClient.invalidateQueries({
      queryKey: ["subjects"],
    }),
    queryClient.invalidateQueries({
      queryKey: ["grades"],
    }),
    queryClient.invalidateQueries({
      queryKey: ["assessments"],
    }),
  ]);
}

