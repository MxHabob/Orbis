import { RedisClient } from "bun";

let sharedRedisClient: RedisClient | null = null;

/**
 * Get or create a shared Redis client instance.
 * This ensures we reuse the same connection for both cache and memory providers.
 * 
 * For local deployments: Redis runs in Docker on localhost:6379
 * For cloud: Uses REDIS_URL from environment
 */
export function getSharedRedisClient(): RedisClient {
  if (sharedRedisClient) {
    return sharedRedisClient;
  }

  const redisUrl = process.env.REDIS_URL;
  if (!redisUrl) {
    console.warn(
      "[Redis] REDIS_URL not set. Cache operations will fail silently. " +
      "For local deployment, run: docker compose -f docker-compose.local.yml up -d"
    );
    // Return a no-op client that won't crash the app
    return createNoOpRedisClient() as any;
  }

  const isProduction =
    process.env.NODE_ENV === "production" || process.env.FLY_APP_NAME;

  sharedRedisClient = new RedisClient(redisUrl, {
    connectionTimeout: isProduction ? 10000 : 5000,
    idleTimeout: 0,
    autoReconnect: true,
    maxRetries: 50,
  });

  sharedRedisClient.onclose = (error) => {
    if (error) {
      console.error("[Redis] Connection closed with error:", error.message);
    } else {
      console.log("[Redis] Connection closed cleanly");
    }
    // Allow reconnection
    sharedRedisClient = null;
  };

  return sharedRedisClient;
}

/**
 * No-op Redis client for when Redis is unavailable.
 * All operations succeed silently — the app works without cache.
 */
function createNoOpRedisClient() {
  const noop = {
    get: async () => null,
    set: async () => "OK",
    del: async () => 0,
    unlink: async () => 0,
    ping: async () => "PONG",
    publish: async () => 0,
    scan: async () => ["0", []],
    subscribe: async () => {},
    onclose: null,
  };
  return noop;
}