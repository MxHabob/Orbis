import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "anthropic",
  name: "Anthropic Claude",
  name_ar: "أنثروبيك كلود",
  category: "ai-provider",
  active: true,
  icon: "Sparkles",
  short_description:
    "Connect Anthropic Claude 3.7 Sonnet, 3.5 Sonnet, and Haiku models for deep reasoning and nuanced agentic capabilities.",
  short_description_ar:
    "ربط نماذج Anthropic Claude مثل Claude 3.7 Sonnet و 3.5 للاستدلال العميق وإمكانات الوكلاء الذكية.",
  description: `Integrate Claude 3.7 Sonnet, Claude 3.5 Sonnet, and Claude 3.5 Haiku into Orbis OS.

**Hybrid Thinking & Reasoning**
Claude 3.7 Sonnet provides hybrid standard response and extended reasoning for complex workflows.

**Precise Tool Execution**
Native tool-use capabilities with rigorous adherence to JSON schemas and complex academic constraints.`,
  description_ar: `ادمج نماذج Claude الرائدة من Anthropic مباشرة في نظام Orbis التعليمي.

**استدلال هجين وتفكير متقدم**
يوفر Claude 3.7 Sonnet تفكيراً استدلالياً عميقاً وموثوقية استثنائية في تنفيذ السياسات المدرسية والأكاديمية المعقدة.

**تنفيذ متقن للأدوات**
دعم أصلي دقيق لاستدعاء الأدوات والالتزام بالمخططات المحددة.`,
  features: [
    "Claude 3.7 Sonnet (Hybrid reasoning & high performance)",
    "Claude 3.5 Sonnet & Claude 3.5 Haiku support",
    "200K token context window support",
    "Native tool use with strict output schema guarantees",
  ],
  features_ar: [
    "نموذج Claude 3.7 Sonnet (استدلال هجين وأداء فائق)",
    "دعم نماذج Claude 3.5 Sonnet و Haiku",
    "نافذة سياق ضخمة تتسع لـ 200 ألف توكن",
    "استدعاء دقيق للأدوات وضمانات عالية للسلامة والموثوقية",
  ],
  settings: [
    {
      id: "api_key",
      label: "API Key",
      label_ar: "مفتاح API",
      description: "Your Anthropic API key (sk-ant-...).",
      description_ar: "مفتاح API الخاص بك من منصة Anthropic Console.",
      type: "password",
      required: true,
      value: "",
      placeholder: "sk-ant-...",
    },
    {
      id: "model",
      label: "Default Model",
      label_ar: "النموذج الافتراضي",
      description: "Default Claude model to use.",
      description_ar: "النموذج الافتراضي لتنفيذ العمليات.",
      type: "select",
      required: true,
      value: "",
    },
    {
      id: "endpoint_url",
      label: "Endpoint URL (Optional)",
      label_ar: "عنوان نقطة النهاية (اختياري)",
      description: "Custom Anthropic endpoint proxy if applicable.",
      description_ar: "عنوان وكيل Anthropic المخصص إن وجد.",
      type: "text",
      required: false,
      value: "https://api.anthropic.com",
      placeholder: "https://api.anthropic.com",
    },
    {
      id: "temperature",
      label: "Temperature",
      label_ar: "درجة الحرارة",
      description: "Sampling temperature (0.0 to 1.0).",
      description_ar: "درجة الحرارة (0.0 إلى 1.0).",
      type: "number",
      required: false,
      value: 0.2,
      validation: { min: 0, max: 1 },
    },
  ],
  adapter_name: "anthropic",
  provider_trait: "LlmProvider",
};

export default config;
