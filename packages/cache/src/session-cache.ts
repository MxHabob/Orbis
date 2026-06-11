import { RedisCache } from "./redis-client";

const cache = new RedisCache("session", 7 * 24 * 60 * 60);

export const sessionCache = {
  get: (key: string) => cache.get(key),
  set: (key: string, value: any) => cache.set(key, value),
  delete: (key: string) => cache.delete(key),

  getByToken: (token: string) => cache.get(`token:${token}`),
  setByToken: (token: string, session: any) =>
    cache.set(`token:${token}`, session),
  setById: (id: string, session: any) => cache.set(`id:${id}`, session),
  invalidate: async (key: string) => {
    await cache.delete(`token:${key}`);
    await cache.delete(`id:${key}`);
  },
};
