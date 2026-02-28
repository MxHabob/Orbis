import { RedisClient } from "bun";

let sharedRedisClient: RedisClient | null = null;

/**
 * Get or create a shared Redis client instance
 * This ensures we reuse the same connection for both cache and memory providers
 */
export function getSharedRedisClient(): RedisClient {
  if (sharedRedisClient) {
    return sharedRedisClient;
  }

  const redisUrl = process.env.REDIS_URL;
  if (!redisUrl) {
    throw new Error("REDIS_URL environment variable is required");
  }

  const isProduction =
    process.env.NODE_ENV === "production" || process.env.FLY_APP_NAME;

  sharedRedisClient = new RedisClient(redisUrl, {
    connectionTimeout: isProduction ? 10000 : 5000,
    idleTimeout: 0,
    autoReconnect: true,
    maxRetries: 50, // Increase retries to handle temporary outages (~2-3 minutes)
  });


  sharedRedisClient.onclose = (error) => {
    if (error) {
      console.error("[Redis] Connection closed with error:", error.message);
    } else {
      console.log("[Redis] Connection closed cleanly");
    }
  };

  return sharedRedisClient;
}