import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "mada",
  name: "Mada",
  name_ar: "مدى",
  category: "payment-gateway",
  active: true,
  beta: false,
  icon: "CreditCard",
  short_description: "Saudi national payment network for direct debit card processing with zero exchange fees.",
  short_description_ar: "شبكة المدفوعات الوطنية السعودية لمعالجة بطاقات الحسم المباشر دون رسوم تحويل إضافية.",
  description: `Direct integration with the Saudi National Payment Scheme (Mada).

**Domestic Processing**
Zero foreign exchange charges for Saudi Riyal transactions with lowest processing fee structure.

**Instant Verification**
Immediate settlement validation for school cashier POS and online parent portal payments.

**SAMA Compliant**
Strict adherence to Saudi Central Bank security architecture and banking interoperability.`,
  description_ar: `ربط مباشر مع شبكة المدفوعات الوطنية السعودية.

**معالجة محلية بالكامل**
رسوم تحصيل منخفضة بدون أي فروقات صرف لعمليات الريال السعودي مع سرعة استجابة فائقة.

**تحقق فوري من الدفعات**
تسجيل فوري لعمليات الدفع عبر البوابة الإلكترونية أو أجهزة نقاط البيع المدرسية.

**مطابقة متطلبات البنك المركزي**
التزام كامل بمعايير أمن المعلومات الصادرة عن البنك المركزي السعودي والتوافق البنكي.`,
  features: [
    "Saudi national debit cards",
    "Lowest transaction fees",
    "Instant fund verification",
    "Native currency processing",
    "Regulatory compliance",
  ],
  features_ar: [
    "بطاقات الحسم المباشر البنكية",
    "أقل عمولة على التحصيل الإلكتروني",
    "تحقق فوري من الرصيد والعملية",
    "معالجة أصلية بالعملة المحلية",
    "مطابقة اشتراطات الجهات التنظيمية",
  ],
  settings: [
    {
      id: "terminal_id",
      label: "Terminal ID",
      label_ar: "معرف نقطة الدفع",
      type: "text",
      required: true,
      value: "",
      description: "Terminal ID provided by your acquiring Saudi bank.",
      description_ar: "رقم المحطة المعرف من البنك المستحوذ.",
      placeholder: "TID...",
    },
    {
      id: "merchant_id",
      label: "Merchant ID",
      label_ar: "معرف التاجر",
      type: "text",
      required: true,
      value: "",
      description: "School merchant ID registered with the bank.",
      description_ar: "رقم التاجر الخاص بالمدرسة في النظام البنكي.",
      placeholder: "MID...",
    },
    {
      id: "shared_secret",
      label: "Shared Secret",
      label_ar: "مفتاح التشفير المشترك",
      type: "password",
      required: true,
      value: "",
      description: "Cryptographic key used to sign transaction payloads.",
      description_ar: "مفتاح التشفير الخاص بتوقيع حزم المعاملات البنكية.",
    },
    {
      id: "gateway_host",
      label: "Bank Gateway Host URL",
      label_ar: "رابط خادم البوابة البنكية",
      type: "url",
      required: true,
      value: "https://pgw.saudigateway.sa/mada/v1",
      description: "Acquiring bank payment gateway endpoint.",
      description_ar: "عنوان خادم البوابة البنكية المعتمد للربط.",
    },
    {
      id: "test_mode",
      label: "Sandbox Mode",
      label_ar: "الوضع التجريبي",
      type: "switch",
      required: false,
      value: false,
      description: "Enable testing against payment simulator endpoint.",
      description_ar: "تفعيل بيئة الاختبار لمحاكاة استجابة الشبكة.",
    },
  ],
  adapter_name: "mada",
  provider_trait: "PaymentGatewayProvider",
};

export default config;
