import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "google-workspace",
  name: "Google Workspace",
  name_ar: "جوجل وورك سبيس",
  category: "identity",
  active: false,
  icon: "Chrome",
  short_description: "Sync users, classes, and calendars with Google Workspace for Education.",
  short_description_ar: "مزامنة المستخدمين والفصول والتقويمات مع Google Workspace للتعليم.",
  description: `Integrate with Google Workspace for Education to sync your school directory.

**User Provisioning**
Automatically create and manage Google accounts for students and staff based on Orbis enrollment data.

**Classroom Sync**
Sync Orbis classes with Google Classroom for seamless assignment management.

**Calendar Integration**
Push school events and exam schedules to Google Calendar.`,
  features: ["User provisioning from enrollment", "Google Classroom sync", "Calendar event sync", "SSO authentication", "Drive document sharing"],
  settings: [
    { id: "domain", label: "Google Domain", type: "text", required: true, value: "", description: "Your Google Workspace domain.", placeholder: "school.edu" },
    { id: "admin_email", label: "Admin Email", type: "text", required: true, value: "", description: "Super admin email for API access.", placeholder: "admin@school.edu" },
    { id: "service_account_key", label: "Service Account Key (JSON)", type: "password", required: true, value: "", description: "Google Cloud service account key in JSON format." },
  ],
  requires_oauth: true,
};

export default config;
