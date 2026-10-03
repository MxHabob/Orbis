import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "microsoft-365",
  name: "Microsoft 365",
  name_ar: "مايكروسوفت 365",
  category: "identity",
  active: true,
  icon: "AppWindow",
  short_description:
    "Sync users, Teams, and enable Microsoft SSO for your organization.",
  short_description_ar:
    "مزامنة المستخدمين وفصول Teams وتفعيل تسجيل الدخول الموحد عبر Microsoft.",
  description: `Integrate with Microsoft 365 for Education and Azure Active Directory (Entra ID) for centralized enterprise identity management.

**Microsoft SSO**
Allow staff and students to authenticate securely using their organization Microsoft accounts.

**Azure AD / Entra ID Sync**
Synchronize users, security groups, and roles directly with Azure Active Directory.

**Teams & Calendar Integration**
Automatically generate Microsoft Teams classes and sync academic timetables to Outlook calendars.`,
  description_ar: `تكامل كامل مع Microsoft 365 للتعليم و Azure AD (Entra ID) لإدارة الهوية الرقمية.

**تسجيل الدخول الموحد (Microsoft SSO)**
تمكين الطلاب والمعلمين والموظفين من تسجيل الدخول الآمن بحسابات مايكروسوفت المؤسسية.

**مزامنة دليل المستخدمين (Azure AD / Entra ID)**
مزامنة الحسابات والمجموعات والصلاحيات تلقائياً مع خوادم مايكروسوفت السحابية.

**تكامل الفصول مع Microsoft Teams**
إنشاء فصول وقنوات Teams آلياً ومزامنة الجداول والتقويم مع Outlook.`,
  features: [
    "Microsoft SSO & Azure AD Login",
    "Azure Active Directory user sync",
    "Microsoft Teams classes sync",
    "Outlook calendar sync",
    "OneDrive document storage integration",
  ],
  features_ar: [
    "تسجيل الدخول الموحد عبر حساب مايكروسوفت",
    "مزامنة المستخدمين مع Azure Active Directory",
    "مزامنة الفصول وقنوات Microsoft Teams",
    "مزامنة التقويم وجداول الحصص مع Outlook",
    "مشاركة المستندات مع OneDrive",
  ],
  settings: [
    {
      id: "tenant_id",
      label: "Azure Tenant ID",
      label_ar: "معرّف المستأجر (Tenant ID)",
      type: "text",
      required: true,
      value: "",
      description:
        "Azure Active Directory / Microsoft Entra Directory (Tenant) ID.",
      description_ar: "معرّف دليل المستأجر في Azure Active Directory.",
      placeholder: "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
    },
    {
      id: "client_id",
      label: "Application (Client) ID",
      label_ar: "معرّف التطبيق (Client ID)",
      type: "text",
      required: true,
      value: "",
      description: "Azure App Registration Application (Client) ID.",
      description_ar: "معرّف تطبيق Azure App Registration.",
      placeholder: "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
    },
    {
      id: "client_secret",
      label: "Client Secret",
      label_ar: "المفتاح السري للعميل (Client Secret)",
      type: "password",
      required: true,
      value: "",
      description: "Azure App Registration client secret.",
      description_ar: "المفتاح السري للعميل المسجل في بوابة Azure.",
    },
    {
      id: "domain",
      label: "Organization Domain",
      label_ar: "نطاق المؤسسة المعتمد",
      type: "text",
      required: false,
      value: "",
      description:
        "Primary Microsoft 365 domain (e.g., school.onmicrosoft.com).",
      description_ar:
        "نطاق البريد المعتمد للمؤسسة (مثال: school.edu أو school.onmicrosoft.com).",
      placeholder: "school.edu.sa",
    },
    {
      id: "enable_sso",
      label: "Enable Microsoft SSO for Organization",
      label_ar: "تفعيل تسجيل الدخول بحساب Microsoft لمنسوبي المؤسسة",
      type: "switch",
      required: false,
      value: true,
      description:
        "Allow users to authenticate using organization Microsoft 365 credentials.",
      description_ar:
        "تمكين المستخدمين من تسجيل الدخول وربط الحسابات ببريد مايكروسوفت المؤسسي.",
    },
    {
      id: "sync_teams",
      label: "Sync Microsoft Teams Classes",
      label_ar: "مزامنة فصول Microsoft Teams",
      type: "switch",
      required: false,
      value: false,
      description:
        "Automatically provision classes and students into Microsoft Teams.",
      description_ar:
        "إنشاء الفصول والمجموعات الدراسية تلقائياً في Microsoft Teams.",
    },
  ],
  requires_oauth: true,
};

export default config;
