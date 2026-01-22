// src/cache/schedule-cache.ts
import { RedisCache } from "./redis-client";

const cache = new RedisCache("schedule", 10 * 60); // 10 minutes - يتغير يوميًا

export const scheduleCache = {
  async getDaily(classId: number, date: string) { // date: YYYY-MM-DD
    return cache.get<any[]>(`daily:${classId}:${date}`);
  },

  async setDaily(classId: number, date: string, lessons: any[]) {
    await cache.set(`daily:${classId}:${date}`, lessons);
  },

  async getWeekly(classId: number, weekStart: string) {
    return cache.get<any>(`weekly:${classId}:${weekStart}`);
  },

  async setWeekly(classId: number, weekStart: string, schedule: any) {
    await cache.set(`weekly:${classId}:${weekStart}`, schedule);
  },

  async invalidateDaily(classId: number, date: string) {
    await cache.delete(`daily:${classId}:${date}`);
  },
};