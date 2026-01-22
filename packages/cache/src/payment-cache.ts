// src/cache/payment-cache.ts
import { RedisCache } from "./redis-client";

const cache = new RedisCache("payment", 30 * 60);

export const paymentCache = {
  async getStudentSummary(studentId: string, yearId: number) {
    return cache.get<any>(`summary:${studentId}:${yearId}`);
  },

  async setStudentSummary(studentId: string, yearId: number, summary: any) {
    await cache.set(`summary:${studentId}:${yearId}`, summary);
  },

  async getOverdueStudents(yearId: number) {
    return cache.get<any[]>(`overdue:${yearId}`);
  },

  async setOverdueStudents(yearId: number, students: any[]) {
    await cache.set(`overdue:${yearId}`, students);
  },

  async invalidateStudent(studentId: string) {
    await cache.delete(`summary:${studentId}:*`);
  },
};