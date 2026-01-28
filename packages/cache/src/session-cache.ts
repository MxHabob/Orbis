import { RedisCache } from "./redis-client";

const cache = new RedisCache("session", 7 * 24 * 60 * 60); // 7 days TTL

export const sessionCache = {
  get: (key: string) => cache.get(key),
  set: (key: string, value: any) => cache.set(key, value),
  delete: (key: string) => cache.delete(key),
  
  // Specific methods for auth
  getByToken: (token: string) => cache.get(`token:${token}`),
  setByToken: (token: string, session: any) => cache.set(`token:${token}`, session),
  setById: (id: string, session: any) => cache.set(`id:${id}`, session),
  invalidate: async (key: string) => {
    // We try to delete both potential keys if we don't know which one 'key' is
    // But usually 'key' passed from better-auth secondaryStorage is the token
    await cache.delete(`token:${key}`);
    await cache.delete(`id:${key}`);
  }
};
