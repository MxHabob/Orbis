// src/cache/parent-cache.ts
import { RedisCache } from "./redis-client";

const cache = new RedisCache("parent", 30 * 60); // 30 minutes

export const parentCache = {
  async getById(parentId: string) {
    return cache.get<any>(`id:${parentId}`);
  },

  async setById(parentId: string, data: any) {
    await cache.set(`id:${parentId}`, data);
  },

  async getByPhone(phone: string) {
    return cache.get<any>(`phone:${phone}`);
  },

  async setByPhone(phone: string, data: any) {
    await cache.set(`phone:${phone}`, data);
  },

  async getChildren(parentId: string) {
    return cache.get<any[]>(`children:${parentId}`);
  },

  async setChildren(parentId: string, children: any[]) {
    await cache.set(`children:${parentId}`, children);
  },

  async invalidate(parentId: string) {
    await Promise.all([
      cache.delete(`id:${parentId}`),
      cache.delete(`phone:*`),
      cache.delete(`children:${parentId}`),
    ]);
  },
};
