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
  type FolderDto,
  type FolderContentsDto,
  type CreateFolderRequest,
  type UpdateFolderRequest,
  type CreateCurriculumFolderRequest,
  type UploadDocumentRequest,
  type StorageUploadRequest,
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
const downloadedDocumentUrls = new Map<string, string>();
let storageDirectoryPromise: Promise<string> | undefined;

/**
 * Transforms a generic document path from the API into a viewable URL.
 * In a desktop context with local files, uses Tauri's convertFileSrc and appDataDir.
 */
export async function resolveDocumentUrl(
  pathOrUrl: string | undefined | null,
  documentId?: string | null,
): Promise<string> {
  if (
    pathOrUrl?.startsWith("http://") ||
    pathOrUrl?.startsWith("https://") ||
    pathOrUrl?.startsWith("data:")
  ) {
    return pathOrUrl;
  }

  if (documentId) {
    const cacheKey = `document:${documentId}`;
    const cachedDocumentUrl = downloadedDocumentUrls.get(cacheKey);
    if (cachedDocumentUrl) return cachedDocumentUrl;

    const file = await api.downloadDocumentFile(documentId);
    const blob = file instanceof Blob ? file : new Blob([file.slice().buffer]);
    const documentUrl = URL.createObjectURL(blob);
    downloadedDocumentUrls.set(cacheKey, documentUrl);
    return documentUrl;
  }

  if (!pathOrUrl) return "";
  if (pathOrUrl.startsWith("blob:")) return pathOrUrl;

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
  } catch (error) {
    console.error("Failed to resolve document path:", pathOrUrl, error);
    throw error;
  }
}

export async function resolveEntityDocumentUrl(
  pathOrUrl: string | undefined | null,
  entityType: string,
  entityId: string,
): Promise<string> {
  if (!pathOrUrl) return "";
  if (
    pathOrUrl.startsWith("http://") ||
    pathOrUrl.startsWith("https://") ||
    pathOrUrl.startsWith("data:") ||
    pathOrUrl.startsWith("blob:") ||
    pathOrUrl.startsWith("asset:")
  ) {
    return pathOrUrl;
  }

  const documents = await api.listDocumentsByEntity(entityType, entityId);
  const matchingDocument = documents.find(
    (document) =>
      document.file_path === pathOrUrl ||
      document.storage_key === pathOrUrl ||
      document.url === pathOrUrl,
  );

  return matchingDocument
    ? resolveDocumentUrl(pathOrUrl, matchingDocument.id)
    : resolveDocumentUrl(pathOrUrl);
}

export async function resolveDocumentImageDataUri(
  source: string,
  entityType: string,
  entityId: string,
): Promise<string> {
  if (/^data:image\/(?:png|jpeg);base64,/i.test(source)) {
    return source;
  }

  const resolvedUrl = await resolveEntityDocumentUrl(
    source,
    entityType,
    entityId,
  );
  const response = await fetch(resolvedUrl);
  if (!response.ok) {
    throw new Error(`Failed to load image (${response.status}).`);
  }

  let blob = await response.blob();
  const mimeType = blob.type.split(";")[0]?.toLowerCase();
  if (!mimeType?.startsWith("image/")) {
    throw new Error(
      `Expected an image, received ${blob.type || "unknown data"}.`,
    );
  }

  if (mimeType !== "image/png" && mimeType !== "image/jpeg") {
    const bitmap = await createImageBitmap(blob);
    try {
      const canvas = document.createElement("canvas");
      canvas.width = bitmap.width;
      canvas.height = bitmap.height;
      const context = canvas.getContext("2d");
      if (!context) throw new Error("Unable to convert image to PNG.");
      context.drawImage(bitmap, 0, 0);
      blob = await new Promise<Blob>((resolve, reject) => {
        canvas.toBlob((result) => {
          if (result) resolve(result);
          else reject(new Error("Unable to convert image to PNG."));
        }, "image/png");
      });
    } finally {
      bitmap.close();
    }
  }

  return new Promise<string>((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => {
      if (typeof reader.result === "string") resolve(reader.result);
      else reject(new Error("Unable to read image data."));
    };
    reader.onerror = () =>
      reject(reader.error ?? new Error("Unable to read image data."));
    reader.readAsDataURL(blob);
  });
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
  return createInfiniteQuery<
    DocumentPageResult,
    Error,
    DocumentPageResult,
    readonly unknown[],
    string | undefined
  >(() => {
    const f = filter ? filter() : undefined;
    return {
      queryKey: queryKeys.document.list(
        f ? { entity_type: f.entityType, entity_id: f.entityId } : undefined,
      ),
      placeholderData: (previousData) => previousData,
      queryFn: async () => {
        const items =
          f?.entityType && f?.entityId
            ? await api.listDocumentsByEntity(f.entityType, f.entityId)
            : await api.listDocuments(
                f
                  ? { entity_type: f.entityType, entity_id: f.entityId }
                  : undefined,
              );
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
      queryClient.invalidateQueries({
        queryKey: queryKeys.document.detail(vars.id),
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
      queryClient.invalidateQueries({
        queryKey: queryKeys.document.requirements(),
      });
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
    },
  }));
}

export function useUpdateDocumentRequirement() {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: ({
      id,
      req,
    }: {
      id: string;
      req: UpdateDocumentRequirementRequest;
    }) => api.updateDocumentRequirement(id, req),
    onSuccess: (_, vars) => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.document.requirements(),
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.document.requirement(vars.id),
      });
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
    },
  }));
}

export function useDeleteDocumentRequirement() {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: (id: string) => api.deleteDocumentRequirement(id),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.document.requirements(),
      });
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
    },
  }));
}

// ── Storage & Archive Hooks ─────────────────────────────────

export function useUploadFile(options: { invalidateOnSuccess?: boolean } = {}) {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: async (req: StorageUploadRequest) => {
      return api.uploadFile(req);
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

export function useArchivedDocuments() {
  return createQuery(() => ({
    queryKey: [...queryKeys.document.all, "archived"] as const,
    queryFn: () => api.listArchivedDocuments(),
    staleTime: 2 * 60 * 1000,
  }));
}

export function useTrashDocuments() {
  return createQuery(() => ({
    queryKey: [...queryKeys.document.all, "trash"] as const,
    queryFn: () => api.listTrashDocuments(),
    staleTime: 2 * 60 * 1000,
  }));
}

export function useArchiveDocument() {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: ({ id, reason }: { id: string; reason?: string }) =>
      api.archiveDocument(id, reason),
    onSuccess: (_, vars) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.document.all });
      queryClient.invalidateQueries({
        queryKey: queryKeys.document.detail(vars.id),
      });
    },
  }));
}

export function useRestoreDocument() {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: (id: string) => api.restoreDocument(id),
    onSuccess: (_, id) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.document.all });
      queryClient.invalidateQueries({
        queryKey: queryKeys.document.detail(id),
      });
    },
  }));
}

export function useTrashDocument() {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: (id: string) => api.trashDocument(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.document.all });
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
    },
  }));
}

export function usePurgeDocument() {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: (id: string) => api.purgeDocument(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.document.all });
      queryClient.invalidateQueries({ queryKey: queryKeys.folder.all });
      queryClient.invalidateQueries({ queryKey: queryKeys.student.all });
    },
  }));
}

// ── Folder & Curriculum Knowledge Base Hooks ────────────────

export function useFolders(parentId?: () => string | undefined) {
  return createQuery(() => {
    const pid = parentId ? parentId() : undefined;
    return {
      queryKey: queryKeys.folder.list(pid),
      queryFn: () => api.listFolders(pid),
      staleTime: 2 * 60 * 1000,
    };
  });
}

export function useFolder(id: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.folder.detail(id()),
    queryFn: () => api.getFolder(id()),
    enabled: Boolean(id()),
    staleTime: 5 * 60 * 1000,
  }));
}

export function useFolderContents(id: () => string) {
  return createQuery(() => ({
    queryKey: queryKeys.folder.contents(id()),
    queryFn: () => api.getFolderContents(id()),
    enabled: Boolean(id()),
    staleTime: 2 * 60 * 1000,
  }));
}

export function useStudentFolder(
  studentId: () => string,
  studentName?: () => string | undefined,
) {
  return createQuery(() => {
    const sid = studentId();
    const sname = studentName ? studentName() : undefined;
    return {
      queryKey: queryKeys.folder.student(sid),
      queryFn: () => api.getOrCreateStudentFolder(sid, sname),
      enabled: Boolean(sid),
      staleTime: 5 * 60 * 1000,
    };
  });
}

export function useCurriculumFolders() {
  return createQuery(() => ({
    queryKey: queryKeys.folder.curriculum(),
    queryFn: () => api.listCurriculumFolders(),
    staleTime: 5 * 60 * 1000,
  }));
}

export function useCreateFolder() {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: (req: CreateFolderRequest) => api.createFolder(req),
    onSuccess: (_, vars) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.folder.all });
      if (vars.parent_id) {
        queryClient.invalidateQueries({
          queryKey: queryKeys.folder.contents(vars.parent_id),
        });
      }
    },
  }));
}

export function useCreateCurriculumFolder() {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: (req: CreateCurriculumFolderRequest) =>
      api.getOrCreateCurriculumFolder(req),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.folder.curriculum(),
      });
      queryClient.invalidateQueries({ queryKey: queryKeys.folder.all });
    },
  }));
}

export function useUpdateFolder() {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: ({ id, req }: { id: string; req: UpdateFolderRequest }) =>
      api.updateFolder(id, req),
    onSuccess: (_, vars) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.folder.all });
      queryClient.invalidateQueries({
        queryKey: queryKeys.folder.detail(vars.id),
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.folder.contents(vars.id),
      });
    },
  }));
}

export function useDeleteFolder() {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: (id: string) => api.deleteFolder(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.folder.all });
      queryClient.invalidateQueries({ queryKey: queryKeys.document.all });
    },
  }));
}
