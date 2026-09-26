import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "deepseek",
  name: "DeepSeek",
  name_ar: "ديب سيك",
  category: "ai-provider",
  active: true,
  icon: "Sparkles",
  short_description:
    "Connect DeepSeek-V3 and DeepSeek-R1 for high-performance open reasoning at unbeatable efficiency.",
  short_description_ar:
    "ربط نماذج ديب سيك V3 و R1 للاستدلال والتفكير العميق بكفاءة وسرعة استثنائية.",
  description: `Integrate DeepSeek-V3 and DeepSeek-R1 (Reasoning model) into Orbis OS.

**DeepSeek-R1 Reasoning Power**
Excels at complex math, logic, timetable constraints, and deep academic reasoning.

**DeepSeek-V3 High-Speed Execution**
High-throughput, cost-efficient model ideal for conversational assistance and general educational management.`,
  description_ar: `ادمج نماذج DeepSeek الرائدة في نظام Orbis.

**نموذج الاستدلال العميق DeepSeek-R1**
يتفوق في العمليات المنطقية المعقدة وحسابات الجداول والسياسات الأكاديمية.

**نموذج DeepSeek-V3 فائق السرعة**
أداء ممتاز بتكلفة منخفضة وسرعة استجابة عالية.`,
  features: [
    "DeepSeek-R1 (Advanced reasoning model)",
    "DeepSeek-V3 (High-throughput general model)",
    "OpenAI-compatible wire protocol",
    "Cost-effective token pricing",
  ],
  features_ar: [
    "نموذج الاستدلال المتقدم DeepSeek-R1",
    "نموذج الاستجابة السريعة DeepSeek-V3",
    "توافق كامل مع بروتوكول OpenAI",
    "كفاءة عالية واستهلاك اقتصادي للتوكنز",
  ],
  settings: [
    {
      id: "api_key",
      label: "API Key",
      label_ar: "مفتاح API",
      description: "Your DeepSeek API key (sk-...).",
      description_ar: "مفتاح API الخاص بك من منصة DeepSeek Open Platform.",
      type: "password",
      required: true,
      value: "",
      placeholder: "sk-...",
    },
    {
      id: "model",
      label: "Default Model",
      label_ar: "النموذج الافتراضي",
      description: "Choose DeepSeek model.",
      description_ar: "اختر نموذج ديب سيك الافتراضي.",
      type: "select",
      required: true,
      value: "deepseek-chat",
      options: [
        { label: "DeepSeek-V3 (deepseek-chat — General)", value: "deepseek-chat" },
        { label: "DeepSeek-R1 (deepseek-reasoner — Reasoning)", value: "deepseek-reasoner" },
      ],
    },
    {
      id: "endpoint_url",
      label: "Endpoint URL",
      label_ar: "عنوان نقطة النهاية",
      description: "DeepSeek API endpoint.",
      description_ar: "عنوان API الخاص بـ DeepSeek.",
      type: "text",
      required: false,
      value: "https://api.deepseek.com",
      placeholder: "https://api.deepseek.com",
    },
    {
      id: "temperature",
      label: "Temperature",
      label_ar: "درجة الحرارة",
      description: "Sampling temperature (0.0 to 1.5).",
      description_ar: "درجة الحرارة.",
      type: "number",
      required: false,
      value: 0.2,
      validation: { min: 0, max: 2 },
    },
  ],
  adapter_name: "deepseek",
  provider_trait: "LlmProvider",
};

export default config;
