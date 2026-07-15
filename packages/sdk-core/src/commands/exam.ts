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
  getSubjects(sessionId: string): Promise<ExamSubject[]>;
  createExamSubject(data: {
    session_id: string;
    subject_id: string;
    max_score: number;
    class_subject_id?: string;
    duration_minutes?: number;
    exam_type?: string;
  }): Promise<ExamSubject>;

  listHalls(): Promise<ExamHall[]>;
  getHalls(sessionId: string): Promise<ExamHall[]>;
  createHall(data: any): Promise<ExamHall>;

  listSchedule(sessionId: string): Promise<ExamSchedule[]>;
  getSchedules(sessionId: string): Promise<ExamSchedule[]>;
  createScheduleEntry(data: any): Promise<ExamSchedule>;
  deleteSchedule(scheduleId: string): Promise<void>;
  updateScheduleStatus(scheduleId: string, status: string): Promise<void>;

  listSeating(examSubjectId: string): Promise<SeatingNumber[]>;
  getSeatingNumbers(sessionId: string): Promise<SeatingNumber[]>;
  generateSeating(sessionId: string, userId: string): Promise<void>;
  generateSecrets(sessionId: string, userId: string): Promise<void>;

  getProctors(sessionId: string): Promise<any[]>;
  autoAssignProctors(
    sessionId: string,
    scheduleId: string,
    data: any,
  ): Promise<void>;
  allocateHalls(sessionId: string, data: any): Promise<void>;

  listSolveJobs(sessionId: string): Promise<any[]>;
  startSolve(userId: string, data: any): Promise<any>;
  cancelSolve(jobId: string): Promise<any>;
  applySolve(jobId: string): Promise<any>;
  getSolveJob(jobId: string): Promise<any>;

  generateOrdering(sessionId: string): Promise<void>;
  getOrdering(periodId: string): Promise<any[]>;
  rebuildOrdering(periodId: string, strategy: string): Promise<any[]>;
  freezeOrdering(periodId: string, userId: string): Promise<void>;
}
