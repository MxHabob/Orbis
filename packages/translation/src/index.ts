export { createTranslationEngine, type TranslationEngine } from "./engine";
export { normalizeArabic, normalizeEnglish, normalizeTerm } from "./normalize";
export {
  suggestNameTransliteration,
  suggestPhoneticTransliteration,
} from "./transliteration";
export type {
  DictionaryEntry,
  DictionaryPack,
  SuggestionRequest,
  SuggestionResult,
  TranslationKind,
  TranslationLanguage,
} from "./types";
