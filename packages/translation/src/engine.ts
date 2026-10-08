import { normalizeTerm } from "./normalize";
import type {
  DictionaryEntry,
  DictionaryPack,
  SuggestionRequest,
  SuggestionResult,
  TranslationLanguage,
} from "./types";
import {
  defaultNameDictionary,
  suggestPhoneticTransliteration,
  suggestNameTransliteration,
} from "./transliteration";

const wordPattern = /^[\p{L}\p{M}\p{N}]+$/u;
const whitespacePattern = /^\s+$/u;

function validateEntries(packId: string, entries: readonly DictionaryEntry[]) {
  const localKeys = new Set<string>();
  for (const entry of entries) {
    if (!entry.ar.trim() || !entry.en.trim()) {
      throw new Error(`Dictionary entries cannot be empty: ${packId}`);
    }
    const key = normalizeTerm(entry.ar, "ar");
    if (localKeys.has(key)) {
      throw new Error(`Duplicate dictionary entry in ${packId}: ${entry.ar}`);
    }
    localKeys.add(key);
  }
}

function validatePack(pack: DictionaryPack) {
  if (!pack.id.trim()) throw new Error("Dictionary pack id is required");
  if (!Number.isInteger(pack.version) || pack.version < 1) {
    throw new Error(`Invalid dictionary pack version: ${pack.id}`);
  }
  if (!Array.isArray(pack.entries)) {
    throw new Error(`Dictionary pack entries must be an array: ${pack.id}`);
  }
  validateEntries(pack.id, pack.entries);
  validateEntries(pack.id, pack.nameEntries ?? []);
}

function addEntry(
  dictionary: Map<TranslationLanguage, Map<string, string>>,
  entry: DictionaryEntry,
) {
  dictionary.get("ar")!.set(normalizeTerm(entry.ar, "ar"), entry.en.trim());
  dictionary.get("en")!.set(normalizeTerm(entry.en, "en"), entry.ar.trim());
}

function translateWithDictionary(
  value: string,
  sourceLanguage: TranslationLanguage,
  dictionary: Map<TranslationLanguage, Map<string, string>>,
): SuggestionResult {
  const parts = value.match(/[\p{L}\p{M}\p{N}]+|[^\p{L}\p{M}\p{N}]+/gu) ?? [];
  const wordIndexes = parts
    .map((part, index) => (wordPattern.test(part) ? index : -1))
    .filter((index) => index >= 0);
  if (wordIndexes.length === 0)
    return { status: "unavailable", reason: "empty" };

  const entries = dictionary.get(sourceLanguage)!;
  const replacements = new Map<
    number,
    { text: string; endPartIndex: number }
  >();

  for (let wordIndex = 0; wordIndex < wordIndexes.length; ) {
    let matchedEnd = -1;
    let matchedValue: string | undefined;

    for (let end = wordIndexes.length - 1; end >= wordIndex; end -= 1) {
      let hasOnlySpaces = true;
      for (let phraseIndex = wordIndex; phraseIndex < end; phraseIndex += 1) {
        for (
          let partIndex = wordIndexes[phraseIndex]! + 1;
          partIndex < wordIndexes[phraseIndex + 1]!;
          partIndex += 1
        ) {
          if (!whitespacePattern.test(parts[partIndex]!)) {
            hasOnlySpaces = false;
            break;
          }
        }
        if (!hasOnlySpaces) break;
      }
      if (!hasOnlySpaces) continue;

      const phrase = wordIndexes
        .slice(wordIndex, end + 1)
        .map((partIndex) => parts[partIndex]!)
        .join(" ");
      const normalizedPhrase = normalizeTerm(phrase, sourceLanguage);
      let result = entries.get(normalizedPhrase);
      if (
        result === undefined &&
        sourceLanguage === "ar" &&
        wordIndex === end
      ) {
        const norm = normalizeTerm(parts[wordIndexes[wordIndex]!]!, "ar");
        if (norm.startsWith("ال") && norm.length > 3) {
          result = entries.get(norm.slice(2));
        }
      }
      if (result !== undefined) {
        matchedEnd = end;
        matchedValue = result;
        break;
      }
    }

    if (matchedValue === undefined) {
      const wordPartIndex = wordIndexes[wordIndex]!;
      const current = parts[wordPartIndex]!;
      if (/^\p{N}+$/u.test(current)) {
        replacements.set(wordPartIndex, {
          text: current,
          endPartIndex: wordPartIndex,
        });
        wordIndex += 1;
        continue;
      }
      return { status: "unavailable", reason: "unknown-term" };
    }

    replacements.set(wordIndexes[wordIndex]!, {
      text: matchedValue,
      endPartIndex: wordIndexes[matchedEnd]!,
    });
    wordIndex = matchedEnd + 1;
  }

  let output = "";
  for (let index = 0; index < parts.length; ) {
    const replacement = replacements.get(index);
    if (replacement) {
      output += replacement.text;
      index = replacement.endPartIndex + 1;
    } else {
      output += parts[index];
      index += 1;
    }
  }

  return {
    status: "suggestion",
    text: output,
    method: "dictionary",
    approximate: false,
  };
}

export interface TranslationEngine {
  suggest(request: SuggestionRequest): SuggestionResult;
}

export function createTranslationEngine(
  packs: readonly DictionaryPack[] = [],
): TranslationEngine {
  const dictionary = new Map<TranslationLanguage, Map<string, string>>([
    ["ar", new Map()],
    ["en", new Map()],
  ]);
  const names = new Map(defaultNameDictionary);
  const seenPackIds = new Set<string>();

  for (const pack of packs) {
    validatePack(pack);
    if (seenPackIds.has(pack.id)) {
      throw new Error(`Duplicate dictionary pack id: ${pack.id}`);
    }
    seenPackIds.add(pack.id);
    for (const entry of pack.entries) addEntry(dictionary, entry);
    for (const entry of pack.nameEntries ?? []) {
      names.set(normalizeTerm(entry.ar, "ar"), entry.en.trim());
    }
  }

  return {
    suggest({ text, sourceLanguage, kind }) {
      if (!text.trim()) return { status: "unavailable", reason: "empty" };
      if (kind === "person-name") {
        return suggestNameTransliteration(text, sourceLanguage, names);
      }
      const result = translateWithDictionary(text, sourceLanguage, dictionary);
      if (result.status === "unavailable" && result.reason === "unknown-term") {
        return suggestPhoneticTransliteration(text, sourceLanguage);
      }
      return result;
    },
  };
}
