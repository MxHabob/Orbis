// src/cache/attendance-cache.ts
import { RedisCache } from "./redis-client";

const cache = new RedisCache("attendance", 15 * 60);

export const attendanceCache = {
  async getDaily(classId: number, date: string) {
    return cache.get<any>(`daily:${classId}:${date}`);
  },

  async setDaily(classId: number, date: string, records: any) {
    await cache.set(`daily:${classId}:${date}`, records);
  },

  async getStudentMonthly(studentId: string, yearId: number, month: string) {
    return cache.get<any>(`monthly:${studentId}:${yearId}:${month}`);
  },

  async setStudentMonthly(studentId: string, yearId: number, month: string, summary: any) {
    await cache.set(`monthly:${studentId}:${yearId}:${month}`, summary);
  },

  async invalidateDaily(classId: number, date: string) {
    await cache.delete(`daily:${classId}:${date}`);
  },
};