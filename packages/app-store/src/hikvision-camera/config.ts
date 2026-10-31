import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "hikvision-camera",
  name: "Hikvision Camera",
  name_ar: "كاميرا هيكفيجن",
  category: "surveillance",
  active: true,
  icon: "Camera",
  short_description:
    "Connect Hikvision IP cameras for live surveillance and real-time event monitoring.",
  short_description_ar:
    "ربط كاميرات هيكفيجن IP للمراقبة الحية ومتابعة الأحداث في الوقت الحقيقي.",
  description: `Connect your Hikvision IP cameras to Orbis for real-time video surveillance of your school campus.

**Live Monitoring**
View live camera feeds directly from the AI Center Live page with flexible grid layouts (single, 2×2, 3×3).

**Motion Detection Events**
Receive real-time motion detection and alarm events routed through the EventBridge integration pipeline.

**Attendance Integration**
Optionally integrate camera face-detection events with attendance tracking for automated student check-in/check-out.

**Multi-Camera Support**
Connect multiple cameras and NVRs. Each camera appears as a separate feed in the Live page.`,
  description_ar: `ربط كاميرات هيكفيجن IP بنظام Orbis للمراقبة المباشرة لحرم المدرسة.

**المراقبة الحية**
عرض البث المباشر من الكاميرات في صفحة Live مع تخطيطات مرنة.

**أحداث كشف الحركة**
استقبال تنبيهات كشف الحركة في الوقت الحقيقي.

**تكامل الحضور**
ربط اختياري مع نظام الحضور للتسجيل التلقائي.`,
  features: [
    "Live video feed streaming",
    "Real-time motion detection alerts",
    "Multi-camera grid view (1×1, 2×2, 3×3)",
    "NVR support for centralized recording",
    "Optional attendance integration",
    "Event history and replay",
  ],
  settings: [
    {
      id: "ip_address",
      label: "Camera IP Address or Hostname",
      label_ar: "عنوان IP أو اسم نطاق الكاميرا",
      description: "The IP address or DDNS hostname of the Hikvision camera or NVR.",
      description_ar: "عنوان IP أو اسم نطاق DDNS لكاميرا هيكفيجن أو جهاز NVR.",
      type: "text",
      required: true,
      value: "",
      placeholder: "192.168.1.100 or camera.example.com",
    },
    {
      id: "port",
      label: "Port",
      label_ar: "المنفذ",
      description: "HTTP port for the camera API (default: 80).",
      type: "number",
      required: true,
      value: 80,
      validation: { min: 1, max: 65535 },
    },
    {
      id: "username",
      label: "Username",
      label_ar: "اسم المستخدم",
      description: "Camera admin username.",
      description_ar: "اسم مستخدم مدير الكاميرا.",
      type: "text",
      required: true,
      value: "",
      placeholder: "admin",
    },
    {
      id: "password",
      label: "Password",
      label_ar: "كلمة المرور",
      description: "Camera admin password.",
      description_ar: "كلمة مرور مدير الكاميرا.",
      type: "password",
      required: true,
      value: "",
    },
    {
      id: "auto_events",
      label: "Auto-capture Events",
      label_ar: "التقاط الأحداث تلقائياً",
      description:
        "Automatically capture and route motion detection and alarm events.",
      description_ar: "التقاط وتوجيه أحداث كشف الحركة والإنذارات تلقائياً.",
      type: "switch",
      required: false,
      value: true,
    },
  ],
  adapter_name: "hikvision_camera",
  provider_trait: "IntegrationProvider",
  supports_live: true,
};

export default config;
