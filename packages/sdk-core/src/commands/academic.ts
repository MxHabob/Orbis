/**
 * Academic command interfaces — mirrors svc-academic/ops + svc-organization/ops/cycle.
 * Platform adapters (Desktop/Web) implement these interfaces.
 */
import type { Class, Enrollment, Subject, OrgUnit, ClassSubject, AcademicCycle, StudentGuardian, Document, Notification } from "../types";

export interface AcademicCommands {
  // ── Classes ─────────────────────────────────────────────────
  listClasses(orgId: string, cycleId?: string): Promise<Class[]>;
  getClass(classId: string): Promise<Class>;
  createClass(data: { org_unit_id: string; cycle_id: string; name: string; name_ar?: string; grade_level: string; capacity: number }): Promise<Class>;
  updateClass(classId: string, data: { name: string; name_ar?: string; level: string; division?: string; capacity?: number }): Promise<Class>;
  deleteClass(classId: string): Promise<void>;
  assignTeacher(classId: string, teacherId: string): Promise<Class>;

  // ── Subjects ────────────────────────────────────────────────
  listSubjects(orgId: string): Promise<Subject[]>;
  getSubject(subjectId: string): Promise<Subject>;
  createSubject(data: { code: string; name: string; name_ar?: string }): Promise<Subject>;
  updateSubject(subjectId: string, data: { name: string; name_ar?: string; code: string }): Promise<Subject>;
  deleteSubject(subjectId: string): Promise<void>;

  // ── OrgUnits ────────────────────────────────────────────────
  listOrgUnits(orgId: string): Promise<OrgUnit[]>;
  createOrgUnit(data: { name: string; name_ar?: string; parent_id?: string }): Promise<OrgUnit>;
  updateOrgUnit(unitId: string, data: { name: string; name_ar?: string }): Promise<OrgUnit>;
  deleteOrgUnit(unitId: string): Promise<void>;

  // ── Enrollments ─────────────────────────────────────────────
  listEnrollments(orgId: string, classId?: string, cycleId?: string): Promise<Enrollment[]>;
  getEnrollment(enrollmentId: string): Promise<Enrollment>;
  createEnrollment(data: { org_id: string; cycle_id: string; student_id: string; class_id: string; student_number: string }): Promise<Enrollment>;
  withdrawEnrollment(enrollmentId: string, reason: string): Promise<Enrollment>;
  transferEnrollment(enrollmentId: string, newClassId: string): Promise<Enrollment>;
  checkDuplicate(studentId: string, cycleId: string): Promise<Enrollment | null>;

  // ── Class-Subject ───────────────────────────────────────────
  listClassSubjects(classId: string, cycleId: string): Promise<ClassSubject[]>;
  listTeacherSubjects(teacherId: string, cycleId: string): Promise<ClassSubject[]>;
  createClassSubject(data: { class_id: string; subject_id: string; cycle_id: string; primary_teacher_id?: string; weekly_hours?: number }): Promise<ClassSubject>;
  updateClassSubject(id: string, data: { primary_teacher_id?: string; weekly_hours?: number }): Promise<ClassSubject>;
  deleteClassSubject(id: string): Promise<void>;

  // ── Cycles ──────────────────────────────────────────────────
  listCycles(orgId: string): Promise<AcademicCycle[]>;
  getCycle(cycleId: string): Promise<AcademicCycle>;
  getCurrentCycle(orgId: string): Promise<AcademicCycle>;
  createCycle(data: { name: string; name_ar?: string; cycle_type: string; parent_id?: string; start_date: string; end_date: string; grading_system: string }): Promise<AcademicCycle>;
  updateCycle(cycleId: string, data: { name?: string; start_date?: string; end_date?: string }): Promise<AcademicCycle>;
  setCurrentCycle(cycleId: string): Promise<AcademicCycle>;
  lockCycle(cycleId: string): Promise<AcademicCycle>;
  unlockCycle(cycleId: string): Promise<AcademicCycle>;
  toggleGradeEntry(cycleId: string, open: boolean, start?: string, end?: string): Promise<AcademicCycle>;

  // ── Guardians ───────────────────────────────────────────────
  listGuardiansByStudent(studentId: string): Promise<StudentGuardian[]>;
  searchGuardians(query: string): Promise<StudentGuardian[]>;

  // ── Documents ───────────────────────────────────────────────
  listDocuments(ownerId?: string): Promise<Document[]>;
  createDocument(data: { org_id: string; owner_id: string; name: string; title?: string; item_type: string; url?: string }): Promise<Document>;
  deleteDocument(documentId: string): Promise<void>;

  // ── Notifications ───────────────────────────────────────────
  listNotifications(limit?: number): Promise<Notification[]>;
  markNotificationRead(notificationId: string): Promise<Notification>;
}
