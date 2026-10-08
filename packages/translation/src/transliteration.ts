import { normalizeArabic } from "./normalize";
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
  // Male Names
  احمد: "Ahmad",
  محمد: "Mohammed",
  محمود: "Mahmoud",
  مصطفى: "Mustafa",
  علي: "Ali",
  عمر: "Omar",
  عثمان: "Othman",
  ابراهيم: "Ibrahim",
  اسماعيل: "Ismail",
  يوسف: "Yusuf",
  يعقوب: "Yaqoub",
  يونس: "Younis",
  يحيى: "Yahya",
  ياسين: "Yassin",
  طه: "Taha",
  حمزة: "Hamza",
  خالد: "Khalid",
  وليد: "Walid",
  طارق: "Tariq",
  زياد: "Ziyad",
  سعيد: "Saeed",
  سعد: "Saad",
  سعود: "Saud",
  سالم: "Salem",
  سليم: "Salim",
  سلمان: "Salman",
  سليمان: "Sulaiman",
  سلطان: "Sultan",
  فيصل: "Faisal",
  فهد: "Fahad",
  فارس: "Faris",
  فاروق: "Farooq",
  فادي: "Fadi",
  فراس: "Firas",
  فؤاد: "Fouad",
  فوزي: "Fawzi",
  فتحي: "Fathi",
  فكري: "Fikri",
  حسن: "Hassan",
  حسين: "Hussein",
  حسان: "Hassan",
  حامد: "Hamed",
  حمد: "Hamad",
  حميد: "Hamid",
  حسام: "Hossam",
  هشام: "Hisham",
  هيثم: "Haytham",
  هاني: "Hani",
  حاتم: "Hatem",
  بلال: "Bilal",
  بدر: "Badr",
  بندر: "Bandar",
  بسام: "Bassam",
  باسم: "Bassem",
  بشار: "Bashar",
  بركات: "Barakat",
  بكر: "Bakr",
  برهان: "Burhan",
  ياسر: "Yasser",
  يزن: "Yazan",
  ماجد: "Majed",
  ماهر: "Maher",
  محسن: "Mohsen",
  مختار: "Mokhtar",
  منصور: "Mansour",
  منير: "Munir",
  منتصر: "Muntasir",
  مروان: "Marwan",
  مسعود: "Masoud",
  معاذ: "Muath",
  مهند: "Muhannad",
  مراد: "Murad",
  مرتضى: "Mortada",
  مازن: "Mazen",
  مصعب: "Mus'ab",
  مانع: "Manea",
  مهدي: "Mahdi",
  مدحت: "Medhat",
  ممدوح: "Mamdouh",
  مؤيد: "Moayad",
  مؤمن: "Moamen",
  مطيع: "Mutee",
  نبيل: "Nabil",
  ناجي: "Naji",
  نادر: "Nader",
  ناصر: "Nasser",
  نواف: "Nawaf",
  نايف: "Nayef",
  نوح: "Nouh",
  نزار: "Nizar",
  نسيم: "Naseem",
  نصري: "Nasri",
  نشوان: "Nashwan",
  نعمان: "Noman",
  نعيم: "Naeem",
  نور: "Noor",
  رامي: "Rami",
  رامز: "Ramez",
  رائد: "Raed",
  رياض: "Riyadh",
  راشد: "Rashid",
  رشيد: "Rashid",
  راضي: "Radi",
  رضا: "Reda",
  رفعت: "Rifat",
  رشدي: "Rushdi",
  رمزي: "Ramzi",
  تامر: "Tamer",
  طلال: "Talal",
  طاهر: "Taher",
  توفيق: "Tawfiq",
  عادل: "Adel",
  عصام: "Essam",
  عماد: "Imad",
  عاصم: "Assem",
  عاطف: "Atef",
  عقيل: "Aqeel",
  انيس: "Anis",
  انور: "Anwar",
  ايمن: "Ayman",
  ايهاب: "Ihab",
  اياد: "Iyad",
  اشرف: "Ashraf",
  عزيز: "Aziz",
  عباس: "Abbas",
  عدنان: "Adnan",
  عدي: "Oday",
  عمار: "Ammar",
  عمرو: "Amr",
  عامر: "Amer",
  عمران: "Omran",
  عيسى: "Issa",
  عرفان: "Irfan",
  علام: "Allam",
  عيد: "Eid",
  اكرم: "Akram",
  امين: "Amin",
  ادريس: "Idris",
  الياس: "Elias",
  جابر: "Jaber",
  جميل: "Jamil",
  جمال: "Jamal",
  جعفر: "Jaafar",
  جاسم: "Jasim",
  جلال: "Jalal",
  جهاد: "Jihad",
  جواد: "Jawad",
  قاسم: "Qasim",
  قيس: "Qais",
  قصي: "Qusai",
  كرم: "Karam",
  كريم: "Karim",
  كمال: "Kamal",
  كاظم: "Kazem",
  كنان: "Kinan",
  لؤي: "Luay",
  لطفي: "Lutfi",
  لبيب: "Labib",
  وائل: "Wael",
  وجدي: "Wagdi",
  وجيه: "Wagih",
  وديع: "Wadie",
  وسيم: "Waseem",
  وسام: "Wissam",
  وهيب: "Wahib",
  وضاح: "Waddah",
  وافي: "Wafi",
  زكريا: "Zakariya",
  زكي: "Zaki",
  زهير: "Zuhair",
  ظافر: "Zafer",
  صالح: "Saleh",
  صلاح: "Salah",
  صابر: "Saber",
  صبري: "Sabri",
  صادق: "Sadeq",
  صفوان: "Safwan",
  سراج: "Siraj",
  سيف: "Saif",
  سامر: "Samer",
  سمير: "Samir",
  سامي: "Sami",
  شادي: "Shadi",
  شاكر: "Shaker",
  شريف: "Sharif",
  شوقي: "Shawqi",
  شهاب: "Shehab",
  شعيب: "Shoaib",
  ضياء: "Diaa",

  // Compound Names
  عبدالله: "Abdullah",
  "عبد الله": "Abdullah",
  عبدالرحمن: "Abdulrahman",
  "عبد الرحمن": "Abdulrahman",
  عبدالعزيز: "Abdulaziz",
  "عبد العزيز": "Abdulaziz",
  عبدالكريم: "Abdulkarim",
  "عبد الكريم": "Abdulkarim",
  عبداللطيف: "Abdullatif",
  "عبد اللطيف": "Abdullatif",
  عبدالمجيد: "Abdulmajid",
  "عبد المجيد": "Abdulmajid",
  عبدالملك: "Abdulmalik",
  "عبد الملك": "Abdulmalik",
  عبدالقادر: "Abdulqader",
  "عبد القادر": "Abdulqader",
  عبدالرزاق: "Abdulrazzaq",
  "عبد الرزاق": "Abdulrazzaq",
  عبدالسلام: "Abdulsalam",
  "عبد السلام": "Abdulsalam",
  عبدالوهاب: "Abdulwahab",
  "عبد الوهاب": "Abdulwahab",
  عبدالهادي: "Abdulhadi",
  "عبد الهادي": "Abdulhadi",
  عبدالغني: "Abdulghani",
  "عبد الغني": "Abdulghani",
  عبدالباسط: "Abdulbasit",
  "عبد الباسط": "Abdulbasit",
  عبدالفتاح: "Abdulfattah",
  "عبد الفتاح": "Abdulfattah",
  عبدالحكيم: "Abdulhakim",
  "عبد الحكيم": "Abdulhakim",
  عبدالعليم: "Abdulalim",
  "عبد العليم": "Abdulalim",
  عبدالحليم: "Abdulhalim",
  "عبد الحليم": "Abdulhalim",
  عبدالجليل: "Abduljalil",
  "عبد الجليل": "Abduljalil",
  عبدالخالق: "Abdulkhaliq",
  "عبد الخالق": "Abdulkhaliq",
  عبدالمحسن: "Abdulmuhsin",
  "عبد المحسن": "Abdulmuhsin",
  عبدالصمد: "Abdulsamad",
  "عبد الصمد": "Abdulsamad",
  عبدالغفور: "Abdulghafour",
  "عبد الغفور": "Abdulghafour",
  عبدالشكور: "Abdulshakoor",
  "عبد الشكور": "Abdulshakoor",
  عبدالرؤوف: "Abdulraouf",
  "عبد الرؤوف": "Abdulraouf",
  عبدالواحد: "Abdulwahid",
  "عبد الواحد": "Abdulwahid",
  عبدالوالي: "Abdulwali",
  "عبد الوالي": "Abdulwali",
  عبدالولي: "Abdulwali",
  "عبد الولي": "Abdulwali",
  عبدالحميد: "Abdulhamid",
  "عبد الحميد": "Abdulhamid",
  عبدالباري: "Abdulbari",
  "عبد الباري": "Abdulbari",
  عبدالناصر: "Abdulnasser",
  "عبد الناصر": "Abdulnasser",
  عبدالقدوس: "Abdulquddus",
  "عبد القدوس": "Abdulquddus",
  عبدالغفار: "Abdulghaffar",
  "عبد الغفار": "Abdulghaffar",
  عبدالخبير: "Abdulkhabir",
  "عبد الخبير": "Abdulkhabir",
  عبدالصبور: "Abdulsaboor",
  "عبد الصبور": "Abdulsaboor",
  عبدالعظيم: "Abdulazim",
  "عبد العظيم": "Abdulazim",
  عبدالقدير: "Abdulqadeer",
  "عبد القدير": "Abdulqadeer",
  "عبد القوي": "Abdulqawi",
  عبدالقوي: "Abdulqawi",
  "عبد المطلب": "Abdulmuttalib",
  عبدالمطلب: "Abdulmuttalib",
  "عبد المنان": "Abdulmannan",
  عبدالمنان: "Abdulmannan",

  // Din Compounds
  "نور الدين": "Noor Al-Din",
  نورالدين: "Noor Al-Din",
  "سيف الدين": "Saif Al-Din",
  سيفالدين: "Saif Al-Din",
  "علاء الدين": "Alaa Al-Din",
  علاءالدين: "Alaa Al-Din",
  "ضياء الدين": "Diaa Al-Din",
  ضياءالدين: "Diaa Al-Din",
  "عماد الدين": "Imad Al-Din",
  عمادالدين: "Imad Al-Din",
  "حسام الدين": "Hussam Al-Din",
  حسامالدين: "Hussam Al-Din",
  "بهاء الدين": "Bahaa Al-Din",
  بهاءالدين: "Bahaa Al-Din",
  "جمال الدين": "Jamal Al-Din",
  جمالالدين: "Jamal Al-Din",
  "جلال الدين": "Jalal Al-Din",
  جلالالدين: "Jalal Al-Din",
  "كمال الدين": "Kamal Al-Din",
  كمالالدين: "Kamal Al-Din",
  "صلاح الدين": "Salah Al-Din",
  صلاحالدين: "Salah Al-Din",
  "شرف الدين": "Sharaf Al-Din",
  شرفالدين: "Sharaf Al-Din",
  "تقي الدين": "Taqi Al-Din",
  تقيالدين: "Taqi Al-Din",
  "عز الدين": "Ezz Al-Din",
  عزالدين: "Ezz Al-Din",
  "محي الدين": "Mohi Al-Din",
  "محيي الدين": "Mohi Al-Din",
  محيالدين: "Mohi Al-Din",
  "خير الدين": "Khair Al-Din",
  خيرالدين: "Khair Al-Din",
  "بدر الدين": "Badr Al-Din",
  بدرالدين: "Badr Al-Din",
  "شمس الدين": "Shams Al-Din",
  شمسالدين: "Shams Al-Din",

  // Patronymics & Titles
  "ابو بكر": "Abu Bakr",
  "ابو الحسن": "Abu Al-Hassan",
  "ابو القاسم": "Abu Al-Qasim",
  "ابو الفضل": "Abu Al-Fadl",
  "ابن خلدون": "Ibn Khaldun",
  "ابن سينا": "Ibn Sina",
  "ابن رشد": "Ibn Rushd",
  "ابن الهيثم": "Ibn Al-Haytham",
  "ابن النفيس": "Ibn Al-Nafis",
  "ابن بطوطة": "Ibn Battuta",
  "ابن كثير": "Ibn Kathir",
  "ابن تيمية": "Ibn Taymiyyah",
  "ابن القيم": "Ibn Al-Qayyim",
  "بن علي": "Bin Ali",
  "بن زايد": "Bin Zayed",
  "بن سلمان": "Bin Salman",
  "بن راشد": "Bin Rashid",
  "آل سعود": "Al Saud",
  "آل نهيان": "Al Nahyan",
  "آل ثاني": "Al Thani",
  "آل خليفة": "Al Khalifa",
  "آل صباح": "Al Sabah",
  "الهبوب": "Al Haboob",

  // Female Names
  فاطمة: "Fatimah",
  "فاطمة الزهراء": "Fatimah Al-Zahra",
  فاطمه: "Fatimah",
  مريم: "Maryam",
  مارية: "Maria",
  عائشة: "Aisha",
  عائشه: "Aisha",
  خديجة: "Khadija",
  خديجه: "Khadija",
  زينب: "Zainab",
  رقية: "Ruqayya",
  "ام كلثوم": "Umm Kulthum",
  سارة: "Sarah",
  ساره: "Sarah",
  هاجر: "Hajar",
  اسماء: "Asmaa",
  اسراء: "Israa",
  ايه: "Aya",
  ايات: "Ayat",
  الاء: "Alaa",
  اروى: "Arwa",
  اريج: "Areej",
  اماني: "Amani",
  امل: "Amal",
  امينة: "Amina",
  اميرة: "Amira",
  انيسة: "Anisa",
  احلام: "Ahlam",
  انوار: "Anwar",
  ايمان: "Iman",
  ايناس: "Enas",
  ابتسام: "Ibtisam",
  ابتهال: "Ibtihal",
  بسمة: "Basma",
  بشرى: "Bushra",
  بلقيس: "Bilqis",
  بيان: "Bayan",
  تغريد: "Taghreed",
  تسنيم: "Tasneem",
  حنان: "Hanan",
  حنين: "Haneen",
  هدى: "Huda",
  هلا: "Hala",
  هبة: "Hiba",
  هديل: "Hadeel",
  خلود: "Khulood",
  دعاء: "Duaa",
  دينا: "Dina",
  دلال: "Dalal",
  رانية: "Rania",
  رانيا: "Rania",
  روان: "Rawan",
  ريم: "Reem",
  ريما: "Rima",
  رنا: "Rana",
  رشا: "Rasha",
  رندا: "Randa",
  رزان: "Razan",
  رحاب: "Rehab",
  رغد: "Raghad",
  ربى: "Ruba",
  رولا: "Rola",
  روضة: "Rawda",
  زهراء: "Zahra",
  زين: "Zain",
  زينة: "Zeina",
  سحر: "Sahar",
  سمر: "Samar",
  سميرة: "Samira",
  سهى: "Suha",
  سهام: "Siham",
  سلمى: "Salma",
  سلوى: "Salwa",
  سندس: "Sondos",
  شيماء: "Shaimaa",
  شذى: "Shatha",
  شروق: "Shorooq",
  صفاء: "Safaa",
  صباح: "Sabah",
  عبير: "Abeer",
  عفاف: "Afaf",
  علا: "Ola",
  غادة: "Ghada",
  غدير: "Ghadir",
  فاتن: "Faten",
  فدوى: "Fadwa",
  فريدة: "Farida",
  فيروز: "Fairouz",
  كوثر: "Kawthar",
  ليلى: "Layla",
  ليلي: "Layla",
  لينا: "Lina",
  لما: "Lama",
  لمى: "Lama",
  لبنى: "Loubna",
  مي: "May",
  ميساء: "Maysaa",
  مها: "Maha",
  منال: "Manal",
  منى: "Mona",
  مروة: "Marwa",
  ملك: "Malak",
  مليكة: "Malika",
  ميادة: "Mayada",
  منار: "Manar",
  ندى: "Nada",
  نادية: "Nadia",
  نجوى: "Najwa",
  نهى: "Noha",
  نورة: "Noura",
  نوره: "Norah",
  هند: "Hind",
  وفاء: "Wafaa",
  ولاء: "Walaa",
  وداد: "Widad",
  ياسمين: "Yasmin",
  يسرى: "Yusra",
  جوري: "Jouri",
  جودي: "Judy",
  لمار: "Lamar",
  جود: "Joud",
  ليان: "Lian",
  تالا: "Tala",
  يارا: "Yara",
  ريتال: "Rital",
  دانية: "Dania",
  غزل: "Ghazal",
  جنة: "Janna",
  ريتاج: "Retaj",
  سلسبيل: "Salsabil",
  افنان: "Afnan",
  نجلاء: "Naglaa",
  نرمين: "Nermeen",
  ميس: "Mais",
  سماح: "Samah",
  صفية: "Safiya",
  جوهرة: "Jawhara",
  بثينة: "Buthaina",
  ثريا: "Thuraya",
  سعاد: "Suad",
  جميلة: "Jamila",
  حليمة: "Halima",
  ميمونة: "Maimoona",
  جويرية: "Juwairiya",
  حفصة: "Hafsa",
  سودة: "Sawda",

  // Titles
  الشيخ: "Sheikh",
  الدكتور: "Dr.",
  الدكتورة: "Dr.",
  الاستاذ: "Mr.",
  الاستاذة: "Ms.",
  المهندس: "Eng.",
  المهندسة: "Eng.",
  بن: "Bin",
  ابن: "Ibn",
  بنت: "Bint",
  آل: "Al",
  ابو: "Abu",
  ام: "Umm",
};

export const defaultNameDictionary = new Map(
  Object.entries(commonNames).map(([name, spelling]) => [
    normalizeArabic(name),
    spelling,
  ]),
);

const arabicLetter = /^\p{Script=Arabic}$/u;
const latinLetter = /^\p{Script=Latin}$/u;
const mark = /^\p{M}$/u;
const digit = /^\p{N}$/u;
const wordPattern = /^[\p{L}\p{M}\p{N}]+$/u;
const whitespacePattern = /^\s+$/u;

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
  if (sourceLanguage === "en") {
    const wordRegex = /[\p{L}\p{M}\p{N}]+/gu;
    return value.replace(wordRegex, (word) => transliterateLatinWord(word));
  }

  const parts = value.match(/[\p{L}\p{M}\p{N}]+|[^\p{L}\p{M}\p{N}]+/gu) ?? [];
  const wordIndexes = parts
    .map((part, index) => (wordPattern.test(part) ? index : -1))
    .filter((index) => index >= 0);
  if (wordIndexes.length === 0) return value;

  const replacements = new Map<
    number,
    { text: string; endPartIndex: number }
  >();

  for (let wordIndex = 0; wordIndex < wordIndexes.length; ) {
    let matchedEnd = -1;
    let matchedValue: string | undefined;

    const maxPhraseLen = Math.min(3, wordIndexes.length - wordIndex);
    for (let len = maxPhraseLen; len >= 1; len -= 1) {
      const end = wordIndex + len - 1;
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
      const normalizedPhrase = normalizeArabic(phrase);
      const result = nameDictionary.get(normalizedPhrase);
      if (result !== undefined) {
        matchedEnd = end;
        matchedValue = result;
        break;
      }
    }

    if (matchedValue !== undefined) {
      replacements.set(wordIndexes[wordIndex]!, {
        text: matchedValue,
        endPartIndex: wordIndexes[matchedEnd]!,
      });
      wordIndex = matchedEnd + 1;
    } else {
      const wordPartIndex = wordIndexes[wordIndex]!;
      const current = parts[wordPartIndex]!;
      const normalized = normalizeArabic(current);
      const transliterated =
        nameDictionary.get(normalized) ?? transliterateArabicWord(current);
      replacements.set(wordPartIndex, {
        text: transliterated,
        endPartIndex: wordPartIndex,
      });
      wordIndex += 1;
    }
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

  return output;
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

  const normalized = normalizeArabic(text);

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

export function suggestPhoneticTransliteration(
  value: string,
  sourceLanguage: TranslationLanguage,
): SuggestionResult {
  const text = value.trim();
  if (!text) return { status: "unavailable", reason: "empty" };

  const sourceScript = sourceLanguage === "ar" ? arabicLetter : latinLetter;
  if (![...text].some((character) => sourceScript.test(character))) {
    return { status: "unavailable", reason: "unsupported-script" };
  }

  const transliterated = transliterateText(text, sourceLanguage, new Map());
  if (transliterated === text || !/\p{L}/u.test(transliterated)) {
    return { status: "unavailable", reason: "unsupported-script" };
  }

  return {
    status: "suggestion",
    text: transliterated,
    method: "transliteration",
    approximate: true,
  };
}

