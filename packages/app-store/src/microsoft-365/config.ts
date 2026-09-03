import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "microsoft-365",
  name: "Microsoft 365",
  name_ar: "مايكروسوفت 365",
  category: "identity",
  active: false,
  icon: "AppWindow",
  short_description: "Sync users and calendars with Microsoft 365 and Azure Active Directory.",
  short_description_ar: "مزامنة المستخدمين والتقويمات مع مايكروسوفت 365.",
  description: `Integrate with Microsoft 365 for Education to manage your school's digital identity.

**Azure AD Sync**
Sync users from Orbis to Azure Active Directory for centralized identity management.

**Teams Integration**
Create and manage Teams classes based on Orbis class structure.

**Outlook Calendar**
Push school events and schedules to Outlook calendars.`,
  features: ["Azure AD user sync", "Microsoft Teams classes", "Outlook calendar sync", "SSO via Azure AD", "OneDrive document sharing"],
  settings: [
    { id: "tenant_id", label: "Azure Tenant ID", type: "text", required: true, value: "", description: "Your Azure AD tenant identifier." },
    { id: "client_id", label: "Client ID", type: "text", required: true, value: "", description: "Azure app registration client ID." },
    { id: "client_secret", label: "Client Secret", type: "password", required: true, value: "", description: "Azure app registration client secret." },
  ],
  requires_oauth: true,
};

export default config;
