import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "moyasar",
  name: "Moyasar",
  name_ar: "ميسر",
  category: "payment-gateway",
  active: true,
  beta: false,
  icon: "CreditCard",
  short_description: "Accept Mada debit cards, Apple Pay, STC Pay, and credit cards directly with Moyasar.",
  short_description_ar: "قبول بطاقات مدى والبطاقات الائتمانية والمحافظ الرقمية مباشرة عبر بوابة ميسر.",
  description: `Integrate Moyasar to collect tuition, admission, and school service fees seamlessly.

**Saudi Local Payment Methods**
Direct integration with Mada cards, Apple Pay, and STC Pay for the highest conversion rate among Saudi parents.

**Automated Reconciliation & Invoicing**
Instant webhook notifications trigger automatic payment recording in the student accounting ledger.

**Fast & Secure Compliance**
PCI-DSS Level 1 certified gateway compliant with Saudi Central Bank (SAMA) standards.`,
  description_ar: `ربط بوابة ميسر لتحصيل الرسوم الدراسية ورسوم التسجيل والخدمات المدرسية بكل سهولة.

**طرق الدفع المحلية**
دعم مباشر لبطاقات مدى والمحافظ الرقمية لتحقيق أعلى نسبة نجاح للعمليات مع أولياء الأمور.

**تسوية وفوترة تلقائية**
إشعارات فورية تسجل سندات القبض وتقيد الدفعات في سجلات الطلاب المالية بشكل آلي.

**أمان معتمد ومطابق للمعايير**
معتمد ومطابق لتعليمات البنك المركزي السعودي بالكامل.`,
  features: [
    "Mada debit cards support",
    "Apple Pay & STC Pay",
    "Visa & Mastercard",
    "Automated ledger posting",
    "Instant webhook confirmations",
    "Refund & partial payment management",
  ],
  features_ar: [
    "دعم بطاقات مدى البنكية",
    "دعم المحافظ الرقمية السريعة",
    "بطاقات فيزا وماستركارد",
    "ترحيل تلقائي للقيود واليومية",
    "إشعارات فورية عند الدفع",
    "إدارة الاسترجاع والدفعات الجزئية",
  ],
  settings: [
    {
      id: "publishable_key",
      label: "Publishable Key",
      label_ar: "المفتاح العام",
      type: "text",
      required: true,
      value: "",
      description: "Public API key for frontend checkout forms.",
      description_ar: "مفتاح الواجهة البرمجية العام المستخدم في واجهة الدفع.",
      placeholder: "pk_live_...",
    },
    {
      id: "secret_key",
      label: "Secret Key",
      label_ar: "المفتاح السري",
      type: "password",
      required: true,
      value: "",
      description: "Private API key for server-to-server transaction verification.",
      description_ar: "المفتاح السري المستخدم للتحقق من العمليات وإجراء التسويات من الخادم.",
      placeholder: "sk_live_...",
    },
    {
      id: "supported_methods",
      label: "Payment Methods",
      label_ar: "طرق الدفع المفعلة",
      type: "select",
      required: true,
      value: "all",
      description: "Select which payment methods to accept.",
      description_ar: "حدد وسائل الدفع المقبولة من أولياء الأمور.",
      options: [
        { label: "All Methods", value: "all" },
        { label: "Mada and Apple Pay Only", value: "mada_apple_pay" },
        { label: "Mada Only", value: "mada_only" },
      ],
    },
    {
      id: "webhook_secret",
      label: "Webhook Secret",
      label_ar: "مفتاح أمان الإشعارات الفورية",
      type: "password",
      required: false,
      value: "",
      description: "Secret token to verify webhook signature authenticity.",
      description_ar: "رمز سري للتحقق من صحة الإشعارات الواردة من السيرفر.",
    },
    {
      id: "test_mode",
      label: "Test Mode",
      label_ar: "الوضع التجريبي",
      type: "switch",
      required: false,
      value: true,
      description: "Enable test mode to simulate transactions without real charges.",
      description_ar: "تفعيل الوضع التجريبي لمحاكاة عمليات الدفع دون خصم فعلي.",
    },
  ],
  adapter_name: "moyasar",
  provider_trait: "PaymentGatewayProvider",
};

export default config;
