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
    orgId: string,
    sessionId: string,
    data: {
      name?: string;
      status?: string;
      start_date?: string;
      end_date?: string;
    },
  ): Promise<ExamSession>;

  listExamSubjects(sessionId: string): Promise<ExamSubject[]>;
  getSubjects(orgId: string, sessionId: string): Promise<ExamSubject[]>;
  createExamSubject(data: {
    session_id: string;
    subject_id: string;
    max_score: number;
    class_subject_id?: string;
    duration_minutes?: number;
    exam_type?: string;
  }): Promise<ExamSubject>;

  listHalls(): Promise<ExamHall[]>;
  getHalls(orgId: string, sessionId: string): Promise<ExamHall[]>;
  createHall(orgId: string, data: any): Promise<ExamHall>;

  listSchedule(sessionId: string): Promise<ExamSchedule[]>;
  getSchedules(orgId: string, sessionId: string): Promise<ExamSchedule[]>;
  createScheduleEntry(data: any): Promise<ExamSchedule>;
  deleteSchedule(orgId: string, scheduleId: string): Promise<void>;
  updateScheduleStatus(orgId: string, scheduleId: string, status: string): Promise<void>;

  listSeating(examSubjectId: string): Promise<SeatingNumber[]>;
  getSeatingNumbers(orgId: string, sessionId: string): Promise<SeatingNumber[]>;
  generateSeating(orgId: string, sessionId: string, userId: string): Promise<void>;
  generateSecrets(orgId: string, sessionId: string, userId: string): Promise<void>;
  
  getProctors(orgId: string, sessionId: string): Promise<any[]>;
  autoAssignProctors(orgId: string, sessionId: string, scheduleId: string, data: any): Promise<void>;
  allocateHalls(orgId: string, sessionId: string, data: any): Promise<void>;
  
  listSolveJobs(orgId: string, sessionId: string): Promise<any[]>;
  startSolve(orgId: string, userId: string, data: any): Promise<any>;
  cancelSolve(orgId: string, jobId: string): Promise<any>;
  applySolve(orgId: string, jobId: string): Promise<any>;
  getSolveJob(orgId: string, jobId: string): Promise<any>;

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
