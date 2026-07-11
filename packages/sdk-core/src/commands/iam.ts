/**
 * IAM command interfaces — mirrors svc-iam/ops.
 */
import type { User, UserRole, UserSession } from "../types";

export interface IAMCommands {
  // ── Auth ────────────────────────────────────────────────────
  login(identifier: string, passwordHash: string, ip?: string): Promise<{ user: User; session: UserSession }>;
  logout(sessionId: string): Promise<void>;
  changePassword(userId: string, newPasswordHash: string): Promise<void>;
  listSessions(userId: string): Promise<UserSession[]>;

  // ── Users ───────────────────────────────────────────────────
  listUsers(role?: string, status?: string): Promise<User[]>;
  getUser(userId: string): Promise<User>;
  searchUsers(query: string): Promise<User[]>;
  createUser(data: { full_name: string; name_ar?: string; email?: string; phone?: string; primary_role: string; password_hash?: string }): Promise<User>;
  updateUser(userId: string, data: { full_name?: string; name_ar?: string; email?: string; phone?: string; status?: string }): Promise<User>;
  deleteUser(userId: string): Promise<void>;

  // ── Roles ───────────────────────────────────────────────────
  listUserRoles(userId: string): Promise<UserRole[]>;
  assignRole(data: { user_id: string; role: string; scope_type: string; scope_id?: string }): Promise<UserRole>;
  revokeRole(roleId: string): Promise<void>;
}
