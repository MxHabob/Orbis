export const OFFLINE_QUERY_CACHE_MAX_AGE = 1000 * 60 * 60 * 24 * 7;

export function getOfflineQueryCacheKey(scope: string): string {
  return `orbis-mobile-query-cache:v1:${scope}`;
}
