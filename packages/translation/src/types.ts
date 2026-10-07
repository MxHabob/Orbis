export type TranslationLanguage = "ar" | "en";

export type TranslationKind = "label" | "person-name";

export interface DictionaryEntry {
  ar: string;
  en: string;
}

export interface DictionaryPack {
  id: string;
  version: number;
  entries: readonly DictionaryEntry[];
  nameEntries?: readonly DictionaryEntry[];
}

export interface SuggestionRequest {
  text: string;
  sourceLanguage: TranslationLanguage;
  kind: TranslationKind;
}

export type SuggestionResult =
  | {
      status: "suggestion";
      text: string;
      method: "dictionary" | "transliteration";
      approximate: boolean;
    }
  | {
      status: "unavailable";
      reason: "empty" | "unknown-term" | "unsupported-script";
    };
