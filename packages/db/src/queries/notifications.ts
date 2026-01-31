import { db } from "@db/client";
import { notifications } from "@db/schema";
import { eq, desc, and } from "drizzle-orm";
import { RedisCache } from "@midday/cache";

// Notifications are transient and user-specific, maybe short cache or just direct access for realtime feel?
// Let's cache the "unread count" or similar heavily accessed things.
// For list, maybe 30s cache.

const notifCache = new RedisCache("notifications", 30); 

export const getUserNotifications = async (userId: string, options: { 
    limit?: number, 
    status?: string | string[],
    maxPriority?: number
} = {}) => {
    const { limit = 20, status } = options;
    const cacheKey = `list:${userId}:${limit}:${JSON.stringify(status)}`;
    const cached = await notifCache.get<any>(cacheKey);
    
    if (cached) return cached;

    let conditions = eq(notifications.recipientId, userId);

    if (status) {
        const statusArray = Array.isArray(status) ? status : [status];
        if (statusArray.includes('archived')) {
            conditions = and(conditions, eq(notifications.isArchived, true)) as any;
        } else {
            conditions = and(conditions, eq(notifications.isArchived, false)) as any;
            if (statusArray.length === 1) {
                if (statusArray[0] === 'unread') {
                    conditions = and(conditions, eq(notifications.isRead, false)) as any;
                } else if (statusArray[0] === 'read') {
                    conditions = and(conditions, eq(notifications.isRead, true)) as any;
                }
            }
        }
    }

    const results = await db
        .select()
        .from(notifications)
        .where(conditions)
        .orderBy(desc(notifications.createdAt))
        .limit(limit);

    const resultsWithStatus = results.map(n => ({
        ...n,
        status: n.isArchived ? 'archived' : (n.isRead ? 'read' : 'unread') as 'archived' | 'read' | 'unread'
    }));

    const response = {
        data: resultsWithStatus,
        meta: {
            cursor: null,
            hasPreviousPage: false,
            hasNextPage: results.length === limit
        }
    };

    await notifCache.set(cacheKey, response);
    return response;
};

export const getUnreadCount = async (userId: string) => {
    const cacheKey = `unread:${userId}`;
    const cached = await notifCache.get<number>(cacheKey);
    if (cached !== undefined) return cached;

    const count = await db.$count(notifications, and(
        eq(notifications.recipientId, userId),
        eq(notifications.isRead, false),
        eq(notifications.isArchived, false)
    ));

    await notifCache.set(cacheKey, count, 10);
    return count;
};

export const updateNotificationStatus = async (userId: string, activityId: number, status: string) => {
    const update: Partial<typeof notifications.$inferInsert> = {};
    if (status === 'archived') {
        update.isArchived = true;
    } else if (status === 'read') {
        update.isRead = true;
    } else if (status === 'unread') {
        update.isRead = false;
    }

    await db
        .update(notifications)
        .set(update)
        .where(and(eq(notifications.id, activityId), eq(notifications.recipientId, userId)));

    // Invalidate caches
    // Simple way: clear all user notif cache or specific ones if we had a pattern
    // For now, let's just assume we need to re-fetch
};

export const updateAllNotificationsStatus = async (userId: string, status: string) => {
    const update: Partial<typeof notifications.$inferInsert> = {};
    if (status === 'archived') {
        update.isArchived = true;
    } else if (status === 'read') {
        update.isRead = true;
    }

    await db
        .update(notifications)
        .set(update)
        .where(and(eq(notifications.recipientId, userId), eq(notifications.isArchived, false)));
};

export const markAsRead = async (id: number, userId: string) => {
    await updateNotificationStatus(userId, id, 'read');
};

export type CreateNotificationParams = typeof notifications.$inferInsert;

export const createNotification = async (data: CreateNotificationParams) => {
    const [result] = await db.insert(notifications).values(data).returning();
    
    // Invalidate
    if (result) {
        await notifCache.delete(`list:${result.recipientId}:20`);
        await notifCache.delete(`unread:${result.recipientId}`);

        // Notify via realtime
        await notifCache.publish(`user:${result.recipientId}:notifications`, {
            type: 'NOTIFICATION_CREATED',
            payload: result
        });
    }
    return result;
};
