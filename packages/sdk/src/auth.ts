import { OrbisError } from "./errors.js";
import type { OrbisClient } from "./client.js";
import type { SessionTokens, TokenStore } from "./token-store.js";

export interface LoginRequest {
  identifier: string;
  password: string;
}

export interface LoginResponse {
  user: {
    id: string;
    org_id: string;
    email?: string | null;
    full_name: string;
    primary_role?: string;
    status?: string;
  };
  session: SessionTokens;
}

export class AuthClient {
  constructor(
    private readonly client: OrbisClient,
    private readonly tokenStore: TokenStore,
  ) {}

  async login(input: LoginRequest): Promise<LoginResponse> {
    const response = await this.client.post<LoginResponse>("/api/auth/login", {
      identifier: input.identifier,
      password: input.password,
    });

    if (!response?.session?.accessToken || !response.session.refreshToken) {
      throw new OrbisError("Invalid login response", "AUTH_FAILED");
    }

    await this.tokenStore.set(response.session);
    return response;
  }

  async refresh(): Promise<SessionTokens | null> {
    const current = await this.tokenStore.get();
    if (!current?.refreshToken) {
      return null;
    }

    const response = await this.client.post<{ session?: SessionTokens }>("/api/auth/refresh", {
      refresh_token: current.refreshToken,
    });

    const session = response?.session ?? null;
    if (!session?.accessToken || !session.refreshToken) {
      await this.tokenStore.clear();
      return null;
    }

    await this.tokenStore.set(session);
    return session;
  }

  async logout(): Promise<void> {
    try {
      const current = await this.tokenStore.get();
      if (current?.accessToken) {
        await this.client.post("/api/auth/logout", { token: current.accessToken });
      }
    } finally {
      await this.tokenStore.clear();
    }
  }

  async me(): Promise<unknown> {
    return this.client.get("/api/auth/me");
  }
}
