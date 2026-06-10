import { streamSSE } from "hono/streaming";
import { getSharedRedisClient } from "@orbis/cache";
import type { Context } from "hono";

export const realtimeHandler = async (c: Context) => {
  const user = c.get("user");
  
  if (!user) {
    return c.json({ error: "Unauthorized" }, 401);
  }
  console.log(`[Realtime] Starting SSE stream for user ${user.id}`);
  
  return streamSSE(c, async (stream) => {
    const subscriber = await getSharedRedisClient().duplicate();


    const channels = [
      `user:${user.id}:notifications`,
      `user:${user.id}:updates`,
      "global:updates",
    ];


    for (const channel of channels) {
      subscriber.subscribe(channel, (message) => {
        stream.writeSSE({
          data: message,
          event: channel,
        }).catch((err) => {
          console.error("[Realtime] Error writing SSE:", err);
        });
      });
    }

    console.log(`[Realtime] User ${user.id} subscribed to:`, channels);

    // Initial keep-alive
    await stream.writeSSE({
      data: JSON.stringify({ status: "connected" }),
      event: "connected",
    });

    // Cleanup on disconnect
    stream.onAbort(() => {
      console.log(`[Realtime] Stream aborted for user ${user.id}`);
      subscriber.close();
    });

    // Keep the connection alive with a heartbeat
    while (true) {
      try {
        await stream.writeSSE({
          data: "heartbeat",
          event: "ping",
        });

        await new Promise((resolve) => setTimeout(resolve, 30000));
      } catch (e) {
        break;
      }
    }
  });
};
