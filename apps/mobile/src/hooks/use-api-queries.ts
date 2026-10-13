/**
 * @module @orbis/mobile/hooks/use-api-queries
 *
 * Centralized, performance-optimized React Query hooks using the unified `api` client.
 * Eliminates manual data fetching, providing instant cache,
 * background revalidation, and type-safe query results.
 */

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useAuth } from "@/lib/api";
import { api } from "@/lib/api/client";
import { queryKeys } from "@/lib/api/query-keys";
import type {
  AcademicContextResponse,
  AttendanceDto,
  BatchMarkAttendanceRequest,
  BatchRecordGradesRequest,
  BehaviorRecordDto,
  ClassDto,
  DashboardCountsDto,
  EnrollmentDto,
  ExamScheduleDto,
  ExamSessionDto,
  GradeDto,
  GuardianDto,
  MarkAttendanceRequest,
  OrganizationSettingsDto,
  OrgDto,
  RecordGradeRequest,
  ScheduledLessonDto,
  SectionSubjectDto,
  StudentDto,
  StudentFeeDto,
  SubjectDto,
  TemplateDto,
  TimeSlotDto,
  UserDeviceDto,
  UserDto,
} from "@/lib/api/types";

const STALE_TIME = 1000 * 60 * 5; // 5 minutes cache
const GC_TIME = 1000 * 60 * 15; // 15 minutes garbage collection

/**
 * Helper to safely extract response data from SDK calls
 */
function extractData<T>(response: unknown): T {
  let data = response;
  while (typeof data === "object" && data !== null && "data" in data) {
    data = data.data;
  }
  return data as T;
}

/**
 * Fetch Current Organization Metadata
 */
export function useCurrentOrgQuery() {
  return useQuery({
    queryKey: queryKeys.org.current(),
    queryFn: async (): Promise<OrgDto | null> => {
      try {
        const res = await api.getCurrentOrg({ throwOnError: true });
        return extractData<OrgDto>(res);
      } catch {
        return null;
      }
    },
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

export function useDevicesQuery() {
  return useQuery({
    queryKey: queryKeys.auth.devices(),
    queryFn: async (): Promise<UserDeviceDto[]> => {
      const response = await api.listDevices({ throwOnError: true });
      const devices = extractData<UserDeviceDto[]>(response);
      if (!Array.isArray(devices)) {
        throw new Error("The device list response was not a list.");
      }
      return devices;
    },
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

export function useRevokeDeviceMutation() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async (deviceId: string) => {
      await api.revokeDevice({
        path: { id: deviceId },
        throwOnError: true,
      });
    },
    onSuccess: async () => {
      await queryClient.invalidateQueries({
        queryKey: queryKeys.auth.devices(),
      });
    },
  });
}

/**
 * Fetch Organization / Academic Dashboard Metrics
 */
export function useDashboardQuery() {
  return useQuery({
    queryKey: queryKeys.org.dashboard(),
    queryFn: async (): Promise<DashboardCountsDto | null> => {
      try {
        const res = await api.getDashboard({ throwOnError: true });
        return extractData<DashboardCountsDto>(res);
      } catch {
        return null;
      }
    },
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

/**
 * Fetch Current Academic Context (Year, Active Period)
 */
export function useAcademicContextQuery() {
  return useQuery({
    queryKey: queryKeys.calendar.context(),
    queryFn: async (): Promise<AcademicContextResponse | null> => {
      try {
        const res = await api.getCurrentContext({ throwOnError: true });
        return extractData<AcademicContextResponse>(res);
      } catch {
        return null;
      }
    },
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

/**
 * Fetch Academic Classes List
 */
export function useClassesQuery(enabled = true) {
  return useQuery({
    queryKey: queryKeys.academic.classes(),
    queryFn: async (): Promise<ClassDto[]> => {
      const res = await api.listClasses({ throwOnError: true });
      const classes = extractData<ClassDto[]>(res);
      if (!Array.isArray(classes)) {
        throw new Error("The classes response was not a list.");
      }
      return classes;
    },
    enabled,
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

export interface TeacherRosterEntry {
  student: StudentDto;
  enrollment: EnrollmentDto;
}

export interface TeacherClassData {
  classItem: ClassDto;
  roster: TeacherRosterEntry[];
  classSubjects: SectionSubjectDto[];
  SectionSubject?: SectionSubjectDto[];
  subjects: SubjectDto[];
}

export function useTeacherClassDataQuery(classId?: string) {
  return useQuery({
    queryKey: queryKeys.academic.classRoster(classId ?? ""),
    queryFn: async (): Promise<TeacherClassData> => {
      if (!classId) throw new Error("A class ID is required.");
      const classResponse = await api.getClass({
        path: { id: classId },
        throwOnError: true,
      });
      const classItem = extractData<ClassDto>(classResponse);
      if (!classItem) {
        throw new Error("The class response was invalid.");
      }
      const [enrollmentsResponse, classSubjectsResponse, subjectsResponse] =
        await Promise.all([
          api.listEnrollments({
            query: { class_id: classId },
            throwOnError: true,
          }),
          api.listSectionSubjects({
            query: { class_id: classId, period_id: classItem.period_id },
            throwOnError: true,
          }),
          api.listSubjects({ throwOnError: true }),
        ]);
      const enrollments = extractData<EnrollmentDto[]>(enrollmentsResponse);
      const classSubjects = extractData<SectionSubjectDto[]>(
        classSubjectsResponse,
      );
      const subjects = extractData<SubjectDto[]>(subjectsResponse);
      if (
        !Array.isArray(enrollments) ||
        !Array.isArray(classSubjects) ||
        !Array.isArray(subjects)
      ) {
        throw new Error("The class roster response contained invalid data.");
      }
      const activeEnrollments = enrollments.filter(
        (enrollment) =>
          enrollment.status === "active" &&
          enrollment.class_id === classId &&
          enrollment.period_id === classItem.period_id,
      );
      const activeStudentIds = new Set(
        activeEnrollments.map((enrollment) => enrollment.student_id),
      );
      const studentsById = new Map<string, StudentDto>();
      const pageSize = 100;
      for (
        let page = 1;
        page <= Math.ceil(activeStudentIds.size / pageSize);
        page += 1
      ) {
        const response = await api.listStudents({
          query: {
            class_id: classId,
            period_id: classItem.period_id,
            page,
            page_size: pageSize,
          },
          throwOnError: true,
        });
        const pageOfStudents = extractData<StudentDto[]>(response);
        if (!Array.isArray(pageOfStudents)) {
          throw new Error("The student list response was not a list.");
        }
        for (const student of pageOfStudents) {
          if (activeStudentIds.has(student.id)) {
            studentsById.set(student.id, student);
          }
        }
        if (pageOfStudents.length < pageSize) break;
      }
      return {
        classItem,
        roster: activeEnrollments.flatMap((enrollment) => {
          const student = studentsById.get(enrollment.student_id);
          return student ? [{ student, enrollment }] : [];
        }),
        classSubjects: classSubjects.filter((item) => item.is_active),
        SectionSubject: classSubjects.filter((item) => item.is_active),
        subjects,
      };
    },
    enabled: Boolean(classId),
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

export function useClassAttendanceQuery(classId?: string, date?: string) {
  return useQuery({
    queryKey: queryKeys.assessment.attendance(classId ?? "", date ?? ""),
    queryFn: async (): Promise<AttendanceDto[]> => {
      if (!classId || !date) throw new Error("A class and date are required.");
      const response = await api.client.get<
        { 200: AttendanceDto[] },
        unknown,
        true
      >({
        url: "/api/v1/attendance",
        query: { class_id: classId, date },
        throwOnError: true,
      });
      const attendance = extractData<AttendanceDto[]>(response);
      if (!Array.isArray(attendance)) {
        throw new Error("The class attendance response was not a list.");
      }
      return attendance;
    },
    enabled: Boolean(classId && date),
    staleTime: 30_000,
    gcTime: GC_TIME,
  });
}

export function useClassGradesQuery(classSubjectId?: string) {
  return useQuery({
    queryKey: queryKeys.assessment.classGrades(classSubjectId ?? ""),
    queryFn: async (): Promise<GradeDto[]> => {
      if (!classSubjectId) throw new Error("A class subject is required.");
      const response = await api.client.get<{ 200: GradeDto[] }, unknown, true>(
        {
          url: "/api/v1/grades",
          query: { class_subject_id: classSubjectId },
          throwOnError: true,
        },
      );
      const grades = extractData<GradeDto[]>(response);
      if (!Array.isArray(grades)) {
        throw new Error("The class grades response was not a list.");
      }
      return grades;
    },
    enabled: Boolean(classSubjectId),
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

export function useClassScheduleQuery(classId?: string) {
  return useQuery({
    queryKey: queryKeys.scheduling.lessons({ classSubjectId: classId }),
    queryFn: async (): Promise<StudentScheduleItem[]> => {
      if (!classId) throw new Error("A class ID is required.");
      const [classResponse, subjectResponse] = await Promise.all([
        api.getClass({ path: { id: classId }, throwOnError: true }),
        api.listSubjects({ throwOnError: true }),
      ]);
      const classItem = extractData<ClassDto>(classResponse);
      const subjects = extractData<SubjectDto[]>(subjectResponse);
      if (!classItem || !Array.isArray(subjects)) {
        throw new Error("The class schedule response was invalid.");
      }
      const classSubjectsResponse = await api.listSectionSubjects({
        query: { class_id: classId, period_id: classItem.period_id },
        throwOnError: true,
      });
      const classSubjects = extractData<SectionSubjectDto[]>(
        classSubjectsResponse,
      );
      if (!Array.isArray(classSubjects)) {
        throw new Error("The class subject response was not a list.");
      }
      const activeClassSubjects = classSubjects.filter(
        (item) => item.is_active,
      );
      const [lessonResponses, slotResponse] = await Promise.all([
        Promise.all(
          activeClassSubjects.map((classSubject) =>
            api.client.get<{ 200: ScheduledLessonDto[] }, unknown, true>({
              url: "/api/v1/scheduled-lessons",
              query: { class_subject_id: classSubject.id },
              throwOnError: true,
            }),
          ),
        ),
        api.client.get<{ 200: TimeSlotDto[] }, unknown, true>({
          url: "/api/v1/time-slots",
          query: { period_id: classItem.period_id },
          throwOnError: true,
        }),
      ]);
      const lessons = lessonResponses.flatMap((response) => {
        const data = extractData<ScheduledLessonDto[]>(response);
        if (!Array.isArray(data)) {
          throw new Error("The scheduled lessons response was not a list.");
        }
        return data;
      });
      const timeSlots = extractData<TimeSlotDto[]>(slotResponse);
      if (!Array.isArray(timeSlots)) {
        throw new Error("The time slots response was not a list.");
      }
      const timeSlotsById = new Map(timeSlots.map((slot) => [slot.id, slot]));
      const classSubjectsById = new Map(
        activeClassSubjects.map((item) => [item.id, item]),
      );
      const subjectsById = new Map(
        subjects.map((subject) => [subject.id, subject]),
      );
      return lessons
        .flatMap((lesson) => {
          const timeSlot = timeSlotsById.get(lesson.time_slot_id);
          const classSubject = classSubjectsById.get(lesson.section_subject_id);
          if (!timeSlot || !classSubject) return [];
          return [
            {
              lesson,
              timeSlot,
              subject: subjectsById.get(classSubject.subject_id),
            },
          ];
        })
        .sort(
          (a, b) =>
            a.timeSlot.day_of_week - b.timeSlot.day_of_week ||
            a.timeSlot.slot_index - b.timeSlot.slot_index,
        );
    },
    enabled: Boolean(classId),
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

/**
 * Fetch Single Class Details
 */
export function useClassQuery(classId?: string) {
  return useQuery({
    queryKey: queryKeys.academic.class(classId || ""),
    queryFn: async (): Promise<ClassDto | null> => {
      if (!classId) return null;
      try {
        const res = await api.getClass({
          path: { id: classId },
          throwOnError: true,
        });
        return extractData<ClassDto>(res);
      } catch {
        return null;
      }
    },
    enabled: Boolean(classId),
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

/**
 * Fetch Student Enrollments
 */
export function useEnrollmentsQuery(filter?: {
  classId?: string;
  studentId?: string;
}) {
  return useQuery({
    queryKey: queryKeys.student.enrollments(filter),
    queryFn: async (): Promise<EnrollmentDto[]> => {
      const res = await api.listEnrollments({
        query: { class_id: filter?.classId, student_id: filter?.studentId },
        throwOnError: true,
      });
      const enrollments = extractData<EnrollmentDto[]>(res);
      if (!Array.isArray(enrollments)) {
        throw new Error("The enrollments response was not a list.");
      }
      return enrollments;
    },
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

/**
 * Fetch Grades & Academic Assessments
 */
export function useGradesQuery(filter?: {
  studentId?: string;
  classSubjectId?: string;
}) {
  return useQuery({
    queryKey: queryKeys.assessment.grades(filter),
    queryFn: async (): Promise<GradeDto[]> => {
      if (!filter?.studentId && !filter?.classSubjectId) {
        throw new Error(
          "A student or class subject is required to load grades.",
        );
      }
      const query = filter.studentId
        ? { student_id: filter.studentId }
        : { class_subject_id: filter.classSubjectId };
      const res = await api.client.get<{ 200: GradeDto[] }, unknown, true>({
        url: "/api/v1/grades",
        query,
        throwOnError: true,
      });
      const grades = extractData<GradeDto[]>(res);
      if (!Array.isArray(grades)) {
        throw new Error("The grades response was not a list.");
      }
      return grades;
    },
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

/**
 * Fetch Attendance Logs
 */
export function useAttendanceQuery(filter?: {
  classId?: string;
  studentId?: string;
  date?: string;
}) {
  return useQuery({
    queryKey: queryKeys.assessment.attendance(
      filter?.classId || "",
      filter?.date || "",
    ),
    queryFn: async (): Promise<AttendanceDto[]> => {
      try {
        const res = await api.listAttendance({
          query: {
            class_id: filter?.classId,
            student_id: filter?.studentId,
            date: filter?.date,
          } as any,
          throwOnError: true,
        });
        return extractData<AttendanceDto[]>(res) ?? [];
      } catch {
        return [];
      }
    },
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

export function useStudentQuery(studentId?: string) {
  return useQuery({
    queryKey: queryKeys.student.detail(studentId ?? ""),
    queryFn: async (): Promise<StudentDto> => {
      if (!studentId) throw new Error("A student ID is required.");
      const res = await api.getStudent({
        path: { id: studentId },
        throwOnError: true,
      });
      const student = extractData<StudentDto>(res);
      if (!student || typeof student.id !== "string") {
        throw new Error("The student response was invalid.");
      }
      return student;
    },
    enabled: Boolean(studentId),
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

export function useDigitalIdTemplateQuery(organizationId?: string) {
  return useQuery({
    queryKey: queryKeys.report.templates({
      documentType: "ID_CARD",
      entityType: "student",
      status: "published",
      organizationId,
    }),
    queryFn: async (): Promise<TemplateDto[]> => {
      const response = await api.listTemplates({
        query: {
          document_type: "ID_CARD",
          entity_type: "student",
          status: "published",
        },
        throwOnError: true,
      });
      const templates = extractData<TemplateDto[]>(response);
      if (!Array.isArray(templates)) {
        throw new Error("The digital ID template response was not a list.");
      }
      return templates;
    },
    enabled: Boolean(organizationId),
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

export function useMyStudentQuery(enabled: boolean) {
  return useQuery({
    queryKey: queryKeys.student.detail("me"),
    queryFn: async (): Promise<StudentDto> => {
      const response = await api.client.get<{ 200: StudentDto }, unknown, true>(
        {
          url: "/api/v1/students/me",
          throwOnError: true,
        },
      );
      const student = extractData<StudentDto>(response);
      if (!student || typeof student.id !== "string") {
        throw new Error("The student profile response was invalid.");
      }
      return student;
    },
    enabled,
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

export function useLinkedStudentId(requestedId?: string) {
  const { effectiveContext, user } = useAuth();
  const roles = [
    ...(user?.roles ?? []),
    effectiveContext?.effective_role ?? "",
    ...(effectiveContext?.roles ?? []),
  ].map((role) => role.toLowerCase().replace(/[^a-z]/g, ""));
  const isStudentRole = roles.some((role) => role.includes("student"));
  const ownStudentQuery = useMyStudentQuery(isStudentRole);
  const requestedStudentId =
    requestedId === "me" ? ownStudentQuery.data?.id : requestedId;
  const isDependent = Boolean(
    requestedStudentId &&
      effectiveContext?.dependents.some(
        (dependent) => dependent.student_id === requestedStudentId,
      ),
  );
  const isOwnStudent =
    Boolean(requestedStudentId) &&
    ownStudentQuery.data?.id === requestedStudentId;
  const isLinked = Boolean(requestedStudentId) && (isDependent || isOwnStudent);
  const isResolving =
    isStudentRole &&
    ownStudentQuery.isPending &&
    (requestedId === "me" || Boolean(requestedId));

  return {
    studentId: isLinked ? requestedStudentId : undefined,
    isLinked,
    isResolving,
    isStudentRole,
    identityError: isStudentRole && ownStudentQuery.isError,
    retryIdentity: ownStudentQuery.refetch,
  };
}

export function useStudentAttendanceQuery(studentId?: string) {
  return useQuery({
    queryKey: queryKeys.student.attendance(studentId ?? ""),
    queryFn: async (): Promise<AttendanceDto[]> => {
      if (!studentId) throw new Error("A student ID is required.");
      const res = await api.listStudentAttendance({
        path: { id: studentId },
        throwOnError: true,
      });
      const attendance = extractData<AttendanceDto[]>(res);
      if (!Array.isArray(attendance)) {
        throw new Error("The attendance response was not a list.");
      }
      return attendance;
    },
    enabled: Boolean(studentId),
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

export function useStudentBehaviorQuery(studentId?: string) {
  return useQuery({
    queryKey: queryKeys.student.behavior(studentId ?? ""),
    queryFn: async (): Promise<BehaviorRecordDto[]> => {
      if (!studentId) throw new Error("A student ID is required.");
      const response = await api.listStudentBehavior({
        path: { id: studentId },
        throwOnError: true,
      });
      const records = extractData<BehaviorRecordDto[]>(response);
      if (!Array.isArray(records)) {
        throw new Error("The behavior response was not a list.");
      }
      return records;
    },
    enabled: Boolean(studentId),
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

export interface StudentExamItem {
  schedule: ExamScheduleDto;
  session: ExamSessionDto;
  subject: SubjectDto | undefined;
}

export function useStudentExamsQuery(studentId?: string) {
  return useQuery({
    queryKey: queryKeys.student.exams(studentId ?? ""),
    queryFn: async (): Promise<StudentExamItem[]> => {
      if (!studentId) throw new Error("A student ID is required.");
      const enrollmentResponse = await api.listEnrollments({
        query: { student_id: studentId },
        throwOnError: true,
      });
      const enrollments = extractData<EnrollmentDto[]>(enrollmentResponse);
      if (!Array.isArray(enrollments)) {
        throw new Error("The enrollments response was not a list.");
      }

      const activeEnrollments = enrollments.filter(
        (enrollment) => enrollment.status === "active" && enrollment.class_id,
      );
      if (activeEnrollments.length === 0) return [];

      const classSubjectLists = await Promise.all(
        activeEnrollments.map((enrollment) => {
          if (!enrollment.class_id) {
            throw new Error("An active enrollment is missing its class.");
          }
          return api.listSectionSubjects({
            query: {
              class_id: enrollment.class_id,
              period_id: enrollment.period_id,
            },
            throwOnError: true,
          });
        }),
      );
      const classSubjects = classSubjectLists.flatMap((response) => {
        const items = extractData<SectionSubjectDto[]>(response);
        if (!Array.isArray(items)) {
          throw new Error("The class subjects response was not a list.");
        }
        return items.filter((item) => item.is_active);
      });
      if (classSubjects.length === 0) return [];

      const periodIds = [
        ...new Set(activeEnrollments.map((item) => item.period_id)),
      ];
      const [sessionResponses, subjectsResponse] = await Promise.all([
        Promise.all(
          periodIds.map((periodId) =>
            api.client.get<{ 200: ExamSessionDto[] }, unknown, true>({
              url: "/api/v1/exam-sessions",
              query: { period_id: periodId },
              throwOnError: true,
            }),
          ),
        ),
        api.listSubjects({ throwOnError: true }),
      ]);
      const sessions = sessionResponses.flatMap((response) => {
        const items = extractData<ExamSessionDto[]>(response);
        if (!Array.isArray(items)) {
          throw new Error("The exam sessions response was not a list.");
        }
        return items;
      });
      const subjects = extractData<SubjectDto[]>(subjectsResponse);
      if (!Array.isArray(subjects)) {
        throw new Error("The subjects response was not a list.");
      }
      if (sessions.length === 0) return [];

      const schedulesBySession = await Promise.all(
        sessions.map((session) =>
          api.listSchedules({
            query: { session_id: session.id },
            throwOnError: true,
          }),
        ),
      );
      const schedules = schedulesBySession.flatMap((response) => {
        const items = extractData<ExamScheduleDto[]>(response);
        if (!Array.isArray(items)) {
          throw new Error("The exam schedule response was not a list.");
        }
        return items;
      });
      const sessionById = new Map(
        sessions.map((session) => [session.id, session]),
      );
      const classSubjectById = new Map(
        classSubjects.map((classSubject) => [classSubject.id, classSubject]),
      );
      const subjectById = new Map(
        subjects.map((subject) => [subject.id, subject]),
      );

      return schedules
        .flatMap((schedule) => {
          const session = sessionById.get(schedule.exam_session_id);
          const classSubject = classSubjectById.get(
            schedule.section_subject_id,
          );
          if (!session || !classSubject) return [];
          return [
            {
              schedule,
              session,
              subject: subjectById.get(classSubject.subject_id),
            },
          ];
        })
        .sort(
          (a, b) =>
            a.schedule.exam_date.localeCompare(b.schedule.exam_date) ||
            a.schedule.start_time.localeCompare(b.schedule.start_time),
        );
    },
    enabled: Boolean(studentId),
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

export interface StudentScheduleItem {
  lesson: ScheduledLessonDto;
  timeSlot: TimeSlotDto;
  subject: SubjectDto | undefined;
}

export function useStudentScheduleQuery(studentId?: string) {
  return useQuery({
    queryKey: queryKeys.student.schedule(studentId ?? ""),
    queryFn: async (): Promise<StudentScheduleItem[]> => {
      if (!studentId) throw new Error("A student ID is required.");

      const enrollmentResponse = await api.listEnrollments({
        query: { student_id: studentId },
        throwOnError: true,
      });
      const enrollments = extractData<EnrollmentDto[]>(enrollmentResponse);
      if (!Array.isArray(enrollments)) {
        throw new Error("The enrollments response was not a list.");
      }
      const activeClasses = enrollments.flatMap((enrollment) =>
        enrollment.status === "active" && enrollment.class_id
          ? [{ classId: enrollment.class_id, periodId: enrollment.period_id }]
          : [],
      );
      const classSubjectsByEnrollment = await Promise.all(
        activeClasses.map(async ({ classId, periodId }) => {
          const response = await api.listSectionSubjects({
            query: { class_id: classId, period_id: periodId },
            throwOnError: true,
          });
          const classSubjects = extractData<SectionSubjectDto[]>(response);
          if (!Array.isArray(classSubjects)) {
            throw new Error("The class subjects response was not a list.");
          }
          return classSubjects;
        }),
      );
      const classSubjects = classSubjectsByEnrollment.flat();
      const activeClassSubjects = classSubjects.filter(
        (classSubject) => classSubject.is_active,
      );
      const classSubjectIds = new Set(
        activeClassSubjects.map((classSubject) => classSubject.id),
      );
      if (classSubjectIds.size === 0) return [];

      const periods = [
        ...new Set(activeClasses.map(({ periodId }) => periodId)),
      ];
      const [lessonResponses, timeSlotResponses, subjectResponse] =
        await Promise.all([
          Promise.all(
            activeClassSubjects.map((classSubject) =>
              api.client.get<{ 200: ScheduledLessonDto[] }, unknown, true>({
                url: "/api/v1/scheduled-lessons",
                query: { class_subject_id: classSubject.id },
                throwOnError: true,
              }),
            ),
          ),
          Promise.all(
            periods.map((periodId) =>
              api.client.get<{ 200: TimeSlotDto[] }, unknown, true>({
                url: "/api/v1/time-slots",
                query: { period_id: periodId },
                throwOnError: true,
              }),
            ),
          ),
          api.listSubjects({ throwOnError: true }),
        ]);
      const lessons = lessonResponses.flatMap((response) => {
        const items = extractData<ScheduledLessonDto[]>(response);
        if (!Array.isArray(items)) {
          throw new Error("The lessons response was not a list.");
        }
        return items;
      });
      const timeSlots = timeSlotResponses.flatMap((response) => {
        const items = extractData<TimeSlotDto[]>(response);
        if (!Array.isArray(items)) {
          throw new Error("The time slots response was not a list.");
        }
        return items;
      });
      const subjects = extractData<SubjectDto[]>(subjectResponse);
      if (
        !Array.isArray(lessons) ||
        !Array.isArray(timeSlots) ||
        !Array.isArray(subjects)
      ) {
        throw new Error("The schedule response contained invalid data.");
      }

      const timeSlotsById = new Map(timeSlots.map((slot) => [slot.id, slot]));
      const subjectsById = new Map(
        subjects.map((subject) => [subject.id, subject]),
      );
      const classSubjectsById = new Map(
        activeClassSubjects.map((item) => [item.id, item]),
      );
      return lessons
        .flatMap((lesson) => {
          const timeSlot = timeSlotsById.get(lesson.time_slot_id);
          if (!timeSlot) return [];
          const classSubject = classSubjectsById.get(lesson.section_subject_id);
          return [
            {
              lesson,
              timeSlot,
              subject: classSubject
                ? subjectsById.get(classSubject.subject_id)
                : undefined,
            },
          ];
        })
        .sort(
          (a, b) =>
            a.timeSlot.day_of_week - b.timeSlot.day_of_week ||
            a.timeSlot.slot_index - b.timeSlot.slot_index,
        );
    },
    enabled: Boolean(studentId),
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

/**
 * Fetch Schedules and Lessons
 */
export function useSchedulesQuery(sessionId?: string) {
  return useQuery({
    queryKey: queryKeys.scheduling.lessons({ periodId: sessionId }),
    queryFn: async (): Promise<ScheduledLessonDto[]> => {
      try {
        const res = await api.listLessons({
          query: { period_id: sessionId } as any,
          throwOnError: true,
        });
        return extractData<ScheduledLessonDto[]>(res) ?? [];
      } catch {
        return [];
      }
    },
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

/**
 * Fetch Student Fees and Payments
 */
export function useFeesQuery(studentId?: string) {
  return useQuery({
    queryKey: queryKeys.finance.studentFees({ studentId }),
    queryFn: async (): Promise<StudentFeeDto[]> => {
      if (!studentId) throw new Error("A student ID is required.");
      const response = await api.client.get<
        { 200: StudentFeeDto[] },
        unknown,
        true
      >({
        url: "/api/v1/student-fees",
        query: { student_id: studentId },
        throwOnError: true,
      });
      const fees = extractData<StudentFeeDto[]>(response);
      if (!Array.isArray(fees)) {
        throw new Error("The student fees response was not a list.");
      }
      return fees;
    },
    enabled: Boolean(studentId),
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

/**
 * Fetch Guardians / Linked Parents for a Student
 */
export function useGuardiansQuery(studentId?: string) {
  return useQuery({
    queryKey: queryKeys.student.guardians(studentId || ""),
    queryFn: async (): Promise<GuardianDto[]> => {
      if (!studentId) return [];
      try {
        const res = await api.listGuardians({
          query: { student_id: studentId },
          throwOnError: true,
        });
        return extractData<GuardianDto[]>(res) ?? [];
      } catch {
        return [];
      }
    },
    enabled: Boolean(studentId),
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

/**
 * Fetch User Profile and System Settings
 */
export function useUserQuery(userId?: string) {
  return useQuery({
    queryKey: queryKeys.auth.user(userId || ""),
    queryFn: async (): Promise<UserDto | null> => {
      if (!userId) return null;
      try {
        const res = await api.getUser({
          path: { id: userId },
          throwOnError: true,
        });
        return extractData<UserDto>(res);
      } catch {
        return null;
      }
    },
    enabled: Boolean(userId),
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

export function useSystemSettingsQuery() {
  return useQuery({
    queryKey: queryKeys.org.settings(),
    queryFn: async (): Promise<OrganizationSettingsDto | null> => {
      try {
        const res = await api.getSettings({ throwOnError: true });
        return extractData<OrganizationSettingsDto>(res);
      } catch {
        return null;
      }
    },
    staleTime: STALE_TIME,
    gcTime: GC_TIME,
  });
}

/**
 * Mutation: Record Student Attendance
 */
export function useMarkAttendanceMutation() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async (variables: MarkAttendanceRequest) => {
      const res = await api.markAttendance({
        body: variables,
        throwOnError: true,
      });
      return extractData<AttendanceDto>(res);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.assessment.all });
      queryClient.invalidateQueries({ queryKey: queryKeys.org.dashboard() });
    },
  });
}

export function useBatchMarkAttendanceMutation() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async (variables: BatchMarkAttendanceRequest) => {
      const response = await api.batchMarkAttendance({
        body: variables,
        throwOnError: true,
      });
      return extractData(response);
    },
    onSuccess: async (_data, variables) => {
      await queryClient.invalidateQueries({
        queryKey: queryKeys.assessment.attendance(
          variables.class_id,
          variables.date,
        ),
      });
      await queryClient.invalidateQueries({
        queryKey: queryKeys.assessment.all,
      });
      for (const item of variables.items) {
        await queryClient.invalidateQueries({
          queryKey: queryKeys.student.attendance(item.student_id),
        });
      }
    },
  });
}

/**
 * Mutation: Record Assessment Grade
 */
export function useRecordGradeMutation() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async (variables: RecordGradeRequest) => {
      const res = await api.recordGrade({
        body: variables,
        throwOnError: true,
      });
      return extractData<GradeDto>(res);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.assessment.all });
      queryClient.invalidateQueries({ queryKey: queryKeys.org.dashboard() });
    },
  });
}

export function useBatchRecordGradesMutation() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async (variables: BatchRecordGradesRequest) => {
      const response = await api.batchRecordGrades({
        body: variables,
        throwOnError: true,
      });
      return extractData(response);
    },
    onSuccess: async (_data, variables) => {
      await queryClient.invalidateQueries({
        queryKey: queryKeys.assessment.all,
      });
      for (const item of variables.items) {
        await queryClient.invalidateQueries({
          queryKey: queryKeys.student.detail(item.student_id),
        });
      }
    },
  });
}
