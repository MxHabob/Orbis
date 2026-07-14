/**
 * IAM command interfaces — mirrors svc-iam/ops.
 */
import type { User, UserRole, UserSession } from "../types";

export interface PermissionRule {
  role: string;
  document_type: string;
  level: number;
  only_if_creator: boolean;
  actions: string[];
}

export interface IamCommands {
  // ── Authentication ────────────────────────────────────────────
  login(
    identifier: string,
    passwordHash?: string,
    ip?: string,
  ): Promise<UserSession>;
  logout(sessionId: string): Promise<void>;
  changePassword(userId: string, data: any): Promise<void>;
  listSessions(userId: string): Promise<UserSession[]>;

  // ── Users ───────────────────────────────────────────────────
  listUsers(
    filter?: { role?: string; status?: string },
  ): Promise<User[]>;
  getUser(userId: string): Promise<User>;
  searchUsers(query: string): Promise<User[]>;
  createUser(createdBy: string, data: any): Promise<User>;
  updateUser(userId: string, data: any): Promise<User>;
  deleteUser(userId: string, role?: string): Promise<void>;

  // ── Roles & Permissions ─────────────────────────────────────
  listUserRoles(userId: string): Promise<UserRole[]>;
  assignRole(data: {
    user_id: string;
    role: string;
    scope_type: string;
    scope_id?: string;
  }): Promise<UserRole>;
  revokeRole(roleId: string): Promise<void>;
  changeRole(
    userId: string,
    data: { current_role: string; new_role: string; document_type: string },
  ): Promise<void>;
  saveUserRoles(userId: string, roles: string[]): Promise<void>;

  // ── Rule Engine ─────────────────────────────────────────────
  getRolePermissions(role: string): Promise<PermissionRule[]>;
  saveRolePermissions(role: string, permissions: PermissionRule[]): Promise<void>;
}
