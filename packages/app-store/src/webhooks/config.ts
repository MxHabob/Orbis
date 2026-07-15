import type { OrbisApp } from "../types";

const config: OrbisApp = {
  id: "webhooks",
  name: "Webhooks",
  name_ar: "ويب هوكس",
  category: "automation",
  active: true,
  beta: true,
  icon: "Webhook",
  short_description: "Send real-time event notifications to external systems via HTTP webhooks.",
  short_description_ar: "إرسال إشعارات الأحداث في الوقت الحقيقي للأنظمة الخارجية عبر Webhooks.",
  description: `Push Orbis events to any external system in real-time via HTTP webhooks.

**Event-Driven**
Subscribe to specific events (enrollment, grade, attendance, fee) and receive JSON payloads.

**Custom Endpoints**
Configure multiple webhook endpoints for different systems or workflows.

**Retry Logic**
Failed deliveries are automatically retried with exponential backoff.`,
  features: ["Real-time event delivery", "Event type filtering", "Custom HTTP headers", "Automatic retries", "Delivery logs", "HMAC signature verification"],
  settings: [
    { id: "endpoint_url", label: "Webhook URL", type: "url", required: true, value: "", description: "The URL to send webhook events to.", placeholder: "https://api.example.com/webhooks/orbis" },
    { id: "secret", label: "Signing Secret", type: "password", required: false, value: "", description: "HMAC secret for payload signature verification." },
    { id: "events", label: "Events", type: "text", required: false, value: "*", description: "Comma-separated event types to subscribe to (* for all).", placeholder: "enrollment.created,grade.updated" },
  ],
};

export default config;
