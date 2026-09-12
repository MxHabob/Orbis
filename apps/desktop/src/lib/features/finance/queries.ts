/**
 * Finance domain queries — fees, accounts, payments, journal entries, discount templates.
 * Wraps OrbisApiClient with TanStack Query for automatic caching.
 */
import { createQuery, createMutation, useQueryClient } from "@tanstack/svelte-query";
import {
  api,
  queryKeys,
  type StudentFeeFilterParams,
  type PaymentFilterParams,
  type CreateDiscountTemplateRequest,
  type CreateFeeTemplateRequest,
  type GenerateFeeRequest,
  type ProcessPaymentRequest,
  type CreateAccountRequest,
} from "$lib/api";

/** Treasuries for an organization. */
export function createCashAccountsQuery(
  orgId: () => string,
  enabled: () => boolean = () => true,
) {
  return createQuery(() => ({
    queryKey: queryKeys.finance.accounts(),
    queryFn: async () => {
      const accounts = await api.listAccounts();
      return accounts.filter((account: any) =>
        ["cash", "bank"].includes(account.account_type || account.sub_type),
      );
    },
    enabled: !!orgId() && enabled(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Payments list. */
export function createPaymentsQuery(
  orgId: () => string,
  filter?: () => PaymentFilterParams | undefined,
) {
  return createQuery(() => ({
    queryKey: queryKeys.finance.payments(filter?.()),
    queryFn: () => api.listPayments(filter?.()),
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
export function createTransactionAccountsQuery(orgId: string | (() => string)) {
  const getOrgId = typeof orgId === "function" ? orgId : () => orgId;
  return createQuery(() => ({
    queryKey: queryKeys.finance.accounts(),
    queryFn: () => api.listAccounts(),
    enabled: !!getOrgId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Fee templates query. */
export function createFeeTemplatesQuery(orgId: string | (() => string)) {
  const getOrgId = typeof orgId === "function" ? orgId : () => orgId;
  return createQuery(() => ({
    queryKey: queryKeys.finance.feeTemplates(),
    queryFn: () => api.listFeeTemplates(),
    enabled: !!getOrgId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Student fees. Pass orgId to get all fees for org, or enrollmentId / filter params. */
export function createStudentFeesQuery(
  orgOrEnrollmentId: () => string,
  filter?: () => StudentFeeFilterParams | undefined,
  enabled: () => boolean = () => true,
) {
  return createQuery(() => ({
    queryKey: queryKeys.finance.studentFees(filter?.()),
    queryFn: () => api.listStudentFees(filter?.()),
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

/** Journal entries query. */
export function createJournalEntriesQuery(orgId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.finance.journalEntries(),
    queryFn: () => api.listJournalEntries(),
    enabled: !!orgId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Single journal entry query. */
export function createJournalEntryQuery(id: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.finance.journalEntry(id()),
    queryFn: () => api.getJournalEntry(id()),
    enabled: !!id(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Discount templates query. */
export function createDiscountTemplatesQuery(orgId: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.finance.discountTemplates(),
    queryFn: () => api.listDiscountTemplates(),
    enabled: !!orgId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Create discount template mutation */
export function createDiscountTemplateMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: CreateDiscountTemplateRequest) =>
      api.createDiscountTemplate(req),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.discountTemplates(),
      });
    },
  }));
}

/** Create fee template mutation */
export function createFeeTemplateMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: CreateFeeTemplateRequest) => api.createFeeTemplate(req),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.feeTemplates(),
      });
    },
  }));
}

/** Generate student fee mutation */
export function createGenerateFeeMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: GenerateFeeRequest) => api.generateStudentFee(req),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.studentFees(),
      });
    },
  }));
}

/** Process payment mutation */
export function createProcessPaymentMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: ProcessPaymentRequest) => api.processPayment(req),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.payments(),
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.studentFees(),
      });
    },
  }));
}

/** Create account mutation */
export function createAccountMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: CreateAccountRequest) => api.createAccount(req),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.accounts(),
      });
    },
  }));
}
