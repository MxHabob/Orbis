import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "ollama",
  name: "Local AI (Ollama / vLLM / LM Studio)",
  name_ar: "الذكاء الاصطناعي المحلي (Ollama / vLLM)",
  category: "ai-provider",
  active: true,
  icon: "Sparkles",
  short_description:
    "Run fully private, offline, self-hosted open-source AI models on your local servers or workstation with zero cloud telemetry.",
  short_description_ar:
    "تشغيل نماذج ذكاء اصطناعي محلية ومستضافة ذاتياً بخصوصية كاملة وبدون أي اعتماد على السحابة.",
  description: `Deploy self-hosted, air-gapped, or on-premise AI models directly with Orbis OS.

**100% Data Privacy & Security**
All prompts, student records, and financial inquiries remain strictly on your local network or server.

**Hardware & Engine Flexibility**
Connect to local Ollama ("http://localhost:11434"), vLLM, LM Studio, or custom inference clusters via standard OpenAI API format.`,
  description_ar: `تشغيل نماذج الذكاء الاصطناعي على خوادمك المحلية أو أجهزتك مباشرة.

**خصوصية بيانات وأمان بنسبة 100%**
تبقى جميع البيانات وسجلات الطلاب والاستفسارات المالية داخل شبكتك المحلية دون مغادرتها.

**مرونة فائقة مع محركات الاستدلال**
دعم كامل للربط مع Ollama و vLLM و LM Studio.`,
  features: [
    "Zero-telemetry local and air-gapped inference",
    "Works with Ollama, LM Studio, and vLLM",
    "Support for Llama 3.3, Qwen 2.5, DeepSeek R1, Mistral",
    "Customizable local endpoint URL",
  ],
  features_ar: [
    "خصوصية مطلقة بدون أي إرسال للبيانات خارجياً",
    "توافق مع Ollama و LM Studio و vLLM",
    "دعم Llama 3.3 و Qwen 2.5 و DeepSeek R1",
    "إمكانية تخصيص عنوان السيرفر المحلي",
  ],
  settings: [
    {
      id: "endpoint_url",
      label: "Local Endpoint URL",
      label_ar: "عنوان نقطة النهاية المحلية",
      description: "URL of your local inference server (e.g. http://localhost:11434).",
      description_ar: "عنوان خادم الذكاء الاصطناعي المحلي (مثال: http://localhost:11434).",
      type: "text",
      required: true,
      value: "http://localhost:11434",
      placeholder: "http://localhost:11434",
    },
    {
      id: "model",
      label: "Default Model",
      label_ar: "النموذج الافتراضي",
      description: "Name of the local model pulled in Ollama / vLLM.",
      description_ar: "اسم النموذج المثبت على السيرفر المحلي.",
      type: "text",
      required: true,
      value: "llama3.3:latest",
      placeholder: "llama3.3:latest, qwen2.5:14b, deepseek-r1:8b",
    },
    {
      id: "api_key",
      label: "API Key / Auth Token (Optional)",
      label_ar: "مفتاح API أو رمز التحقق (اختياري)",
      description: "Optional Bearer token for protected local clusters or proxies.",
      description_ar: "رمز التحقق في حال كان السيرفر المحلي محمياً.",
      type: "password",
      required: false,
      value: "",
    },
    {
      id: "temperature",
      label: "Temperature",
      label_ar: "درجة الحرارة",
      description: "Sampling temperature.",
      description_ar: "درجة الحرارة.",
      type: "number",
      required: false,
      value: 0.2,
      validation: { min: 0, max: 2 },
    },
  ],
  adapter_name: "orbis",
  provider_trait: "LlmProvider",
};

export default config;
