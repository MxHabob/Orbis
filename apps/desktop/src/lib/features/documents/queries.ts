import {
  createQuery,
  createInfiniteQuery,
  createMutation,
  useQueryClient,
} from "@tanstack/svelte-query";
import { api, queryKeys } from "$lib/api";
import { convertFileSrc } from "@tauri-apps/api/core";
import { appDataDir, join } from "@tauri-apps/api/path";

// --- Helpers ---

const resolvedDocumentUrls = new Map<string, string>();
let storageDirectoryPromise: Promise<string> | undefined;

/**
 * Transforms a generic document path from the API into a viewable URL.
 * In a desktop context with local files, uses Tauri's convertFileSrc and appDataDir.
 */
export async function resolveDocumentUrl(
  pathOrUrl: string | undefined | null,
): Promise<string> {
  if (!pathOrUrl) return "";

  if (pathOrUrl.startsWith("http://") || pathOrUrl.startsWith("https://")) {
    return pathOrUrl;
  }

  const cachedUrl = resolvedDocumentUrls.get(pathOrUrl);
  if (cachedUrl) return cachedUrl;

  try {
    storageDirectoryPromise ??= appDataDir().then((dataDir) => join(dataDir, "storage"));
    const storageDirectory = await storageDirectoryPromise;

    let relativePath = pathOrUrl;
    if (relativePath.startsWith("/storage/")) {
      relativePath = relativePath.replace("/storage/", "");
    }

    const absolutePath = await join(storageDirectory, relativePath);
    const resolvedUrl = convertFileSrc(absolutePath);
    resolvedDocumentUrls.set(pathOrUrl, resolvedUrl);
    return resolvedUrl;
  } catch (e) {
    console.warn("Tauri path resolution failed, using fallback:", e);
    return pathOrUrl;
  }
}

export function getDocumentSource(document: {
  url?: string | null;
  path?: string | null;
}): string {
  return document.url || document.path || "";
}

export function useDocuments(filter: () => {
  ownerId?: string;
  entityType?: string;
  category?: string;
  search?: string;
  assetType?: string;
  schoolYear?: string;
  term?: string;
  entityId?: string;
}) {
  return createInfiniteQuery(() => ({
    queryKey: queryKeys.document.all,
    placeholderData: (previousData: any) => previousData,
    queryFn: async () => {
      const items = await api.listDocuments();
      return {
        items,
        count: items.length,
        has_more: false,
        next_cursor: null,
      };
    },
    initialPageParam: undefined as string | undefined,
    getNextPageParam: (lastPage: any) => {
      if (lastPage?.has_more && lastPage?.next_cursor) {
        return lastPage.next_cursor;
      }
      return undefined;
    },
  }));
}

// --- Mutations ---

export function useUploadDocument(options: { invalidateOnSuccess?: boolean } = {}) {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: async (payload: {
      file: File;
      entity_type: string;
      entity_id: string;
      category: string;
      visibility: string;
    }) => {
      return api.uploadDocument({
        name: payload.file.name,
        mime_type: payload.file.type || "application/octet-stream",
        entity_type: payload.entity_type,
        entity_id: payload.entity_id,
        category: payload.category,
        visibility: payload.visibility,
        path: payload.file.name,
        size: payload.file.size,
      });
    },
    onSuccess: () => {
      if (options.invalidateOnSuccess === false) return;
      queryClient.invalidateQueries({
        queryKey: queryKeys.document.all,
      });
    },
  }));
}

export function useDeleteDocument() {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: (id: string) => api.deleteDocument(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.document.all });
    },
  }));
}

