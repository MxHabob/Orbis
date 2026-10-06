<!--
  @component LocaleSettings
  A reusable locale/language selection row using `SearchableSelect` and `SettingsRow`.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { locales } from "$lib/paraglide/runtime";
import SearchableSelect from "$lib/shared/components/searchable-select.svelte";
import SettingsRow from "$lib/shared/components/settings-row.svelte";

interface LocaleOption {
  code: string;
  name: string;
  nativeName: string;
  flag: string;
  region?: string;
}

const localeOptions: LocaleOption[] = [
  { code: "ar", name: "Arabic", nativeName: "العربية", flag: "🌐" },
  { code: "en", name: "English", nativeName: "English", flag: "🌐" },
  { code: "ar-YE", name: "Arabic (Yemen)", nativeName: "العربية (اليمن)", flag: "🇾🇪", region: m.shared_ui_country_yemen() },
  { code: "ar-SA", name: "Arabic (Saudi Arabia)", nativeName: "العربية (السعودية)", flag: "🇸🇦", region: m.shared_ui_country_saudi_arabia() },
  { code: "ar-EG", name: "Arabic (Egypt)", nativeName: "العربية (مصر)", flag: "🇪🇬", region: m.shared_ui_country_egypt() },
  { code: "ar-AE", name: "Arabic (UAE)", nativeName: "العربية (الإمارات)", flag: "🇦🇪", region: m.shared_ui_country_united_arab_emirates() },
  { code: "en-US", name: "English (United States)", nativeName: "English (US)", flag: "🇺🇸", region: "United States" },
  { code: "en-GB", name: "English (United Kingdom)", nativeName: "English (UK)", flag: "🇬🇧", region: "United Kingdom" },
];

let {
  title = m.settings_general_locale(),
  description = m.settings_general_locale_desc(),
  currentLocale = $bindable("ar"),
  selectionMode = "language",
  disabled = false,
  last = false,
  onchange,
}: {
  title?: string;
  description?: string;
  currentLocale?: string;
  selectionMode?: "language" | "regional";
  disabled?: boolean;
  last?: boolean;
  onchange?: (locale: string) => void | boolean | Promise<void | boolean>;
} = $props();

const supportedLocales = $derived(
  localeOptions.filter((option) =>
    selectionMode === "regional"
      ? true
      : (locales as readonly string[]).includes(option.code),
  ),
);
const selectedLocale = $derived.by(() => {
  if (supportedLocales.some((option) => option.code === currentLocale)) {
    return currentLocale;
  }
  const language = currentLocale.split("-")[0]?.toLowerCase();
  return supportedLocales.find((option) => option.code === language)?.code ?? locales[0];
});

function getLocaleValue(loc: LocaleOption): string {
  return loc.code;
}

function getLocaleLabel(loc: LocaleOption): string {
  return `${loc.flag} ${loc.nativeName} (${loc.name})`;
}

function getLocaleSubtitle(loc: LocaleOption): string | undefined {
  return loc.region;
}

function getLocaleBadge(loc: LocaleOption): string | undefined {
  return loc.code;
}

async function handleChange(val: string | null) {
  const next = supportedLocales.find((option) => option.code === val)?.code;
  if (!next || next === currentLocale) return;
  const previous = currentLocale;
  currentLocale = next;
  const saved = await onchange?.(next);
  if (saved === false) currentLocale = previous;
}
</script>

<SettingsRow {title} {description} {last}>
  {#snippet children()}
    <SearchableSelect
      value={selectedLocale}
      items={supportedLocales}
      placeholder={m.settings_general_locale()}
      searchPlaceholder={m.shared_ui_search_locales_placeholder()}
      emptyText={m.shared_ui_no_matching_locales()}
      onchange={handleChange}
      {disabled}
      getItemValue={getLocaleValue}
      getItemLabel={getLocaleLabel}
      getItemSubtitle={getLocaleSubtitle}
      getItemBadge={getLocaleBadge}
      class="w-full"
    />
  {/snippet}
</SettingsRow>
