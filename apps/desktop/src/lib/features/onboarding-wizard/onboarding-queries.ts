import { createQuery } from "@tanstack/svelte-query";
import { api, queryClient, queryKeys } from "$lib/api";
import type { OnboardingStateDto } from "$lib/api/types";

export function onboardingStateQueryOptions(orgId: string) {
  return {
    queryKey: queryKeys.onboarding.state(orgId),
    queryFn: () => api.onboardingGetState(orgId),
  };
}

export function createOnboardingStateQuery(
  orgId: () => string,
  enabled = true,
) {
  return createQuery(() => ({
    ...onboardingStateQueryOptions(orgId()),
    enabled: enabled && Boolean(orgId()),
  }));
}

export async function fetchLatestOnboardingState(
  orgId: string,
): Promise<OnboardingStateDto> {
  return queryClient.fetchQuery({
    ...onboardingStateQueryOptions(orgId),
    staleTime: 0,
  });
}
