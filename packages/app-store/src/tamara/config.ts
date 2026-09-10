import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "tamara",
  name: "Tamara",
  name_ar: "تمارا",
  category: "payment-gateway",
  active: true,
  beta: false,
  icon: "Coins",
  short_description:
    "Saudi Sharia-compliant Buy-Now-Pay-Later payment platform for splitting school fees.",
  short_description_ar:
    "منصة التقسيط السعودية المتوافقة مع الشريعة الإسلامية لتقسيط الرسوم الدراسية.",
  description: `Tamara BNPL integration for easy, Sharia-certified school fee installment plans.

**Flexible Installments**
Offer parents flexible split options (Pay in 2, 3, or 4 installments) with instant credit pre-qualification.

**Guaranteed School Receivables**
Full transaction settlement deposited directly into the school's bank account on day one.

**Seamless Parent Experience**
High conversion checkout widget embedded within the Orbis parent portal and cashier receipts.`,
  description_ar: `ربط منصة تمارا لحلول الدفع الآجل وتقسيط الرسوم الدراسية المتوافقة مع الضوابط الشرعية.

**خيارات تقسيط مرنة**
إتاحة الدفع على أقساط متعددة مع فحص أهلية ائتماني فوري وسلس لولي الأمر.

**مستحقات مضمونة للمدرسة**
إيداع كامل المبلغ مباشرة في الحساب البنكي للمدرسة من اليوم الأول بدون متابعة تحصيل.

**تجربة مستخدم سهلة**
تكامل سلس مع بوابة أولياء الأمور وسندات القبض داخل نظام أوربيس.`,
  features: [
    "Sharia-certified financing",
    "Split into 2, 3, or 4 payments",
    "100% guaranteed school payouts",
    "Instant digital onboarding",
    "Automatic settlement webhooks",
  ],
  features_ar: [
    "تمويل معتمد ومتوافق مع الشريعة",
    "تقسيط على دفعات ميسرة",
    "مستحقات مضمونة بنسبة كاملة للمدرسة",
    "اعتماد وموافقة رقمية فورية",
    "إشعارات تسوية تلقائية للقيود المحاسبية",
  ],
  settings: [
    {
      id: "api_token",
      label: "API Token",
      label_ar: "رمز الربط السري",
      type: "password",
      required: true,
      value: "",
      description: "Tamara Merchant API Bearer Token.",
      description_ar: "رمز الوصول السري لتاجر تمارا لإنشاء وتأكيد طلبات الدفع.",
    },
    {
      id: "public_key",
      label: "Public Key",
      label_ar: "المفتاح العام",
      type: "text",
      required: true,
      value: "",
      description: "Tamara public key for frontend promotional widgets.",
      description_ar:
        "المفتاح العام المستخدم لإظهار شارات التقسيط في واجهة المستخدم.",
    },
    {
      id: "notification_token",
      label: "Notification Token",
      label_ar: "رمز أمان الإشعارات",
      type: "password",
      required: false,
      value: "",
      description: "Token used to verify incoming webhook alerts from Tamara.",
      description_ar: "رمز التحقق من صحة الإشعارات الواردة من تمارا.",
    },
    {
      id: "installments_count",
      label: "Default Installment Plan",
      label_ar: "خطة التقسيط الافتراضية",
      type: "select",
      required: true,
      value: "4",
      description: "Select available installment structures.",
      description_ar: "حدد خطط التقسيط المتاحة للمدرسة.",
      options: [
        { label: "Pay in 4 Installments", value: "4" },
        { label: "Pay in 3 Installments", value: "3" },
        { label: "Pay in 2 Installments", value: "2" },
      ],
    },
    {
      id: "sandbox_mode",
      label: "Sandbox Mode",
      label_ar: "الوضع التجريبي",
      type: "switch",
      required: false,
      value: true,
      description: "Enable Tamara sandbox testing environment.",
      description_ar: "تفعيل البيئة التجريبية لاختبار دورة حياة العمليات.",
    },
  ],
  adapter_name: "tamara",
  provider_trait: "PaymentGatewayProvider",
};

export default config;
