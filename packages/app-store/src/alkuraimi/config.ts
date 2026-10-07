import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "alkuraimi",
  name: "Al-Kuraimi Bank",
  name_ar: "بنك الكريمي للتمويل الأصغر الإسلامي",
  category: "payment-gateway",
  active: false,
  beta: false,
  icon: "CreditCard",
  short_description:
    "Link business applications and websites with Al-Kuraimi through its API Link service.",
  short_description_ar:
    "ربط تطبيقات ومواقع الأعمال ببنك الكريمي عبر خدمة الربط بالـ API.",
  description: `Al-Kuraimi's API Link service allows businesses to link their applications or websites with the bank to facilitate customer payments.

**Integration onboarding**
Merchant eligibility, API specifications, credentials, payment confirmation, and reconciliation flows must be agreed with Al-Kuraimi before implementation.

[View the official API Link service](https://kuraimibank.com/en/services/50).`,
  description_ar: `تتيح خدمة الربط بالـ API من بنك الكريمي ربط تطبيقات ومواقع الأعمال بالبنك لتسهيل مدفوعات العملاء.

**التهيئة للتكامل**
يجب تأكيد متطلبات التاجر ومواصفات الواجهة وبيانات الاعتماد وآلية تأكيد المدفوعات والتسوية مع البنك قبل تنفيذ الربط.

[عرض خدمة الربط الرسمية](https://kuraimibank.com/en/services/50).`,
  features: [
    "API-based connection between business applications and Al-Kuraimi",
    "Facilitates customer payments through the bank",
    "Requires merchant onboarding and bank-provided API specifications",
  ],
  features_ar: [
    "ربط تطبيقات الأعمال بالبنك عبر واجهة API",
    "تسهيل مدفوعات العملاء من خلال البنك",
    "يتطلب تهيئة التاجر والحصول على مواصفات API من البنك",
  ],
  settings: [],
};

export default config;
