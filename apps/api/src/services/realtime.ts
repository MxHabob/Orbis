import { streamSSE } from "hono/streaming";
import { getSharedRedisClient } from "@midday/cache";
import { RedisClient } from "bun";
import type { Context } from "hono";

export const realtimeHandler = async (c: Context) => {
  const user = c.get("user");
  
  if (!user) {
    return c.json({ error: "Unauthorized" }, 401);
  }

  return streamSSE(c, async (stream) => {
    const redisUrl = process.env.REDIS_URL;
    if (!redisUrl) {
      console.error("REDIS_URL is missing");
      return;
    }

    // Create a dedicated subscriber connection for this stream
    const subscriber = new RedisClient(redisUrl, {
      idleTimeout: 0,
      autoReconnect: true,
    });

    const channels = [
      `user:${user.id}:notifications`,
      `user:${user.id}:updates`,
      "global:updates"
    ];

    console.log(`[Realtime] User ${user.id} connected to channels:`, channels);

    // Initial keep-alive or connection confirmation
    await stream.writeSSE({
      data: JSON.stringify({ status: "connected" }),
      event: "connected",
    });

    const onMessage = async (message: string, channel: string) => {
      await stream.writeSSE({
        data: message,
        event: channel,
      });
    };

    for (const channel of channels) {
      subscriber.subscribe(channel, onMessage);
    }

    // Keep the connection alive with a heartbeat every 30 seconds
    const heartbeat = setInterval(async () => {
      try {
        await stream.writeSSE({
          data: "heartbeat",
          event: "ping",
        });
      } catch (e) {
        clearInterval(heartbeat);
      }
    }, 30000);

    stream.onAbort(async () => {
      console.log(`[Realtime] User ${user.id} disconnected`);
      clearInterval(heartbeat);
      for (const channel of channels) {
        subscriber.unsubscribe(channel);
      }
      subscriber.close();
    });
  });
};
