import type { OrbisApp } from "../types";

const config: OrbisApp = {
  id: "sms-provider",
  name: "SMS Gateway",
  name_ar: "بوابة الرسائل القصيرة",
  category: "communication",
  active: true,
  beta: true,
  icon: "Smartphone",
  short_description: "Send SMS notifications to parents and staff via Twilio, Vonage, or other SMS providers.",
  short_description_ar: "إرسال رسائل نصية قصيرة لأولياء الأمور والموظفين.",
  description: `Connect an SMS gateway to send critical notifications that demand immediate attention.

**Urgent Alerts**
Send time-sensitive notifications like emergency closures, absence alerts, and fee deadlines.

**Multiple Providers**
Support for Twilio, Vonage, MessageBird, and other SMS providers.

**Bulk SMS**
Send notifications to entire classes or the whole school at once.`,
  features: ["Urgent SMS alerts", "Multiple provider support", "Bulk SMS sending", "Delivery tracking", "Template messages"],
  settings: [
    { id: "provider", label: "SMS Provider", type: "select", required: true, value: "twilio", description: "Select your SMS gateway provider.", options: [{ label: "Twilio", value: "twilio" }, { label: "Vonage", value: "vonage" }, { label: "MessageBird", value: "messagebird" }] },
    { id: "account_sid", label: "Account SID", type: "text", required: true, value: "", description: "Your SMS provider account identifier.", placeholder: "AC..." },
    { id: "auth_token", label: "Auth Token", type: "password", required: true, value: "", description: "Authentication token for the SMS API." },
    { id: "from_number", label: "Sender Number", type: "text", required: true, value: "", description: "The phone number to send SMS from.", placeholder: "+1234567890" },
  ],
  provider_trait: "NotificationProvider",
};

export default config;
