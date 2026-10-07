import type { SuggestionResult, TranslationLanguage } from "./types";

const arabicToLatin: Readonly<Record<string, string>> = {
  ا: "a",
  ب: "b",
  ت: "t",
  ث: "th",
  ج: "j",
  ح: "h",
  خ: "kh",
  د: "d",
  ذ: "dh",
  ر: "r",
  ز: "z",
  س: "s",
  ش: "sh",
  ص: "s",
  ض: "d",
  ط: "t",
  ظ: "z",
  ع: "'",
  غ: "gh",
  ف: "f",
  ق: "q",
  ك: "k",
  ل: "l",
  م: "m",
  ن: "n",
  ه: "h",
  و: "w",
  ي: "y",
  ى: "a",
  ة: "h",
  ء: "'",
  أ: "a",
  إ: "i",
  آ: "aa",
  ؤ: "w'",
  ئ: "y'",
  پ: "p",
  چ: "ch",
  ڤ: "v",
  گ: "g",
};

const latinDigraphs: Readonly<Record<string, string>> = {
  th: "ث",
  sh: "ش",
  kh: "خ",
  gh: "غ",
  dh: "ذ",
  ch: "تش",
  ph: "ف",
  oo: "و",
  ee: "ي",
  ou: "و",
};

const latinToArabic: Readonly<Record<string, string>> = {
  a: "ا",
  b: "ب",
  c: "ك",
  d: "د",
  e: "ي",
  f: "ف",
  g: "ج",
  h: "ه",
  i: "ي",
  j: "ج",
  k: "ك",
  l: "ل",
  m: "م",
  n: "ن",
  o: "و",
  p: "ب",
  q: "ق",
  r: "ر",
  s: "س",
  t: "ت",
  u: "و",
  v: "ف",
  w: "و",
  x: "كس",
  y: "ي",
  z: "ز",
};

const commonNames: Readonly<Record<string, string>> = {
  احمد: "Ahmad",
  ابراهيم: "Ibrahim",
  "ابو بكر": "Abu Bakr",
  اسماعيل: "Ismail",
  اسراء: "Israa",
  اسماء: "Asmaa",
  ايمن: "Ayman",
  ايه: "Aya",
  بدر: "Badr",
  بلال: "Bilal",
  تسنيم: "Tasneem",
  جاسم: "Jasim",
  حسن: "Hassan",
  حسين: "Hussein",
  خالد: "Khalid",
  خديجة: "Khadija",
  رنا: "Rana",
  ريم: "Reem",
  سارة: "Sarah",
  سعيد: "Saeed",
  سلمان: "Salman",
  سليمان: "Sulaiman",
  عائشة: "Aisha",
  عبدالله: "Abdullah",
  "عبد الرحمن": "Abdulrahman",
  عبدالرحمن: "Abdulrahman",
  علي: "Ali",
  عمر: "Omar",
  فاطمة: "Fatimah",
  فهد: "Fahad",
  ليلى: "Layla",
  محمد: "Mohammed",
  مريم: "Maryam",
  مصطفى: "Mustafa",
  نور: "Noor",
  هند: "Hind",
  يوسف: "Yusuf",
};

export const defaultNameDictionary = new Map(
  Object.entries(commonNames).map(([name, spelling]) => [
    name.normalize("NFKC"),
    spelling,
  ]),
);

const arabicLetter = /^\p{Script=Arabic}$/u;
const latinLetter = /^\p{Script=Latin}$/u;
const mark = /^\p{M}$/u;
const digit = /^\p{N}$/u;

function hasOnlyScript(value: string, language: TranslationLanguage): boolean {
  for (const character of value) {
    if (
      /\s/u.test(character) ||
      digit.test(character) ||
      !/\p{L}/u.test(character)
    ) {
      continue;
    }
    if (!(language === "ar" ? arabicLetter : latinLetter).test(character)) {
      return false;
    }
  }
  return true;
}

function transliterateArabicWord(word: string): string {
  let output = "";
  for (const character of word) {
    if (mark.test(character)) continue;
    output += arabicToLatin[character] ?? character;
  }
  return output;
}

function transliterateLatinWord(word: string): string {
  const normalized = word
    .normalize("NFD")
    .replace(/\p{M}/gu, "")
    .toLocaleLowerCase("en");
  let output = "";

  for (let index = 0; index < normalized.length; ) {
    const digraph = normalized.slice(index, index + 2);
    if (latinDigraphs[digraph]) {
      output += latinDigraphs[digraph];
      index += 2;
      continue;
    }

    const character = normalized[index]!;
    output += latinToArabic[character] ?? character;
    index += 1;
  }

  return output;
}

function transliterateText(
  value: string,
  sourceLanguage: TranslationLanguage,
  nameDictionary: ReadonlyMap<string, string>,
): string {
  const wordPattern = /[\p{L}\p{M}\p{N}]+/gu;
  return value.replace(wordPattern, (word) => {
    if (sourceLanguage === "en") return transliterateLatinWord(word);
    const normalized = word
      .normalize("NFKC")
      .replace(/[\u064B-\u065F\u0670\u06D6-\u06ED]/gu, "")
      .replace(/[أإآٱ]/gu, "ا")
      .replace(/ى/gu, "ي");
    return nameDictionary.get(normalized) ?? transliterateArabicWord(word);
  });
}

export function suggestNameTransliteration(
  value: string,
  sourceLanguage: TranslationLanguage,
  nameDictionary: ReadonlyMap<string, string> = defaultNameDictionary,
): SuggestionResult {
  const text = value.trim();
  if (!text) return { status: "unavailable", reason: "empty" };

  if (!hasOnlyScript(text, sourceLanguage)) {
    return { status: "unavailable", reason: "unsupported-script" };
  }

  const normalized = text
    .normalize("NFKC")
    .replace(/[\u064B-\u065F\u0670\u06D6-\u06ED]/gu, "")
    .replace(/[أإآٱ]/gu, "ا")
    .replace(/ى/gu, "ي")
    .toLocaleLowerCase(sourceLanguage === "ar" ? "ar" : "en")
    .replace(/\s+/gu, " ")
    .trim();

  if (sourceLanguage === "ar") {
    const knownName = nameDictionary.get(normalized);
    if (knownName) {
      return {
        status: "suggestion",
        text: knownName,
        method: "dictionary",
        approximate: true,
      };
    }
  }

  const transliterated = transliterateText(
    text,
    sourceLanguage,
    nameDictionary,
  );
  if (!/\p{L}/u.test(transliterated)) {
    return { status: "unavailable", reason: "unknown-term" };
  }

  return {
    status: "suggestion",
    text: transliterated,
    method: "transliteration",
    approximate: true,
  };
}
