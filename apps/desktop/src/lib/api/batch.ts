/**
 * API Page Bundles — batch multiple related API calls into a single Promise.
 *
 * Reduces waterfall requests when entering a page that needs multiple datasets.
 * Uses Promise.allSettled to ensure partial failures don't block the entire load.
 */

type BundleSpec = Record<string, () => Promise<any>>;

type BundleResult<T extends BundleSpec> = {
  [K in keyof T]: Awaited<ReturnType<T[K]>> | null;
};

type BundleErrors<T extends BundleSpec> = {
  [K in keyof T]?: string;
};

export interface PageBundleResult<T extends BundleSpec> {
  /** Resolved data (null for failed calls). */
  data: BundleResult<T>;
  /** Error messages keyed by failed call name. */
  errors: BundleErrors<T>;
  /** Whether all calls succeeded. */
  ok: boolean;
}

/**
 * Execute multiple API calls in parallel and return results as a named bundle.
 */
export async function loadPageBundle<T extends BundleSpec>(
  spec: T,
): Promise<PageBundleResult<T>> {
  const keys = Object.keys(spec) as (keyof T)[];
  const promises = keys.map((key) => spec[key as string]());
  const results = await Promise.allSettled(promises);

  const data = {} as BundleResult<T>;
  const errors = {} as BundleErrors<T>;
  let ok = true;

  results.forEach((result, i) => {
    const key = keys[i];
    if (result.status === "fulfilled") {
      (data as any)[key] = result.value;
    } else {
      (data as any)[key] = null;
      (errors as any)[key] = result.reason?.message || String(result.reason);
      ok = false;
    }
  });

  return { data, errors, ok };
}

/**
 * Cursor-paginated request type — matches backend CursorRequest.
 */
export interface CursorRequest {
  cursor?: string | null;
  limit?: number;
}

/**
 * Cursor-paginated response type — matches backend CursorPage<T>.
 */
export interface CursorPage<T> {
  items: T[];
  count: number;
  next_cursor: string | null;
  has_more: boolean;
}
