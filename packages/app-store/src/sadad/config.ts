import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "sadad",
  name: "SADAD",
  name_ar: "سداد",
  category: "payment-gateway",
  active: true,
  beta: false,
  icon: "Receipt",
  short_description: "Saudi national Electronic Bill Presentment and Payment system for automated invoice settlement through online banking.",
  short_description_ar: "نظام سداد الوطني لعرض وسداد الفواتير الإلكترونية عبر التطبيقات البنكية وأجهزة الصراف الآلي.",
  description: `SADAD payment system integration for automatic bill issuing and settlement.

**Biller Integration**
Automatically issue SADAD bill numbers for every student tuition invoice generated in Orbis.

**Pay via Any Saudi Bank**
Parents can settle invoices directly through online banking, mobile apps, or ATMs using their SADAD bill number.

**Instant Reconciliation**
Automatic batch and real-time reconciliation updates the school's general ledger upon bank settlement.`,
  description_ar: `ربط نظام سداد الوطني لإصدار الفواتير وتحصيلها آلياً عبر القنوات البنكية.

**إصدار فواتير سداد آلياً**
توليد رقم سداد مميز لكل فاتورة رسوم دراسية تصدر للطالب في نظام أوربيس.

**السداد عبر جميع البنوك**
سداد مباشر من التطبيقات البنكية أو أجهزة الصراف الآلي برقم الفاتورة.

**مطابقة بنكية وتسوية فورية**
مزامنة تلقائية للدفعات المحصلة مع قيود اليومية وسجلات الطلاب المحاسبية.`,
  features: [
    "Automatic SADAD bill number generation",
    "Payable across all banks and ATMs",
    "Real-time payment notifications",
    "Automated reconciliation with general ledger",
    "Regulatory compliant billing system",
  ],
  features_ar: [
    "توليد آلي لرقم فاتورة سداد",
    "إمكانية السداد عبر كافة البنوك وأجهزة الصراف",
    "إشعارات سداد فورية ومباشرة",
    "مطابقة وتسوية تلقائية مع سجلات الحسابات العامة",
    "نظام فوترة معتمد ومطابق للاشتراطات",
  ],
  settings: [
    {
      id: "biller_id",
      label: "Biller Code",
      label_ar: "رمز المفوتر",
      type: "text",
      required: true,
      value: "",
      description: "Official SADAD Biller Code assigned to the school.",
      description_ar: "رمز المفوتر المعتمد الخاص بالمدرسة.",
      placeholder: "e.g. 123",
    },
    {
      id: "access_key",
      label: "API Access Key",
      label_ar: "مفتاح الربط البرمجي",
      type: "password",
      required: true,
      value: "",
      description: "Secure API credential for SADAD Gateway.",
      description_ar: "مفتاح الربط البرمجي السري لبوابة سداد.",
    },
    {
      id: "endpoint_url",
      label: "Gateway Endpoint URL",
      label_ar: "عنوان خادم البوابة",
      type: "url",
      required: true,
      value: "https://b2b.sadad.com/v1/bills",
      description: "SADAD enterprise service bus endpoint URL.",
      description_ar: "رابط خادم الربط الإلكتروني لبوابة سداد.",
    },
    {
      id: "test_mode",
      label: "Testing Mode",
      label_ar: "الوضع التجريبي",
      type: "switch",
      required: false,
      value: false,
      description: "Use User Acceptance Testing environment.",
      description_ar: "استخدام بيئة الاختبار للتحقق من تكامل الفواتير.",
    },
  ],
  adapter_name: "sadad",
  provider_trait: "PaymentGatewayProvider",
};

export default config;
