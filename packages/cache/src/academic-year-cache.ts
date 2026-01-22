// src/cache/academic-year-cache.ts
import { RedisCache } from "./redis-client";

const cache = new RedisCache("academic-year", 60 * 60); // 1 hour

export const academicYearCache = {
  async getCurrent() {
    return cache.get<any>("current");
  },

  async setCurrent(year: any) {
    await cache.set("current", year);
  },

  async getTerms(yearId: number) {
    return cache.get<any[]>(`terms:${yearId}`);
  },

  async setTerms(yearId: number, terms: any[]) {
    await cache.set(`terms:${yearId}`, terms);
  },

  async invalidateCurrent() {
    await cache.delete("current");
  },

  async invalidateAll() {
    await cache.delete("current");
    // يمكن توسيعها لمسح كل المفاتيح إذا لزم الأمر
  },
};