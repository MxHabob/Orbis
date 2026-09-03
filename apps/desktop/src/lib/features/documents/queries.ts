import {
  createQuery,
  createInfiniteQuery,
  createMutation,
  useQueryClient,
} from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";
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
    // If running in Tauri desktop, get absolute path to app_data/storage
    storageDirectoryPromise ??= appDataDir().then((dataDir) => join(dataDir, "storage"));
    const storageDirectory = await storageDirectoryPromise;
    // Assuming backend path like "1/users/avatars/abc.png"
    // And base path is "app_data/storage" relative to dataDir
    // We construct absolute path: dataDir + "/storage/" + pathOrUrl (if path isn't absolute already)

    // Clean up if it starts with /storage/
    let relativePath = pathOrUrl;
    if (relativePath.startsWith("/storage/")) {
      relativePath = relativePath.replace("/storage/", "");
    }

    const absolutePath = await join(storageDirectory, relativePath);
    const resolvedUrl = convertFileSrc(absolutePath);
    resolvedDocumentUrls.set(pathOrUrl, resolvedUrl);
    return resolvedUrl;
  } catch (e) {
    // Fallback if not in Tauri
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

export const documentKeys = {
  all: ["documents"] as const,
  byFilter: (filter: {
    ownerId?: string;
    entityType?: string;
    category?: string;
    search?: string;
    assetType?: string;
    schoolYear?: string;
    term?: string;
    entityId?: string;
  }) => ["documents", "filter", filter] as const,
};

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
    queryKey: documentKeys.byFilter(filter()),
    placeholderData: (previousData: any) => previousData,
    queryFn: ({ pageParam }) => {
      const currentFilter = filter();
      return commands.document.getDocuments({
        ownerId: currentFilter.ownerId,
        entityType: currentFilter.entityType,
        category: currentFilter.category || undefined,
        cursor: pageParam,
        limit: 50,
        filter:
          currentFilter.search?.trim() || currentFilter.entityId || currentFilter.assetType || currentFilter.schoolYear || currentFilter.term
            ? {
                search: currentFilter.search?.trim() || undefined,
                conditions: [
                  ...(currentFilter.entityId
                    ? [{ field: "entity_id", operator: "eq", value: currentFilter.entityId }]
                    : []),
                  ...(currentFilter.assetType
                    ? [{ field: "asset_type", operator: "eq", value: currentFilter.assetType }]
                    : []),
                  ...(currentFilter.schoolYear
                    ? [{ field: "school_year", operator: "eq", value: currentFilter.schoolYear }]
                    : []),
                  ...(currentFilter.term
                    ? [{ field: "term", operator: "eq", value: currentFilter.term }]
                    : []),
                ],
                logic: "and",
              }
            : undefined,
      });
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
      const bytes = Array.from(new Uint8Array(await payload.file.arrayBuffer()));
      return commands.document.uploadDocument(
        payload.file.name,
        payload.file.type || "application/octet-stream",
        bytes,
        payload.entity_type,
        payload.entity_id,
        payload.category,
        payload.visibility,
      );
    },
    onSuccess: (data, variables) => {
      if (options.invalidateOnSuccess === false) return;
      queryClient.invalidateQueries({
        queryKey: documentKeys.all,
      });
    },
  }));
}

export function useDeleteDocument() {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: (id: string) => commands.document.deleteDocument(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: documentKeys.all });
    },
  }));
}
