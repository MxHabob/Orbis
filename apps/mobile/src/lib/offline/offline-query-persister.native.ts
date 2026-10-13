import * as Crypto from "expo-crypto";
import * as SecureStore from "expo-secure-store";
import * as SQLite from "expo-sqlite";
import { createAsyncStoragePersister } from "@tanstack/query-async-storage-persister";
import { getOfflineQueryCacheKey } from "./offline-query-cache";

const DATABASE_NAME = "orbis-mobile-offline-cache.db";
const ENCRYPTION_KEY_STORAGE_KEY = "orbis_offline_cache_encryption_key";

let databasePromise: Promise<SQLite.SQLiteDatabase> | undefined;

async function openEncryptedDatabase(): Promise<SQLite.SQLiteDatabase> {
  if (!(await SecureStore.isAvailableAsync())) {
    throw new Error("Secure storage is unavailable; offline cache cannot be encrypted.");
  }

  let encryptionKey = await SecureStore.getItemAsync(
    ENCRYPTION_KEY_STORAGE_KEY,
  );
  if (!encryptionKey) {
    const bytes = await Crypto.getRandomBytesAsync(32);
    encryptionKey = Array.from(bytes, (byte) =>
      byte.toString(16).padStart(2, "0"),
    ).join("");
    await SecureStore.setItemAsync(
      ENCRYPTION_KEY_STORAGE_KEY,
      encryptionKey,
      { keychainAccessible: SecureStore.AFTER_FIRST_UNLOCK },
    );
  }

  if (!/^[\da-f]{64}$/i.test(encryptionKey)) {
    throw new Error("The offline cache encryption key is invalid.");
  }

  const database = await SQLite.openDatabaseAsync(DATABASE_NAME);
  try {
    await database.execAsync(`PRAGMA key = "x'${encryptionKey}'";`);
    const cipher = await database.getFirstAsync<{
      cipher_version?: string;
    }>("PRAGMA cipher_version");
    if (!cipher?.cipher_version) {
      throw new Error("SQLCipher is not enabled in this app build.");
    }

    await database.execAsync(`
      CREATE TABLE IF NOT EXISTS persisted_query_cache (
        cache_key TEXT PRIMARY KEY NOT NULL,
        cache_value TEXT NOT NULL,
        updated_at INTEGER NOT NULL
      );
    `);
    return database;
  } catch (error) {
    await database.closeAsync();
    throw error;
  }
}

function getDatabase(): Promise<SQLite.SQLiteDatabase> {
  if (!databasePromise) {
    databasePromise = openEncryptedDatabase().catch((error: unknown) => {
      databasePromise = undefined;
      throw error;
    });
  }
  return databasePromise;
}

export function createOfflineQueryPersister(scope: string) {
  const storageKey = getOfflineQueryCacheKey(scope);
  return createAsyncStoragePersister({
    key: storageKey,
    throttleTime: 1000,
    storage: {
      async getItem(key) {
        const database = await getDatabase();
        const row = await database.getFirstAsync<{ cache_value: string }>(
          "SELECT cache_value FROM persisted_query_cache WHERE cache_key = ?",
          key,
        );
        return row?.cache_value ?? null;
      },
      async setItem(key, value) {
        const database = await getDatabase();
        await database.runAsync(
          `INSERT OR REPLACE INTO persisted_query_cache
            (cache_key, cache_value, updated_at) VALUES (?, ?, ?)`,
          key,
          value,
          Date.now(),
        );
      },
      async removeItem(key) {
        const database = await getDatabase();
        await database.runAsync(
          "DELETE FROM persisted_query_cache WHERE cache_key = ?",
          key,
        );
      },
    },
  });
}
