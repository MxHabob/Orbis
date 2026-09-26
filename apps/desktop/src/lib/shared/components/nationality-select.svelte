<!--
  @component NationalitySelect
  Dedicated Nationality & Country selector powered by @orbis/location.
  Supports bilingual Arabic/English nationality names, flags, search, and ISO codes.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { l } from "$lib/core/i18n";
import { getLocale } from "$lib/paraglide/runtime";
import { countries } from "@orbis/location/countries-intl";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { Input } from "@orbis/ui/input";
import { Search } from "@orbis/icons";

interface Props {
  value?: string;
  placeholder?: string;
  disabled?: boolean;
  size?: "sm" | "default";
  class?: string;
  onchange?: () => void;
}

let {
  value = $bindable(""),
  placeholder = m.common_select_nationality?.() || "اختر الجنسية...",
  disabled = false,
  size = "default",
  class: className = "",
  onchange,
}: Props = $props();

// Common Arabic nationalities dictionary mapping ISO alpha2 to Arabic nationality / demonym
const ARABIC_NATIONALITIES: Record<string, { ar: string; en: string }> = {
  SA: { ar: "سعودي", en: "Saudi" },
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
  YE: { ar: "يمني", en: "Yemeni" },
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
const PRIORITY_CODES = ["SA", "EG", "AE", "JO", "SY", "LB", "PS", "KW", "QA", "BH", "OM", "YE", "IQ", "SD", "MA", "IN", "PK", "BD", "PH", "US", "GB"];

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

  // Sort with priority codes on top, then alphabetical
  return mapped.sort((a, b) => {
    const aPri = PRIORITY_CODES.indexOf(a.alpha2);
    const bPri = PRIORITY_CODES.indexOf(b.alpha2);
    if (aPri !== -1 && bPri !== -1) return aPri - bPri;
    if (aPri !== -1) return -1;
    if (bPri !== -1) return 1;
    return a.demonym.localeCompare(b.demonym);
  });
});

let searchQuery = $state("");

const filteredCountries = $derived.by(() => {
  if (!searchQuery.trim()) return countryList;
  const q = searchQuery.toLowerCase().trim();
  return countryList.filter(
    (c) =>
      c.name.toLowerCase().includes(q) ||
      c.nameAr.toLowerCase().includes(q) ||
      c.demonym.toLowerCase().includes(q) ||
      c.alpha2.toLowerCase().includes(q) ||
      c.alpha3.toLowerCase().includes(q)
  );
});

// Resolve label for selected value (handles ISO codes or full names)
function getSelectedLabel(val: string): { label: string; emoji: string } {
  if (!val) return { label: placeholder, emoji: "" };
  const clean = val.trim();
  const match = countryList.find(
    (c) =>
      c.alpha2.toUpperCase() === clean.toUpperCase() ||
      c.alpha3.toUpperCase() === clean.toUpperCase() ||
      c.demonym.toLowerCase() === clean.toLowerCase() ||
      c.name.toLowerCase() === clean.toLowerCase() ||
      c.nameAr.toLowerCase() === clean.toLowerCase()
  );
  if (match) {
    return { label: match.demonym, emoji: match.emoji };
  }
  return { label: val, emoji: "🌐" };
}

const selectedDisplay = $derived(getSelectedLabel(value));

function handleSelect(newVal: string | undefined) {
  if (!newVal) return;
  value = newVal;
  onchange?.();
}
</script>

<Select
  type="single"
  value={value}
  onValueChange={handleSelect}
  {disabled}
>
  <SelectTrigger class="{size === 'sm' ? 'h-8 text-xs' : 'h-9 text-xs'} bg-card text-start {className}">
    <div class="flex items-center gap-2 truncate">
      {#if selectedDisplay.emoji}
        <span class="text-sm shrink-0">{selectedDisplay.emoji}</span>
      {/if}
      <span class="truncate">{selectedDisplay.label}</span>
    </div>
  </SelectTrigger>
  <SelectContent class="text-xs font-sans max-h-64 w-64 p-0">
    <!-- Search Bar inside dropdown -->
    <div class="p-2 border-b border-border/50 sticky top-0 bg-popover z-10">
      <div class="relative">
        <Search size={13} class="absolute inset-s-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
        <Input
          type="text"
          placeholder={l("ابحث عن الجنسية أو الدولة...", "Search nationality...")}
          class="h-7 ps-7 text-[11px]"
          bind:value={searchQuery}
        />
      </div>
    </div>

    <div class="max-h-52 overflow-y-auto p-1">
      {#each filteredCountries as c (c.alpha2)}
        <SelectItem value={c.demonym}>
          <div class="flex items-center justify-between w-full gap-2">
            <div class="flex items-center gap-2">
              <span class="text-sm shrink-0">{c.emoji}</span>
              <span class="font-medium text-foreground">{c.demonym}</span>
            </div>
            <span class="text-[10px] font-mono text-muted-foreground opacity-60">
              {c.alpha2}
            </span>
          </div>
        </SelectItem>
      {/each}
      {#if filteredCountries.length === 0}
        <div class="p-3 text-center text-xs text-muted-foreground italic">
          {l("لا توجد جنسيات مطابقة للبحث", "No matching nationalities found")}
        </div>
      {/if}
    </div>
  </SelectContent>
</Select>
