import type { OrbisApp } from "../types";

const config: OrbisApp = {
  id: "ollama-ai",
  name: "Ollama",
  name_ar: "أولاما",
  category: "ai-provider",
  active: true,
  icon: "Brain",
  short_description:
    "Run AI models locally with Ollama — no internet dependency, full data privacy.",
  short_description_ar:
    "تشغيل نماذج الذكاء الاصطناعي محلياً عبر Ollama — بدون إنترنت، خصوصية كاملة.",
  description: `Ollama is the default local-first AI provider for Orbis (ADQ-3 architecture).

**Local-First Privacy**
All AI processing happens on your machine. Student data never leaves your network — perfect for schools with strict data privacy requirements.

**Supported Models**
Run Llama 3.2, Mistral, Phi-3, and other open-source models. Switch models without changing any configuration.

**Function Calling**
Full tool-calling support — AI agents can execute 28+ registered actions across 7 domains (academic, finance, scheduling, etc.).

**Embeddings**
Generate text embeddings for semantic search and context retrieval using local models.`,
  description_ar: `أولاما هو مزود الذكاء الاصطناعي المحلي الافتراضي لنظام Orbis.

**خصوصية محلية أولاً**
جميع عمليات الذكاء الاصطناعي تتم على جهازك. بيانات الطلاب لا تغادر شبكتك أبداً.

**النماذج المدعومة**
تشغيل Llama 3.2, Mistral, Phi-3 وغيرها من النماذج مفتوحة المصدر.

**استدعاء الأدوات**
دعم كامل لاستدعاء الأدوات — الوكلاء يمكنهم تنفيذ 28+ إجراء عبر 7 مجالات.`,
  features: [
    "Local-first AI — no internet required",
    "Full data privacy for student records",
    "Tool/function calling support",
    "Text embeddings for semantic search",
    "Multiple model support (Llama, Mistral, Phi)",
    "Health check monitoring",
  ],
  features_ar: [
    "ذكاء اصطناعي محلي — بدون إنترنت",
    "خصوصية كاملة لسجلات الطلاب",
    "دعم استدعاء الأدوات",
    "تضمينات نصية للبحث الدلالي",
    "دعم نماذج متعددة",
    "مراقبة حالة الاتصال",
  ],
  settings: [
    {
      id: "base_url",
      label: "Ollama URL",
      label_ar: "عنوان Ollama",
      description: "The base URL where Ollama is running.",
      description_ar: "العنوان الأساسي حيث يعمل Ollama.",
      type: "url",
      required: true,
      value: "http://localhost:11434",
      placeholder: "http://localhost:11434",
    },
    {
      id: "model",
      label: "Default Model",
      label_ar: "النموذج الافتراضي",
      description: "The default model to use for AI agents.",
      description_ar: "النموذج الافتراضي المستخدم للوكلاء.",
      type: "select",
      required: true,
      value: "llama3.2",
      options: [
        { label: "Llama 3.2", value: "llama3.2" },
        { label: "Llama 3.1 8B", value: "llama3.1:8b" },
        { label: "Mistral 7B", value: "mistral" },
        { label: "Phi-3 Mini", value: "phi3:mini" },
        { label: "Gemma 2 9B", value: "gemma2:9b" },
      ],
    },
    {
      id: "temperature",
      label: "Temperature",
      label_ar: "درجة الحرارة",
      description: "Controls randomness in responses (0.0 = deterministic, 1.0 = creative).",
      type: "number",
      required: false,
      value: 0.7,
      validation: { min: 0, max: 2 },
    },
    {
      id: "max_tokens",
      label: "Max Tokens",
      label_ar: "الحد الأقصى للرموز",
      description: "Maximum number of tokens in AI responses.",
      type: "number",
      required: false,
      value: 2048,
      validation: { min: 256, max: 8192 },
    },
  ],
  adapter_name: "ollama",
  provider_trait: "LlmProvider",
};

export default config;
