export interface AgentCommands {
  listSessions(): Promise<any[]>;
  createSession(type: string): Promise<any>;
  query(
    sessionId: string,
    text: string,
    context?: any,
    onChunk?: (chunk: string) => void,
  ): Promise<string>;
  openStandaloneWindow(): Promise<void>;
}
