export interface AgentCommands {
  listSessions(orgId: string): Promise<any[]>;
  createSession(orgId: string, type: string): Promise<any>;
  query(orgId: string, sessionId: string, text: string, context?: any): Promise<string>;
  openStandaloneWindow(orgId: string): Promise<void>;
}
