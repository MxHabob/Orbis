/**
 * Finance domain queries — fees, treasuries, payments.
 * Wraps IPC commands with TanStack Query for automatic caching.
 */
import { createQuery } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";

/** Treasuries for an organization. */
export function createTreasuriesQuery() {
  return createQuery({
    queryKey: ["treasuries"],
    queryFn: () => commands.finance.listTreasuries(),
    staleTime: 5 * 60 * 1000,
  });
}

/** Payments list. */
export function createPaymentsQuery(enrollmentId?: string, feeId?: string) {
  return createQuery({
    queryKey: ["payments", enrollmentId, feeId],
    queryFn: () => commands.finance.listPayments(enrollmentId, feeId),
    staleTime: 2 * 60 * 1000,
  });
}

/** Ledger accounts query. */
export function createLedgerAccountsQuery() {
  return createQuery({
    queryKey: ["ledger-accounts"],
    queryFn: () => commands.finance.listLedgerAccounts(),
    staleTime: 5 * 60 * 1000,
  });
}

/** Transaction/leaf accounts query. */
export function createTransactionAccountsQuery() {
  return createQuery({
    queryKey: ["transaction-accounts"],
    queryFn: () => commands.finance.listTransactionAccounts(),
    staleTime: 5 * 60 * 1000,
  });
}

/** Detailed account statement query. */
export function createAccountStatementQuery(
  accountId: string,
  startDate: string,
  endDate: string,
) {
  return createQuery({
    queryKey: ["account-statement", accountId, startDate, endDate],
    queryFn: () =>
      commands.finance.getAccountStatement(accountId, startDate, endDate),
    enabled: !!accountId && !!startDate && !!endDate,
    staleTime: 1 * 60 * 1000,
  });
}

/** Fee templates query. */
export function createFeeTemplatesQuery() {
  return createQuery({
    queryKey: ["fee-templates"],
    queryFn: () => commands.finance.listFeeTemplates(),
    staleTime: 5 * 60 * 1000,
  });
}

/** Student fees for an enrollment. */
export function createStudentFeesQuery(enrollmentId: string) {
  return createQuery({
    queryKey: ["student-fees", enrollmentId],
    queryFn: () => commands.finance.listStudentFees(enrollmentId),
    enabled: !!enrollmentId,
    staleTime: 2 * 60 * 1000,
  });
}

/** Discount templates query. */
export function createDiscountTemplatesQuery() {
  return createQuery({
    queryKey: ["discount-templates"],
    queryFn: () => commands.finance.listDiscountTemplates(),
    staleTime: 5 * 60 * 1000,
  });
}

/** Currencies query. */
export function createCurrenciesQuery() {
  return createQuery({
    queryKey: ["currencies"],
    queryFn: () => commands.finance.listCurrencies(),
    staleTime: 10 * 60 * 1000,
  });
}
