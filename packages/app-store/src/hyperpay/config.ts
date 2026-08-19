import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "hyperpay",
  name: "HyperPay",
  name_ar: "هايبر باي",
  category: "payment-gateway",
  active: true,
  beta: false,
  icon: "Layers",
  short_description: "Enterprise payment orchestration platform supporting Mada, Apple Pay, Visa, Mastercard, and SADAD.",
  short_description_ar: "بوابة المدفوعات المتكاملة للربط الموحد مع بطاقات مدى والبطاقات الائتمانية وسداد.",
  description: `HyperPay enterprise multi-channel payment gateway aggregator for educational institutions.

**All-in-One Gateway**
Unified processing for Mada, Apple Pay, Visa, Mastercard, SADAD, American Express, and UnionPay.

**Advanced Fraud Prevention**
Integrated 3D Secure 2.0 with machine learning risk evaluation to protect school transactions.

**Smart Routing**
Automatic transaction routing to multiple acquiring banks for maximum uptime and payment success rates.`,
  description_ar: `بوابة هايبر باي الموحدة للمؤسسات والمدارس الكبرى.

**بوابة دفع شاملة**
تكامل موحد مع مدى والمحافظ الرقمية والبطاقات الائتمانية وسداد من نقطة ربط واحدة.

**حماية متقدمة ومكافحة الاحتيال**
دعم أنظمة الحماية المتقدمة وتقييم المخاطر الذكي لحماية المعاملات المدرسية.

**توجيه ذكي للعمليات**
توزيع تلقائي للعمليات على عدة بنوك مستحوذة لضمان أعلى نسبة نجاح للعمليات.`,
  features: [
    "Unified multi-gateway processing",
    "Mada & Apple Pay native checkout",
    "3D Secure 2.0 authentication",
    "Smart acquirer routing",
    "Detailed settlement reports",
  ],
  features_ar: [
    "بوابة موحدة لجميع قنوات الدفع",
    "دعم مباشر لبطاقات مدى والمحافظ الرقمية",
    "حماية وتأمين متقدم للعمليات",
    "توجيه ذكي بين البنوك المستحوذة",
    "تقارير تسوية وتصفية تفصيلية",
  ],
  settings: [
    {
      id: "access_token",
      label: "Access Token",
      label_ar: "رمز الوصول المعتمد",
      type: "password",
      required: true,
      value: "",
      description: "HyperPay API Access Token from your merchant dashboard.",
      description_ar: "رمز الوصول المعتمد من لوحة تحكم التاجر.",
    },
    {
      id: "entity_id_mada",
      label: "Entity ID for Mada",
      label_ar: "معرف القناة لبطاقات مدى",
      type: "text",
      required: true,
      value: "",
      description: "Entity ID designated for Mada card processing.",
      description_ar: "رقم القناة المخصص لعمليات بطاقات مدى.",
      placeholder: "8a8294...",
    },
    {
      id: "entity_id_cards",
      label: "Entity ID for Credit Cards",
      label_ar: "معرف القناة للبطاقات الائتمانية",
      type: "text",
      required: true,
      value: "",
      description: "Entity ID designated for Credit Cards & Apple Pay.",
      description_ar: "رقم القناة المخصص للبطاقات الائتمانية والمحافظ الرقمية.",
      placeholder: "8a8294...",
    },
    {
      id: "test_mode",
      label: "Test Mode",
      label_ar: "الوضع التجريبي",
      type: "switch",
      required: false,
      value: true,
      description: "Toggle between HyperPay test and production environments.",
      description_ar: "التبديل بين البيئة التجريبية والبيئة الحية.",
    },
  ],
  adapter_name: "hyperpay",
  provider_trait: "PaymentGatewayProvider",
};

export default config;
