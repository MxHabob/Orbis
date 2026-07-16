/** IAM domain types. */

export interface User {
  id: string;
  org_id: string;
  email?: string | null;
  phone?: string | null;
  full_name: string;
  name_ar?: string | null;
  name_en?: string | null;
  avatar?: string | null;
  sex?: string | null;
  date_of_birth?: string | null;
  nationality?: string | null;
  national_id?: string | null;
  primary_role: string;
  status: string;
  last_login_at?: string | null;
  prefs: string;
  meta: string;
  is_deleted: number;
  created_at: string;
  updated_at: string;
  role?: string;
}

export interface UserRole {
  id: string;
  user_id: string;
  role: string;
  org_id: string;
  scope_type: string;
  scope_id?: string;
  valid_from?: string;
  valid_until?: string;
  is_primary: boolean;
  assigned_by?: string;
  assigned_at: string;
}

export interface UserSession {
  id: string;
  user_id: string;
  org_id: string;
  token: string;
  device_name?: string | null;
  is_active: number;
  expires_at: string;
  last_activity_at: string;
  created_at: string;
  is_current?: number;
  last_active_at?: string | null;
  ip_address?: string | null;
  device_type?: string | null;
  user_agent?: string | null;
  browser?: string | null;
  os?: string | null;
}

export interface Permission {
  id: number;
  code: string;
  name: string;
  name_ar?: string;
  description?: string;
  module: string;
  action: string;
  resource: string;
  requires_scope: boolean;
  scope_types?: Record<string, any>;
  is_system: boolean;
  is_active: boolean;
  created_at: string;
}

export interface Person {
  party_id: string;
  first_name: string;
  last_name: string;
  email?: string;
  phone?: string;
  national_id?: string;
  created_at: string;
  updated_at: string;
}
