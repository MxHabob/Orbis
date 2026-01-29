import { RedisCache } from "./redis-client";

const cache = new RedisCache("suggested-actions", 7 * 24 * 60 * 60); 

export interface SuggestedActionUsage {
  actionId: string;
  count: number;
  lastUsed: Date;
}

export const suggestedActionsCache = {
  getUsage: async (
    teamId: string,
    userId: string,
    actionId: string,
  ): Promise<SuggestedActionUsage | undefined> => {
    const key = `${teamId}:${userId}:${actionId}`;
    return cache.get<SuggestedActionUsage>(key);
  },

  getAllUsage: async (
    teamId: string,
    userId: string,
  ): Promise<Record<string, SuggestedActionUsage>> => {
    const summaryKey = `${teamId}:${userId}:summary`;
    const summary =
      await cache.get<Record<string, SuggestedActionUsage>>(summaryKey);
    return summary || {};
  },

  incrementUsage: async (
    teamId: string,
    userId: string,
    actionId: string,
  ): Promise<void> => {
    const key = `${teamId}:${userId}:${actionId}`;
    const summaryKey = `${teamId}:${userId}:summary`;

    // Get current usage
    const currentUsage = (await cache.get<SuggestedActionUsage>(key)) || {
      actionId,
      count: 0,
      lastUsed: new Date(),
    };

    // Update usage
    const updatedUsage: SuggestedActionUsage = {
      ...currentUsage,
      count: currentUsage.count + 1,
      lastUsed: new Date(),
    };

    // Update individual action cache
    await cache.set(key, updatedUsage);

    // Update summary cache
    const currentSummary =
      (await cache.get<Record<string, SuggestedActionUsage>>(summaryKey)) || {};
    currentSummary[actionId] = updatedUsage;
    await cache.set(summaryKey, currentSummary);
  },

  // Clear usage for a specific action
  clearUsage: async (
    teamId: string,
    userId: string,
    actionId: string,
  ): Promise<void> => {
    const key = `${teamId}:${userId}:${actionId}`;
    const summaryKey = `${teamId}:${userId}:summary`;

    await cache.delete(key);

    // Update summary
    const currentSummary =
      (await cache.get<Record<string, SuggestedActionUsage>>(summaryKey)) || {};
    delete currentSummary[actionId];
    await cache.set(summaryKey, currentSummary);
  },

  // Clear all usage for a user in a team
  clearAllUsage: async (teamId: string, userId: string): Promise<void> => {
    const summaryKey = `${teamId}:${userId}:summary`;
    const summary =
      (await cache.get<Record<string, SuggestedActionUsage>>(summaryKey)) || {};

    // Delete all individual action caches
    for (const actionId of Object.keys(summary)) {
      const key = `${teamId}:${userId}:${actionId}`;
      await cache.delete(key);
    }

    // Delete summary
    await cache.delete(summaryKey);
  },
};
