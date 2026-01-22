// src/cache/class-cache.ts
import { RedisCache } from "./redis-client";

const cache = new RedisCache("class", 20 * 60); // 20 minutes

export const classCache = {
  async getStudents(classId: number) {
    return cache.get<any[]>(`students:${classId}`);
  },

  async setStudents(classId: number, students: any[]) {
    await cache.set(`students:${classId}`, students);
  },

  async getDetails(classId: number) {
    return cache.get<any>(`details:${classId}`);
  },

  async setDetails(classId: number, details: any) {
    await cache.set(`details:${classId}`, details);
  },

  async getSubjects(classId: number) {
    return cache.get<any[]>(`subjects:${classId}`);
  },

  async setSubjects(classId: number, subjects: any[]) {
    await cache.set(`subjects:${classId}`, subjects);
  },

  async invalidate(classId: number) {
    await Promise.all([
      cache.delete(`students:${classId}`),
      cache.delete(`details:${classId}`),
      cache.delete(`subjects:${classId}`),
    ]);
  },
};