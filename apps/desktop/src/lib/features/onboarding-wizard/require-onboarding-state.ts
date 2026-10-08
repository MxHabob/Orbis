import type { OnboardingStateDto } from "$lib/api/types";

export function requireOnboardingState(
  state: OnboardingStateDto | null,
  failureMessage: string,
): OnboardingStateDto {
  if (!state) {
    throw new Error(failureMessage);
  }
  return state;
}
