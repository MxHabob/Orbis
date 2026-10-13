import { createAsyncStoragePersister } from "@tanstack/query-async-storage-persister";

const sessionCache = new Map<string, string>();

export function createOfflineQueryPersister(scope: string) {
  return createAsyncStoragePersister({
    key: `orbis-mobile-query-cache:v1:${scope}`,
    throttleTime: 1000,
    storage: {
      async getItem(key) {
        return sessionCache.get(key) ?? null;
      },
      async setItem(key, value) {
        sessionCache.set(key, value);
      },
      async removeItem(key) {
        sessionCache.delete(key);
      },
    },
  });
}
