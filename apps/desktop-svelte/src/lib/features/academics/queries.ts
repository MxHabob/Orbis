/**
 * Academic domain queries — cycles, setup status.
 * Wraps IPC commands with TanStack Query for automatic caching.
 */
import { createQuery } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";

/** All academic cycles for an organization. */
export function createCyclesQuery(orgId: string) {
  return createQuery({
    queryKey: ["cycles", orgId],
    queryFn: () => commands.academics.getCycles(orgId),
    enabled: !!orgId,
    staleTime: 10 * 60 * 1000, // Cycles rarely change — 10 min
  });
}

/** Classes for an organization. */
export function createClassesQuery(orgId: string) {
  return createQuery({
    queryKey: ["classes", orgId],
    queryFn: () => commands.classes.getAll(orgId),
    enabled: !!orgId,
    staleTime: 5 * 60 * 1000,
  });
}

/** Subjects for an organization. */
export function createSubjectsQuery(orgId: string) {
  return createQuery({
    queryKey: ["subjects", orgId],
    queryFn: () => commands.subjects.getAll(orgId),
    enabled: !!orgId,
    staleTime: 10 * 60 * 1000, // Subjects rarely change
  });
}
