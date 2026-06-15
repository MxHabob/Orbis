import { useCallback, useState } from "react";
import { commands } from "@orbis/desktop-client/commands";
import { getOrgId, getUserId } from "@/lib/session";

type UploadOptions = {
  file: File;
  path: string[];
  bucket: string;
};

type UploadResult = {
  url: string;
  path: string;
};

export function useUpload() {
  const [isLoading, setIsLoading] = useState(false);

  const uploadFile = useCallback(
    async (options: UploadOptions): Promise<UploadResult> => {
      setIsLoading(true);
      try {
        const orgId = getOrgId();
        const userId = getUserId();
        const relativePath = options.path.filter(Boolean).join("/");
        const storageKey = [options.bucket, relativePath, options.file.name]
          .filter(Boolean)
          .join("/");

        const doc = await commands.documents.create({
          orgId,
          ownerId: userId,
          uploadedBy: userId,
          name: options.file.name,
          bucket: options.bucket,
          key: storageKey,
          path: storageKey,
          size: options.file.size,
          mimeType: options.file.type,
          type: "file",
        });

        const url =
          (doc as { url?: string }).url ??
          (doc as { key?: string }).key ??
          storageKey;

        return { url, path: storageKey };
      } finally {
        setIsLoading(false);
      }
    },
    [],
  );

  return { uploadFile, isLoading };
}
