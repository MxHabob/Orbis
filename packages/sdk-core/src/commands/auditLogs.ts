export interface AuditLogsCommands {
  getAll(orgId: string): Promise<any[]>;
}
