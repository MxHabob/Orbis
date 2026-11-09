/**
 * Workflow & Approval queries.
 */
import {
  createQuery,
  createMutation,
  useQueryClient,
} from "@tanstack/svelte-query";
import {
  api,
  queryKeys,
  type OverrideRequestDto,
  type SubmitOverrideRequestInput,
  type ApproveOverrideInput,
  type RejectOverrideInput,
} from "$lib/api";

/** Query for listing pending approvals by role or general overrides */
export function createPendingApprovalsQuery(
  orgId: () => string,
  approverRole?: () => string,
) {
  return createQuery<OverrideRequestDto[]>(() => ({
    queryKey: queryKeys.workflow.overrides(
      orgId(),
      approverRole?.() || "pending",
    ),
    queryFn: async () => {
      const overrides = await api.listOverrides("pending");
      const role = approverRole?.();
      if (role && role !== "all") {
        return overrides.filter(
          (o) => o.current_approver_role === role || !o.current_approver_role,
        );
      }
      return overrides;
    },
    enabled: !!orgId(),
    staleTime: 30 * 1000,
  }));
}

/** Query for listing all override requests */
export function createOverrideRequestsQuery(
  orgId: () => string,
  status?: () => string | undefined,
) {
  return createQuery<OverrideRequestDto[]>(() => ({
    queryKey: queryKeys.workflow.overrides(orgId(), status?.() || "all"),
    queryFn: () => api.listOverrides(status?.()),
    enabled: !!orgId(),
    staleTime: 30 * 1000,
  }));
}

/** Query for getting a single override request detail */
export function createOverrideDetailQuery(requestId: () => string) {
  return createQuery<OverrideRequestDto>(() => ({
    queryKey: queryKeys.workflow.override(requestId()),
    queryFn: () => api.getOverride(requestId()),
    enabled: !!requestId(),
    staleTime: 30 * 1000,
  }));
}

/** Mutation for submitting an override request */
export function createSubmitOverrideMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: SubmitOverrideRequestInput) => api.submitOverride(req),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.workflow.all });
    },
  }));
}

/** Mutation for approving an override */
export function createApproveOverrideMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: ApproveOverrideInput) => api.approveOverride(req),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.workflow.all });
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
      queryClient.invalidateQueries({ queryKey: queryKeys.academic.all });
    },
  }));
}

/** Mutation for rejecting an override */
export function createRejectOverrideMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: RejectOverrideInput) => api.rejectOverride(req),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.workflow.all });
    },
  }));
}
