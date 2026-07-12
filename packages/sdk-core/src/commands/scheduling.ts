/**
 * Scheduling command interfaces — mirrors svc-scheduling/ops.
 *
 * Migrated: cycleId → yearId for scheduling scope.
 */
import type { TimeSlot, ScheduledLesson, Room, SolveJob } from "../types";

export interface SchedulingCommands {
  // ── Time Slots ──────────────────────────────────────────────
  listTimeSlots(orgId: string, yearId: string): Promise<TimeSlot[]>;
  createTimeSlot(data: { year_id: string; slot_number: number; name?: string; start_time: string; end_time: string; duration_minutes: number; is_break: boolean }): Promise<TimeSlot>;
  updateTimeSlot(slotId: string, data: { name?: string; start_time?: string; end_time?: string }): Promise<TimeSlot>;
  deleteTimeSlot(slotId: string): Promise<void>;

  // ── Lessons ─────────────────────────────────────────────────
  listLessonsByClass(orgId: string, yearId: string, classId: string): Promise<ScheduledLesson[]>;
  listLessonsByTeacher(orgId: string, yearId: string, teacherId: string): Promise<ScheduledLesson[]>;
  createLesson(data: { year_id: string; class_id: string; class_subject_id: string; subject_id: string; teacher_id: string; time_slot_id: string; day_of_week: number; room_id?: string }): Promise<ScheduledLesson>;
  updateLesson(lessonId: string, data: { teacher_id?: string; room_id?: string; topic?: string; status?: string }): Promise<ScheduledLesson>;
  deleteLesson(lessonId: string): Promise<void>;
  checkConflicts(orgId: string, yearId: string, teacherId: string, timeSlotId: string, dayOfWeek: number, roomId?: string): Promise<ScheduledLesson[]>;

  // ── Rooms ───────────────────────────────────────────────────
  listRooms(orgId: string): Promise<Room[]>;
  createRoom(data: { name: string; name_ar?: string; capacity?: number; room_type: string }): Promise<Room>;
  updateRoom(roomId: string, data: { name?: string; capacity?: number }): Promise<Room>;
  deleteRoom(roomId: string): Promise<void>;

  // ── Solver ──────────────────────────────────────────────────
  startSolver(yearId: string, mode?: string): Promise<SolveJob>;
  getSolverJob(jobId: string): Promise<SolveJob>;
  listSolverJobs(yearId: string): Promise<SolveJob[]>;
  cancelSolver(jobId: string): Promise<SolveJob>;
  applySolver(jobId: string): Promise<SolveJob>;
}
