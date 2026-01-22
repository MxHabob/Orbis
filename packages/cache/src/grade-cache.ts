// src/cache/grade-cache.ts
import { RedisCache } from "./redis-client";

const cache = new RedisCache("grade", 20 * 60);

export const gradeCache = {
  async getStudentSubject(studentId: string, curriculumSubjectId: number, termId?: number) {
    const key = termId 
      ? `student:${studentId}:subject:${curriculumSubjectId}:term:${termId}`
      : `student:${studentId}:subject:${curriculumSubjectId}`;
    return cache.get<any>(key);
  },

  async setStudentSubject(studentId: string, curriculumSubjectId: number, data: any, termId?: number) {
    const key = termId 
      ? `student:${studentId}:subject:${curriculumSubjectId}:term:${termId}`
      : `student:${studentId}:subject:${curriculumSubjectId}`;
    await cache.set(key, data);
  },

  async getClassReport(classId: number, termId: number) {
    return cache.get<any>(`class-report:${classId}:${termId}`);
  },

  async setClassReport(classId: number, termId: number, report: any) {
    await cache.set(`class-report:${classId}:${termId}`, report);
  },

  async invalidateStudent(studentId: string) {
    // يفضل استخدام pipeline أو keys pattern في الإنتاج
    await cache.delete(`student:${studentId}:*`);
  },
};