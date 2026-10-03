<!--
  @component NationalitySelect
  Dedicated Nationality & Country selector powered by @orbis/location and SearchableSelect.
  Supports bilingual Arabic/English nationality names, flags, search, and ISO codes.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { l } from "$lib/core/i18n";
import { getLocale } from "$lib/paraglide/runtime";
import { countries } from "@orbis/location/countries-intl";
import SearchableSelect from "$lib/shared/components/searchable-select.svelte";

interface Props {
  value?: string;
  placeholder?: string;
  disabled?: boolean;
  size?: "sm" | "default";
  class?: string;
  id?: string;
  onchange?: (val: string | null) => void;
}

let {
  value = $bindable(""),
  placeholder = m.common_select_nationality?.() || "اختر الجنسية...",
  disabled = false,
  size = "default",
  class: className = "",
  id = undefined,
  onchange,
}: Props = $props();

// Common Arabic nationalities dictionary mapping ISO alpha2 to Arabic nationality / demonym
const ARABIC_NATIONALITIES: Record<string, { ar: string; en: string }> = {
  SA: { ar: "سعودي", en: "Saudi" },
  YE: { ar: "يمني", en: "Yemeni" },
  EG: { ar: "مصري", en: "Egyptian" },
  AE: { ar: "إماراتي", en: "Emirati" },
  JO: { ar: "أردني", en: "Jordanian" },
  SY: { ar: "سوري", en: "Syrian" },
  LB: { ar: "لبناني", en: "Lebanese" },
  PS: { ar: "فلسطيني", en: "Palestinian" },
  KW: { ar: "كويتي", en: "Kuwaiti" },
  QA: { ar: "قطري", en: "Qatari" },
  BH: { ar: "بحريني", en: "Bahraini" },
  OM: { ar: "عماني", en: "Omani" },
  IQ: { ar: "عراقي", en: "Iraqi" },
  SD: { ar: "سوداني", en: "Sudanese" },
  MA: { ar: "مغربي", en: "Moroccan" },
  DZ: { ar: "جزائري", en: "Algerian" },
  TN: { ar: "تونسي", en: "Tunisian" },
  LY: { ar: "ليبي", en: "Libyan" },
  SO: { ar: "صومالي", en: "Somali" },
  DJ: { ar: "جيبوتي", en: "Djiboutian" },
  MR: { ar: "موريتاني", en: "Mauritanian" },
  IN: { ar: "هندي", en: "Indian" },
  PK: { ar: "باكستاني", en: "Pakistani" },
  BD: { ar: "بنغلاديشي", en: "Bangladeshi" },
  PH: { ar: "فلبيني", en: "Filipino" },
  ID: { ar: "إندونيسي", en: "Indonesian" },
  MY: { ar: "ماليزي", en: "Malaysian" },
  TR: { ar: "تركي", en: "Turkish" },
  US: { ar: "أمريكي", en: "American" },
  GB: { ar: "بريطاني", en: "British" },
  CA: { ar: "كندي", en: "Canadian" },
  AU: { ar: "أسترالي", en: "Australian" },
  FR: { ar: "فرنسي", en: "French" },
  DE: { ar: "ألماني", en: "German" },
  IT: { ar: "إيطالي", en: "Italian" },
  ES: { ar: "إسباني", en: "Spanish" },
  RU: { ar: "روسي", en: "Russian" },
  CN: { ar: "صيني", en: "Chinese" },
  JP: { ar: "ياباني", en: "Japanese" },
  KR: { ar: "كوري جنوبي", en: "South Korean" },
  BR: { ar: "برازيلي", en: "Brazilian" },
  MX: { ar: "مكسيكي", en: "Mexican" },
  NG: { ar: "نيجيري", en: "Nigerian" },
  ET: { ar: "إثيوبي", en: "Ethiopian" },
  ZA: { ar: "جنوب أفريقي", en: "South African" },
};

// Priority countries displayed at top of list
const PRIORITY_CODES = ["YE", "SA", "EG", "AE", "JO", "SY", "LB", "PS", "KW", "QA", "BH", "OM", "IQ", "SD", "MA", "IN", "PK", "BD", "PH", "US", "GB"];

interface CountryItem {
  alpha2: string;
  alpha3: string;
  name: string;
  nameAr: string;
  emoji: string;
  demonym: string;
}

const countryList = $derived.by<CountryItem[]>(() => {
  const isAr = getLocale() === "ar";
  const mapped: CountryItem[] = countries.map((c) => {
    const known = ARABIC_NATIONALITIES[c.alpha2];
    const demonym = known ? (isAr ? known.ar : known.en) : c.name;
    const nameAr = known ? known.ar : c.name;
    return {
      alpha2: c.alpha2,
      alpha3: c.alpha3,
      name: c.name,
      nameAr,
      emoji: c.emoji || "🌐",
      demonym,
    };
  });

  return mapped.sort((a, b) => {
    const aPri = PRIORITY_CODES.indexOf(a.alpha2);
    const bPri = PRIORITY_CODES.indexOf(b.alpha2);
    if (aPri !== -1 && bPri !== -1) return aPri - bPri;
    if (aPri !== -1) return -1;
    if (bPri !== -1) return 1;
    return a.demonym.localeCompare(b.demonym);
  });
});

function getCountryValue(c: CountryItem): string {
  return c.demonym;
}

function getCountryLabel(c: CountryItem): string {
  return `${c.emoji} ${c.demonym} (${c.name})`;
}

function getCountrySubtitle(c: CountryItem): string | undefined {
  return `${c.name} — ${c.nameAr !== c.name ? c.nameAr : ""}`;
}

function getCountryBadge(c: CountryItem): string | undefined {
  return c.alpha2;
}

function handleChange(val: string | null) {
  const next = val || "";
  value = next;
  onchange?.(next);
}
</script>

<SearchableSelect
  bind:value
  items={countryList}
  {placeholder}
  searchPlaceholder={l("ابحث عن الجنسية أو الدولة...", "Search nationality...")}
  emptyText={l("لا توجد جنسيات مطابقة للبحث", "No matching nationalities found")}
  {disabled}
  {size}
  {id}
  getItemValue={getCountryValue}
  getItemLabel={getCountryLabel}
  getItemSubtitle={getCountrySubtitle}
  getItemBadge={getCountryBadge}
  onchange={handleChange}
  class={className}
/>
