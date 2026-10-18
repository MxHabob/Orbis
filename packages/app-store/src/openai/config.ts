import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "openai",
  name: "OpenAI",
  name_ar: "أوبن إيه آي",
  category: "ai-provider",
  active: true,
  icon: "Sparkles",
  short_description:
    "Connect OpenAI GPT-4o, o1, and o3-mini models for cutting-edge reasoning and agentic workflows.",
  short_description_ar:
    "ربط نماذج OpenAI مثل GPT-4o و o1 و o3-mini للاستدلال المتقدم وتدفقات عمل الوكلاء.",
  description: `Integrate OpenAI's industry-leading GPT-4o, GPT-4.5, and reasoning models (o1, o3-mini) directly into Orbis OS.

**State-of-the-Art Intelligence**
Leverage frontier models for complex multi-step reasoning, mathematical calculations, and automated report generation.

**Robust Function Calling**
Seamless JSON function calling allowing agents to interact with all Orbis domain services securely.

**Flexible Configuration**
Supports custom organization IDs, fine-tuned models, and configurable max token budgets.`,
  description_ar: `ادمج نماذج OpenAI الرائدة مثل GPT-4o و GPT-4.5 ونماذج الاستدلال (o1, o3-mini) مباشرة في نظام Orbis.

**ذكاء واستدلال فائق**
استفد من أقوى نماذج الذكاء الاصطناعي للاستدلال متعدد الخطوات وإجراء الحسابات المعقدة وإنشاء التقارير آلياً.

**استدعاء الأدوات بدقة عالية**
دعم كامل لاستدعاء وظائف وأدوات Orbis التفاعلية.`,
  features: [
    "Frontier reasoning models (GPT-4o, o1, o3-mini, GPT-4.5)",
    "High-speed function and tool calling",
    "Multilingual excellence in Arabic and English",
    "Custom organization ID and token budget control",
  ],
  features_ar: [
    "أحدث نماذج الاستدلال (GPT-4o, o1, o3-mini, GPT-4.5)",
    "استدعاء سريع وموثوق للأدوات والوظائف",
    "دعم ممتاز للغتين العربية والإنجليزية",
    "تحكم دقيق بميزانية التوكنز ومعرف المنظمة",
  ],
  settings: [
    {
      id: "api_key",
      label: "API Key",
      label_ar: "مفتاح API",
      description: "Your OpenAI API key (sk-...).",
      description_ar: "مفتاح API الخاص بك من منصة OpenAI.",
      type: "password",
      required: true,
      value: "",
      placeholder: "sk-proj-...",
    },
    {
      id: "model",
      label: "Default Model",
      label_ar: "النموذج الافتراضي",
      description: "Default OpenAI model for completions and agents.",
      description_ar: "النموذج الافتراضي لإنشاء المحتوى وتنفيذ مهام الوكلاء.",
      type: "select",
      required: true,
      value: "",
    },
    {
      id: "endpoint_url",
      label: "API Base URL (Optional)",
      label_ar: "عنوان نقطة النهاية (اختياري)",
      description: "Custom OpenAI compatible proxy or Azure gateway URL.",
      description_ar: "عنوان بوابة أو وكيل متوافق مع بروتوكول OpenAI.",
      type: "text",
      required: false,
      value: "https://api.openai.com",
      placeholder: "https://api.openai.com",
    },
    {
      id: "organization_id",
      label: "Organization ID (Optional)",
      label_ar: "معرف المنظمة (اختياري)",
      description: "Optional OpenAI Organization ID header (org-...).",
      description_ar: "معرف منظمة OpenAI الاختياري.",
      type: "text",
      required: false,
      value: "",
      placeholder: "org-...",
    },
    {
      id: "temperature",
      label: "Temperature",
      label_ar: "درجة الحرارة",
      description: "Sampling temperature (0.0 = deterministic, 1.0 = creative).",
      description_ar: "درجة الحرارة (0.0 = دقيق، 1.0 = إبداعي).",
      type: "number",
      required: false,
      value: 0.2,
      validation: { min: 0, max: 2 },
    },
  ],
  adapter_name: "openai",
  provider_trait: "LlmProvider",
};

export default config;
