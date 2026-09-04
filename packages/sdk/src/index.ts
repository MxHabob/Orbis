export { OrbisError, extractErrorMessage } from "./errors.js";
export type { ErrorCode, OrbisErrorPayload } from "./errors.js";

export { AuthClient } from "./auth.js";
export type { LoginRequest, LoginResponse } from "./auth.js";

export { createMemoryTokenStore } from "./token-store.js";
export type { SessionTokens, TokenStore } from "./token-store.js";

export { createRestTransport } from "./rest-transport.js";
export type { RestTransportOptions, ApiEnvelope } from "./rest-transport.js";

export { createOrbisClient, OrbisClient } from "./client.js";
export type { OrbisClientOptions } from "./client.js";

export { ResourceClient, OrbisResources, createResourceClient, createOrbisResources } from "./resources.js";
export type {
  PageMeta,
  PagedResponse,
  ResourceClientOptions,
} from "./resources.js";

export type { HttpMethod, Transport, TransportRequest } from "./transport.js";

export type * from "./generated/types.gen.js";
