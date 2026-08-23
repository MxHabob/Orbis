/**
 * Finance domain queries — fees, treasuries, payments.
 * Wraps IPC commands with TanStack Query for automatic caching.
 */
import { createQuery } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";

/** Treasuries for an organization. */
export function createTreasuriesQuery(orgId: () => string) {
  return createQuery(() => ({
    queryKey: ["treasuries", orgId()],
    queryFn: () => commands.finance.listTreasuries(),
    enabled: !!orgId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Payments list. */
export function createPaymentsQuery(orgId: () => string) {
  return createQuery(() => ({
    queryKey: ["payments", orgId()],
    queryFn: () => commands.finance.listPayments(),
    enabled: !!orgId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Ledger accounts query. */
export function createAccountsQuery(orgId: () => string) {
  return createQuery(() => ({
    queryKey: ["ledger-accounts", orgId()],
    queryFn: () => commands.finance.listAccounts(),
    enabled: !!orgId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Transaction/leaf accounts query. */
export function createTransactionAccountsQuery(orgId: string) {
  return createQuery(() => ({
    queryKey: ["transaction-accounts", orgId],
    queryFn: () => commands.finance.listTransactionAccounts(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Detailed account statement query. */
export function createAccountStatementQuery(
  accountId: () => string,
  startDate: () => string,
  endDate: () => string,
) {
  return createQuery(() => ({
    queryKey: ["account-statement", accountId(), startDate(), endDate()],
    queryFn: () =>
      commands.finance.getAccountStatement(accountId(), startDate(), endDate()),
    enabled: !!accountId() && !!startDate() && !!endDate(),
    staleTime: 1 * 60 * 1000,
  }));
}

/** Fee templates query. */
export function createFeeTemplatesQuery(orgId: string) {
  return createQuery(() => ({
    queryKey: ["fee-templates", orgId],
    queryFn: () => commands.finance.listFeeTemplates(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Student fees. Pass orgId to get all fees for org, or enrollmentId to get fees for specific enrollment. */
export function createStudentFeesQuery(
  orgOrEnrollmentId: () => string,
  isOrgId: boolean = true,
) {
  return createQuery(() => ({
    queryKey: ["student-fees", orgOrEnrollmentId()],
    queryFn: () =>
      isOrgId
        ? commands.finance.listStudentFeesByOrg()
        : commands.finance.listStudentFeesByEnrollment(orgOrEnrollmentId()),
    enabled: !!orgOrEnrollmentId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Discount templates query. */
export function createDiscountTemplatesQuery(orgId: string) {
  return createQuery(() => ({
    queryKey: ["discount-templates", orgId],
    queryFn: () => commands.finance.listDiscounts(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Currencies query. */
export function createCurrenciesQuery(orgId: string) {
  return createQuery(() => ({
    queryKey: ["currencies", orgId],
    queryFn: () => commands.finance.listCurrencies(),
    enabled: !!orgId,
    staleTime: 10 * 60 * 1000,
  }));
}
