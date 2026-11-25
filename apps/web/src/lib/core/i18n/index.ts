// ── Core i18n Utilities ─────────────────────────────────────────────
// Organization-aware formatting and locale detection.

export {
  formatCurrency,
  formatDate,
  formatPercent,
  formatNumber,
  formatDecimal,
  getWeekdayOptions,
} from "./format";
export { l, isAr } from "./locale";
export {
  changeAppLocale,
  localeState,
  type AppLocale,
} from "./locale-state.svelte";
