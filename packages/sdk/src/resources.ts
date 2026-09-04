import type { OrbisClient } from "./client.js";

export interface PageMeta {
  page?: number;
  limit?: number;
  total?: number;
  hasMore?: boolean;
}

export interface PagedResponse<T> {
  data: T[];
  meta?: PageMeta;
}

export interface ResourceClientOptions {
  client: OrbisClient;
  basePath: string;
}

export class ResourceClient {
  constructor(private readonly options: ResourceClientOptions) {}

  private joinPath(path: string): string {
    const sanitizedBase = this.options.basePath.replace(/\/+$/, "");
    const sanitizedPath = path.startsWith("/") ? path : `/${path}`;
    return `${sanitizedBase}${sanitizedPath}`;
  }

  async list<T = unknown>(path: string, query?: Record<string, string | number | boolean | null | undefined>): Promise<T[] | PagedResponse<T>> {
    return this.options.client.get<T[] | PagedResponse<T>>(this.joinPath(path), query);
  }

  async get<T = unknown>(path: string, query?: Record<string, string | number | boolean | null | undefined>): Promise<T> {
    return this.options.client.get<T>(this.joinPath(path), query);
  }

  async create<T = unknown>(path: string, body: unknown): Promise<T> {
    return this.options.client.post<T>(this.joinPath(path), body);
  }

  async update<T = unknown>(path: string, body: unknown): Promise<T> {
    return this.options.client.put<T>(this.joinPath(path), body);
  }

  async patch<T = unknown>(path: string, body: unknown): Promise<T> {
    return this.options.client.patch<T>(this.joinPath(path), body);
  }

  async remove<T = unknown>(path: string): Promise<T> {
    return this.options.client.del<T>(this.joinPath(path));
  }
}

export function createResourceClient(options: ResourceClientOptions): ResourceClient {
  return new ResourceClient(options);
}

export class OrbisResources {
  readonly dashboard: ResourceClient;
  readonly classes: ResourceClient;
  readonly periods: ResourceClient;
  readonly attendance: ResourceClient;
  readonly documents: ResourceClient;

  constructor(private readonly client: OrbisClient) {
    this.dashboard = createResourceClient({ client: this.client, basePath: "/api/dashboard" });
    this.classes = createResourceClient({ client: this.client, basePath: "/api/classes" });
    this.periods = createResourceClient({ client: this.client, basePath: "/api/periods" });
    this.attendance = createResourceClient({ client: this.client, basePath: "/api/attendance" });
    this.documents = createResourceClient({ client: this.client, basePath: "/api/documents" });
  }
}

export function createOrbisResources(client: OrbisClient): OrbisResources {
  return new OrbisResources(client);
}
