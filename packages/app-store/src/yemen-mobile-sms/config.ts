import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "yemen-mobile-sms",
  name: "Yemen Mobile Business SMS",
  name_ar: "رسائل يمن موبايل للأعمال",
  category: "communication",
  active: false,
  beta: false,
  icon: "ChatBubbles",
  short_description:
    "Business SMS notifications for institutions and organizations in Yemen.",
  short_description_ar: "خدمة رسائل أعمال للمؤسسات والجهات في اليمن.",
  description: `Yemen Mobile's official business messaging information describes bulk and banking notifications for businesses, institutions, and organizations.

**Integration requirements**
Public pages do not document an SMS gateway API, authentication method, or delivery callbacks for Orbis. Confirm business onboarding and technical access with Yemen Mobile before enabling automated messaging.

[Business banking and bulk notifications](https://yemenmobile.com.ye/خدمات-الرسائل-البنكية-والبالك) · [SMS Wallet package](https://yemenmobile.com.ye/باقة-sms-wallet-للمحافظ-المالية).`,
  description_ar: `توضح خدمات يمن موبايل للأعمال إمكانية إرسال الرسائل البنكية ورسائل البالك للمؤسسات والشركات والجهات، كما توفر باقة SMS Wallet للإشعارات المالية الخاصة بالمحافظ.

**متطلبات التكامل**
لا توضح الصفحات العامة واجهة API لبوابة رسائل أو آلية مصادقة وتقارير تسليم يمكن لأوربيس استخدامها. يجب تأكيد الاشتراك التجاري والتفاصيل التقنية مع يمن موبايل قبل تفعيل الإرسال الآلي.

[خدمات الرسائل البنكية والبالك](https://yemenmobile.com.ye/خدمات-الرسائل-البنكية-والبالك) · [باقة SMS Wallet](https://yemenmobile.com.ye/باقة-sms-wallet-للمحافظ-المالية).`,
  features: [
    "Business and institutional SMS notifications",
    "Banking and bulk messaging services",
    "SMS Wallet financial notifications",
    "Public messaging API availability requires confirmation",
  ],
  features_ar: [
    "رسائل SMS للمؤسسات والجهات",
    "خدمات الرسائل البنكية والبالك",
    "إشعارات مالية للمحافظ عبر باقة SMS Wallet",
    "يلزم التأكد من توفر API للإرسال الآلي",
  ],
  settings: [],
};

export default config;
