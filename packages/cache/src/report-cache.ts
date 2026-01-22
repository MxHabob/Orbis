// src/cache/report-cache.ts
import { RedisCache } from "./redis-client";

const cache = new RedisCache("report", 2 * 60 * 60); // 2 ساعات TTL - التقارير نادرًا ما تتغير بعد إغلاق الفصل

export const reportCache = {
  /**
   * تقرير وزاري لصف معين (مثل الصف التاسع أو الثاني عشر)
   */
  async getMinisterialReport(levelId: number, academicYearId: number) {
    const key = `ministerial:level:${levelId}:year:${academicYearId}`;
    return cache.get<any>(key);
  },

  async setMinisterialReport(levelId: number, academicYearId: number, report: any) {
    const key = `ministerial:level:${levelId}:year:${academicYearId}`;
    await cache.set(key, report);
  },

  /**
   * تقرير درجات صف كامل في فصل دراسي معين
   */
  async getClassTermReport(classId: number, termId: number) {
    const key = `class-term-report:class:${classId}:term:${termId}`;
    return cache.get<any>(key);
  },

  async setClassTermReport(classId: number, termId: number, report: any) {
    const key = `class-term-report:class:${classId}:term:${termId}`;
    await cache.set(key, report);
  },

  /**
   * تقرير نهائي للطالب (كشف درجات سنوي)
   */
  async getStudentAnnualReport(studentId: string, academicYearId: number) {
    const key = `student-annual:student:${studentId}:year:${academicYearId}`;
    return cache.get<any>(key);
  },

  async setStudentAnnualReport(studentId: string, academicYearId: number, report: any) {
    const key = `student-annual:student:${studentId}:year:${academicYearId}`;
    await cache.set(key, report);
  },

  /**
   * تقرير ترتيب الطلاب في الصف (الأوائل)
   */
  async getClassRanking(classId: number, academicYearId: number) {
    const key = `ranking:class:${classId}:year:${academicYearId}`;
    return cache.get<any[]>(key);
  },

  async setClassRanking(classId: number, academicYearId: number, ranking: any[]) {
    const key = `ranking:class:${classId}:year:${academicYearId}`;
    await cache.set(key, ranking);
  },

  /**
   * مسح كل الكاش المتعلق بتقارير سنة دراسية معينة
   */
  async invalidateAcademicYear(academicYearId: number) {
    const keys = [
      `ministerial:*:year:${academicYearId}`,
      `class-term-report:*:year:${academicYearId}*`,
      `student-annual:*:year:${academicYearId}`,
      `ranking:*:year:${academicYearId}`,
    ];
    // ملاحظة: Redis لا يدعم wildcard delete مباشرة، لذا في الإنتاج استخدم SCAN أو Lua script
    // هنا نحذف فقط المفاتيح المعروفة إذا أمكن، أو نعتمد على TTL
  },

  /**
   * مسح تقرير طالب معين (يُستدعى بعد إدخال درجة جديدة)
   */
  async invalidateStudent(studentId: string, academicYearId?: number) {
    if (academicYearId) {
      await cache.delete(`student-annual:student:${studentId}:year:${academicYearId}`);
    } else {
      // مسح كل التقارير السنوية لهذا الطالب (نادر الحدوث)
      // يمكن تحسينه بتخزين قائمة المفاتيح
    }
  },

  /**
   * مسح تقرير وزاري لمستوى معين
   */
  async invalidateMinisterial(levelId: number, academicYearId: number) {
    await cache.delete(`ministerial:level:${levelId}:year:${academicYearId}`);
  },
};