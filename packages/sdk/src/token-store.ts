export interface SessionTokens {
  accessToken: string;
  refreshToken: string;
  expiresAt?: string;
  refreshExpiresAt?: string;
}

export interface TokenStore {
  get(): Promise<SessionTokens | null>;
  set(session: SessionTokens): Promise<void>;
  clear(): Promise<void>;
}

export class MemoryTokenStore implements TokenStore {
  private session: SessionTokens | null = null;

  async get(): Promise<SessionTokens | null> {
    return this.session;
  }

  async set(session: SessionTokens): Promise<void> {
    this.session = session;
  }

  async clear(): Promise<void> {
    this.session = null;
  }
}

export function createMemoryTokenStore(): TokenStore {
  return new MemoryTokenStore();
}
