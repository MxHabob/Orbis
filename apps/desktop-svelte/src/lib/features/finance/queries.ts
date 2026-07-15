/**
 * Finance domain queries — fees, treasuries, payments.
 * Wraps IPC commands with TanStack Query for automatic caching.
 */
import { createQuery } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";

/** Treasuries for an organization. */
export function createTreasuriesQuery(orgId: () => string) {
  return createQuery({
    get queryKey() {
      return ["treasuries", orgId()];
    },
    get queryFn() {
      return () => commands.finance.listTreasuries();
    },
    get enabled() {
      return !!orgId();
    },
    staleTime: 5 * 60 * 1000,
  });
}

/** Payments list. */
export function createPaymentsQuery(orgId: () => string) {
  return createQuery({
    get queryKey() {
      return ["payments", orgId()];
    },
    get queryFn() {
      return () => commands.finance.listPayments();
    },
    get enabled() {
      return !!orgId();
    },
    staleTime: 2 * 60 * 1000,
  });
}

/** Ledger accounts query. */
export function createAccountsQuery(orgId: () => string) {
  return createQuery({
    get queryKey() {
      return ["ledger-accounts", orgId()];
    },
    queryFn: () => commands.finance.listAccounts(),
    get enabled() {
      return !!orgId();
    },
    staleTime: 5 * 60 * 1000,
  });
}

/** Transaction/leaf accounts query. */
export function createTransactionAccountsQuery(orgId: string) {
  return createQuery({
    queryKey: ["transaction-accounts", orgId],
    queryFn: () => commands.finance.listTransactionAccounts(),
    staleTime: 5 * 60 * 1000,
  });
}

/** Detailed account statement query. */
export function createAccountStatementQuery(
  accountId: () => string,
  startDate: () => string,
  endDate: () => string,
) {
  return createQuery({
    get queryKey() {
      return ["account-statement", accountId(), startDate(), endDate()];
    },
    queryFn: () =>
      commands.finance.getAccountStatement(accountId(), startDate(), endDate()),
    get enabled() {
      return !!accountId() && !!startDate() && !!endDate();
    },
    staleTime: 1 * 60 * 1000,
  });
}

/** Fee templates query. */
export function createFeeTemplatesQuery(orgId: string) {
  return createQuery({
    queryKey: ["fee-templates", orgId],
    queryFn: () => commands.finance.listFeeTemplates(),
    staleTime: 5 * 60 * 1000,
  });
}

/** Student fees. Pass orgId to get all fees for org, or enrollmentId to get fees for specific enrollment. */
export function createStudentFeesQuery(
  orgOrEnrollmentId: () => string,
  isOrgId: boolean = true,
) {
  return createQuery({
    get queryKey() {
      return ["student-fees", orgOrEnrollmentId()];
    },
    get queryFn() {
      return () =>
        isOrgId
          ? commands.finance.listAllStudentFees(orgOrEnrollmentId())
          : commands.finance.listStudentFees(orgOrEnrollmentId());
    },
    get enabled() {
      return !!orgOrEnrollmentId();
    },
    staleTime: 2 * 60 * 1000,
  });
}

/** Discount templates query. */
export function createDiscountTemplatesQuery(orgId: string) {
  return createQuery({
    queryKey: ["discount-templates", orgId],
    queryFn: () => commands.finance.listDiscountTemplates(),
    staleTime: 5 * 60 * 1000,
  });
}

/** Currencies query. */
export function createCurrenciesQuery(orgId: string) {
  return createQuery({
    queryKey: ["currencies", orgId],
    queryFn: () => commands.finance.listCurrencies(),
    enabled: !!orgId,
    staleTime: 10 * 60 * 1000,
  });
}
