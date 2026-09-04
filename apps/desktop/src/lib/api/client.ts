import { invoke } from '@tauri-apps/api/core';
import { createOrg, createOrgUnit } from '@orbis/sdk';
import type { CreateOrgData, CreateOrgResponse, CreateOrgUnitData, CreateOrgUnitResponse } from '@orbis/sdk';

/**
 * The unified interface for the API Client.
 * All frontend features should depend on this interface instead of raw Fetch or Tauri Invoke.
 */
export interface ApiClient {
    organizations: {
        create(data: CreateOrgData['body']): Promise<CreateOrgResponse>;
        createUnit(data: CreateOrgUnitData['body']): Promise<CreateOrgUnitResponse>;
    };
}

/**
 * RemoteClient: Uses the generated SDK (HTTP/Axum -> PostgreSQL)
 * Used in Remote Desktop mode, Web, and Mobile.
 */
export class RemoteClient implements ApiClient {
    organizations = {
        create: async (data: CreateOrgData['body']) => {
            const { data: res, error } = await createOrg({ body: data });
            if (error) throw new Error(String(error));
            return res!;
        },
        createUnit: async (data: CreateOrgUnitData['body']) => {
            const { data: res, error } = await createOrgUnit({ body: data });
            if (error) throw new Error(String(error));
            return res!;
        }
    };
}

/**
 * LocalClient: Uses Tauri IPC to bypass HTTP completely.
 * Native Rust invocation directly to the Application Core -> SQLite.
 * Uses the exact same Types from @orbis/sdk for perfect type safety.
 */
export class LocalClient implements ApiClient {
    organizations = {
        create: async (data: CreateOrgData['body']) => {
            // Tauri commands expect snake_case payload mirroring Rust structs
            const orgId = await invoke<string>('create_organization', {
                name: data.name,
                slug: data.slug,
                timezone: data.timezone,
                locale: data.locale,
            });
            return {
                id: orgId,
                name: data.name,
                slug: data.slug,
            } as CreateOrgResponse;
        },
        createUnit: async (data: CreateOrgUnitData['body']) => {
            const unitId = await invoke<string>('create_org_unit', {
                orgId: data.org_id,
                name: data.name,
                unitType: data.unit_type,
                parentId: data.parent_id,
                dataIsolation: data.data_isolation,
            });
            return {
                id: unitId,
                path: "", // Mocked for local response mapping until further refinement
                depth: 1,
            } as CreateOrgUnitResponse;
        }
    };
}

// Global factory logic to determine execution mode
export const createApiClient = (): ApiClient => {
    // In a real app, this would check localStorage or a settings store
    const isLocalMode = !!window.__TAURI_INTERNALS__; // Naive check for Tauri context

    if (isLocalMode) {
        return new LocalClient();
    }
    return new RemoteClient();
};

export const api = createApiClient();
