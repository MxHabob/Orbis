"use client";

import { useCallback, useState } from "react";

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
        const apiUrl =
          process.env.NEXT_PUBLIC_API_URL ?? "http://localhost:3002";
        const formData = new FormData();
        formData.append("file", options.file);
        formData.append("bucket", options.bucket);
        formData.append("path", options.path.filter(Boolean).join("/"));

        const res = await fetch(`${apiUrl}/api/upload`, {
          method: "POST",
          body: formData,
          credentials: "include",
        });

        if (!res.ok) {
          const err = await res.json().catch(() => ({ error: res.statusText }));
          throw new Error(err.error ?? "Upload failed");
        }

        const data = (await res.json()) as UploadResult;
        return data;
      } finally {
        setIsLoading(false);
      }
    },
    [],
  );

  return { uploadFile, isLoading };
}
