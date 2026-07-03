/**
 * Orbis Tables — Unified table system
 *
 * Usage:
 *   import { DataTable } from "$lib/tables";
 *   import { users, withIndex } from "$lib/tables/definitions";
 */

export { DataTable } from "./core";
export { withIndex } from "./definitions";
export * as tableDefs from "./definitions";
export * from "./renderers";
