/**
 * @orbis/sdk-web — Web SDK for the Orbis platform.
 *
 * Implements OrbisCommands via HTTP fetch (API Gateway).
 * Drop-in replacement for @orbis/sdk-desktop in web apps.
 *
 * Usage:
 * ```ts
 * import { configure, commands } from "@orbis/sdk-web";
 *
 * configure({
 *   baseUrl: "https://api.orbis.school",
 *   getToken: () => localStorage.getItem("token"),
 *   onUnauthorized: () => router.push("/login"),
 * });
 *
 * const classes = await commands.academic.listClasses(orgId);
 * ```
 */

export { commands } from "./commands.js";
export { configure, request, http } from "./http.js";
export type { HttpClientConfig } from "./http.js";
export { OrbisError, extractErrorMessage } from "@orbis/sdk-core";
export type { OrbisCommands } from "@orbis/sdk-core";
export type * from "@orbis/sdk-core";
