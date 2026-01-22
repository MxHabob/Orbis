// src/cache/student-cache.ts
import { RedisCache } from "./redis-client";

const cache = new RedisCache("student", 30 * 60); // 30 minutes

export const studentCache = {
  async getById(studentId: string) {
    return cache.get<any>(`id:${studentId}`);
  },

  async setById(studentId: string, data: any) {
    await cache.set(`id:${studentId}`, data);
  },

  async getByNumber(studentNumber: string) {
    return cache.get<any>(`number:${studentNumber}`);
  },

  async setByNumber(studentNumber: string, data: any) {
    await cache.set(`number:${studentNumber}`, data);
  },

  async getCurrentClass(studentId: string) {
    return cache.get<any>(`current-class:${studentId}`);
  },

  async setCurrentClass(studentId: string, classData: any) {
    await cache.set(`current-class:${studentId}`, classData);
  },

  async invalidate(studentId: string) {
    await Promise.all([
      cache.delete(`id:${studentId}`),
      cache.delete(`number:*`), // يفضل استخدام Redis keys pattern لاحقًا إذا كثرت العمليات
      cache.delete(`current-class:${studentId}`),
    ]);
  },
};