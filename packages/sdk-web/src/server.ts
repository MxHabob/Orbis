/**
 * Server-side configuration for @orbis/sdk-web.
 *
 * For Next.js Server Components / Server Actions where you need
 * to call the API Gateway from the server side.
 *
 * Usage in a Server Action:
 * ```ts
 * "use server";
 * import { configureServer } from "@orbis/sdk-web/server";
 * import { commands } from "@orbis/sdk-web";
 *
 * configureServer({ getToken: async () => cookies().get("token")?.value ?? null });
 * const classes = await commands.academic.listClasses(orgId);
 * ```
 */

import { configure } from "./http";

interface ServerConfig {
  /** API Gateway URL — defaults to ORBIS_API_URL env var. */
  baseUrl?: string;
  /** Async token getter for server-side (e.g. from cookies). */
  getToken: () => string | null | Promise<string | null>;
}

export function configureServer(config: ServerConfig): void {
  const baseUrl = config.baseUrl ?? process.env.ORBIS_API_URL ?? "http://localhost:3001";

  // Wrap async getter in sync wrapper (configure expects sync)
  let cachedToken: string | null = null;
  const syncGetToken = () => cachedToken;

  // Pre-resolve token
  const resolved = config.getToken();
  if (resolved instanceof Promise) {
    resolved.then((t) => { cachedToken = t; });
  } else {
    cachedToken = resolved;
  }

  configure({
    baseUrl,
    getToken: syncGetToken,
  });
}
