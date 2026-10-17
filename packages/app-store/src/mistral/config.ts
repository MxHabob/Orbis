import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "mistral",
  name: "Mistral AI",
  name_ar: "ميسترال AI",
  category: "ai-provider",
  active: true,
  icon: "Sparkles",
  short_description:
    "Connect Mistral Large, Mistral Small, and Codestral for sovereign, efficient European enterprise AI models.",
  short_description_ar:
    "ربط نماذج Mistral Large و Mistral Small لنماذج ذكاء اصطناعي مؤسسية عالية الكفاءة.",
  description: `Deploy Mistral AI's premier language and code generation models inside Orbis OS.

**Mistral Large**
Top-tier reasoning, multilingual proficiency, and strict JSON output capabilities.

**Codestral & Pixtral**
Specialized models for code generation, mathematical logic, and document vision comprehension.`,
  description_ar: `ادمج نماذج Mistral AI المتقدمة في منصة Orbis.

**Mistral Large**
استدلال متقدم ودعم لغات متعددة وتوافق عالي مع مخرجات JSON المنظمة.`,
  features: [
    "Mistral Large 2 (Flagship frontier model)",
    "Mistral Small & NeMo support",
    "Codestral & Pixtral multimodal models",
    "OpenAI-compatible endpoints",
  ],
  features_ar: [
    "نموذج Mistral Large الرائد",
    "نماذج Mistral Small و Codestral و Pixtral",
    "توافق كامل مع واجهات OpenAI",
  ],
  settings: [
    {
      id: "api_key",
      label: "API Key",
      label_ar: "مفتاح API",
      description: "Your Mistral API key from console.mistral.ai.",
      description_ar: "مفتاح API الخاص بك من منصة Mistral Console.",
      type: "password",
      required: true,
      value: "",
      placeholder: "...",
    },
    {
      id: "model",
      label: "Default Model",
      label_ar: "النموذج الافتراضي",
      description: "Default Mistral model.",
      description_ar: "النموذج الافتراضي.",
      type: "select",
      required: true,
      value: "",
    },
    {
      id: "endpoint_url",
      label: "Endpoint URL",
      label_ar: "عنوان نقطة النهاية",
      description: "Mistral API base URL.",
      description_ar: "عنوان API الخاص بـ Mistral.",
      type: "text",
      required: false,
      value: "https://api.mistral.ai",
      placeholder: "https://api.mistral.ai",
    },
  ],
  adapter_name: "mistral",
  provider_trait: "LlmProvider",
};

export default config;
