/**
 * Scheduling command interfaces — mirrors svc-scheduling/ops.
 */
import type { TimeSlot, ScheduledLesson, Room, SolveJob } from "../types";

export interface SchedulingCommands {
  // ── Time Slots ──────────────────────────────────────────────
  listTimeSlots(cycleId: string): Promise<TimeSlot[]>;
  createTimeSlot(data: { cycle_id: string; slot_number: number; name?: string; start_time: string; end_time: string; duration_minutes: number; is_break: boolean }): Promise<TimeSlot>;
  updateTimeSlot(slotId: string, data: { name?: string; start_time?: string; end_time?: string }): Promise<TimeSlot>;
  deleteTimeSlot(slotId: string): Promise<void>;

  // ── Lessons ─────────────────────────────────────────────────
  listLessonsByClass(cycleId: string, classId: string): Promise<ScheduledLesson[]>;
  listLessonsByTeacher(cycleId: string, teacherId: string): Promise<ScheduledLesson[]>;
  createLesson(data: { cycle_id: string; class_id: string; class_subject_id: string; subject_id: string; teacher_id: string; time_slot_id: string; day_of_week: number; room_id?: string }): Promise<ScheduledLesson>;
  updateLesson(lessonId: string, data: { teacher_id?: string; room_id?: string; topic?: string; status?: string }): Promise<ScheduledLesson>;
  deleteLesson(lessonId: string): Promise<void>;
  checkConflicts(cycleId: string, teacherId: string, timeSlotId: string, dayOfWeek: number, roomId?: string): Promise<ScheduledLesson[]>;

  // ── Rooms ───────────────────────────────────────────────────
  listRooms(): Promise<Room[]>;
  createRoom(data: { name: string; name_ar?: string; capacity?: number; room_type: string }): Promise<Room>;
  updateRoom(roomId: string, data: { name?: string; capacity?: number }): Promise<Room>;
  deleteRoom(roomId: string): Promise<void>;

  // ── Solver ──────────────────────────────────────────────────
  startSolver(cycleId: string, mode?: string): Promise<SolveJob>;
  getSolverJob(jobId: string): Promise<SolveJob>;
  listSolverJobs(cycleId: string): Promise<SolveJob[]>;
  cancelSolver(jobId: string): Promise<SolveJob>;
  applySolver(jobId: string): Promise<SolveJob>;
}
