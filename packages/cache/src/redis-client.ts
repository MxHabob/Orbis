import { getSharedRedisClient } from "./shared-redis";

export class RedisCache {
  private prefix: string;
  private defaultTTL: number;

  constructor(prefix: string, defaultTTL: number = 30 * 60) {
    this.prefix = prefix;
    this.defaultTTL = defaultTTL;
  }

  // Get client lazily
  private get redis() {
    return getSharedRedisClient();
  }

  private getKey(key: string): string {
    return `${this.prefix}:${key}`;
  }

  async get<T>(key: string): Promise<T | undefined> {
    try {
      const value = await this.redis.get(this.getKey(key));
      if (value === null) return undefined;

      // Safe JSON parsing with fallback to raw value
      try {
        // Only attempt to parse if it looks like JSON (starts with { or [ or is a boolean/number)
        if (typeof value === "string" && (
            value.startsWith("{") || 
            value.startsWith("[") || 
            value === "true" || 
            value === "false" || 
            !isNaN(Number(value))
          )) {
          return JSON.parse(value) as T;
        }
        return value as unknown as T;
      } catch {
        return value as unknown as T;
      }
    } catch (error: any) {
      this.logError("get", key, error);
      return undefined;
    }
  }

  async set(key: string, value: unknown, ttlSeconds?: number): Promise<void> {
    try {
      const serializedValue =
        typeof value === "string" ? value : JSON.stringify(value);  

      const redisKey = this.getKey(key);
      const ttl = ttlSeconds ?? this.defaultTTL;  

      if (ttl > 0) {
        await this.redis.set(redisKey, serializedValue, "EX", ttl);
      } else {
        await this.redis.set(redisKey, serializedValue);
      }
    } catch (error: any) {
      this.logError("set", key, error);
    }
  }

  /**
   * Delete a single key. 
   * Uses UNLINK (non-blocking) for better performance on large values.
   */
  async delete(key: string): Promise<void> {
    try {
      // UNLINK is better than DEL in 2026 as it reclaims memory asynchronously
      await (this.redis as any).unlink?.(this.getKey(key)) || await this.redis.del(this.getKey(key));
    } catch (error: any) {
      this.logError("delete", key, error);
    }
  }

  /**
   * Delete keys matching a pattern.
   * Uses SCAN (non-blocking) instead of KEYS to prevent blocking the Redis event loop.
   */
  async deletePattern(pattern: string): Promise<void> {
    try {
      const fullPattern = this.getKey(pattern);
      let cursor = "0";
      
      do {
        // SCAN provides a non-blocking way to iterate through keys
        const [nextCursor, keys] = await (this.redis as any).scan(cursor, "MATCH", fullPattern, "COUNT", 100);
        cursor = nextCursor;
        
        if (keys && keys.length > 0) {
          // UNLINK all found keys at once
          await (this.redis as any).unlink?.(...keys) || await this.redis.del(...keys);
        }
      } while (cursor !== "0");
      
    } catch (error: any) {
      this.logError("deletePattern", pattern, error);
    }
  }

  /**
   * Invalidate the entire cache prefix.
   */
  async invalidate(): Promise<void> {
    await this.deletePattern("*");
  }

  async publish(channel: string, data: unknown): Promise<void> {
    try {
      const serializedData =
        typeof data === "string" ? data : JSON.stringify(data);
      await this.redis.publish(channel, serializedData);
    } catch (error: any) {
      this.logError("publish", channel, error);
    }
  }

  async healthCheck(): Promise<void> {
    try {
      const reply = await this.redis.ping();
      if (reply !== "PONG") {
        throw new Error("Ping did not return PONG");
      }
    } catch (error) {
      throw new Error(`Redis health check failed: ${error}`);
    }
  }

  private logError(operation: string, key: string, error: any): void {
    if (error?.code !== "ERR_REDIS_CONNECTION_CLOSED") {
      console.error(
        `[RedisCache] ${operation} error [${error?.code || 'unknown'}] for ${this.prefix} cache, key "${key}":`, 
        error?.message || error
      );
    }
  }
}