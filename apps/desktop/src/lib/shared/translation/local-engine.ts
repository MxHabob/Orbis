import type {
  SuggestionRequest,
  SuggestionResult,
  TranslationEngine,
} from "@orbis/translation";

let enginePromise: Promise<TranslationEngine> | undefined;

async function getEngine(): Promise<TranslationEngine> {
  if (!enginePromise) {
    enginePromise = Promise.all([
      import("@orbis/translation"),
      import("@orbis/translation/academic"),
    ]).then(([translation, academic]) =>
      translation.createTranslationEngine([academic.academicDictionaryPack]),
    );
  }

  return enginePromise;
}

export async function suggestLocalTranslation(
  request: SuggestionRequest,
): Promise<SuggestionResult> {
  const engine = await getEngine();
  return engine.suggest(request);
}
