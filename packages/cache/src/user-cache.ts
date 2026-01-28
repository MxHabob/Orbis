import { RedisCache } from "./redis-client";

// Redis-based cache for users shared across all server instances
const cache = new RedisCache("user", 30 * 60); // 30 minutes TTL

export const userCache = {
  get: (key: string): Promise<any | undefined> => cache.get(key),
  set: (key: string, value: any): Promise<void> => cache.set(key, value),
  delete: (key: string): Promise<void> => cache.delete(key),

  // Specific methods for auth
  setById: (id: string, user: any) => cache.set(id, user),
  setByEmail: (email: string, user: any) => cache.set(`email:${email}`, user),
  invalidate: async (id: string, user?: any) => {
    await cache.delete(id);
    if (user?.email) {
      await cache.delete(`email:${user.email}`);
    }
  }
};
