import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "groq",
  name: "Groq Cloud",
  name_ar: "جروك كلاود (Groq)",
  category: "ai-provider",
  active: true,
  icon: "Sparkles",
  short_description:
    "Ultra-low latency AI inference running open models (Llama 3.3 70B, DeepSeek R1 Distill) at lightning speed on LPU hardware.",
  short_description_ar:
    "استدلال ذكاء اصطناعي فائق السرعة لنماذج مفتوحة المصدر (Llama 3.3 70B و DeepSeek R1) عبر شرائح LPU.",
  description: `Deploy high-throughput, real-time AI agents on Groq's LPU Inference Engine.

**Ultra-Fast Tokens Per Second**
Experience near-instantaneous responses (500+ tokens/sec) for fluid conversational experiences.

**Top Open Models**
Run Meta Llama 3.3 70B, DeepSeek R1 Distill, and Mistral with native tool support.`,
  description_ar: `شغل وكلاء الذكاء الاصطناعي بسرعات فائقة عبر منصة Groq السحابية.

**أعلى سرعة توليد نصوص**
سرعات تصل إلى أكثر من 500 توكن في الثانية لتجربة محادثة واستدعاء أدوات سلسة وفورية.`,
  features: [
    "Sub-second AI completions via LPU hardware",
    "Llama 3.3 70B Versatile support",
    "DeepSeek R1 Distill reasoning support",
    "OpenAI-compatible wire format",
  ],
  features_ar: [
    "استجابة فورية فائقة السرعة",
    "دعم Llama 3.3 70B و DeepSeek R1",
    "توافق كامل مع معايير OpenAI",
  ],
  settings: [
    {
      id: "api_key",
      label: "API Key",
      label_ar: "مفتاح API",
      description: "Your Groq API key (gsk_...).",
      description_ar: "مفتاح API الخاص بك من منصة GroqCloud.",
      type: "password",
      required: true,
      value: "",
      placeholder: "gsk_...",
    },
    {
      id: "model",
      label: "Default Model",
      label_ar: "النموذج الافتراضي",
      description: "Default Groq model.",
      description_ar: "النموذج الافتراضي على Groq.",
      type: "select",
      required: true,
      value: "llama-3.3-70b-versatile",
      options: [
        { label: "Llama 3.3 70B Versatile (Recommended)", value: "llama-3.3-70b-versatile" },
        { label: "DeepSeek R1 Distill Llama 70B", value: "deepseek-r1-distill-llama-70b" },
        { label: "Llama 3.1 8B Instant (Ultra Fast)", value: "llama-3.1-8b-instant" },
        { label: "Mixtral 8x7B 32k", value: "mixtral-8x7b-32768" },
      ],
    },
    {
      id: "endpoint_url",
      label: "Endpoint URL",
      label_ar: "عنوان نقطة النهاية",
      description: "Groq OpenAI-compatible endpoint.",
      description_ar: "عنوان نقطة نهاية Groq.",
      type: "text",
      required: false,
      value: "https://api.groq.com/openai",
      placeholder: "https://api.groq.com/openai",
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
  adapter_name: "groq",
  provider_trait: "LlmProvider",
};

export default config;
