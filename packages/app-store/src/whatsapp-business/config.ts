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
    "Send WhatsApp notifications to parents through your connected Twilio WhatsApp sender.",
  short_description_ar:
    "إرسال إشعارات واتساب إلى أولياء الأمور عبر مرسل Twilio المتصل.",
  description: `Connect a Twilio WhatsApp-enabled sender to deliver Orbis notifications to parents.

**Verified Connection**
Orbis checks your Twilio account credentials before saving the integration. Actual delivery also requires an active WhatsApp sender in Twilio.

**WhatsApp Delivery**
Send supported Orbis WhatsApp notifications using the existing notification adapter. Twilio and WhatsApp messaging charges may apply.`,
  description_ar: `اربط مرسل WhatsApp مفعلاً لدى Twilio لإرسال إشعارات أوربيس إلى أولياء الأمور.

**التحقق من الربط**
يتحقق أوربيس من بيانات حساب Twilio قبل حفظ التكامل. ويتطلب الإرسال الفعلي مرسلاً نشطاً لواتساب في Twilio.

**إرسال رسائل واتساب**
إرسال إشعارات أوربيس المدعومة عبر موصّل الإشعارات الحالي. قد تُطبق رسوم Twilio وWhatsApp.`,
  features: [
    "Twilio WhatsApp sender integration",
    "Credential verification before saving",
    "Send supported Orbis WhatsApp notifications",
  ],
  features_ar: [
    "الربط بمرسل WhatsApp عبر Twilio",
    "التحقق من بيانات الحساب قبل الحفظ",
    "إرسال إشعارات أوربيس المدعومة عبر WhatsApp",
  ],
  settings: [
    {
      id: "provider",
      label: "WhatsApp Provider",
      label_ar: "مزود WhatsApp",
      description: "WhatsApp delivery is currently verified through Twilio.",
      description_ar: "يتم التحقق من إرسال WhatsApp حالياً عبر Twilio.",
      type: "select",
      required: true,
      value: "twilio",
      options: [{ label: "Twilio", value: "twilio" }],
    },
    {
      id: "account_sid",
      label: "Twilio Account SID",
      label_ar: "معرف حساب Twilio",
      description: "Account SID for the Twilio account with WhatsApp enabled.",
      description_ar: "معرف حساب Twilio الذي فُعّل فيه WhatsApp.",
      type: "password",
      required: true,
      value: "",
      placeholder: "AC...",
    },
    {
      id: "auth_token",
      label: "Twilio Auth Token",
      label_ar: "رمز مصادقة Twilio",
      description: "Authentication token for the Twilio API.",
      description_ar: "رمز المصادقة الخاص بواجهة Twilio API.",
      type: "password",
      required: true,
      value: "",
    },
    {
      id: "from_number",
      label: "WhatsApp Sender Number",
      label_ar: "رقم مرسل WhatsApp",
      description:
        "Twilio WhatsApp-enabled sender number in international format, without the whatsapp: prefix.",
      description_ar:
        "رقم المرسل المفعّل لواتساب لدى Twilio بالصيغة الدولية، دون بادئة whatsapp:.",
      type: "text",
      required: true,
      value: "",
      placeholder: "+1234567890",
    },
  ],
  adapter_name: "whatsapp-business",
  provider_trait: "NotificationProvider",
  notification_channels: ["whatsapp"],
};

export default config;
