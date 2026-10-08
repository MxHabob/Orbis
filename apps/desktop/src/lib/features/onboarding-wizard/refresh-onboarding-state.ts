import type { OnboardingStateDto } from "$lib/api/types";
import { systemStore } from "$lib/stores";
import { requireOnboardingState } from "./require-onboarding-state";
import { fetchLatestOnboardingState } from "./onboarding-queries";

export async function refreshOnboardingState(
  orgId: string,
  failureMessage: string,
): Promise<OnboardingStateDto> {
  const state = requireOnboardingState(
    await fetchLatestOnboardingState(orgId),
    failureMessage,
  );
  systemStore.onboardingState = state;
  return state;
}
