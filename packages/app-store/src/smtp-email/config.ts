import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "smtp-email",
  name: "Email (SendGrid)",
  name_ar: "البريد الإلكتروني (SendGrid)",
  category: "communication",
  active: true,
  icon: "Mail",
  short_description:
    "Send automated email notifications through SendGrid.",
  short_description_ar:
    "إرسال إشعارات البريد الإلكتروني تلقائياً عبر SendGrid.",
  description: `Connect a SendGrid API key to send email notifications through Orbis.

**Automated Notifications**
Send grade reports, attendance alerts, fee reminders, and enrollment confirmations through the configured SendGrid account.`,
  description_ar: `اربط مفتاح API من SendGrid لإرسال إشعارات البريد الإلكتروني عبر Orbis، مثل تقارير الدرجات وتنبيهات الحضور وتذكيرات الرسوم وتأكيدات التسجيل.`,
  features: [
    "Automated email notifications",
    "SendGrid API integration",
    "Grade reports and attendance alerts",
    "Fee reminders and enrollment confirmations",
  ],
  settings: [
    {
      id: "api_key",
      label: "SendGrid API Key",
      label_ar: "مفتاح API من SendGrid",
      description: "API key used by Orbis to authenticate with SendGrid.",
      description_ar:
        "مفتاح API بصلاحية Mail Send وUser Profile Read لإرسال البريد والتحقق من الاتصال.",
      type: "password",
      required: true,
      value: "",
    },
    {
      id: "from_email",
      label: "Verified Sender Email",
      label_ar: "بريد المرسل المعتمد",
      description:
        "A sender email address verified in your SendGrid account.",
      description_ar: "عنوان بريد مرسل تم اعتماده في حساب SendGrid.",
      type: "email",
      required: true,
      value: "",
      placeholder: "notifications@your-school.edu",
    },
    {
      id: "from_name",
      label: "Sender Name",
      label_ar: "اسم المرسل",
      description: "Optional display name shown to email recipients.",
      description_ar: "اسم اختياري يظهر للمستلمين كاسم المرسل.",
      type: "text",
      required: false,
      value: "",
      placeholder: "Orbis School",
    },
  ],
  adapter_name: "sendgrid",
  legacy_provider_name: "smtp-email",
  provider_trait: "NotificationProvider",
  notification_channels: ["email"],
};

export default config;
