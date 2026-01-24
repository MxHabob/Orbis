// src/cache/lesson-cache.ts
import { RedisCache } from "./redis-client";

const cache = new RedisCache("lesson", 30 * 60); // 30 minutes

export const lessonCache = {
  async getById(lessonId: number) {
    return cache.get<any>(`id:${lessonId}`);
  },

  async setById(lessonId: number, data: any) {
    await cache.set(`id:${lessonId}`, data);
  },

  async getClassSchedule(classId: number, academicYearId?: number) {
    const key = academicYearId
      ? `class-schedule:${classId}:${academicYearId}`
      : `class-schedule:${classId}`;
    return cache.get<any[]>(key);
  },

  async setClassSchedule(
    classId: number,
    schedule: any[],
    academicYearId?: number,
  ) {
    const key = academicYearId
      ? `class-schedule:${classId}:${academicYearId}`
      : `class-schedule:${classId}`;
    await cache.set(key, schedule);
  },

  async getTeacherSchedule(teacherId: string, academicYearId?: number) {
    const key = academicYearId
      ? `teacher-schedule:${teacherId}:${academicYearId}`
      : `teacher-schedule:${teacherId}`;
    return cache.get<any[]>(key);
  },

  async setTeacherSchedule(
    teacherId: string,
    schedule: any[],
    academicYearId?: number,
  ) {
    const key = academicYearId
      ? `teacher-schedule:${teacherId}:${academicYearId}`
      : `teacher-schedule:${teacherId}`;
    await cache.set(key, schedule);
  },

  async invalidate(lessonId: number) {
    await Promise.all([
      cache.delete(`id:${lessonId}`),
      cache.delete(`class-schedule:*`),
      cache.delete(`teacher-schedule:*`),
    ]);
  },

  async invalidateClassSchedule(classId: number) {
    await cache.delete(`class-schedule:${classId}*`);
  },

  async invalidateTeacherSchedule(teacherId: string) {
    await cache.delete(`teacher-schedule:${teacherId}*`);
  },
};
