import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "orbis-ai",
  name: "Orbis AI",
  name_ar: "أوربيس AI",
  category: "ai-provider",
  active: true,
  icon: "Sparkles",
  short_description:
    "The native intelligent core AI of the Orbis platform, developed by Mulverse.",
  short_description_ar:
    "الذكاء الاصطناعي الذكي الأصلي لمنصة Orbis، والمطور من قبل Mulverse.",
  description: `Orbis AI is the native artificial intelligence built specifically for this platform. Developed by [Mulverse](https://www.mulverse.com/), it powers intelligent agentic workflows, natural language commands, scheduling assistants, and automated administrative operations.

**Native Platform Integration**
Built from the ground up to integrate with Orbis's academic, financial, and scheduling systems.

**Agentic Workflows**
Fully optimized for complex tool-calling and autonomous agent operations across all school systems.

**Developed by Mulverse**
Leverages the robust infrastructure of the Mulverse Network for advanced reasoning and low-latency API access.`,
  description_ar: `أوربيس AI هو الذكاء الاصطناعي الأصلي المصمم خصيصاً لهذه المنصة. تم تطويره بواسطة [Mulverse](https://www.mulverse.com/)، وهو يدعم تدفقات العمل الذكية القائمة على الوكلاء، والأوامر باللغة الطبيعية، ومساعدي الجدولة، والعمليات الإدارية المؤتمتة.

**تكامل أصلي مع المنصة**
تم بناؤه من الأساس ليتكامل مع الأنظمة الأكاديمية والمالية وأنظمة الجدولة في Orbis.

**تدفقات عمل الوكلاء**
مهيأ بالكامل لعمليات استدعاء الأدوات المعقدة وعمليات الوكلاء المستقلين عبر جميع أنظمة المدرسة.

**مطور بواسطة Mulverse**
يستفيد من البنية التحتية القوية لشبكة Mulverse للتحليل المتقدم والوصول بزمن استجابة منخفض.`,
  features: [
    "Native Orbis platform AI core",
    "Developed by Mulverse",
    "Autonomous agentic workflow execution",
    "Dual language support (English & Arabic)",
    "Optimized function/tool calling",
  ],
  features_ar: [
    "النواة الذكية الأصلية لمنصة Orbis",
    "مطور من قبل Mulverse",
    "تنفيذ تدفقات عمل الوكلاء المستقلين",
    "دعم مزدوج للغتين (العربية والإنجليزية)",
    "استدعاء محسّن للوظائف والأدوات",
  ],
  settings: [
    {
      id: "api_key",
      label: "Mulverse API Key",
      label_ar: "مفتاح API الخاص بـ Mulverse",
      description:
        "Your Mulverse Cloud API key to authorize Orbis AI requests.",
      description_ar:
        "مفتاح API الخاص بـ Mulverse Cloud لتفويض طلبات Orbis AI.",
      type: "password",
      required: true,
      value: "",
    },
    {
      id: "model",
      label: "Model Version",
      label_ar: "إصدار النموذج",
      description: "Choose the version of Orbis AI to deploy.",
      description_ar: "اختر إصدار أوربيس AI لتشغيله.",
      type: "select",
      required: true,
      value: "orbis-large-v1",
      options: [
        { label: "Orbis Large V1", value: "orbis-large-v1" },
        { label: "Orbis Chat Pro", value: "orbis-chat-pro" },
      ],
    },
  ],
  adapter_name: "orbis_ai",
  provider_trait: "LlmProvider",
};

export default config;
