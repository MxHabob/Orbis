import type { OrbisColumnDef } from "@orbis/svelte-grid";
import type { AccountDto } from "$lib/api";
import { formatCurrency, l } from "$lib/core/i18n";
import { formatDateKey } from "$lib/core/i18n/format";
import * as m from "$lib/paraglide/messages";
import { exportToCsv } from "$lib/shared/utils/export";
import { normalizeArabic, normalizeDigits } from "$lib/shared/utils/search";
import {
  accountBalance,
  currencyFractionDigits,
  summarizeLedger,
} from "./ledger-report";

type ReportRow = Record<string, unknown>;
type LedgerSummary = ReturnType<typeof summarizeLedger>;

export function formatReportAmount(amount: number, currency: string) {
  const digits = currencyFractionDigits(currency);
  return formatCurrency(amount, currency, {
    minimumFractionDigits: digits,
    maximumFractionDigits: digits,
  });
}

export function statementTotal(
  name: string,
  summaries: ReadonlyMap<string, LedgerSummary>,
  select: (summary: LedgerSummary) => number,
  section = "",
): ReportRow {
  return {
    section,
    account_name: name,
    ...Object.fromEntries(
      [...summaries].map(([code, summary]) => [code, select(summary)]),
    ),
  };
}

export function statementRows(
  accounts: AccountDto[],
  groups: ReadonlyArray<readonly [string, keyof LedgerSummary]>,
  summaries: ReadonlyMap<string, LedgerSummary>,
): ReportRow[] {
  if (!accounts.length) return [];
  const rows: ReportRow[] = [];
  let index = 1;
  for (const [section, type] of groups) {
    for (const account of accounts) {
      if (account.account_type !== type) continue;
      rows.push({
        __idx: index++,
        section,
        account_code: account.code,
        account_name: l(account.name_ar, account.name),
        ...Object.fromEntries(
          [...summaries.keys()].map((code) => [
            code,
            accountBalance(account, code),
          ]),
        ),
      });
    }
    rows.push(
      statementTotal(
        m.finance_report_balance_total(),
        summaries,
        (summary) => summary[type],
        section,
      ),
    );
  }
  return rows;
}

export function searchReportRows<T extends ReportRow>(
  rows: T[],
  search: string,
) {
  const normalize = (value: string) =>
    normalizeArabic(normalizeDigits(value.toLowerCase()));
  const terms = normalize(search.trim()).split(/\s+/).filter(Boolean);
  if (!terms.length) return rows;
  return rows.filter((row) => {
    const text = normalize(
      [row.account_code, row.account_name, row.account_type, row.section]
        .filter(Boolean)
        .join(" "),
    );
    return terms.every((term) => text.includes(term));
  });
}

export function exportReport(
  filename: string,
  rows: ReportRow[],
  columns: OrbisColumnDef[],
) {
  return exportToCsv(
    filename,
    rows,
    columns.flatMap((column) => {
      const key = column.prop ?? column.id;
      if (!key || key === "__idx" || column.hidden) return [];
      return [
        {
          header: column.header ?? column.name ?? key,
          accessor: (row: ReportRow) => {
            const value = row[key];
            if (typeof value === "number") return value;
            if (typeof value !== "string") return "";
            return /^[\s\uFEFF]*[=+\-@]|^[\t\r\n]/.test(value)
              ? `'${value}`
              : value;
          },
        },
      ];
    }),
    formatDateKey(new Date()),
  );
}
