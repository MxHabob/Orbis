/**
 * Workflow & Approval queries.
 */
import {
  createInfiniteQuery,
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
  type OverrideRequestListParams,
} from "$lib/api";

/** Query for listing pending approvals by role or general overrides */
export function createPendingApprovalsQuery(
  orgId: () => string,
  approverRole?: () => string,
) {
  const getFilter = () => ({
    status: "pending",
    role:
      approverRole?.() && approverRole?.() !== "all"
        ? approverRole()
        : undefined,
    page: 1,
    page_size: 50,
  });
  return createQuery<OverrideRequestDto[]>(() => ({
    queryKey: queryKeys.workflow.overrides(
      orgId(),
      getFilter(),
    ),
    queryFn: async () => (await api.listOverrides(getFilter())).items,
    enabled: !!orgId(),
    staleTime: 30 * 1000,
  }));
}

/** Query for listing all override requests */
export function createOverrideRequestsQuery(
  orgId: () => string,
  filter?: () => OverrideRequestListParams,
) {
  const getFilter = () => filter?.() ?? {};
  return createInfiniteQuery(() => {
    const params = getFilter();
    return {
      queryKey: queryKeys.workflow.overrides(orgId(), params),
      queryFn: ({ pageParam }) =>
        api.listOverrides({
          ...params,
          page: pageParam,
          page_size: 30,
        }),
      initialPageParam: 1,
      getNextPageParam: (lastPage) =>
        lastPage.page < lastPage.total_pages ? lastPage.page + 1 : undefined,
      enabled: !!orgId(),
      staleTime: 30 * 1000,
    };
  });
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
