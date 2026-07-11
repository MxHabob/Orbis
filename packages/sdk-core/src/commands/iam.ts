/**
 * IAM command interfaces — mirrors svc-iam/ops.
 */
import type { User, UserRole, UserSession } from "../types";

export interface IAMCommands {
  // ── Auth ────────────────────────────────────────────────────
  login(identifier: string, passwordHash: string, ip?: string): Promise<{ user: User; session: UserSession }>;
  logout(sessionId: string): Promise<void>;
  changePassword(userId: string, data: { new_password: string } | string): Promise<void>;
  listSessions(userId: string): Promise<UserSession[]>;

  // ── Users ───────────────────────────────────────────────────
  listUsers(orgId: string, filter?: { role?: string; status?: string }): Promise<User[]>;
  getUser(userId: string): Promise<User>;
  searchUsers(query: string): Promise<User[]>;
  createUser(orgId: string, createdBy: string, data: any): Promise<User>;
  updateUser(userId: string, data: any): Promise<User>;
  deleteUser(userId: string, role?: string): Promise<void>;

  // ── Roles ───────────────────────────────────────────────────
  listUserRoles(userId: string): Promise<UserRole[]>;
  assignRole(data: { user_id: string; role: string; scope_type: string; scope_id?: string }): Promise<UserRole>;
  revokeRole(roleId: string): Promise<void>;
  changeRole(userId: string, data: { role: string }): Promise<void>;
  saveUserRoles(userId: string, roles: string[]): Promise<void>;
}
