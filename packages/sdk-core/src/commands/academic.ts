/**
 * Academic command interfaces — mirrors svc-academic/ops + svc-organization/ops/period.
 * Platform adapters (Desktop/Web) implement these interfaces.
 *
 * Migrated: periods → periods, periodId → periodId/periodId.
 */
import type {
  Class,
  Enrollment,
  Subject,
  OrgUnit,
  ClassSubject,
  AcademicPeriod,
  StudentGuardian,
  Document,
  Notification,
} from "../types";

export interface AcademicCommands {
  // ── Classes ─────────────────────────────────────────────────
  listClasses(periodId?: string): Promise<Class[]>;
  getClass(classId: string): Promise<Class>;
  createClass(data: {
    org_unit_id: string;
    period_id: string;
    name: string;
    name_ar?: string;
    grade_level: string;
    capacity: number;
  }): Promise<Class>;
  updateClass(
    classId: string,
    data: {
      name: string;
      name_ar?: string;
      level: string;
      division?: string;
      capacity?: number;
    },
  ): Promise<Class>;
  deleteClass(classId: string): Promise<void>;
  assignTeacher(classId: string, teacherId: string): Promise<Class>;

  // ── Subjects ────────────────────────────────────────────────
  listSubjects(): Promise<Subject[]>;
  getSubject(subjectId: string): Promise<Subject>;
  createSubject(data: {
    code: string;
    name: string;
    name_ar?: string;
  }): Promise<Subject>;
  updateSubject(
    subjectId: string,
    data: { name: string; name_ar?: string; code: string },
  ): Promise<Subject>;
  deleteSubject(subjectId: string): Promise<void>;

  // ── OrgUnits ────────────────────────────────────────────────
  listOrgUnits(): Promise<OrgUnit[]>;
  createOrgUnit(data: {
    name: string;
    name_ar?: string;
    parent_id?: string;
  }): Promise<OrgUnit>;
  updateOrgUnit(
    unitId: string,
    data: { name: string; name_ar?: string },
  ): Promise<OrgUnit>;
  deleteOrgUnit(unitId: string): Promise<void>;

  // ── Enrollments ─────────────────────────────────────────────
  listEnrollments(
    classId?: string,
    periodId?: string,
  ): Promise<Enrollment[]>;
  getEnrollment(enrollmentId: string): Promise<Enrollment>;
  createEnrollment(data: {
    period_id: string;
    student_id: string;
    class_id: string;
    student_number: string;
  }): Promise<Enrollment>;
  withdrawEnrollment(
    enrollmentId: string,
    reason: string,
  ): Promise<Enrollment>;
  transferEnrollment(
    enrollmentId: string,
    newClassId: string,
  ): Promise<Enrollment>;
  checkDuplicate(studentId: string, periodId: string): Promise<Enrollment | null>;

  // ── Class-Subject ───────────────────────────────────────────
  listClassSubjects(
    classId: string,
    periodId?: string,
  ): Promise<ClassSubject[]>;
  listTeacherSubjects(
    teacherId: string,
    periodId?: string,
  ): Promise<ClassSubject[]>;
  createClassSubject(data: {
    class_id: string;
    subject_id: string;
    period_id: string;
    primary_teacher_id?: string;
    weekly_hours?: number;
  }): Promise<ClassSubject>;
  updateClassSubject(
    id: string,
    data: { primary_teacher_id?: string; weekly_hours?: number },
  ): Promise<ClassSubject>;
  deleteClassSubject(id: string): Promise<void>;

  // ── Periods (was Periods) ────────────────────────────────────
  listPeriods(): Promise<AcademicPeriod[]>;
  listPeriodsByYear(periodId: string): Promise<AcademicPeriod[]>;
  getPeriod(periodId: string): Promise<AcademicPeriod>;
  createPeriod(data: {
    name: string;
    name_ar?: string;
    period_type: string;
    parent_id?: string;
    start_date: string;
    end_date: string;
    grading_system?: string;
  }): Promise<AcademicPeriod>;
  lockPeriod(periodId: string): Promise<void>;
  unlockPeriod(periodId: string): Promise<void>;
  togglePeriodEnrollment(
    periodId: string,
    open: boolean,
  ): Promise<void>;
  togglePeriodGradeEntry(
    periodId: string,
    open: boolean,
  ): Promise<void>;

  listGuardiansByStudent(
    studentId: string,
  ): Promise<StudentGuardian[]>;
  searchGuardians(query: string): Promise<StudentGuardian[]>;

  // ── Documents ───────────────────────────────────────────────
  listDocuments(ownerId?: string): Promise<Document[]>;
  createDocument(data: any): Promise<Document>;
  deleteDocument(documentId: string): Promise<void>;

  // ── Notifications ───────────────────────────────────────────
  listNotifications(limit?: number): Promise<Notification[]>;
  markNotificationRead(notificationId: string): Promise<Notification>;
}
