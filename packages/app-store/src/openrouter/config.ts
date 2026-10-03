import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "openrouter",
  name: "OpenRouter",
  name_ar: "أوبن راوتر (OpenRouter)",
  category: "ai-provider",
  active: true,
  icon: "Sparkles",
  short_description:
    "Unified gateway to 200+ AI models from OpenAI, Anthropic, Google, Meta, Mistral, and DeepSeek via a single API key.",
  short_description_ar:
    "بوابة موحدة للوصول إلى أكثر من 200 نموذج ذكاء اصطناعي بمفتاح API واحد.",
  description: `Route requests dynamically to any model in the global AI ecosystem with fallback routing and automatic load balancing.`,
  description_ar: `توجيه طلبات الذكاء الاصطناعي إلى أي نموذج عالمي مع دعم التبديل التلقائي وتوزيع الأحمال.`,
  features: [
    "Access to 200+ models via one API key",
    "Automatic failover and rate limit routing",
    "Standardized OpenAI-compatible schema",
  ],
  features_ar: [
    "الوصول لأكثر من 200 نموذج بمفتاح واحد",
    "تبديل تلقائي وتجاوز لحدود الاستخدام",
    "توافق كامل مع معايير OpenAI",
  ],
  settings: [
    {
      id: "api_key",
      label: "API Key",
      label_ar: "مفتاح API",
      description: "Your OpenRouter API key (sk-or-...).",
      description_ar: "مفتاح API الخاص بك من openrouter.ai.",
      type: "password",
      required: true,
      value: "",
      placeholder: "sk-or-...",
    },
    {
      id: "model",
      label: "Default Model",
      label_ar: "النموذج الافتراضي",
      description: "Model identifier on OpenRouter.",
      description_ar: "معرف النموذج في OpenRouter.",
      type: "select",
      required: true,
      value: "openai/gpt-4o",
      options: [
        { label: "OpenAI GPT-4o", value: "openai/gpt-4o" },
        { label: "OpenAI GPT-4o mini", value: "openai/gpt-4o-mini" },
        { label: "Claude 3.7 Sonnet (Anthropic)", value: "anthropic/claude-3.7-sonnet" },
        { label: "Claude 3.5 Sonnet (Anthropic)", value: "anthropic/claude-3.5-sonnet" },
        { label: "Claude 3.5 Haiku (Anthropic)", value: "anthropic/claude-3.5-haiku" },
        { label: "DeepSeek R1 (Reasoning)", value: "deepseek/deepseek-r1" },
        { label: "DeepSeek V3 (Chat)", value: "deepseek/deepseek-chat" },
        { label: "Google Gemini 2.5 Flash", value: "google/gemini-2.5-flash" },
        { label: "Google Gemini 2.5 Pro", value: "google/gemini-2.5-pro" },
        { label: "Llama 3.3 70B (Meta)", value: "meta-llama/llama-3.3-70b-instruct" },
        { label: "Mistral Large", value: "mistralai/mistral-large" },
      ],
    },
    {
      id: "endpoint_url",
      label: "Endpoint URL",
      label_ar: "عنوان نقطة النهاية",
      description: "OpenRouter endpoint URL.",
      description_ar: "عنوان نقطة نهاية OpenRouter.",
      type: "text",
      required: false,
      value: "https://openrouter.ai/api/v1",
      placeholder: "https://openrouter.ai/api/v1",
    },
  ],
  adapter_name: "openrouter",
  provider_trait: "LlmProvider",
};

export default config;
