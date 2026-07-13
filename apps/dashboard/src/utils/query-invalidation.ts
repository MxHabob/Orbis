import type { QueryClient } from "@tanstack/react-query";

type GradeGridInput = {
  classId: number;
  subjectId: number;
  termId: number;
  month: number;
  academicPeriodId: number;
};

export async function invalidateStudentDomain(
  queryClient: QueryClient,
  trpc: any,
) {
  await Promise.all([
    queryClient.invalidateQueries({
      queryKey: trpc.students.getAll.queryKey(),
    }),
    queryClient.invalidateQueries({
      queryKey: trpc.students.getByClassId.queryKey(),
    }),
    queryClient.invalidateQueries({
      queryKey: trpc.students.getById.queryKey(),
    }),
    queryClient.invalidateQueries({
      queryKey: trpc.classes.getById.queryKey(),
    }),
    queryClient.invalidateQueries({
      queryKey: trpc.grades.getClassStatistics.queryKey(),
    }),
  ]);
}

export async function invalidateGradesDomain(
  queryClient: QueryClient,
  trpc: any,
  input: GradeGridInput | null,
) {
  if (!input) {
    await queryClient.invalidateQueries({
      queryKey: trpc.grades.getGridData.queryKey(),
    });
    return;
  }

  await Promise.all([
    queryClient.invalidateQueries({
      queryKey: trpc.grades.getGridData.queryKey(input),
    }),
    queryClient.invalidateQueries({
      queryKey: trpc.grades.getClassStatistics.queryKey({
        classId: String(input.classId),
        academicPeriodId: input.academicPeriodId,
      }),
    }),
  ]);
}

export async function invalidateClassDomain(
  queryClient: QueryClient,
  trpc: any,
  classId?: number | null,
) {
  await Promise.all([
    queryClient.invalidateQueries({
      queryKey: trpc.classes.getAll.queryKey(),
    }),
    queryClient.invalidateQueries({
      queryKey: classId
        ? trpc.classes.getById.queryKey({ id: classId })
        : trpc.classes.getById.queryKey(),
    }),
    queryClient.invalidateQueries({
      queryKey: trpc.students.getAll.queryKey(),
    }),
    queryClient.invalidateQueries({
      queryKey: trpc.students.getByClassId.queryKey(),
    }),
    queryClient.invalidateQueries({
      queryKey: trpc.grades.getGridData.queryKey(),
    }),
    queryClient.invalidateQueries({
      queryKey: trpc.grades.getClassStatistics.queryKey(),
    }),
    queryClient.invalidateQueries({
      queryKey: trpc.assessments.getByClass.queryKey(),
    }),
  ]);
}

export async function invalidateSubjectDomain(
  queryClient: QueryClient,
  trpc: any,
  subjectId?: number | null,
) {
  await Promise.all([
    queryClient.invalidateQueries({
      queryKey: trpc.subjects.getAll.queryKey(),
    }),
    queryClient.invalidateQueries({
      queryKey: subjectId
        ? trpc.subjects.getById.queryKey({ id: subjectId })
        : trpc.subjects.getById.queryKey(),
    }),
    queryClient.invalidateQueries({
      queryKey: trpc.grades.getGridData.queryKey(),
    }),
    queryClient.invalidateQueries({
      queryKey: trpc.assessments.getByClass.queryKey(),
    }),
  ]);
}
