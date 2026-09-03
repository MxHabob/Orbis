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
const previewUrls = new Map<string, Promise<string>>();

/** Create a small display preview while keeping the original URL for full preview/download. */
export function createDocumentPreviewUrl(source: string): Promise<string> {
  const cachedPreview = previewUrls.get(source);
  if (cachedPreview) return cachedPreview;

  const preview = new Promise<string>((resolve) => {
    const image = new Image();
    image.onload = () => {
      const maxSize = 320;
      const scale = Math.min(1, maxSize / Math.max(image.naturalWidth, image.naturalHeight));
      const canvas = document.createElement("canvas");
      canvas.width = Math.max(1, Math.round(image.naturalWidth * scale));
      canvas.height = Math.max(1, Math.round(image.naturalHeight * scale));
      const context = canvas.getContext("2d");

      if (!context) {
        resolve(source);
        return;
      }

      try {
        context.drawImage(image, 0, 0, canvas.width, canvas.height);
        canvas.toBlob(
          (blob) => resolve(blob ? URL.createObjectURL(blob) : source),
          "image/jpeg",
          0.72,
        );
      } catch {
        resolve(source);
      }
    };
    image.onerror = () => resolve(source);
    image.src = source;
  });

  previewUrls.set(source, preview);
  return preview;
}

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
}) {
  return createInfiniteQuery(() => ({
    queryKey: documentKeys.byFilter(filter()),
    queryFn: ({ pageParam }) => {
      const currentFilter = filter();
      return commands.document.getDocuments({
        ownerId: currentFilter.ownerId,
        entityType: currentFilter.entityType,
        category: currentFilter.category || undefined,
        cursor: pageParam,
        limit: 50,
        filter:
          currentFilter.search?.trim() || currentFilter.assetType || currentFilter.schoolYear || currentFilter.term
            ? {
                search: currentFilter.search?.trim() || undefined,
                conditions: [
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
