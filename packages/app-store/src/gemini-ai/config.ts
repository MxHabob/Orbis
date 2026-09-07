import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "gemini-ai",
  name: "Google Gemini",
  name_ar: "جوجل جيميناي",
  category: "ai-provider",
  active: true,
  icon: "Sparkles",
  short_description:
    "Connect Google Gemini as your cloud AI provider for advanced reasoning and analysis.",
  short_description_ar:
    "ربط جوجل جيميناي كمزود ذكاء اصطناعي سحابي للتحليل والاستدلال المتقدم.",
  description: `Use Google Gemini alongside or instead of Ollama for cloud-powered AI capabilities.

**Advanced Reasoning**
Gemini excels at complex multi-step reasoning — ideal for academic policy analysis and financial auditing.

**Function Calling**
Full support for tool calling — agents can execute all 28+ registered actions across 7 domains.

**Multilingual Excellence**
Superior Arabic and English support for prompts, responses, and educational content generation.

**Vision Support**
Analyze images and documents — useful for OCR, document verification, and visual inspections.`,
  description_ar: `استخدم جوجل جيميناي بجانب أو بدلاً من Ollama لقدرات ذكاء اصطناعي سحابية.

**استدلال متقدم**
جيميناي يتفوق في الاستدلال متعدد الخطوات — مثالي لتحليل السياسات الأكاديمية.

**دعم ثنائي اللغة**
دعم ممتاز للعربية والإنجليزية في المطالبات والاستجابات.`,
  features: [
    "Cloud-powered advanced AI reasoning",
    "Full function/tool calling support",
    "Superior Arabic language support",
    "Vision/image analysis capabilities",
    "Multiple model variants (Flash, Pro)",
    "Automatic fallback when Ollama is unavailable",
  ],
  settings: [
    {
      id: "api_key",
      label: "API Key",
      label_ar: "مفتاح API",
      description: "Your Google Gemini API key from Google AI Studio.",
      description_ar: "مفتاح API الخاص بك من Google AI Studio.",
      type: "password",
      required: true,
      value: "",
      placeholder: "AIza...",
    },
    {
      id: "model",
      label: "Default Model",
      label_ar: "النموذج الافتراضي",
      description: "The default Gemini model to use.",
      type: "select",
      required: true,
      value: "gemini-3.8-flash",
      options: [
        { label: "Gemini 3.8 Flash", value: "gemini-3.8-flash" },
        { label: "Gemini 3.7 Flash", value: "gemini-3.7-flash" },
        { label: "Gemini 3.5 Flash", value: "gemini-3.5-flash" },
        { label: "Gemini 3.5 Flash-Lite", value: "gemini-3.5-flash-lite" },
        { label: "Gemini 2.5 Flash", value: "gemini-2.5-flash" },
        { label: "Gemini 2.5 Flash-Lite", value: "gemini-2.5-flash-lite" },
      ],
    },
    {
      id: "temperature",
      label: "Temperature",
      label_ar: "درجة الحرارة",
      description: "Controls randomness (0.0 = deterministic, 1.0 = creative).",
      type: "number",
      required: false,
      value: 0.7,
      validation: { min: 0, max: 2 },
    },
  ],
  adapter_name: "gemini",
  provider_trait: "LlmProvider",
};

export default config;
