import type { createAsyncStoragePersister } from "@tanstack/query-async-storage-persister";

export declare function createOfflineQueryPersister(
  scope: string,
): ReturnType<typeof createAsyncStoragePersister>;
