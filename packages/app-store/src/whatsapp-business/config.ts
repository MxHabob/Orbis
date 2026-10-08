import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "whatsapp-business",
  name: "WhatsApp Business",
  name_ar: "واتساب للأعمال",
  category: "communication",
  active: true,
  beta: true,
  icon: "MessageCircle",
  short_description:
    "Send instant notifications and communicate with parents via WhatsApp Business API.",
  short_description_ar:
    "إرسال إشعارات فورية والتواصل مع أولياء الأمور عبر واتساب للأعمال.",
  description: `Connect WhatsApp Business API to reach parents where they are — on their phones.

**Instant Notifications**
Send attendance alerts, grade updates, and fee reminders directly to parents' WhatsApp.

**Two-Way Communication**
Parents can reply to messages, ask questions, and receive AI-powered responses.

**Template Messages**
Pre-approved message templates for common notifications ensure compliance with WhatsApp policies.

**Rich Media**
Send report cards as PDFs, photos of school events, and interactive buttons.`,
  description_ar: `ربط WhatsApp Business API للوصول لأولياء الأمور حيث هم — على هواتفهم.

**إشعارات فورية**
إرسال تنبيهات الحضور وتحديثات الدرجات مباشرة لواتساب أولياء الأمور.

**تواصل ثنائي الاتجاه**
يمكن لأولياء الأمور الرد على الرسائل وطرح الأسئلة.`,
  features: [
    "Instant parent notifications",
    "Two-way messaging with AI responses",
    "Pre-approved message templates",
    "Report cards and documents as PDFs",
    "Attendance and grade alerts",
    "Bulk messaging to class groups",
  ],
  settings: [
    {
      id: "phone_number_id",
      label: "Phone Number ID",
      label_ar: "معرف رقم الهاتف",
      description: "Your WhatsApp Business phone number ID from Meta.",
      type: "text",
      required: true,
      value: "",
      placeholder: "1234567890",
    },
    {
      id: "access_token",
      label: "Access Token",
      label_ar: "رمز الوصول",
      description: "Permanent access token from Meta Business Suite.",
      type: "password",
      required: true,
      value: "",
    },
    {
      id: "business_account_id",
      label: "Business Account ID",
      label_ar: "معرف حساب الأعمال",
      description: "Your WhatsApp Business Account ID.",
      type: "text",
      required: true,
      value: "",
    },
    {
      id: "auto_attendance",
      label: "Auto Attendance Alerts",
      label_ar: "تنبيهات الحضور التلقائية",
      description:
        "Automatically notify parents when their child is marked absent.",
      type: "switch",
      required: false,
      value: true,
    },
    {
      id: "auto_grades",
      label: "Auto Grade Notifications",
      label_ar: "إشعارات الدرجات التلقائية",
      description: "Automatically send grade reports when published.",
      type: "switch",
      required: false,
      value: false,
    },
  ],
  adapter_name: undefined, // Not yet implemented in Rust
  provider_trait: "NotificationProvider",
  notification_channels: ["whatsapp"],
};

export default config;
