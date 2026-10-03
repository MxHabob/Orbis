import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "smtp-email",
  name: "Email (SMTP)",
  name_ar: "البريد الإلكتروني (SMTP)",
  category: "communication",
  active: true,
  icon: "Mail",
  short_description:
    "Send automated email notifications to parents, teachers, and staff via SMTP.",
  short_description_ar:
    "إرسال إشعارات بريد إلكتروني تلقائية لأولياء الأمور والمعلمين والموظفين.",
  description: `Connect your SMTP email server to enable automated email notifications across Orbis.

**Automated Notifications**
Send grade reports, attendance alerts, fee reminders, and enrollment confirmations automatically.

**HTML Templates**
Rich HTML email templates with school branding for professional communication.

**Bulk Sending**
Send notifications to entire classes, grade levels, or the whole school at once.

**Delivery Tracking**
Track email delivery status and handle bounces gracefully.`,
  description_ar: `ربط خادم البريد الإلكتروني SMTP لتفعيل الإشعارات التلقائية في Orbis.

**إشعارات تلقائية**
إرسال تقارير الدرجات، تنبيهات الحضور، تذكيرات الرسوم تلقائياً.

**إرسال جماعي**
إرسال إشعارات لفصول كاملة أو المدرسة بأكملها.`,
  features: [
    "Automated email notifications",
    "HTML templates with school branding",
    "Bulk sending to classes or school-wide",
    "Grade reports and attendance alerts",
    "Fee reminders and payment receipts",
    "Delivery status tracking",
  ],
  settings: [
    {
      id: "host",
      label: "SMTP Host",
      label_ar: "خادم SMTP",
      description: "The SMTP server hostname.",
      type: "text",
      required: true,
      value: "",
      placeholder: "smtp.gmail.com",
    },
    {
      id: "port",
      label: "SMTP Port",
      label_ar: "منفذ SMTP",
      description: "SMTP port (587 for TLS, 465 for SSL, 25 for unencrypted).",
      type: "number",
      required: true,
      value: 587,
      validation: { min: 1, max: 65535 },
    },
    {
      id: "username",
      label: "Username",
      label_ar: "اسم المستخدم",
      description: "SMTP authentication username (usually your email).",
      type: "text",
      required: true,
      value: "",
      placeholder: "school@example.com",
    },
    {
      id: "password",
      label: "Password",
      label_ar: "كلمة المرور",
      description: "SMTP authentication password or app password.",
      type: "password",
      required: true,
      value: "",
    },
    {
      id: "from_name",
      label: "Sender Name",
      label_ar: "اسم المرسل",
      description: "Display name for outgoing emails.",
      type: "text",
      required: false,
      value: "",
      placeholder: "Orbis School",
    },
    {
      id: "use_tls",
      label: "Use TLS",
      label_ar: "استخدام TLS",
      description: "Enable TLS encryption for secure email delivery.",
      type: "switch",
      required: false,
      value: true,
    },
  ],
  adapter_name: "smtp",
  provider_trait: "NotificationProvider",
};

export default config;
