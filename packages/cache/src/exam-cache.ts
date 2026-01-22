// src/cache/exam-cache.ts
import { RedisCache } from "./redis-client";

const cache = new RedisCache("exam", 60 * 60); // 1 ساعة TTL - قد تتغير اللجان أحيانًا

export const examCache = {
  /**
   * جدول الامتحانات لفصل دراسي أو سنة كاملة
   */
  async getSchedule(academicYearId: number, termId?: number) {
    const key = termId
      ? `schedule:year:${academicYearId}:term:${termId}`
      : `schedule:year:${academicYearId}`;
    return cache.get<any[]>(key);
  },

  async setSchedule(academicYearId: number, schedule: any[], termId?: number) {
    const key = termId
      ? `schedule:year:${academicYearId}:term:${termId}`
      : `schedule:year:${academicYearId}`;
    await cache.set(key, schedule);
  },

  /**
   * تفاصيل لجنة امتحانية (مكان، سعة، طلاب)
   */
  async getCommittee(committeeId: number) {
    return cache.get<any>(`committee:${committeeId}`);
  },

  async setCommittee(committeeId: number, committee: any) {
    await cache.set(`committee:${committeeId}`, committee);
  },

  /**
   * رقم جلوس الطالب ورقم اللجنة
   */
  async getStudentSeating(studentId: string, examId?: number) {
    const key = examId
      ? `seating:student:${studentId}:exam:${examId}`
      : `seating:student:${studentId}`;
    return cache.get<any>(key);
  },

  async setStudentSeating(studentId: string, seating: any, examId?: number) {
    const key = examId
      ? `seating:student:${studentId}:exam:${examId}`
      : `seating:student:${studentId}`;
    await cache.set(key, seating);
  },

  /**
   * قائمة الطلاب في لجنة معينة (للطباعة أو العرض)
   */
  async getCommitteeStudents(committeeId: number) {
    return cache.get<any[]>(`committee-students:${committeeId}`);
  },

  async setCommitteeStudents(committeeId: number, students: any[]) {
    await cache.set(`committee-students:${committeeId}`, students);
  },

  /**
   * امتحانات اليوم الحالي (لعرضها في الداشبورد)
   */
  async getTodayExams(date: string) { // date: YYYY-MM-DD
    return cache.get<any[]>(`today:${date}`);
  },

  async setTodayExams(date: string, exams: any[]) {
    await cache.set(`today:${date}`, exams, 24 * 60 * 60); // 24 ساعة
  },

  /**
   * مسح كل الكاش الخاص بامتحان معين
   */
  async invalidateExam(examId: number) {
    await Promise.all([
      cache.delete(`schedule:*:exam:${examId}*`),
      cache.delete(`seating:*:exam:${examId}`),
      // يمكن إضافة المزيد حسب الحاجة
    ]);
  },

  /**
   * مسح كاش رقم الجلوس للطالب (بعد تعديل التوزيع)
   */
  async invalidateStudentSeating(studentId: string) {
    await cache.delete(`seating:student:${studentId}*`);
  },

  /**
   * مسح جدول الامتحانات لسنة دراسية
   */
  async invalidateSchedule(academicYearId: number) {
    await cache.delete(`schedule:year:${academicYearId}*`);
  },
};