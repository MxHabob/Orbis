/**
 * Academic Year & Promotion queries — TanStack Query hooks.
 * Wraps IPC commands with automatic caching & deduplication.
 */
import { createQuery, createMutation, useQueryClient } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";
import type {
  AcademicYearInfo,
  AcademicYear,
  CreateAcademicYearRequest,
  PromotionJob,
  PromotionDetail,
  PromotionConfig,
  OverrideRequest,
} from "@orbis/desktop-client/types";

// ── Academic Years ──────────────────────────────────────────────────

/** All academic years for an organization. */
export function createAcademicYearsQuery(orgId: string) {
  return createQuery({
    queryKey: ["academicYears", orgId],
    queryFn: () => commands.academicYears.list(orgId),
    enabled: !!orgId,
    staleTime: 10 * 60 * 1000, // Years rarely change — 10 min
  });
}

/** The currently active academic year. */
export function createCurrentYearQuery() {
  return createQuery({
    queryKey: ["academicYears", "current"],
    queryFn: () => commands.academicYears.getCurrent(),
    staleTime: 30 * 60 * 1000, // Very stable — 30 min
  });
}

/** Check if current year is archived (read-only). */
export function createIsArchivedQuery() {
  return createQuery({
    queryKey: ["academicYears", "isArchived"],
    queryFn: () => commands.academicYears.isArchived(),
    staleTime: 30 * 60 * 1000,
  });
}

/** Create a new academic year. */
export function createAcademicYearMutation(orgId: string) {
  const queryClient = useQueryClient();
  return createMutation({
    mutationFn: (request: CreateAcademicYearRequest) =>
      commands.academicYears.create(orgId, request),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["academicYears"] });
    },
  });
}

/** Switch to a different academic year. */
export function createSwitchYearMutation() {
  const queryClient = useQueryClient();
  return createMutation({
    mutationFn: (yearId: string) => commands.academicYears.switch(yearId),
    onSuccess: () => {
      // Invalidate ALL queries since the database has changed
      queryClient.invalidateQueries();
    },
  });
}

/** Archive an academic year. */
export function createArchiveYearMutation() {
  const queryClient = useQueryClient();
  return createMutation({
    mutationFn: ({ yearId, userId }: { yearId: string; userId: string }) =>
      commands.academicYears.archive(yearId, userId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["academicYears"] });
    },
  });
}

// ── Promotion ───────────────────────────────────────────────────────

/** All promotion jobs for an organization. */
export function createPromotionJobsQuery(orgId: string) {
  return createQuery({
    queryKey: ["promotionJobs", orgId],
    queryFn: () => commands.promotion.getJobs(orgId),
    enabled: !!orgId,
    staleTime: 2 * 60 * 1000,
  });
}

/** Single promotion job by ID. */
export function createPromotionJobQuery(jobId: string) {
  return createQuery({
    queryKey: ["promotionJobs", "detail", jobId],
    queryFn: () => commands.promotion.getJob(jobId),
    enabled: !!jobId,
    staleTime: 30 * 1000,
  });
}

/** Promotion details (per-student decisions) for a job. */
export function createPromotionDetailsQuery(jobId: string) {
  return createQuery({
    queryKey: ["promotionDetails", jobId],
    queryFn: () => commands.promotion.getDetails(jobId),
    enabled: !!jobId,
    staleTime: 30 * 1000,
  });
}

/** Start a promotion dry run. */
export function createDryRunMutation(orgId: string) {
  const queryClient = useQueryClient();
  return createMutation({
    mutationFn: (config: PromotionConfig) =>
      commands.promotion.dryRun(orgId, config),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["promotionJobs"] });
    },
  });
}

/** Execute a confirmed promotion. */
export function createExecutePromotionMutation() {
  const queryClient = useQueryClient();
  return createMutation({
    mutationFn: (jobId: string) => commands.promotion.execute(jobId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["promotionJobs"] });
      queryClient.invalidateQueries({ queryKey: ["academicYears"] });
    },
  });
}

/** Override a promotion decision for a student. */
export function createOverrideDecisionMutation() {
  const queryClient = useQueryClient();
  return createMutation({
    mutationFn: (request: OverrideRequest) =>
      commands.promotion.override(request),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: ["promotionDetails"] });
    },
  });
}

/** Rollback a completed promotion. */
export function createRollbackMutation() {
  const queryClient = useQueryClient();
  return createMutation({
    mutationFn: ({ jobId, userId }: { jobId: string; userId: string }) =>
      commands.promotion.rollback(jobId, userId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["promotionJobs"] });
      queryClient.invalidateQueries({ queryKey: ["academicYears"] });
    },
  });
}
