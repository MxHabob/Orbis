import type { OrbisApp } from "./types";

const websites: Record<string, string> = {
  "active-directory":
    "https://learn.microsoft.com/windows-server/identity/ad-ds/get-started/virtual-dc/active-directory-domain-services-overview",
  alkuraimi: "https://kuraimibank.com/en/services/50",
  anthropic: "https://console.anthropic.com/",
  authen: "https://app.authen.io/",
  "barcode-scanner": "https://www.zebra.com/us/en/products/scanners.html",
  "cloud-storage":
    "https://docs.aws.amazon.com/AmazonS3/latest/userguide/Welcome.html",
  deepseek: "https://platform.deepseek.com/",
  fingerprint: "https://www.zkteco.com/en/",
  "gemini-ai": "https://aistudio.google.com/",
  "google-workspace": "https://workspace.google.com/",
  groq: "https://console.groq.com/",
  "hikvision-camera": "https://www.hikvision.com/en/",
  hyperpay: "https://www.hyperpay.com/",
  jaib: "https://e-jaib.com/ar",
  mada: "https://www.saudipayments.com/",
  "microsoft-365": "https://admin.microsoft.com/",
  mistral: "https://console.mistral.ai/",
  moyasar: "https://dashboard.moyasar.com/",
  "mulverse-cloud": "https://www.mulverse.com/",
  "ocr-service": "https://cloud.google.com/vision/docs",
  ollama: "https://ollama.com/",
  openai: "https://platform.openai.com/",
  openrouter: "https://openrouter.ai/",
  "orbis-ai": "https://www.mulverse.com/",
  orbis_mobile: "https://www.mulverse.com/",
  printer: "https://www.pwg.org/ipp/everywhere.html",
  "push-notifications": "https://firebase.google.com/docs/cloud-messaging",
  rfid: "https://www.gs1.org/standards/epc-rfid",
  sadad: "https://www.sadad.com/",
  "sms-provider": "https://www.twilio.com/docs/messaging",
  "smtp-email": "https://docs.sendgrid.com/ui/account-and-settings/api-keys",
  "solver-service": "https://www.mulverse.com/",
  stripe: "https://dashboard.stripe.com/",
  tabby: "https://business.tabby.ai/",
  tamara: "https://tamara.co/en-SA/merchant",
  "whatsapp-business": "https://www.twilio.com/docs/whatsapp/api",
  "yemen-mobile-sms": "https://yemenmobile.com.ye/",
  "yemen-wallet":
    "https://play.google.com/store/apps/details?id=com.s2m.yamenpost.merchant.android",
};

const connectionSummaries: Record<
  OrbisApp["category"],
  { en: string; ar: string }
> = {
  "ai-provider": {
    en: "To connect, create an API key with the provider, then enter it and choose a model below. Orbis sends a minimal test request that may incur provider usage charges.",
    ar: "للربط، أنشئ مفتاح API لدى المزود، ثم أدخله واختر النموذج أدناه. يرسل أوربيس طلب اختبار قصيراً قد تترتب عليه رسوم استخدام من المزود.",
  },
  surveillance: {
    en: "To connect, enable the camera API, then enter its network address and credentials below.",
    ar: "للربط، فعّل API الكاميرا، ثم أدخل عنوانها وبيانات الدخول أدناه.",
  },
  communication: {
    en: "To connect, activate a business account, verify the sender, then enter the requested credentials below.",
    ar: "للربط، فعّل حساب أعمال واعتمد هوية المرسل، ثم أدخل بيانات الاعتماد المطلوبة أدناه.",
  },
  identity: {
    en: "To connect, create or enable the organization integration in the provider's admin portal, then enter its account details below.",
    ar: "للربط، أنشئ تكامل المؤسسة أو فعّله من لوحة إدارة المزود، ثم أدخل بيانات الحساب المطلوبة أدناه.",
  },
  hardware: {
    en: "To connect, install a compatible device on your network, then enter its connection details below when available.",
    ar: "للربط، ثبّت جهازاً متوافقاً على شبكتك، ثم أدخل بيانات الاتصال به أدناه إن توفرت.",
  },
  storage: {
    en: "To connect, create a storage location and access credentials with the provider, then enter them below. Orbis verifies access with a temporary write and delete.",
    ar: "للربط، أنشئ مساحة تخزين وبيانات وصول لدى المزود، ثم أدخلها أدناه. يتحقق أوربيس من الوصول بإنشاء ملف مؤقت ثم حذفه.",
  },
  "ai-service": {
    en: "To connect, enable the service API, then select the provider and enter its API key below.",
    ar: "للربط، فعّل API الخدمة، ثم اختر المزود وأدخل مفتاح API أدناه.",
  },
  "ai-vision": {
    en: "To connect, request organization API access, then enter the service endpoint and API key below.",
    ar: "للربط، اطلب صلاحية API للمؤسسة، ثم أدخل عنوان الخدمة ومفتاح API أدناه.",
  },
  "payment-gateway": {
    en: "To connect, complete merchant onboarding and obtain test credentials, then enter the requested details below.",
    ar: "للربط، أكمل تسجيل التاجر واحصل على بيانات بيئة الاختبار، ثم أدخل التفاصيل المطلوبة أدناه.",
  },
  automation: {
    en: "To connect, deploy the service and make its endpoint reachable from Orbis, then enter the endpoint below.",
    ar: "للربط، انشر الخدمة واجعل عنوانها متاحاً لأوربيس، ثم أدخل العنوان أدناه.",
  },
};

const appSpecificSummaries: Record<string, { en: string; ar: string }> = {
  authen: {
    en: "Enter your Authen API URL and key. Orbis verifies access with a generated blank test image; no user image is sent, but the check may count toward your Authen usage.",
    ar: "أدخل عنوان Authen ومفتاح API. يتحقق أوربيس من الوصول بصورة اختبار اصطناعية فارغة؛ لا تُرسل صور مستخدمين، لكن قد يُحتسب الطلب ضمن استخدام Authen.",
  },
  alkuraimi: {
    en: "Contact Al-Kuraimi to request merchant API specifications and confirm Orbis support before proceeding.",
    ar: "تواصل مع بنك الكريمي لطلب مواصفات API للتجار وتأكد من دعم أوربيس قبل المتابعة.",
  },
  jaib: {
    en: "Contact JAIB to confirm merchant API availability and Orbis compatibility before proceeding.",
    ar: "تواصل مع جيب للتأكد من توفر API للتجار وتوافقه مع أوربيس قبل المتابعة.",
  },
  "smtp-email": {
    en: "Create a SendGrid API key with Mail Send and User Profile Read access, verify a sender email, then enter the key and sender details below.",
    ar: "أنشئ مفتاح SendGrid بصلاحية Mail Send وUser Profile Read، واعتمد بريد المرسل، ثم أدخل المفتاح وبيانات المرسل أدناه.",
  },
  "whatsapp-business": {
    en: "Enable a WhatsApp sender in Twilio, then enter its Account SID, Auth Token, and sender number below. Orbis verifies the Twilio account before saving; actual WhatsApp delivery requires an active sender.",
    ar: "فعّل مرسل WhatsApp في Twilio، ثم أدخل Account SID وAuth Token ورقم المرسل أدناه. يتحقق أوربيس من حساب Twilio قبل الحفظ؛ ويتطلب الإرسال الفعلي مرسلاً نشطاً.",
  },
  orbis_mobile: {
    en: "To connect, publish your active public HTTPS endpoint in the Mulverse deployment dashboard.",
    ar: "للربط، انشر عنوان HTTPS العام والنشط في لوحة نشر Mulverse.",
  },
  "yemen-mobile-sms": {
    en: "Contact Yemen Mobile for official SMS API requirements; a public integration API is not currently documented.",
    ar: "تواصل مع يمن موبايل لمعرفة متطلبات SMS API الرسمية؛ لا تتوفر حالياً وثائق عامة لواجهة التكامل.",
  },
  "yemen-wallet": {
    en: "Contact Yemen Post to confirm merchant API availability; the published app does not document an integration API.",
    ar: "تواصل مع البريد اليمني للتأكد من توفر API للتجار؛ لا يوثّق التطبيق المنشور واجهة تكامل.",
  },
};

const appsWithLiveVerifier = new Set([
  "authen",
  "anthropic",
  "cloud-storage",
  "deepseek",
  "gemini-ai",
  "groq",
  "mistral",
  "moyasar",
  "ollama",
  "openai",
  "openrouter",
  "sms-provider",
  "smtp-email",
  "solver-service",
  "whatsapp-business",
]);

const appsAvailableWithoutLiveVerifier = new Set(["orbis_mobile"]);

export function addConnectionSummary(app: OrbisApp): OrbisApp {
  const website = websites[app.id];
  if (!website) {
    throw new Error(
      `Missing official website for app-store entry "${app.id}".`,
    );
  }

  const summary =
    appSpecificSummaries[app.id] ?? connectionSummaries[app.category];
  const canVerifyConnection = appsWithLiveVerifier.has(app.id);
  const isAvailable =
    canVerifyConnection || appsAvailableWithoutLiveVerifier.has(app.id);
  const moreEn = `[Learn more](${website})`;
  const moreAr = `[لمعرفة المزيد](${website})`;
  const verificationNoteEn = isAvailable
    ? ""
    : "\n\n**Orbis availability:** Orbis cannot verify this integration yet, so it cannot be connected from the App Store.";
  const verificationNoteAr = isAvailable
    ? ""
    : "\n\n**توفر الربط في أوربيس:** لا يستطيع أوربيس التحقق من هذا التكامل حالياً، لذلك لا يمكن ربطه من المتجر بعد.";

  return {
    ...app,
    active: app.active && isAvailable,
    description: `${app.description}\n\n**Connect:** ${summary.en} ${moreEn}${verificationNoteEn}`,
    description_ar: `${app.description_ar ?? ""}\n\n**طريقة الربط:** ${summary.ar} ${moreAr}${verificationNoteAr}`,
  };
}
