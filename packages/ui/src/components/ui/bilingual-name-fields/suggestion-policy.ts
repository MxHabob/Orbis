export interface SuggestionTargetState {
  targetValue: string;
  targetContainsCurrentSuggestion: boolean;
  linkedSourceStillMatches: boolean;
  mayReplaceUneditedInitialValue: boolean;
}

export function canApplyTranslationSuggestion(
  state: SuggestionTargetState,
): boolean {
  return (
    !state.targetValue.trim() ||
    state.targetContainsCurrentSuggestion ||
    state.linkedSourceStillMatches ||
    state.mayReplaceUneditedInitialValue
  );
}
