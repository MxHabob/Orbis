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

**Timefold Constraint Optimization**
Powered by the dedicated Timefold solver service to resolve complex NP-hard school timetables with high efficiency and score calculation.

**Key Constraint Optimization**
- Complete teacher, classroom, and student class conflict prevention.
- Teacher unavailability and preferred working day constraints.
- Distribution of subject weekly hours and break slot avoidance.
- Exam conflict group separation and daily exam limits.`,
  description_ar: `يقوم محرك الجداول والجدولة الذكية بتوليد الجداول الدراسية وجداول الاختبارات تلقائياً عبر خدمة Timefold Solver المتخصصة.

**محرك Timefold لحل القيود المعقدة**
يعتمد على خدمة الجدولة لمعالجة مشاكل الجداول الكبيرة والتوزيع العادل للحصص مع منع التعارضات بدقة متناهية.

**القيود وقواعد الجدولة المعتمدة**
- منع تعارض حصص المعلمين، القاعات، والفصول الدراسية بالكامل.
- مراعاة فترات عدم توفر المعلمين والأيام المفضلة.
- توزيع الحصص الأسبوعية بالتساوي وتجنب حصص الراحة والفسح.
- منع تعارض مجموعات الاختبارات وتحديد الحد الأقصى لاختبارات اليوم الواحد.`,
  features: [
    "Dedicated Timefold Constraint Optimization",
    "Teacher, Room, and Class conflict prevention",
    "Teacher availability & preferred slots",
    "Exam scheduling & conflict groups",
    "Incremental & Full timetable generation modes",
  ],
  features_ar: [
    "محرك Timefold لحل القيود وجدولة الحصص بكفاءة عالية",
    "منع تعارض المعلمين، القاعات، والفصول الدراسية",
    "مراعاة أوقات توفر المعلمين وساعات العمل",
    "جدولة الاختبارات وتوزيع مجموعات التعارض",
    "أوضاع توليد الجدول بالكامل أو ملء الحصص المفقودة فقط",
  ],
  settings: [
    {
      id: "endpoint_url",
      label: "Solver Service URL",
      label_ar: "رابط خدمة الجدولة (Solver URL)",
      description: "Endpoint URL of the running solver-service application.",
      description_ar: "عنوان تشغيل خدمة الجدولة solver-service.",
      type: "url",
      required: true,
      value: "http://localhost:8081",
      placeholder: "http://localhost:8081",
    },
    {
      id: "timeout_seconds",
      label: "Solving Timeout (Seconds)",
      label_ar: "المهلة الزمنية للحل (بالثواني)",
      description: "Maximum time allowed for solving before returning the best solution found.",
      description_ar: "أقصى وقت مسموح به لحساب وتوليد الجدول قبل اعتماد أفضل حل.",
      type: "number",
      required: false,
      value: 30,
    },
  ],
  adapter_name: "solver_service",
  provider_trait: "SchedulingSolverPort",
};

export default config;
