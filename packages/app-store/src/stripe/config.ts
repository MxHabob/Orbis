import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "stripe",
  name: "Stripe",
  name_ar: "سترايب",
  category: "payment-gateway",
  active: true,
  beta: false,
  icon: "Globe",
  short_description: "Global financial infrastructure supporting international tuition payments, multi-currency cards, and Apple Pay.",
  short_description_ar: "بنية المدفوعات العالمية لقبول البطاقات الدولية والعملات المتعددة والمحافظ الرقمية.",
  description: `Accept international payments from expatriate parents and international school students worldwide.

**135+ Currencies Supported**
Automatic real-time currency conversion for international school campuses and overseas students.

**Advanced Payment Elements**
Embed Stripe Elements for seamless, localized checkout with Apple Pay, Google Pay, and link payments.

**Automated Recurring Billing**
Automate recurring monthly and semester installment charging with smart card retry logic.`,
  description_ar: `قبول المدفوعات الدولية من أولياء الأمور والطلاب الدوليين في المدارس العالمية بكل سهولة.

**دعم العملات المتعددة**
تحويل فوري لأسعار الصرف للمدارس العالمية والطلاب من خارج الدولة.

**عناصر دفع مدمجة وحديثة**
تضمين عناصر دفع متطورة لدعم المحافظ الرقمية والبطاقات بسلاسة وأمان.

**استقطاع دوري للرسوم**
جدولة استقطاع الأقساط الدورية تلقائياً مع محاولات ذكية في حال تعثر البطاقة.`,
  features: [
    "Global card processing (Visa, MC, Amex)",
    "Multi-currency support (135+ currencies)",
    "Apple Pay & Google Pay",
    "Automated recurring payment schedules",
    "Stripe Radar fraud protection",
  ],
  features_ar: [
    "معالجة البطاقات الدولية والمحلية",
    "دعم أكثر من 135 عملة دولية",
    "دعم المحافظ الرقمية السريعة",
    "جدولة الاستقطاعات الدورية تلقائياً",
    "حماية متقدمة من الاحتيال المالي",
  ],
  settings: [
    {
      id: "publishable_key",
      label: "Publishable Key",
      label_ar: "المفتاح العام",
      type: "text",
      required: true,
      value: "",
      description: "Stripe public key for client-side checkout.",
      description_ar: "مفتاح سترايب العام لواجهات الدفع في التطبيق.",
      placeholder: "pk_live_...",
    },
    {
      id: "secret_key",
      label: "Secret Key",
      label_ar: "المفتاح السري",
      type: "password",
      required: true,
      value: "",
      description: "Stripe private API key for creating payment intents.",
      description_ar: "المفتاح السري الخاص بإنشاء الدفعات والتأكيد في السيرفر.",
      placeholder: "sk_live_...",
    },
    {
      id: "webhook_secret",
      label: "Webhook Signing Secret",
      label_ar: "مفتاح توقيع الإشعارات التلقائية",
      type: "password",
      required: false,
      value: "",
      description: "Endpoint secret to verify webhook events from Stripe.",
      description_ar: "رمز التحقق من توقيع الإشعارات التلقائية الصادرة من الخادم.",
      placeholder: "whsec_...",
    },
    {
      id: "test_mode",
      label: "Test Mode",
      label_ar: "الوضع التجريبي",
      type: "switch",
      required: false,
      value: true,
      description: "Use Stripe test keys and mock card details.",
      description_ar: "استخدام بيئة الاختبار ومحاكاة البطاقات.",
    },
  ],
  adapter_name: "stripe",
  provider_trait: "PaymentGatewayProvider",
};

export default config;
