import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "orbis_mobile",
  name: "Orbis Mobile",
  name_ar: "أوربيس موبايل",
  category: "identity",
  active: true,
  icon: "Smartphone",
  short_description:
    "Enable Orbis Mobile so users can find their institution's Orbis server.",
  short_description_ar:
    "فعّل تطبيق أوربس للموبايل ليتمكن المستخدمون من العثور على خادم مؤسستهم.",
  description: `Orbis Mobile connects users directly to their institution's Orbis server. Enable this integration to show its settings page. Manage the institution's active production endpoint in the Mulverse deployment dashboard; mobile registration searches the Mulverse institution directory and then connects directly to Orbis.

**Security**
The directory publishes only active public HTTPS endpoints. It does not grant an account or access to school data.`,
  description_ar: `يربط تطبيق أوربس للموبايل المستخدمين مباشرة بخادم Orbis الخاص بمؤسستهم. فعّل هذا التكامل لإظهار صفحة إعداداته. أدر عنوان الإنتاج النشط للمؤسسة في لوحة النشرات في Mulverse؛ يبحث التسجيل في دليل المؤسسات ثم يتصل مباشرةً بخادم Orbis.

**الأمان**
ينشر الدليل عناوين HTTPS العامة والنشطة فقط. ولا يمنح حسابًا أو وصولًا إلى بيانات المدرسة.`,
  features: [
    "Direct authentication with the institution's Orbis API",
    "Institution search through the Mulverse endpoint directory",
    "Active public production endpoints only",
  ],
  features_ar: [
    "مصادقة مباشرة مع واجهة Orbis الخاصة بالمؤسسة",
    "البحث عن المؤسسة عبر دليل العناوين في Mulverse",
    "عناوين الإنتاج العامة النشطة فقط",
  ],
  settings: [],
};

export default config;
