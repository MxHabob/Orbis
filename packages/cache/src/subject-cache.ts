// src/cache/subject-cache.ts
import { RedisCache } from "./redis-client";

const cache = new RedisCache("subject", 60 * 60); // 60 minutes

export const subjectCache = {
  async getById(subjectId: number) {
    return cache.get<any>(`id:${subjectId}`);
  },

  async setById(subjectId: number, data: any) {
    await cache.set(`id:${subjectId}`, data);
  },

  async getByCode(code: string) {
    return cache.get<any>(`code:${code}`);
  },

  async setByCode(code: string, data: any) {
    await cache.set(`code:${code}`, data);
  },

  async getCurriculumSubjects(academicYearId: number, levelId?: number) {
    const key = levelId
      ? `curriculum:${academicYearId}:${levelId}`
      : `curriculum:${academicYearId}`;
    return cache.get<any[]>(key);
  },

  async setCurriculumSubjects(
    academicYearId: number,
    data: any[],
    levelId?: number,
  ) {
    const key = levelId
      ? `curriculum:${academicYearId}:${levelId}`
      : `curriculum:${academicYearId}`;
    await cache.set(key, data);
  },

  async invalidate(subjectId: number) {
    await Promise.all([
      cache.delete(`id:${subjectId}`),
      cache.delete(`code:*`),
      cache.delete(`curriculum:*`),
    ]);
  },
};
