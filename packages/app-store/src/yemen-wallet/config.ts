import type { OrbisApp } from "../types";
import { Logo } from "./assets/logo";

const config: OrbisApp = {
  logo: Logo,
  id: "yemen-wallet",
  name: "Yemen Wallet for Business",
  name_ar: "محفظة اليمن للأعمال",
  category: "payment-gateway",
  active: false,
  beta: false,
  icon: "Banknotes",
  short_description:
    "A merchant wallet app for receiving customer payments by phone number or QR code.",
  short_description_ar:
    "تطبيق محفظة للتجار لاستلام مدفوعات العملاء عبر رقم الهاتف أو رمز QR.",
  description: `Yemen Wallet for Business is the official Yemen Post merchant and agent app. Its published listing says merchants can accept customer payments from Yemen e-wallets by phone number or QR code, and create fixed-amount QR codes.

**Integration requirements**
The published merchant app does not document a public API for creating invoice-linked payment requests or confirming transactions in Orbis. Merchant onboarding and any technical integration must be confirmed with Yemen Post.

[View the official merchant app](https://play.google.com/store/apps/details?id=com.s2m.yamenpost.merchant.android).`,
  description_ar: `محفظة اليمن للأعمال هي تطبيق الوكلاء والتجار الرسمي من البريد اليمني. يذكر وصف التطبيق إمكانية استقبال مدفوعات العملاء من المحافظ اليمنية عبر رقم الهاتف أو رمز QR، وإنشاء رموز QR بمبالغ ثابتة.

**متطلبات التكامل**
لا يوضح وصف التطبيق واجهة API عامة لإنشاء طلبات دفع مرتبطة بالفواتير أو تأكيد العمليات تلقائيًا في أوربيس. يجب تأكيد تسجيل التاجر وأي تكامل تقني مع البريد اليمني.

[عرض تطبيق التجار الرسمي](https://play.google.com/store/apps/details?id=com.s2m.yamenpost.merchant.android).`,
  features: [
    "Merchant payment acceptance by phone number or QR code",
    "Fixed-amount QR codes",
    "Supports payments from Yemen e-wallets",
    "Public Orbis API integration not documented",
  ],
  features_ar: [
    "استلام مدفوعات التجار عبر رقم الهاتف أو رمز QR",
    "إنشاء رموز QR بمبالغ ثابتة",
    "استقبال المدفوعات من المحافظ اليمنية",
    "لا يتوفر توثيق عام لواجهة API خاصة بتكامل أوربيس",
  ],
  settings: [],
};

export default config;
