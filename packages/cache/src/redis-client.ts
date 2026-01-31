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

      try {
        return JSON.parse(value) as T;
      } catch {
        // fallback to raw string (backward compatibility)
        return value as unknown as T;
      }
    } catch (error) {
      console.error(`Redis get error for ${this.prefix} cache, key "${key}":`, error);
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
        // Use positional arguments for EX
        await this.redis.set(redisKey, serializedValue, "EX", ttl);
      } else {
        await this.redis.set(redisKey, serializedValue);
      }
    } catch (error) {
      console.error(`Redis set error for ${this.prefix} cache, key "${key}":`, error);
    }
  }

  async delete(key: string): Promise<void> {
    try {
      await this.redis.del(this.getKey(key));
    } catch (error) {
      console.error(`Redis delete error for ${this.prefix} cache, key "${key}":`, error);
    }
  }

  async publish(channel: string, data: unknown): Promise<void> {
    try {
      const serializedData =
        typeof data === "string" ? data : JSON.stringify(data);
      await this.redis.publish(channel, serializedData);
    } catch (error) {
      console.error(`Redis publish error on channel ${channel}:`, error);
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
}