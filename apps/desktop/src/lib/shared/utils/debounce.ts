/**
 * Creates a debounced version of a function.
 * The function will only execute after `ms` milliseconds have elapsed
 * since the last invocation.
 *
 * @example
 * ```ts
 * const debouncedSearch = debounce((query: string) => {
 *   store.searchQuery = query;
 * }, 150);
 * ```
 */
export function debounce<T extends (...args: any[]) => any>(
  fn: T,
  ms: number,
): (...args: Parameters<T>) => void {
  let timer: ReturnType<typeof setTimeout>;
  return (...args) => {
    clearTimeout(timer);
    timer = setTimeout(() => fn(...args), ms);
  };
}
