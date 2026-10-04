import { createQuery, createMutation, useQueryClient } from "@tanstack/svelte-query";
import * as m from "$lib/paraglide/messages";
import {
  api,
  queryKeys,
  type AccountDto,
  type StudentFeeDto,
  type PaymentDto,
  type CurrencyDto,
  type JournalEntryDto,
  type DiscountTemplateDto,
  type FeeTemplateDto,
  type StudentFeeFilterParams,
  type PaymentFilterParams,
  type CreateDiscountTemplateRequest,
  type CreateFeeTemplateRequest,
  type UpdateFeeTemplateRequest,
  type GenerateFeeRequest,
  type ProcessPaymentRequest,
  type CreateAccountRequest,
  type UpdateAccountRequest,
  type CreateJournalEntryRequest,
  type ReverseJournalRequest,
  type SplitFeeRequest,
  type RefundStudentFeeRequest,
  type StatementResponseDto,
  type StatementFilterParams,
  type ProcessDisbursementRequest,
  type CreateSubAccountRequest,
  type SettleGatewayPayoutRequest,
  type PostPayrollRequest,
  type DisbursePayrollRequest,
} from "$lib/api";

/** Treasuries for an organization. */
export function createCashAccountsQuery(
  orgId: () => string,
  enabled: () => boolean = () => true,
) {
  return createQuery<AccountDto[]>(() => ({
    queryKey: queryKeys.finance.accounts(orgId()),
    queryFn: async () => {
      const accounts = await api.listAccounts();
      return accounts.filter((account: AccountDto) =>
        ["cash", "bank"].includes(account.account_type || (account as { sub_type?: string }).sub_type || ""),
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
  return createQuery<PaymentDto[]>(() => ({
    queryKey: queryKeys.finance.payments(orgId(), filter?.()),
    queryFn: () => api.listPayments(filter?.()),
    enabled: !!orgId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Ledger accounts query. */
export function createAccountsQuery(orgId: () => string) {
  return createQuery<AccountDto[]>(() => ({
    queryKey: queryKeys.finance.accounts(orgId()),
    queryFn: () => api.listAccounts(),
    enabled: !!orgId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Transaction/leaf accounts query. */
export function createTransactionAccountsQuery(orgId: string | (() => string)) {
  const getOrgId = typeof orgId === "function" ? orgId : () => orgId;
  return createQuery<AccountDto[]>(() => ({
    queryKey: queryKeys.finance.accounts(getOrgId()),
    queryFn: () => api.listAccounts(),
    enabled: !!getOrgId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Fee templates query. */
export function createFeeTemplatesQuery(orgId: string | (() => string)) {
  const getOrgId = typeof orgId === "function" ? orgId : () => orgId;
  return createQuery<FeeTemplateDto[]>(() => ({
    queryKey: queryKeys.finance.feeTemplates(getOrgId()),
    queryFn: () => api.listFeeTemplates(),
    enabled: !!getOrgId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Student fees. Pass orgId to get all fees for org, or enrollmentId / filter params. */
export function createStudentFeesQuery(
  orgOrEnrollmentId: () => string,
  filter?: (() => StudentFeeFilterParams | undefined) | StudentFeeFilterParams,
  enabled: (() => boolean) | boolean = () => true,
) {
  const getFilter = typeof filter === "function" ? filter : () => (typeof filter === "object" ? filter : undefined);
  const getEnabled = typeof enabled === "function" ? enabled : () => (typeof enabled === "boolean" ? enabled : true);
  return createQuery<StudentFeeDto[]>(() => ({
    queryKey: queryKeys.finance.studentFees(orgOrEnrollmentId(), getFilter()),
    queryFn: () => api.listStudentFees(getFilter()),
    enabled: !!orgOrEnrollmentId() && getEnabled(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Currencies query. */
export function createCurrenciesQuery(orgId: () => string) {
  return createQuery<CurrencyDto[]>(() => ({
    queryKey: queryKeys.finance.currencies(),
    queryFn: () => api.listCurrencies(),
    enabled: !!orgId(),
    staleTime: 10 * 60 * 1000,
  }));
}

/** Journal entries query. */
export function createJournalEntriesQuery(orgId: () => string) {
  return createQuery<JournalEntryDto[]>(() => ({
    queryKey: queryKeys.finance.journalEntries(orgId()),
    queryFn: () => api.listJournalEntries(),
    enabled: !!orgId(),
    staleTime: 2 * 60 * 1000,
  }));
}

/** Single journal entry query. */
export function createJournalEntryQuery(id: () => string) {
  return createQuery<JournalEntryDto>(() => ({
    queryKey: queryKeys.finance.journalEntry(id()),
    queryFn: () => api.getJournalEntry(id()),
    enabled: !!id(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Discount templates query. */
export function createDiscountTemplatesQuery(orgId: () => string) {
  return createQuery<DiscountTemplateDto[]>(() => ({
    queryKey: queryKeys.finance.discountTemplates(orgId()),
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

export interface AvailablePaymentMethod {
  id: string; // e.g. "cash", "bank_transfer", "check", "mada", "credit_card", "apple_pay"
  name: string;
  name_en: string;
  category: "manual" | "bank" | "electronic";
  provider_name?: string;
  is_digital_gateway: boolean;
  requires_source_token: boolean;
}

/**
 * Dynamically resolves active payment methods for the organization based on:
 * 1. Base institutional manual methods (Cash, Bank Transfer, Check).
 * 2. Active external payment integrations enabled in the Orbis Integration Layer.
 */
export function createAvailablePaymentMethodsQuery(orgId: () => string) {
  return createQuery<AvailablePaymentMethod[]>(() => ({
    queryKey: [...queryKeys.integration.list(orgId()), "payment-methods"],
    queryFn: async () => {
      const integrations = await api.listIntegrations().catch(() => []);
      const activePaymentIntegrations = integrations.filter(
        (i) =>
          (i.category === "payment" || i.category === "payment-gateway") &&
          i.is_enabled,
      );

      // Base standard institutional methods
      const methods: AvailablePaymentMethod[] = [
        {
          id: "cash",
          name: m.finance_ui_payment_method_cash(),
          name_en: "Cash",
          category: "manual",
          is_digital_gateway: false,
          requires_source_token: false,
        },
        {
          id: "bank_transfer",
          name: m.finance_ui_payment_method_bank_transfer(),
          name_en: "Bank Transfer",
          category: "bank",
          is_digital_gateway: false,
          requires_source_token: false,
        },
        {
          id: "check",
          name: m.finance_ui_payment_method_bank_check(),
          name_en: "Bank Check",
          category: "bank",
          is_digital_gateway: false,
          requires_source_token: false,
        },
      ];

      // Dynamically resolve methods from active integrations
      for (const config of activePaymentIntegrations) {
        const provider = (config.provider_name || "").toLowerCase().trim();
        const settings = config.settings as Record<string, any> | undefined;

        if (provider === "mada") {
          methods.push({
            id: "mada",
            name: m.finance_ui_payment_method_mada(),
            name_en: "Mada Debit Card",
            category: "electronic",
            provider_name: "mada",
            is_digital_gateway: true,
            requires_source_token: true,
          });
        } else if (provider === "moyasar") {
          methods.push({
            id: "mada",
            name: m.finance_ui_payment_method_mada_moyasar(),
            name_en: "Mada (Moyasar)",
            category: "electronic",
            provider_name: "moyasar",
            is_digital_gateway: true,
            requires_source_token: true,
          });
          methods.push({
            id: "credit_card",
            name: m.finance_ui_payment_method_credit_card_moyasar(),
            name_en: "Credit Card (Moyasar)",
            category: "electronic",
            provider_name: "moyasar",
            is_digital_gateway: true,
            requires_source_token: true,
          });
          methods.push({
            id: "apple_pay",
            name: "Apple Pay",
            name_en: "Apple Pay (Moyasar)",
            category: "electronic",
            provider_name: "moyasar",
            is_digital_gateway: true,
            requires_source_token: true,
          });
        } else if (provider === "stripe") {
          methods.push({
            id: "credit_card",
            name: m.finance_ui_payment_method_credit_card_stripe(),
            name_en: "Credit Card (Stripe)",
            category: "electronic",
            provider_name: "stripe",
            is_digital_gateway: true,
            requires_source_token: true,
          });
          methods.push({
            id: "apple_pay",
            name: "Apple Pay (Stripe)",
            name_en: "Apple Pay",
            category: "electronic",
            provider_name: "stripe",
            is_digital_gateway: true,
            requires_source_token: true,
          });
        } else if (Array.isArray(settings?.methods)) {
          for (const m of settings.methods) {
            methods.push({
              id: String(m).toLowerCase(),
              name: `${m} (${config.provider_name})`,
              name_en: `${m} (${config.provider_name})`,
              category: "electronic",
              provider_name: config.provider_name,
              is_digital_gateway: true,
              requires_source_token: true,
            });
          }
        } else {
          // Generic gateway adapter
          methods.push({
            id: "credit_card",
            name: m.finance_ui_payment_method_gateway({ provider: config.provider_name ?? "" }),
            name_en: `Payment Gateway (${config.provider_name})`,
            category: "electronic",
            provider_name: config.provider_name,
            is_digital_gateway: true,
            requires_source_token: true,
          });
        }
      }

      // Deduplicate by method id
      const uniqueMethods: AvailablePaymentMethod[] = [];
      const seenIds = new Set<string>();
      for (const m of methods) {
        if (!seenIds.has(m.id)) {
          seenIds.add(m.id);
          uniqueMethods.push(m);
        }
      }

      return uniqueMethods;
    },
    enabled: !!orgId(),
    staleTime: 5 * 60 * 1000,
  }));
}

/** Update account mutation */
export function createUpdateAccountMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: ({ id, req }: { id: string; req: UpdateAccountRequest }) =>
      api.updateAccount(id, req),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.accounts(),
      });
    },
  }));
}

/** Delete account mutation */
export function createDeleteAccountMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (id: string) => api.deleteAccount(id),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.accounts(),
      });
    },
  }));
}

/** Update fee template mutation */
export function createUpdateFeeTemplateMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: ({ id, req }: { id: string; req: UpdateFeeTemplateRequest }) =>
      api.updateFeeTemplate(id, req),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.feeTemplates(),
      });
    },
  }));
}

/** Delete fee template mutation */
export function createDeleteFeeTemplateMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (id: string) => api.deleteFeeTemplate(id),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.feeTemplates(),
      });
    },
  }));
}

/** Delete discount template mutation */
export function createDeleteDiscountTemplateMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (id: string) => api.deleteDiscountTemplate(id),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.discountTemplates(),
      });
    },
  }));
}

/** Delete student fee mutation */
export function createDeleteStudentFeeMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (id: string) => api.deleteStudentFee(id),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.studentFees(),
      });
    },
  }));
}

/** Delete payment mutation */
export function createDeletePaymentMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (id: string) => api.deletePayment(id),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.payments(),
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.studentFees(),
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.accounts(),
      });
    },
  }));
}

/** Create journal entry mutation */
export function createCreateJournalEntryMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: CreateJournalEntryRequest) => api.createJournalEntry(req),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.journalEntries(),
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.accounts(),
      });
    },
  }));
}

/** Reverse journal entry mutation */
export function createReverseJournalEntryMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: ({ id, req }: { id: string; req: ReverseJournalRequest }) =>
      api.reverseJournalEntry(id, req),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.journalEntries(),
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.accounts(),
      });
    },
  }));
}

/** Delete journal entry mutation */
export function createDeleteJournalEntryMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (id: string) => api.deleteJournalEntry(id),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.journalEntries(),
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.accounts(),
      });
    },
  }));
}

/** Split student fee mutation */
export function createSplitStudentFeeMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: ({ id, req }: { id: string; req: SplitFeeRequest }) =>
      api.splitStudentFee(id, req),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.studentFees(),
      });
    },
  }));
}

/** Refund student fee mutation */
export function createRefundStudentFeeMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: ({ id, req }: { id: string; req: RefundStudentFeeRequest }) =>
      api.refundStudentFee(id, req),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.studentFees(),
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.payments(),
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.accounts(),
      });
    },
  }));
}

/** Account statement query */
export function createAccountStatementQuery(
  filter: () => StatementFilterParams,
  enabled: () => boolean = () => true,
) {
  return createQuery<StatementResponseDto>(() => ({
    queryKey: queryKeys.finance.statement(filter()),
    queryFn: () => api.getAccountStatement(filter()),
    enabled: enabled(),
    staleTime: 60 * 1000,
  }));
}

/** Process disbursement (سند صرف) mutation */
export function createProcessDisbursementMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: ProcessDisbursementRequest) => api.processDisbursement(req),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.accounts(),
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.journalEntries(),
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.statement(),
      });
    },
  }));
}

/** Create sub-account mutation */
export function createCreateSubAccountMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: CreateSubAccountRequest) => api.createSubAccount(req),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.accounts(),
      });
    },
  }));
}

/** Settle gateway payout mutation */
export function createSettleGatewayPayoutMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: SettleGatewayPayoutRequest) => api.settleGatewayPayout(req),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.accounts(),
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.journalEntries(),
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.statement(),
      });
    },
  }));
}

/** Post payroll accrual mutation */
export function createPostPayrollAccrualMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: PostPayrollRequest) => api.postPayrollAccrual(req),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.accounts(),
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.journalEntries(),
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.statement(),
      });
    },
  }));
}

/** Disburse payroll mutation */
export function createDisbursePayrollMutation() {
  const queryClient = useQueryClient();
  return createMutation(() => ({
    mutationFn: (req: DisbursePayrollRequest) => api.disbursePayroll(req),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.accounts(),
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.journalEntries(),
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.finance.statement(),
      });
    },
  }));
}

