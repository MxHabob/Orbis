import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "push-notifications",
  name: "Push Notifications",
  name_ar: "الإشعارات الفورية",
  category: "communication",
  active: false,
  icon: "Bell",
  short_description: "Send push notifications to mobile devices via Firebase Cloud Messaging (FCM).",
  short_description_ar: "إرسال إشعارات فورية للأجهزة المحمولة عبر Firebase.",
  description: `Send push notifications to parents' and staff's mobile devices for real-time alerts.

**Firebase Integration**
Uses Firebase Cloud Messaging (FCM) for reliable cross-platform delivery (Android & iOS).

**Topic-Based**
Subscribe users to topics (class, grade level, school-wide) for targeted notifications.`,
  features: ["Cross-platform push (Android & iOS)", "Topic-based subscriptions", "Silent data notifications", "Rich notifications with images"],
  settings: [
    { id: "fcm_server_key", label: "FCM Server Key", type: "password", required: true, value: "", description: "Firebase Cloud Messaging server key." },
    { id: "project_id", label: "Firebase Project ID", type: "text", required: true, value: "", description: "Your Firebase project identifier.", placeholder: "orbis-school-xxx" },
  ],
  provider_trait: "NotificationProvider",
};

export default config;
