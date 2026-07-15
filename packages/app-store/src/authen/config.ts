import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "authen",
  name: "Authen",
  name_ar: "أوثين",
  category: "ai-vision",
  active: true,
  icon: "ScanFace",
  short_description: "A secure digital face storage and biometric identity platform developed by Mulverse.",
  short_description_ar: "منصة آمنة لتخزين الوجوه الرقمية وإدارة الهوية البيومترية مطورة من قبل Mulverse.",
  description: `Authen is a state-of-the-art platform to store and manage digital face signatures. Developed by Mulverse, it provides privacy-compliant biometric data storage that integrates seamlessly with Orbis.

**Biometric Database**
Store digital face information securely using advanced, irreversible vector templates.

**Mulverse Ecosystem Integration**
Fully developed by [Mulverse](https://www.mulverse.com/) to work natively with Orbis Computer Vision systems.

**Privacy-First Design**
Designed to follow strict user consent and privacy standards for facial recognition databases.`,
  description_ar: `Authen هي منصة متطورة لتخزين وإدارة الوجوه الرقمية البيومترية. تم تطويرها بواسطة [Mulverse](https://www.mulverse.com/)، وتوفر تخزيناً آمناً لبيانات الهوية الحيوية متوافقاً مع متطلبات الخصوصية ليتكامل بسلاسة مع نظام Orbis.

**قاعدة بيانات بيومترية**
تخزين معلومات الوجوه الرقمية بأمان باستخدام نماذج متجهات مشفرة غير قابلة للعكس.

**تكامل مع بيئة Mulverse**
تم تطويرها بالكامل بواسطة Mulverse لتعمل بشكل أصلي مع أنظمة الرؤية الحاسوبية في Orbis.

**التصميم يركز على الخصوصية**
مصمم للامتثال لأعلى معايير موافقة المستخدم وحماية البيانات البيومترية.`,
  features: [
    "Secure digital face storage",
    "Developed by Mulverse",
    "High-speed face matching vector search",
    "GDPR & privacy compliance controls",
    "Real-time sync with Orbis cameras",
  ],
  features_ar: [
    "تخزين آمن للوجوه الرقمية",
    "مطور من قبل Mulverse",
    "بحث سريع عن مطابقة متجهات الوجوه",
    "ضوابط الامتثال لخصوصية البيانات",
    "مزامنة فورية مع كاميرات Orbis",
  ],
  settings: [
    {
      id: "api_key",
      label: "API Key",
      label_ar: "مفتاح API",
      description: "Your Authen API key from Mulverse Console.",
      description_ar: "مفتاح API الخاص بك من لوحة تحكم Mulverse.",
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
      value: "https://api.mulverse.com/authen",
      placeholder: "https://api.mulverse.com/authen",
    },
  ],
  adapter_name: "authen",
  provider_trait: "IntegrationProvider",
};

export default config;
