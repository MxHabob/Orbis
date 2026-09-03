import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "authen",
  name: "Authen",
  name_ar: "أوثن",
  category: "ai-vision",
  active: true,
  icon: "ScanFace",
  short_description: "A secure digital verification and biometric identity platform developed by Mulverse.",
  short_description_ar: "منصة آمنة للتحقق الرقمي وإدارة الهوية البيومترية مطورة من قبل Mulverse.",
  description: `Authen is a state-of-the-art digital verification platform to store and manage digital face signatures. Developed by Mulverse, it provides privacy-compliant biometric data storage that integrates seamlessly with Orbis.

**Get Started & Integration**
1. Open the Authen system at [app.authen.io](https://app.authen.io).
2. Generate your secure API key from the Developer Console.
3. Learn more about service capabilities, specifications, and plans at [authen.io](https://authen.io).

**Biometric Database**
Store digital face information securely using advanced, irreversible vector templates.

**Mulverse Ecosystem Integration**
Fully developed by [Mulverse](https://www.mulverse.com/) to work natively with Orbis Computer Vision systems.

**Privacy-First Design**
Designed to follow strict user consent and privacy standards for facial recognition databases.`,
  description_ar: `Authen هي منصة متطورة للتحقق الرقمي وتخزين وإدارة الوجوه الرقمية البيومترية. تم تطويرها بواسطة [Mulverse](https://www.mulverse.com/)، وتوفر تخزيناً آمناً لبيانات الهوية الحيوية متوافقاً مع متطلبات الخصوصية ليتكامل بسلاسة مع نظام Orbis.

**بدء الاستخدام والربط**
1. قم بفتح نظام Authen عبر الرابط [app.authen.io](https://app.authen.io).
2. قم بإنشاء مفتاح الـ API الآمن الخاص بك من لوحة تحكم المطورين.
3. تفضل بزيارة الموقع الرسمي [authen.io](https://authen.io) لمعرفة تفاصيل الخدمة والمميزات وخطط الأسعار المتاحة.

**قاعدة بيانات بيومترية**
تخزين معلومات الوجوه الرقمية بأمان باستخدام نماذج متجهات مشفرة غير قابلة للعكس.

**تكامل مع بيئة Mulverse**
تم تطويرها بالكامل بواسطة Mulverse لتعمل بشكل أصلي مع أنظمة الرؤية الحاسوبية في Orbis.

**التصميم يركز على الخصوصية**
مصمم للامتثال لأعلى معايير موافقة المستخدم وحماية البيانات البيومترية.`,
  features: [
    "Uses the Authen algorithm, the smartest algorithm for simulating artificial perception",
    "Leverages state-of-the-art patented algorithms",
    "Advanced deepfake video and image detection",
    "Developer REST API & MCP WebSocket server",
  ],
  features_ar: [
    "تستخدم خوارزمية Authen، أذكى خوارزمية لمحاكاة الإدراك الاصطناعي",
    "تعتمد على خوارزميات متطورة حاصلة على براءة اختراع",
    "كشف متقدم عن التزييف العميق للصور والفيديو",
    "واجهات برمجية REST API وخادم MCP WebSocket مباشر",
  ],
  settings: [
    {
      id: "api_key",
      label: "API Key",
      label_ar: "مفتاح API",
      description: "Your Authen API key (starts with athn_sk_).",
      description_ar: "مفتاح API الخاص بـ Authen (يبدأ بـ athn_sk_).",
      type: "password",
      required: true,
      value: "",
    },
    {
      id: "api_url",
      label: "API URL",
      label_ar: "رابط الـ API",
      description: "The Authen service endpoint URL.",
      description_ar: "عنوان الخدمة الخاص بـ Authen.",
      type: "url",
      required: true,
      value: "https://api.authen.io",
      placeholder: "https://api.authen.io",
    },
    {
      id: "mcp_url",
      label: "MCP WebSocket URL",
      label_ar: "رابط الـ MCP WebSocket",
      description: "The Authen Model Context Protocol (MCP) real-time WebSocket connection URL.",
      description_ar: "رابط اتصال WebSocket المباشر لبروتوكول سياق النموذج (MCP) الخاص بـ Authen.",
      type: "url",
      required: false,
      value: "wss://api.authen.io/ws/mcp",
      placeholder: "wss://api.authen.io/ws/mcp",
    },
  ],
  adapter_name: "authen",
  provider_trait: "IntegrationProvider",
};

export default config;
