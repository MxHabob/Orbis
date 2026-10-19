/**
 * Students domain queries & mutations — students, enrollments, guardians, attendance, behavior, documents, and finance.
 * Wraps OrbisApiClient with TanStack Query and queryKeys factory.
 */
import {
  createInfiniteQuery,
  createQuery,
  createMutation,
  useQueryClient,
} from "@tanstack/svelte-query";
import * as m from "$lib/paraglide/messages";
import {
  api,
  extractErrorMessage,
  queryKeys,
  type ClassDto,
  type StudentDetailDto,
  type StudentFilterParams,
  type EnrollmentFilterParams,
  type RegisterStudentInput,
  type UpdateStudentStatusInput,
  type UpdateStudentProfileInput,
  type ChangeStudyModeInput,
  type TransferClassInput,
  type EnrollStudentRequest,
  type AssignGuardianRequest,
  type RecordBehaviorRequest,
  type UploadDocumentRequest,
} from "$lib/api";
import { studentsStore } from "./store.svelte";
import { debounce } from "$lib/shared/utils/debounce";
import { exportToCsv } from "$lib/shared/utils/export";
import { formatDate } from "$lib/core/i18n";
import { toast } from "@orbis/ui";
import { academicContext } from "$lib/stores";

/** Query and actions for the students directory screen. */
export function createStudentDirectoryQuery(orgId: () => string) {
  const getStudentFilter = () => ({
    class_id: studentsStore.selectedClassId ?? undefined,
    status: studentsStore.statusFilter === "all" ? undefined : studentsStore.statusFilter,
    search: studentsStore.searchQuery.trim() || undefined,
  });

  const studentsQuery = createInfiniteQuery(() => ({
    queryKey: queryKeys.student.list(
      orgId(),
      getStudentFilter() as Record<string, unknown>,
    ),
    queryFn: ({ pageParam }) =>
      api.listStudents({
        ...getStudentFilter(),
        page: pageParam,
        page_size: 50,
      }),
    initialPageParam: 1,
    getNextPageParam: (lastPage) =>
      lastPage.page < lastPage.total_pages ? lastPage.page + 1 : undefined,
    enabled: !!orgId(),
    staleTime: 2 * 60 * 1000,
  }));
  const classesQuery = createQuery<ClassDto[]>(() => ({
    queryKey: queryKeys.academic.classes(orgId()),
    queryFn: () => api.listSections(),
    enabled: !!orgId(),
    staleTime: 5 * 60 * 1000,
  }));
  const enrollmentsQuery = createQuery(() => ({
    queryKey: [
      ...queryKeys.student.all,
      "directory-enrollments",
      orgId(),
      academicContext.effectivePeriodId,
    ],
    queryFn: async () => {
      const periodId = academicContext.effectivePeriodId;
      if (!periodId) return [];

      const enrollments = [];
      let page = 1;
      let pageItems: Awaited<ReturnType<typeof api.listEnrollments>> = [];
      do {
        pageItems = await api.listEnrollments({
          period_id: periodId,
          page,
          page_size: 100,
        });
        enrollments.push(...pageItems);
        page += 1;
      } while (pageItems.length === 100);
      return enrollments;
    },
    enabled: !!orgId() && !!academicContext.effectivePeriodId,
    staleTime: 2 * 60 * 1000,
  }));

  const updateSearch = debounce((query: string) => {
    studentsStore.searchQuery = query;
  }, 150);

  function setStatusFilter(status: string) {
    studentsStore.statusFilter = status;
  }

  function setClassFilter(classId: string | null) {
    studentsStore.selectedClassId = classId;
  }

  function clearFilters() {
    studentsStore.selectedClassId = null;
    studentsStore.statusFilter = "active";
    studentsStore.searchQuery = "";
  }

  async function loadMoreStudents() {
    const result = await studentsQuery.fetchNextPage();
    if (result.isError) throw result.error;
  }

  function retry() {
    void Promise.all([studentsQuery.refetch(), classesQuery.refetch()]);
  }

  async function exportStudents(selectedStudentIds: Set<string>) {
    try {
      let dataset: StudentDetailDto[];
      if (selectedStudentIds.size > 0) {
        const studentIds = [...selectedStudentIds];
        dataset = [];
        for (let index = 0; index < studentIds.length; index += 5) {
          const batch = studentIds.slice(index, index + 5);
          dataset.push(...await Promise.all(batch.map((id) => api.getStudent(id))));
        }
      } else {
        const filter = {
          ...getStudentFilter(),
          page_size: 100,
        };
        const firstPage = await api.listStudents({ ...filter, page: 1 });
        dataset = [...firstPage.items];

        for (let page = 2; page <= firstPage.total_pages; page += 5) {
          const pageNumbers = Array.from(
            { length: Math.min(5, firstPage.total_pages - page + 1) },
            (_, index) => page + index,
          );
          const pages = await Promise.all(
            pageNumbers.map((pageNumber) =>
              api.listStudents({ ...filter, page: pageNumber }),
            ),
          );
          dataset.push(...pages.flatMap((result) => result.items));
        }
      }

      const classNames = new Map(
        (classesQuery.data ?? []).map((classItem) => [classItem.id, classItem.name]),
      );
      exportToCsv(
        `Students_Roster_${orgId()}`,
        dataset,
        [
          { header: m.operations_ui_student_number(), accessor: (student) => student.student?.student_number || "—" },
          {
            header: m.operations_ui_student_name(),
            accessor: (student) =>
              (student.person?.first_name && student.person?.last_name
                ? `${student.person.first_name} ${student.person.last_name}`
                : "") ||
              student.person?.full_name ||
              m.operations_ui_student_fallback(),
          },
          {
            header: m.operations_ui_class(),
            accessor: (student) =>
              classNames.get(student.enrollment?.section_id ?? "") ||
              student.enrollment?.section_id ||
              "—",
          },
          { header: m.operations_ui_status(), accessor: (student) => student.student?.status || "active" },
          {
            header: m.operations_ui_admission_date(),
            accessor: (student) =>
              student.student?.admitted_at ? formatDate(student.student.admitted_at, "date") : "—",
          },
        ],
      );
      toast.success(m.operations_ui_student_export_success());
    } catch (error) {
      toast.error(extractErrorMessage(error));
    }
  }

  return {
    studentsQuery,
    classesQuery,
    enrollmentsQuery,
    get classes() {
      return classesQuery.data ?? [];
    },
    get error() {
      return (studentsQuery.data?.pages.length ? undefined : studentsQuery.error) ||
        classesQuery.error;
    },
    get errorMessage() {
      const error = (studentsQuery.data?.pages.length ? undefined : studentsQuery.error) ||
        classesQuery.error;
      return error
        ? m.students_load_error({ error: extractErrorMessage(error) })
        : "";
    },
    get statusFilter() {
      return studentsStore.statusFilter;
    },
    get selectedClassId() {
      return studentsStore.selectedClassId;
    },
    get hasActiveFilters() {
      return studentsStore.selectedClassId !== null ||
        studentsStore.statusFilter !== "active" ||
        studentsStore.searchQuery !== "";
    },
    get hasMore() {
      return studentsQuery.hasNextPage ?? false;
    },
    updateSearch,
    setStatusFilter,
    setClassFilter,
    clearFilters,
    loadMoreStudents,
    retry,
    exportStudents,
  };
}

/** Paginated and filtered students query (StudentDetail items). */
export function createStudentsQuery(
  orgId: () => string,
  filter?: () => StudentFilterParams | undefined,
) {
  return createQuery(() => ({
    queryKey: queryKeys.student.list(orgId(), filter?.() as Record<string, unknown>),
    queryFn: () => api.listStudents(filter?.()),
    enabled: !!orgId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Single student profile query (StudentDetail). */
export function createStudentQuery(
  idOrOrg: () => string,
  maybeId?: () => string,
) {
  const studentId = () => (maybeId ? maybeId() : idOrOrg());
  return createQuery(() => ({
    queryKey: queryKeys.student.detail(studentId()),
    queryFn: () => api.getStudent(studentId()),
    enabled: !!studentId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** All enrollments for an organization. */
export function createEnrollmentsQuery(
  orgId: () => string,
  filter?: () => EnrollmentFilterParams | undefined,
) {
  return createQuery(() => ({
    queryKey: queryKeys.student.enrollments(orgId(), filter?.() as Record<string, unknown>),
    queryFn: () => api.listEnrollments(filter?.()),
    enabled: !!orgId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Single enrollment for detail pages. */
export function createEnrollmentQuery(
  idOrOrg: () => string,
  maybeId?: () => string,
) {
  const enrollmentId = () => (maybeId ? maybeId() : idOrOrg());
  return createQuery(() => ({
    queryKey: queryKeys.student.enrollment(enrollmentId()),
    queryFn: () => api.getEnrollment(enrollmentId()),
    enabled: !!enrollmentId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Guardians list for a student. */
export function createGuardiansQuery(studentId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.student.guardians(studentId()),
    queryFn: () => api.listGuardians(studentId()),
    enabled: !!studentId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Attendance records list for a student. */
export function createStudentAttendanceQuery(studentId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.student.attendance(studentId()),
    queryFn: () => api.listStudentAttendance(studentId()),
    enabled: !!studentId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Behavior records list for a student. */
export function createStudentBehaviorQuery(studentId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.student.behavior(studentId()),
    queryFn: () => api.listStudentBehaviorRecords(studentId()),
    enabled: !!studentId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Documents attached to a student entity. */
export function createStudentDocumentsQuery(studentId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.student.documents(studentId()),
    queryFn: () => api.listDocumentsByEntity("student", studentId()),
    enabled: !!studentId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Evaluated document requirements for a student. */
export function createStudentDocumentRequirementsQuery(
  studentId: () => string,
  gradeLevelId?: () => string | undefined,
  isNational?: () => boolean | undefined,
  isTransfer?: () => boolean | undefined,
) {
  return createQuery(() => ({
    queryKey: queryKeys.student.documentRequirements(studentId()),
    queryFn: () => api.evaluateStudentDocumentRequirements(studentId(), {
      grade_level_id: gradeLevelId?.(),
      is_national: isNational?.(),
      is_transfer: isTransfer?.(),
    }),
    enabled: !!studentId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Financial fees assigned to a student. */
export function createStudentFeesQuery(studentId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.student.fees(studentId()),
    queryFn: () => api.listStudentFees({ student_id: studentId() }),
    enabled: !!studentId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Financial payments made by or on behalf of a student. */
export function createStudentPaymentsQuery(studentId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.student.payments(studentId()),
    queryFn: () => api.listPayments({ student_id: studentId() }),
    enabled: !!studentId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Register student mutation */
export function createRegisterStudentMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: RegisterStudentInput) => api.registerStudent(req),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
    },
  }));
}

/** Update student status mutation */
export function createUpdateStudentStatusMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: UpdateStudentStatusInput) => api.updateStudentStatus(req),
    onSuccess: (_, vars) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.student.detail(vars.student_id || vars.studentId || "") });
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
    },
  }));
}

/** Update student profile mutation */
export function createUpdateStudentProfileMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: UpdateStudentProfileInput) => api.updateStudentProfile(req),
    onSuccess: (_, vars) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.student.detail(vars.student_id || vars.studentId || "") });
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
    },
  }));
}

/** Soft delete student mutation */
export function createDeleteStudentMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (studentId: string) => api.deleteStudent(studentId),
    onSuccess: (_, studentId) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.student.detail(studentId) });
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
    },
  }));
}

/** Restore soft deleted student mutation */
export function createRestoreStudentMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (studentId: string) => api.restoreStudent(studentId),
    onSuccess: (_, studentId) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.student.detail(studentId) });
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
    },
  }));
}

/** Change study mode mutation */
export function createChangeStudyModeMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: ChangeStudyModeInput) => api.changeStudyMode(req),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
    },
  }));
}

/** Transfer class mutation */
export function createTransferClassMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: TransferClassInput) => api.transferClass(req),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
      queryClient.invalidateQueries({ queryKey: queryKeys.academic.all });
    },
  }));
}

/** Enroll student mutation */
export function createEnrollStudentMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: EnrollStudentRequest) => api.enrollStudent(req),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
    },
  }));
}

/** Assign guardian mutation */
export function createAssignGuardianMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: AssignGuardianRequest) => api.assignGuardian(req),
    onSuccess: (_, vars) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.student.guardians(vars.student_id) });
    },
  }));
}

/** Remove guardian mutation */
export function createRemoveGuardianMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: ({ studentId, guardianId }: { studentId: string; guardianId: string }) =>
      api.removeGuardian(studentId, guardianId),
    onSuccess: (_, vars) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.student.guardians(vars.studentId) });
    },
  }));
}

/** Record behavior mutation */
export function createRecordBehaviorMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: RecordBehaviorRequest) => api.recordBehavior(req),
    onSuccess: (_, vars) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.student.behavior(vars.student_id || "") });
    },
  }));
}

/** Upload document mutation */
export function createUploadStudentDocumentMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: UploadDocumentRequest) => api.uploadDocument(req),
    onSuccess: (_, vars) => {
      if (vars.entity_id) {
        queryClient.invalidateQueries({ queryKey: queryKeys.student.documents(vars.entity_id) });
        queryClient.invalidateQueries({ queryKey: queryKeys.student.documentRequirements(vars.entity_id) });
      }
    },
  }));
}

/** Delete document mutation */
export function createDeleteDocumentMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (docId: string) => api.deleteDocument(docId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
    },
  }));
}
