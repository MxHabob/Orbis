import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "mulverse-cloud",
  name: "Mulverse Cloud",
  name_ar: "سحابة مالفيرس",
  category: "storage",
  active: false,
  icon: "Cloud",
  short_description:
    "Secure, reliable, and scalable cloud storage solutions developed by Mulverse for student records and file backups.",
  short_description_ar:
    "حلول تخزين سحابية آمنة وموثوقة وقابلة للتوسع مطورة من قبل Mulverse لملفات الطلاب والنسخ الاحتياطي.",
  description: `Mulverse Cloud is a secure cloud storage provider that integrates directly with Orbis. Store, share, and backup academic documents, school media, and biometric templates with enterprise-grade encryption.

**Developed by Mulverse**
Designed specifically to handle high-volume assets from [Mulverse](https://www.mulverse.com/) and Orbis native applications.

**High-Speed Cloud Backups**
Configure automatic backup schedules for student records, database snapshots, and application logs.

**Secure & Compliant**
Uses state-of-the-art encryption protocols for data at rest and in transit.`,
  description_ar: `Mulverse Cloud هو مزود تخزين سحابي آمن يتكامل مباشرة مع Orbis. قم بتخزين ومشاركة المستندات الأكاديمية ووسائط المدرسة والنسخ الاحتياطي مع تشفير على مستوى المؤسسات.

**مطور بواسطة Mulverse**
تم تصميمه خصيصاً للتعامل مع الأصول والملفات كبيرة الحجم من تطبيقات [Mulverse](https://www.mulverse.com/) وتطبيقات Orbis الأصلية.

**نسخ احتياطي سحابي عالي السرعة**
تكوين جداول النسخ الاحتياطي التلقائي لسجلات الطلاب ولقطات قاعدة البيانات وسجلات التطبيقات.

**آمن ومتوافق**
يستخدم أحدث بروتوكولات التشفير للبيانات المخزنة وأثناء النقل.`,
  features: [
    "High-speed cloud backups",
    "Enterprise-grade encryption",
    "Developed by Mulverse",
    "99.9% uptime SLA",
    "Automated folder structures",
  ],
  features_ar: [
    "نسخ احتياطي سحابي عالي السرعة",
    "تشفير على مستوى المؤسسات",
    "مطور من قبل Mulverse",
    "ضمان تشغيل بنسبة 99.9%",
    "تنظيم تلقائي للمجلدات",
  ],
  settings: [
    {
      id: "bucket",
      label: "Bucket Name",
      label_ar: "اسم الحاوية",
      description: "Storage bucket or container name.",
      description_ar: "اسم حاوية التخزين السحابي.",
      type: "text",
      required: true,
      value: "",
      placeholder: "mulverse-school-docs",
    },
    {
      id: "access_key",
      label: "Access Key",
      label_ar: "مفتاح الوصول",
      description: "Mulverse Cloud access key ID.",
      description_ar: "معرف مفتاح الوصول الخاص بـ Mulverse Cloud.",
      type: "text",
      required: true,
      value: "",
    },
    {
      id: "secret_key",
      label: "Secret Key",
      label_ar: "المفتاح السري",
      description: "Mulverse Cloud secret access key.",
      description_ar: "المفتاح السري للوصول الخاص بـ Mulverse Cloud.",
      type: "password",
      required: true,
      value: "",
    },
  ],
  adapter_name: "mulverse_storage",
  provider_trait: "StorageProvider",
};

export default config;
