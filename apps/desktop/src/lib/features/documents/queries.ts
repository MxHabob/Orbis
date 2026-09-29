import {
  createQuery,
  createInfiniteQuery,
  createMutation,
  useQueryClient,
} from "@tanstack/svelte-query";
import {
  api,
  queryKeys,
  type DocumentDto,
  type UploadDocumentRequest,
  type UpdateDocumentRequest,
  type DocumentRequirementDto,
  type CreateDocumentRequirementRequest,
  type UpdateDocumentRequirementRequest,
  type StudentDocumentFulfillmentDto,
} from "$lib/api";
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

  // 1. Direct URLs (web, data, blob, Tauri asset protocol)
  if (
    pathOrUrl.startsWith("http://") ||
    pathOrUrl.startsWith("https://") ||
    pathOrUrl.startsWith("data:") ||
    pathOrUrl.startsWith("blob:") ||
    pathOrUrl.startsWith("asset:")
  ) {
    return pathOrUrl;
  }

  const cachedUrl = resolvedDocumentUrls.get(pathOrUrl);
  if (cachedUrl) return cachedUrl;

  try {
    // 2. If it's already an absolute Windows or Unix path, convert directly
    const isWindowsAbsolute =
      /^[a-zA-Z]:[\\/]/.test(pathOrUrl) || pathOrUrl.startsWith("\\\\");
    const isPosixAbsolute =
      pathOrUrl.startsWith("/") && !pathOrUrl.startsWith("/storage/");

    if (isWindowsAbsolute || isPosixAbsolute) {
      const resolvedUrl = convertFileSrc(pathOrUrl);
      resolvedDocumentUrls.set(pathOrUrl, resolvedUrl);
      return resolvedUrl;
    }

    // 3. Handle relative storage paths
    storageDirectoryPromise ??= appDataDir().then((dataDir) =>
      join(dataDir, "storage"),
    );
    const storageDirectory = await storageDirectoryPromise;

    let relativePath = pathOrUrl;
    if (relativePath.startsWith("/storage/")) {
      relativePath = relativePath.slice(9);
    } else if (relativePath.startsWith("\\storage\\")) {
      relativePath = relativePath.slice(9);
    } else if (relativePath.startsWith("storage/")) {
      relativePath = relativePath.slice(8);
    } else if (relativePath.startsWith("storage\\")) {
      relativePath = relativePath.slice(8);
    }

    // Clean any leading slash or backslash
    relativePath = relativePath.replace(/^[/\\]+/, "");

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
  file_path?: string | null;
}): string {
  return document.url || document.file_path || "";
}

export interface DocumentPageResult {
  items: DocumentDto[];
  count: number;
  has_more: boolean;
  next_cursor: string | null;
}

export function useDocuments(
  filter?: () => {
    entityType?: string;
    entityId?: string;
  },
) {
  return createInfiniteQuery<DocumentPageResult, Error, DocumentPageResult, readonly unknown[], string | undefined>(() => {
    const f = filter ? filter() : undefined;
    return {
      queryKey: queryKeys.document.list(f ? { entity_type: f.entityType, entity_id: f.entityId } : undefined),
      placeholderData: (previousData) => previousData,
      queryFn: async () => {
        const items = f?.entityType && f?.entityId
          ? await api.listDocumentsByEntity(f.entityType, f.entityId)
          : await api.listDocuments(f ? { entity_type: f.entityType, entity_id: f.entityId } : undefined);
        return {
          items,
          count: items.length,
          has_more: false,
          next_cursor: null,
        };
      },
      initialPageParam: undefined,
      getNextPageParam: (lastPage) => {
        if (lastPage?.has_more && lastPage?.next_cursor) {
          return lastPage.next_cursor;
        }
        return undefined;
      },
    };
  });
}

export function useDocumentsByEntity(
  entityType: () => string,
  entityId: () => string,
) {
  return createQuery(() => ({
    queryKey: queryKeys.document.byEntity(entityType(), entityId()),
    queryFn: () => api.listDocumentsByEntity(entityType(), entityId()),
    enabled: Boolean(entityType() && entityId()),
    staleTime: 2 * 60 * 1000,
  }));
}

export function useDocument(id: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.document.detail(id()),
    queryFn: () => api.getDocument(id()),
    enabled: Boolean(id()),
    staleTime: 5 * 60 * 1000,
  }));
}

export function useDocumentRequirements() {
  return createQuery(() => ({
    queryKey: queryKeys.document.requirements(),
    queryFn: () => api.listDocumentRequirements(),
    staleTime: 5 * 60 * 1000,
  }));
}

export function useDocumentRequirement(id: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.document.requirement(id()),
    queryFn: () => api.getDocumentRequirement(id()),
    enabled: Boolean(id()),
    staleTime: 5 * 60 * 1000,
  }));
}

export function useStudentDocumentRequirements(
  studentId: () => string,
  params?: () => {
    grade_level_id?: string;
    is_national?: boolean;
    is_transfer?: boolean;
  },
) {
  return createQuery(() => {
    const p = params ? params() : undefined;
    return {
      queryKey: queryKeys.document.studentFulfillment(studentId(), p),
      queryFn: () => api.evaluateStudentDocumentRequirements(studentId(), p),
      enabled: Boolean(studentId()),
      staleTime: 3 * 60 * 1000,
    };
  });
}

// --- Mutations ---

export function useUploadDocument(
  options: { invalidateOnSuccess?: boolean } = {},
) {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: async (req: UploadDocumentRequest) => {
      return api.uploadDocument(req);
    },
    onSuccess: () => {
      if (options.invalidateOnSuccess === false) return;
      queryClient.invalidateQueries({
        queryKey: queryKeys.document.all,
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.student.all,
      });
    },
  }));
}

export function useUpdateDocument() {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: ({ id, req }: { id: string; req: UpdateDocumentRequest }) =>
      api.updateDocument(id, req),
    onSuccess: (_, vars) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.document.all });
      queryClient.invalidateQueries({ queryKey: queryKeys.document.detail(vars.id) });
    },
  }));
}

export function useDeleteDocument() {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: (id: string) => api.deleteDocument(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.document.all });
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
    },
  }));
}

export function useCreateDocumentRequirement() {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: (req: CreateDocumentRequirementRequest) =>
      api.createDocumentRequirement(req),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.document.requirements() });
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
    },
  }));
}

export function useUpdateDocumentRequirement() {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: ({ id, req }: { id: string; req: UpdateDocumentRequirementRequest }) =>
      api.updateDocumentRequirement(id, req),
    onSuccess: (_, vars) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.document.requirements() });
      queryClient.invalidateQueries({ queryKey: queryKeys.document.requirement(vars.id) });
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
    },
  }));
}

export function useDeleteDocumentRequirement() {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: (id: string) => api.deleteDocumentRequirement(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.document.requirements() });
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
    },
  }));
}
