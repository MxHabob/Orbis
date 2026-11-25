import { orgContext } from "$lib/stores/org-context.svelte";
import { createAccountsQuery, createCurrenciesQuery } from "../../queries";
import { reportCurrencies, summarizeLedger } from "./ledger-report";

export function createLedgerReport(orgId: () => string) {
  const accountsQuery = createAccountsQuery(orgId);
  const currenciesQuery = createCurrenciesQuery(orgId);
  const accounts = $derived(accountsQuery.data ?? []);
  const currencies = $derived(reportCurrencies(accounts, currenciesQuery.data ?? [], orgContext.baseCurrency));
  const summaries = $derived(new Map(currencies.columns.map(({ code }) => [code, summarizeLedger(accounts, code)])));

  return {
    get accounts() { return accounts; },
    get activeCurrencies() { return currencies.columns; },
    get baseCurrency() { return currencies.base; },
    get summaries() { return summaries; },
    get isLoading() { return accountsQuery.isLoading || currenciesQuery.isLoading; },
    get queryError() { return accountsQuery.error || currenciesQuery.error; },
    refetchAll() {
      void accountsQuery.refetch();
      void currenciesQuery.refetch();
    },
  };
}
