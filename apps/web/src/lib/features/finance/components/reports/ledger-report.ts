import type { AccountDto, CurrencyDto } from "$lib/api";

type LedgerAccount = Pick<AccountDto, "account_type" | "balances">;

export function accountBalance(account: LedgerAccount, currency: string): number {
  if (!account.balances || typeof account.balances !== "object") return 0;
  const balances = account.balances as Record<string, unknown>;
  const amount = Number(balances[currency.toUpperCase()] ?? balances[currency.toLowerCase()] ?? 0);
  return Number.isFinite(amount) ? amount : 0;
}

export function reportCurrencies(
  accounts: readonly LedgerAccount[],
  currencies: CurrencyDto[],
  organizationCurrency: string,
) {
  const base = (currencies.find((currency) => currency.is_base)?.code || organizationCurrency).toUpperCase();
  const byCode = new Map(currencies.map((currency) => [currency.code.toUpperCase(), currency]));
  const codes = new Set(currencies.filter((currency) => currency.is_active !== false)
    .map((currency) => currency.code.toUpperCase()));
  if (base) codes.add(base);
  for (const account of accounts) {
    for (const code of Object.keys(account.balances ?? {})) {
      if (/^[a-z]{3}$/i.test(code)) codes.add(code.toUpperCase());
    }
  }
  const columns = [...codes].sort((left, right) => {
    if (left === base) return -1;
    if (right === base) return 1;
    return left.localeCompare(right);
  }).map((code) => ({ code, name: byCode.get(code)?.name ?? code }));
  return { columns, base: base || columns[0]?.code || "" };
}

export function debitCredit(balance: number, type: AccountDto["account_type"]) {
  const signed = type === "liability" || type === "equity" || type === "revenue"
    ? -balance
    : balance;
  return { debit: Math.max(0, signed), credit: Math.max(0, -signed) };
}

export function summarizeLedger(accounts: readonly LedgerAccount[], currency: string) {
  const totals = { asset: 0, liability: 0, equity: 0, revenue: 0, expense: 0 };
  let debit = 0;
  let credit = 0;
  for (const account of accounts) {
    const type = account.account_type as keyof typeof totals;
    if (!Object.hasOwn(totals, type)) continue;
    const amount = accountBalance(account, currency);
    totals[type] += amount;
    const sides = debitCredit(amount, type);
    debit += sides.debit;
    credit += sides.credit;
  }
  const netIncome = totals.revenue - totals.expense;
  return {
    ...totals,
    debit,
    credit,
    netIncome,
    liabilitiesAndEquity: totals.liability + totals.equity + netIncome,
  };
}

export function currencyFractionDigits(currency: string) {
  return new Intl.NumberFormat("en", { style: "currency", currency })
    .resolvedOptions().maximumFractionDigits ?? 2;
}

export function balancesMatch(left: number, right: number, currency: string) {
  const digits = currencyFractionDigits(currency);
  return Math.abs(left - right) < 0.5 * 10 ** -digits;
}
