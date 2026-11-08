import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "solver-service",
  name: "Timetable & Schedule Solver",
  name_ar: "محرك الجداول والجدولة الذكية",
  category: "automation",
  active: true,
  icon: "CalendarRange",
  short_description:
    "Automated timetable and exam schedule generation powered by the Timefold Solver engine.",
  short_description_ar:
    "توليد الجداول الدراسية وجداول الاختبارات تلقائياً باستخدام محرك Timefold المتقدم.",
  description: `The Timetable & Schedule Solver provides automated scheduling optimization for school timetables and examination sessions.

**Timefold Constraint Optimization & Zero-Resource Sidecar**
- In Desktop mode, runs as an automated **Tauri Native Sidecar** requiring zero setup, zero Docker, and zero background resource consumption (launches on-demand and auto-sleeps when idle).
- In Web/Cloud mode, connects seamlessly to any dedicated Solver Service cluster.

**Key Constraint Optimization**
- Complete teacher, classroom, and student class conflict prevention.
- Teacher unavailability and preferred working day constraints.
- Distribution of subject weekly hours and break slot avoidance.
- Exam conflict group separation and daily exam limits.`,
  description_ar: `يقوم محرك الجداول والجدولة الذكية بتوليد الجداول الدراسية وجداول الاختبارات تلقائياً عبر خدمة Timefold Solver المتخصصة.

**محرك Timefold مع إدارة الموارد الذكية (Zero-Resource Sidecar)**
- في تطبيق الديسكتوب: يعمل تلقائياً كـ **Tauri Sidecar مدمج** بدون أي إعداد أو Docker، وبدون استهلاك موارد في الخلفية (يبدأ عند الطلب ويتوقف تلقائياً عند الخمول).
- في وضع السحابة / الويب: يتصل مباشرة بأي خادم جدولة خارجي بكل مرونة.

**القيود وقواعد الجدولة المعتمدة**
- منع تعارض حصص المعلمين، القاعات، والفصول الدراسية بالكامل.
- مراعاة فترات عدم توفر المعلمين والأيام المفضلة.
- توزيع الحصص الأسبوعية بالتساوي وتجنب حصص الراحة والفسح.
- منع تعارض مجموعات الاختبارات وتحديد الحد الأقصى لاختبارات اليوم الواحد.`,
  features: [
    "Native Tauri Embedded Sidecar (Zero Docker & Zero Setup)",
    "On-Demand Lifecycle (0% Idle CPU & Auto-Sleep)",
    "Dedicated Timefold Constraint Optimization",
    "Teacher, Room, and Class conflict prevention",
    "Teacher availability & preferred slots",
    "Exam scheduling & conflict groups",
  ],
  features_ar: [
    "تشغيل مدمج تلقائياً كـ Tauri Sidecar بدون Docker",
    "إدارة استهلاك الموارد (إيقاف تلقائي عند الخمول وتوفير الذاكرة)",
    "محرك Timefold لحل القيود وجدولة الحصص بكفاءة عالية",
    "منع تعارض المعلمين، القاعات، والفصول الدراسية",
    "مراعاة أوقات توفر المعلمين وساعات العمل",
    "جدولة الاختبارات وتوزيع مجموعات التعارض",
  ],
  settings: [
    {
      id: "mode",
      label: "Deployment Mode",
      label_ar: "نمط التشغيل والربط",
      description:
        "Choose between embedded desktop sidecar or external server cluster.",
      description_ar:
        "اختر بين التشغيل المدمج في الديسكتوب (Sidecar) أو خادم خارجي.",
      type: "select",
      required: true,
      value: "embedded",
      options: [
        {
          label: "Embedded Desktop Sidecar (Auto On-Demand)",
          value: "embedded",
        },
        { label: "Remote Solver Cluster (Custom URL)", value: "remote" },
      ],
    },
    {
      id: "endpoint_url",
      label: "Solver Service URL",
      label_ar: "رابط خدمة الجدولة (Solver URL)",
      description: "Endpoint URL (Default: http://127.0.0.1:8081).",
      description_ar: "عنوان الخدمة (الافتراضي: http://127.0.0.1:8081).",
      type: "url",
      required: true,
      value: "http://127.0.0.1:8081",
      placeholder: "http://127.0.0.1:8081",
    },
    {
      id: "idle_timeout_minutes",
      label: "Auto-Sleep Idle Timeout (Minutes)",
      label_ar: "مهلة الإيقاف التلقائي عند الخمول (دقائق)",
      description:
        "Automatically stops the desktop sidecar when idle to free RAM & CPU.",
      description_ar:
        "إيقاف خدمة الديسكتوب تلقائياً عند عدم الاستخدام لتحرير موارد الجهاز.",
      type: "number",
      required: false,
      value: 5,
    },
    {
      id: "timeout_seconds",
      label: "Solving Timeout (Seconds)",
      label_ar: "المهلة الزمنية للحل (بالثواني)",
      description:
        "Maximum time allowed for solving before returning the best solution found.",
      description_ar:
        "أقصى وقت مسموح به لحساب وتوليد الجدول قبل اعتماد أفضل حل.",
      type: "number",
      required: false,
      value: 30,
    },
  ],
  adapter_name: "solver_service",
  provider_trait: "SchedulingSolverPort",
};

export default config;
