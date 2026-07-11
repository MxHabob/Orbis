export interface PermissionRule {
  role: string;
  document_type: string;
  level: number;
  only_if_creator: boolean;
  actions: string[];
}

export interface PermissionsCommands {
  getRolePermissions(): Promise<PermissionRule[]>;
  saveRolePermissions(role: string, rules: PermissionRule[]): Promise<void>;
}
