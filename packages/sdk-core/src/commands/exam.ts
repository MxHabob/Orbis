/**
 * Exam command interfaces — mirrors svc-exam/ops.
 *
 * Migrated: periodId → periodId.
 */
import type {
  ExamSession,
  ExamSubject,
  ExamSchedule,
  ExamHall,
  SeatingNumber,
} from "../types";

export interface ExamCommands {
  listSessions(periodId: string): Promise<ExamSession[]>;
  getSession(sessionId: string): Promise<ExamSession>;
  createSession(data: {
    period_id: string;
    name: string;
    name_ar?: string;
    session_type: string;
    start_date: string;
    end_date: string;
  }): Promise<ExamSession>;
  updateSession(
    sessionId: string,
    data: {
      name?: string;
      status?: string;
      start_date?: string;
      end_date?: string;
    },
  ): Promise<ExamSession>;

  listExamSubjects(sessionId: string): Promise<ExamSubject[]>;
  createExamSubject(data: {
    session_id: string;
    subject_id: string;
    max_score: number;
    class_subject_id?: string;
    duration_minutes?: number;
    exam_type?: string;
  }): Promise<ExamSubject>;

  listHalls(): Promise<ExamHall[]>;
  createHall(data: {
    name: string;
    name_ar?: string;
    capacity: number;
  }): Promise<ExamHall>;

  listSchedule(sessionId: string): Promise<ExamSchedule[]>;
  createScheduleEntry(data: {
    exam_session_id: string;
    exam_subject_id: string;
    exam_date: string;
    start_time: string;
    end_time: string;
    period: string;
  }): Promise<ExamSchedule>;

  listSeating(examSubjectId: string): Promise<SeatingNumber[]>;
  generateOrdering(sessionId: string): Promise<void>;
  getOrdering(orgId: string, periodId: string): Promise<any[]>;
  rebuildOrdering(
    orgId: string,
    periodId: string,
    strategy: string,
  ): Promise<any[]>;
  freezeOrdering(
    orgId: string,
    periodId: string,
    userId: string,
  ): Promise<void>;
}
