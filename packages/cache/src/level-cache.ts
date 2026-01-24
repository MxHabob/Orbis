// src/cache/level-cache.ts
import { RedisCache } from "./redis-client";

const cache = new RedisCache("level", 120 * 60); // 120 minutes (2 hours) - rarely changes

export const levelCache = {
  async getById(levelId: number) {
    return cache.get<any>(`id:${levelId}`);
  },

  async setById(levelId: number, data: any) {
    await cache.set(`id:${levelId}`, data);
  },

  async getAll() {
    return cache.get<any[]>("all");
  },

  async setAll(levels: any[]) {
    await cache.set("all", levels);
  },

  async getByStage(stage: "basic" | "secondary") {
    return cache.get<any[]>(`stage:${stage}`);
  },

  async setByStage(stage: "basic" | "secondary", levels: any[]) {
    await cache.set(`stage:${stage}`, levels);
  },

  async invalidate(levelId?: number) {
    if (levelId) {
      await cache.delete(`id:${levelId}`);
    }
    await Promise.all([cache.delete("all"), cache.delete("stage:*")]);
  },
};
