import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "tabby",
  name: "Tabby",
  name_ar: "تابي",
  category: "payment-gateway",
  active: true,
  beta: false,
  icon: "CalendarCheck",
  short_description:
    "Enable parents to split tuition and school fees into 4 interest-free monthly installments.",
  short_description_ar:
    "تمكين أولياء الأمور من تقسيط الرسوم المدرسية على 4 دفعات شهرية ميسرة بدون فوائد.",
  description: `Tabby Buy-Now-Pay-Later (BNPL) tuition installment solution for schools.

**Split School Fees into 4**
Parents pay 25% upfront at registration and the remainder over 3 monthly installments, boosting enrollment conversion.

**Upfront Payout to School**
The school receives the full tuition fee settlement upfront from Tabby, eliminating collection and credit risk.

**Automated Tracking**
Installment approval and repayment updates sync automatically with the student fee balance.`,
  description_ar: `حل التقسيط المرن من تابي للرسوم المدرسية والتعليمية.

**تقسيط الرسوم على 4 دفعات**
يدفع ولي الأمر الدفعة الأولى مقدماً عند التسجيل والباقي على أقساط شهرية ميسرة.

**تحصيل فوري للمدرسة**
تحصل المدرسة على إجمالي الرسوم كاملة مقدماً مع ضمان التحصيل بالكامل.

**مزامنة آلية للرسوم**
تسجيل الدفعة واعتماد خطة السداد في سجلات الطالب المحاسبية بصورة تلقائية.`,
  features: [
    "4 interest-free installments",
    "Upfront full payout to school",
    "Zero collection risk for administration",
    "Sharia-compliant financing",
    "Instant credit check for parents",
  ],
  features_ar: [
    "تقسيط على 4 دفعات بدون فوائد",
    "تحصيل كامل المبلغ للمدرسة مقدماً",
    "انعدام مخاطر التعثر المالي على الإدارة",
    "تمويل متوافق مع الضوابط الشرعية",
    "موافقة ائتمانية فورية لولي الأمر",
  ],
  settings: [
    {
      id: "public_key",
      label: "Public Key",
      label_ar: "المفتاح العام",
      type: "text",
      required: true,
      value: "",
      description: "Tabby public merchant key for the checkout widget.",
      description_ar:
        "المفتاح العام الخاص بتابي لعرض خيار التقسيط في واجهة الدفع.",
      placeholder: "pk_test_...",
    },
    {
      id: "secret_key",
      label: "Secret Key",
      label_ar: "المفتاح السري",
      type: "password",
      required: true,
      value: "",
      description:
        "Tabby private API key for creating checkout sessions and captures.",
      description_ar: "المفتاح السري لإنشاء جلسات الدفع وتأكيد الحجوزات.",
      placeholder: "sk_test_...",
    },
    {
      id: "merchant_code",
      label: "Merchant Code",
      label_ar: "رمز التاجر",
      type: "text",
      required: true,
      value: "",
      description: "Tabby assigned merchant ID code.",
      description_ar: "رمز تعريف المدرسة المعتمد لدى تابي.",
      placeholder: "SA...",
    },
    {
      id: "installments_count",
      label: "Installment Plan",
      label_ar: "خطة التقسيط",
      type: "select",
      required: true,
      value: "4",
      description: "Number of monthly installments offered to parents.",
      description_ar: "عدد الأقساط الشهرية المتاحة لأولياء الأمور.",
      options: [
        { label: "Pay in 4 Installments", value: "4" },
        { label: "Pay in 3 Installments", value: "3" },
      ],
    },
    {
      id: "test_mode",
      label: "Sandbox Mode",
      label_ar: "الوضع التجريبي",
      type: "switch",
      required: false,
      value: true,
      description: "Enable Tabby sandbox testing mode.",
      description_ar: "تفعيل الوضع التجريبي لاختبار دورة حياة التقسيط.",
    },
  ],
  adapter_name: "tabby",
  provider_trait: "PaymentGatewayProvider",
};

export default config;
