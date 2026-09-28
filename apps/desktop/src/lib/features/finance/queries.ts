import { createQuery, createMutation, useQueryClient } from "@tanstack/svelte-query";
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
  type GenerateFeeRequest,
  type ProcessPaymentRequest,
  type CreateAccountRequest,
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
          name: "نقداً",
          name_en: "Cash",
          category: "manual",
          is_digital_gateway: false,
          requires_source_token: false,
        },
        {
          id: "bank_transfer",
          name: "تحويل بنكي",
          name_en: "Bank Transfer",
          category: "bank",
          is_digital_gateway: false,
          requires_source_token: false,
        },
        {
          id: "check",
          name: "شيك مصرفي",
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
            name: "بطاقة مدى (Mada)",
            name_en: "Mada Debit Card",
            category: "electronic",
            provider_name: "mada",
            is_digital_gateway: true,
            requires_source_token: true,
          });
        } else if (provider === "moyasar") {
          methods.push({
            id: "mada",
            name: "مدى (عبر Moyasar)",
            name_en: "Mada (Moyasar)",
            category: "electronic",
            provider_name: "moyasar",
            is_digital_gateway: true,
            requires_source_token: true,
          });
          methods.push({
            id: "credit_card",
            name: "بطاقة ائتمانية (Visa / MasterCard)",
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
            name: "بطاقة ائتمان (Stripe)",
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
            name: `بوابة دفع إلكتروني (${config.provider_name})`,
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

