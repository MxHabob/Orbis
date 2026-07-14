/**
 * Assessment command interfaces — mirrors svc-assessment/ops.
 *
 * Migrated: periodId → periodId.
 */
import type {
  Grade,
  Attendance,
  BehaviorRecord,
  Assessment,
  GradeReportCard,
  AttendanceReport,
  GradeAnalytics,
} from "../types";

export interface AssessmentCommands {
  // ── Grades ──────────────────────────────────────────────────
  listGrades(
    periodId: string,
    enrollmentId?: string,
    classSubjectId?: string,
  ): Promise<Grade[]>;
  submitGrade(
    periodId: string,
    data: {
      enrollment_id: string;
      subject_name: string;
      score: number;
      max_score: number;
      exam_type: string;
      semester: string;
      weight?: number;
    },
  ): Promise<Grade>;
  submitBulkGrades(
    periodId: string,
    grades: Array<{
      enrollment_id: string;
      subject_name: string;
      score: number;
      max_score: number;
      exam_type: string;
      semester: string;
    }>,
  ): Promise<Grade[]>;
  updateGrade(
    gradeId: string,
    data: {
      score: number;
      max_score: number;
      notes?: string;
      change_reason?: string;
    },
  ): Promise<Grade>;
  getReportCard(
    enrollmentId: string,
    periodId: string,
  ): Promise<GradeReportCard>;
  getGradeAnalytics(
    classId: string,
    periodId: string,
  ): Promise<GradeAnalytics>;

  // ── Attendance ──────────────────────────────────────────────
  listAttendance(
    periodId: string,
    classId?: string,
    date?: string,
    enrollmentId?: string,
  ): Promise<Attendance[]>;
  recordAttendance(
    periodId: string,
    data: {
      enrollment_id: string;
      class_id: string;
      date: string;
      status: string;
      notes?: string;
    },
  ): Promise<Attendance>;
  recordBulkAttendance(
    periodId: string,
    data: {
      class_id: string;
      date: string;
      period?: string;
      records: Array<{ enrollment_id: string; status: string; notes?: string }>;
    },
  ): Promise<Attendance[]>;
  getAttendanceReport(
    periodId: string,
    enrollmentId: string,
  ): Promise<AttendanceReport>;

  // ── Behavior ────────────────────────────────────────────────
  listBehavior(studentId: string): Promise<BehaviorRecord[]>;
  updateBehavior(
    recordId: string,
    data: { action_taken?: string; status?: string; resolution_notes?: string },
  ): Promise<BehaviorRecord>;
  deleteBehavior(recordId: string): Promise<void>;
  submitBehavior(data: any): Promise<any>;

  // ── Assessments ─────────────────────────────────────────────
  listAssessments(
    periodId: string,
    classSubjectId?: string,
  ): Promise<Assessment[]>;
  createAssessment(data: {
    period_id: string;
    class_subject_id: string;
    title: string;
    title_ar?: string;
    item_type: string;
    max_score: number;
    weight: number;
    grade_group_name: string;
  }): Promise<Assessment>;
  updateAssessment(
    assessmentId: string,
    data: { title?: string; max_score?: number; weight?: number },
  ): Promise<Assessment>;
  deleteAssessment(assessmentId: string): Promise<void>;
}
