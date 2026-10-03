/**
 * Request Deduplication.
 *
 * When multiple components request the same data simultaneously
 * (e.g., TitleBar + SearchModal both calling getPeriods), this ensures
 * only ONE API call is made and the result is shared.
 */
const inflight = new Map<string, Promise<any>>();

export function dedup<T>(key: string, fn: () => Promise<T>): Promise<T> {
  const existing = inflight.get(key);
  if (existing) return existing as Promise<T>;

  const promise = fn().finally(() => {
    inflight.delete(key);
  });

  inflight.set(key, promise);
  return promise;
}
