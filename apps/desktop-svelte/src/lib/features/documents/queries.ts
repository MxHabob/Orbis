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

  try {
    // If running in Tauri desktop, get absolute path to app_data/storage
    const dataDir = await appDataDir();
    // Assuming backend path like "1/users/avatars/abc.png"
    // And base path is "app_data/storage" relative to dataDir
    // We construct absolute path: dataDir + "/storage/" + pathOrUrl (if path isn't absolute already)

    // Clean up if it starts with /storage/
    let relativePath = pathOrUrl;
    if (relativePath.startsWith("/storage/")) {
      relativePath = relativePath.replace("/storage/", "");
    }

    const absolutePath = await join(dataDir, "storage", relativePath);
    return convertFileSrc(absolutePath);
  } catch (e) {
    // Fallback if not in Tauri
    console.warn("Tauri path resolution failed, using fallback:", e);
    return pathOrUrl;
  }
}

export const documentKeys = {
  all: ["documents"] as const,
  byFilter: (filter: {
    ownerId?: string;
    entityType?: string;
    category?: string;
  }) => ["documents", "filter", filter] as const,
};

export function useDocuments(filter: {
  ownerId?: string;
  entityType?: string;
  category?: string;
}) {
  return createInfiniteQuery(() => ({
    queryKey: documentKeys.byFilter(filter),
    queryFn: ({ pageParam }) =>
      commands.document.getDocuments({
        ownerId: filter.ownerId,
        entityType: filter.entityType,
        category: filter.category,
        cursor: pageParam,
        limit: 50,
      }),
    initialPageParam: undefined as string | undefined,
    getNextPageParam: (lastPage: any) => lastPage?.next_cursor,
  }));
}

// --- Mutations ---

export function useUploadDocument() {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: (payload: {
      file: File;
      entity_type: string;
      entity_id: string;
      category: string;
      visibility: string;
    }) => commands.document.uploadDocument(payload),
    onSuccess: (data, variables) => {
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
