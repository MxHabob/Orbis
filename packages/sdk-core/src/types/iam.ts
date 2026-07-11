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
}

export interface UserRole {
  id: string;
  user_id: string;
  role: string;
  org_id: string;
  scope_type: string;
  scope_id?: string | null;
  is_primary: number;
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
}

export interface Permission {
  id: number;
  code: string;
  name: string;
  name_ar?: string | null;
  module: string;
  action: string;
  resource: string;
  is_system: number;
  is_active: number;
}
