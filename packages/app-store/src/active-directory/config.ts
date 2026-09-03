import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "active-directory",
  name: "Active Directory",
  name_ar: "الدليل النشط",
  category: "identity",
  active: false,
  icon: "ServerCog",
  short_description: "Sync users with on-premises Active Directory or LDAP server.",
  short_description_ar: "مزامنة المستخدمين مع خادم Active Directory أو LDAP المحلي.",
  description: `Connect to your on-premises Active Directory or LDAP server for centralized user management.

**User Sync**
Import and sync users from AD/LDAP to Orbis. Map AD groups to Orbis roles automatically.

**SSO Authentication**
Allow staff to log into Orbis using their existing AD credentials.`,
  features: ["LDAP/AD user import", "Group-to-role mapping", "SSO authentication", "Automatic user provisioning", "Password sync"],
  settings: [
    { id: "server_url", label: "LDAP Server URL", type: "url", required: true, value: "", description: "LDAP/AD server URL.", placeholder: "ldap://192.168.1.10:389" },
    { id: "bind_dn", label: "Bind DN", type: "text", required: true, value: "", description: "Distinguished name for LDAP bind.", placeholder: "CN=admin,DC=school,DC=local" },
    { id: "bind_password", label: "Bind Password", type: "password", required: true, value: "", description: "Password for LDAP bind." },
    { id: "base_dn", label: "Base DN", type: "text", required: true, value: "", description: "Base DN for user search.", placeholder: "OU=Users,DC=school,DC=local" },
  ],
};

export default config;
