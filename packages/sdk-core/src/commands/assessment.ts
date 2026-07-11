/**
 * Assessment command interfaces — mirrors svc-assessment/ops.
 */
import type { Grade, Attendance, BehaviorRecord, Assessment, GradeReportCard, AttendanceReport, GradeAnalytics } from "../types";

export interface AssessmentCommands {
  // ── Grades ──────────────────────────────────────────────────
  listGrades(cycleId: string, enrollmentId?: string, classSubjectId?: string): Promise<Grade[]>;
  submitGrade(cycleId: string, data: { enrollment_id: string; subject_name: string; score: number; max_score: number; exam_type: string; semester: string; weight?: number }): Promise<Grade>;
  submitBulkGrades(cycleId: string, grades: Array<{ enrollment_id: string; subject_name: string; score: number; max_score: number; exam_type: string; semester: string }>): Promise<Grade[]>;
  updateGrade(gradeId: string, data: { score: number; max_score: number; notes?: string; change_reason?: string }): Promise<Grade>;
  getReportCard(enrollmentId: string, cycleId: string): Promise<GradeReportCard>;
  getGradeAnalytics(orgId: string, classId: string, cycleId: string): Promise<GradeAnalytics>;

  // ── Attendance ──────────────────────────────────────────────
  listAttendance(orgId: string, cycleId: string, classId?: string, date?: string, enrollmentId?: string): Promise<Attendance[]>;
  recordAttendance(cycleId: string, data: { enrollment_id: string; class_id: string; date: string; status: string; notes?: string }): Promise<Attendance>;
  recordBulkAttendance(cycleId: string, data: { class_id: string; date: string; period?: string; records: Array<{ enrollment_id: string; status: string; notes?: string }> }): Promise<Attendance[]>;
  getAttendanceReport(cycleId: string, enrollmentId: string): Promise<AttendanceReport>;

  // ── Behavior ────────────────────────────────────────────────
  listBehavior(orgId: string, studentId: string): Promise<BehaviorRecord[]>;
  updateBehavior(orgId: string, recordId: string, data: { action_taken?: string; status?: string; resolution_notes?: string }): Promise<BehaviorRecord>;
  deleteBehavior(orgId: string, recordId: string): Promise<void>;
  submitBehavior(orgId: string, data: any): Promise<any>;

  // ── Assessments ─────────────────────────────────────────────
  listAssessments(cycleId: string, classSubjectId?: string): Promise<Assessment[]>;
  createAssessment(data: { cycle_id: string; class_subject_id: string; title: string; title_ar?: string; item_type: string; max_score: number; weight: number; grade_group_name: string }): Promise<Assessment>;
  updateAssessment(assessmentId: string, data: { title?: string; max_score?: number; weight?: number }): Promise<Assessment>;
  deleteAssessment(assessmentId: string): Promise<void>;
}
