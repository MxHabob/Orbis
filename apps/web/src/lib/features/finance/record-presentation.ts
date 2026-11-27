import type { OrbisColumnDef } from "@orbis/svelte-grid";
import type { PaymentDto } from "$lib/api/types";
import * as m from "$lib/paraglide/messages";
import { orgContext } from "$lib/stores/org-context.svelte";

export function financeRecordColumn(): OrbisColumnDef {
  return {
    id: "__finance_record",
    header: m.finance_ui_open_record(),
    type: "action",
    width: 88,
    readonly: true,
    sortable: false,
    meta: {
      items: [{ id: "view", icon: "Eye", label: m.finance_ui_open_record() }],
    },
  };
}

export function paymentMethodLabel(value: string) {
  switch (value.toLowerCase()) {
    case "cash":
      return m.finance_ui_payment_method_cash();
    case "bank":
    case "bank_transfer":
      return m.finance_ui_payment_method_bank_transfer();
    case "check":
      return m.finance_ui_payment_method_check();
    case "credit_card":
      return m.finance_ui_payment_method_credit_card_moyasar();
    case "mada":
      return m.finance_ui_payment_method_mada();
    case "apple_pay":
      return m.finance_ui_payment_moyasar_apple_pay();
    default:
      return value;
  }
}

export function receiptPrintContext(
  payment: PaymentDto,
  payer = payment.payer_name ?? "",
  description = payment.reference ?? "",
) {
  return {
    receipt: {
      number: payment.receipt_number || payment.id.slice(0, 8),
      date: payment.paid_at || payment.created_at,
      payerName: payer,
      studentName: payment.student_id ? payer : "",
      amount: Number(payment.amount),
      amountWords: "",
      description,
      paymentMethod: paymentMethodLabel(payment.method),
      referenceNumber: payment.reference ?? "",
      cashierName: "",
    },
    $org: {
      name: orgContext.name,
      name_ar: orgContext.nameAr,
      currency: payment.currency_code || orgContext.baseCurrency,
      logo_url: orgContext.logo,
      primary_color: orgContext.primaryColor,
    },
  };
}
