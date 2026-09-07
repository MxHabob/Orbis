import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "google-workspace",
  name: "Google Workspace",
  name_ar: "جوجل وورك سبيس",
  category: "identity",
  active: true,
  icon: "Chrome",
  short_description: "Sync users, classes, and enable Google SSO for your organization.",
  short_description_ar: "مزامنة المستخدمين والفصول وتفعيل تسجيل الدخول الموحد عبر حساب Google للمؤسسة.",
  description: `Integrate with Google Workspace for Education to manage enterprise authentication and directory synchronization.

**SSO & Google Login**
Allow staff, teachers, and students to log in to Orbis with their official school Google accounts.

**User Provisioning**
Automatically create and manage Google accounts based on Orbis student and employee rosters.

**Classroom & Calendar Sync**
Sync Orbis classes and timetables directly with Google Classroom and Google Calendar.`,
  description_ar: `تكامل كامل مع Google Workspace للتعليم لإدارة الهوية الرقمية ومزامنة الحسابات.

**تسجيل الدخول الموحد (Google SSO)**
تمكين المعلمين والطلاب والموظفين من تسجيل الدخول إلى النظام باستخدام بريدهم المدرسي/المؤسسي المعتمد.

**مزامنة المستخدمين (Directory Provisioning)**
إنشاء وإدارة حسابات Google تلقائياً بناءً على بيانات القبول والتسجيل في النظام.

**مزامنة الفصول والتقويم**
ربط الفصول الدراسية والواجبات مباشرة مع Google Classroom وجداول الحصص مع Google Calendar.`,
  features: [
    "Google SSO & Identity Linking",
    "Allowed Domain Restriction",
    "User provisioning from enrollment",
    "Google Classroom sync",
    "Calendar event sync",
  ],
  features_ar: [
    "تسجيل الدخول الموحد وربط الهوية عبر Google",
    "تقييد الدخول بنطاق المؤسسة المعتمد",
    "مزامنة بيانات الطلاب والموظفين",
    "تكامل الفصول مع Google Classroom",
    "مزامنة التقويم وجداول الحصص",
  ],
  settings: [
    {
      id: "domain",
      label: "Organization Domain",
      label_ar: "نطاق المؤسسة المعتمد",
      type: "text",
      required: true,
      value: "",
      description: "Primary Google Workspace domain (e.g., school.edu.sa).",
      description_ar: "نطاق البريد الإلكتروني المعتمد للمؤسسة (مثال: school.edu.sa).",
      placeholder: "school.edu.sa",
    },
    {
      id: "client_id",
      label: "OAuth Client ID",
      label_ar: "معرّف العميل (Client ID)",
      type: "text",
      required: false,
      value: "",
      description: "Google Cloud OAuth 2.0 Web Client ID for organization SSO.",
      description_ar: "معرّف عميل OAuth 2.0 من Google Cloud لتفعيل الدخول الموحد.",
      placeholder: "xxxxxx.apps.googleusercontent.com",
    },
    {
      id: "client_secret",
      label: "OAuth Client Secret",
      label_ar: "المفتاح السري للعميل (Client Secret)",
      type: "password",
      required: false,
      value: "",
      description: "Google Cloud OAuth client secret.",
      description_ar: "المفتاح السري لعميل OAuth من منصة Google Cloud.",
    },
    {
      id: "enable_sso",
      label: "Enable Google SSO for Organization",
      label_ar: "تفعيل تسجيل الدخول بحساب Google لمنسوبي المؤسسة",
      type: "switch",
      required: false,
      value: true,
      description: "Allow users to log in and link their account using the organization Google domain.",
      description_ar: "السماح للمستخدمين بتسجيل الدخول وربط حساباتهم باستخدام بريد المؤسسة الرسمي.",
    },
    {
      id: "sync_classroom",
      label: "Sync Google Classroom",
      label_ar: "مزامنة Google Classroom",
      type: "switch",
      required: false,
      value: false,
      description: "Automatically synchronize classes and courses with Google Classroom.",
      description_ar: "مزامنة الفصول الدراسية والمقررات تلقائياً مع Google Classroom.",
    },
    {
      id: "admin_email",
      label: "Super Admin Email",
      label_ar: "بريد المدير العام (Super Admin)",
      type: "text",
      required: false,
      value: "",
      description: "Google Workspace super admin email for API directory access.",
      description_ar: "البريد الإلكتروني لمدير النظام بصلاحيات Super Admin لإدارة المزامنة.",
      placeholder: "admin@school.edu.sa",
    },
  ],
  requires_oauth: true,
};

export default config;
