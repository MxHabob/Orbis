import { db } from "@db/client";
import { notifications } from "@db/schema";
import { eq, desc, and } from "drizzle-orm";
import { RedisCache } from "@midday/cache";

// Notifications are transient and user-specific, maybe short cache or just direct access for realtime feel?
// Let's cache the "unread count" or similar heavily accessed things.
// For list, maybe 30s cache.

const notifCache = new RedisCache("notifications", 30); 

export const getUserNotifications = async (userId: string, limit = 20) => {
  const cacheKey = `list:${userId}:${limit}`;
  const cached = await notifCache.get(cacheKey);
  
  if (cached) return cached;

  const results = await db
    .select()
    .from(notifications)
    .where(eq(notifications.recipientId, userId))
    .orderBy(desc(notifications.createdAt))
    .limit(limit);

  if (results) {
    await notifCache.set(cacheKey, results);
  }
  return results;
};

export const getUnreadCount = async (userId: string) => {
    // This might be better as a raw count query
    // But for now, let's keep it simple.
    // Dynamic things are tricky to cache without aggressive invalidation.
    // Midday style often prefers performance, so maybe no cache for realtime counters unless necessary.
    
    // Attempting cache for 10 seconds to burst-protect
    const cacheKey = `unread:${userId}`;
    const cached = await notifCache.get(cacheKey);
    if (cached) return cached;

    const count = await db.$count(notifications, and(
        eq(notifications.recipientId, userId),
        eq(notifications.isRead, false)
    ));

    await notifCache.set(cacheKey, count, 10);
    return count;
};

export const markAsRead = async (id: number, userId: string) => {
    await db
        .update(notifications)
        .set({ isRead: true })
        .where(and(eq(notifications.id, id), eq(notifications.recipientId, userId)));

    // Invalidate
    await notifCache.delete(`list:${userId}:20`); // Assuming default limit
    await notifCache.delete(`unread:${userId}`);
};

export type CreateNotificationParams = typeof notifications.$inferInsert;

export const createNotification = async (data: CreateNotificationParams) => {
    const [result] = await db.insert(notifications).values(data).returning();
    
    // Invalidate
    if (result) {
        await notifCache.delete(`list:${result.recipientId}:20`);
        await notifCache.delete(`unread:${result.recipientId}`);
    }
    return result;
};
