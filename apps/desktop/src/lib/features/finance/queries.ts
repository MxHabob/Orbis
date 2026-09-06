/**
 * Finance domain queries — fees, accounts, payments.
 * Wraps OrbisApiClient with TanStack Query for automatic caching.
 */
import { createQuery } from "@tanstack/svelte-query";
import { api, queryKeys } from "$lib/api";

/** Treasuries for an organization. */
export function createCashAccountsQuery(
  orgId: () => string,
  enabled: () => boolean = () => true,
) {
  return createQuery(() => ({
    queryKey: queryKeys.finance.accounts(),
    queryFn: async () => {
      const accounts = await api.listAccounts();
      return accounts.filter((account: any) => ["cash", "bank"].includes(account.account_type || account.sub_type));
    },
    enabled: !!orgId() && enabled(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Payments list. */
export function createPaymentsQuery(orgId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.finance.payments(),
    queryFn: () => api.listPayments(),
    enabled: !!orgId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Ledger accounts query. */
export function createAccountsQuery(orgId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.finance.accounts(),
    queryFn: () => api.listAccounts(),
    enabled: !!orgId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Transaction/leaf accounts query. */
export function createTransactionAccountsQuery(orgId: string) {
  return createQuery(() => ({
    queryKey: queryKeys.finance.accounts(),
    queryFn: () => api.listAccounts(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Fee templates query. */
export function createFeeTemplatesQuery(orgId: string) {
  return createQuery(() => ({
    queryKey: queryKeys.finance.feeTemplates(),
    queryFn: () => api.listFeeTemplates(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Student fees. Pass orgId to get all fees for org, or enrollmentId to get fees for specific enrollment. */
export function createStudentFeesQuery(
  orgOrEnrollmentId: () => string,
  isOrgId: boolean = true,
  enabled: () => boolean = () => true,
) {
  return createQuery(() => ({
    queryKey: queryKeys.finance.studentFees(isOrgId ? undefined : { enrollmentId: orgOrEnrollmentId() }),
    queryFn: () =>
      isOrgId
        ? api.listStudentFees()
        : api.listStudentFees({ enrollmentId: orgOrEnrollmentId() }),
    enabled: !!orgOrEnrollmentId() && enabled(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Currencies query. */
export function createCurrenciesQuery(orgId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.finance.currencies(),
    queryFn: () => api.listCurrencies(),
    enabled: !!orgId(),
    staleTime: 10 * 60 * 1000,
  }));
}

