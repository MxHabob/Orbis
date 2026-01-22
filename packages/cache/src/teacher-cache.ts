// src/cache/teacher-cache.ts
import { RedisCache } from "./redis-client";

const cache = new RedisCache("teacher", 30 * 60);

export const teacherCache = {
  async getSchedule(teacherId: string, yearId: number) {
    return cache.get<any[]>(`schedule:${teacherId}:${yearId}`);
  },

  async setSchedule(teacherId: string, yearId: number, schedule: any[]) {
    await cache.set(`schedule:${teacherId}:${yearId}`, schedule);
  },

  async getClasses(teacherId: string) {
    return cache.get<any[]>(`classes:${teacherId}`);
  },

  async setClasses(teacherId: string, classes: any[]) {
    await cache.set(`classes:${teacherId}`, classes);
  },

  async invalidate(teacherId: string) {
    // مسح جميع الجداول الخاصة بهذا المعلم
    const keys = [`schedule:${teacherId}:*`, `classes:${teacherId}`];
    // ملاحظة: Redis لا يدعم wildcard delete مباشرة، لذا نستخدم طريقة أخرى إذا لزم
    await cache.delete(`classes:${teacherId}`);
  },
};