import { useQuery } from "@tanstack/react-query";
import { useTRPC } from "@/ipc/query-adapter";

type FileUrlOptions = {
  type?: "download" | "preview" | "url";
  documentId?: string;
  url?: string;
};

export function useFileUrl(options: FileUrlOptions | null) {
  const trpc = useTRPC();

  const signedUrlQuery = useQuery({
    ...trpc.documents.getSignedUrl.queryOptions({ id: options?.documentId! }),
    enabled: !!options?.documentId,
    staleTime: 3600 * 1000, // 1 hour
  });

  if (!options) {
    return {
      url: null,
      isLoading: false,
      hasFileKey: false,
    };
  }

  if (options.url) {
    return {
      url: options.url,
      isLoading: false,
      hasFileKey: true,
    };
  }

  return {
    url: signedUrlQuery.data?.url ?? null,
    isLoading: signedUrlQuery.isLoading,
    hasFileKey: !!signedUrlQuery.data?.url,
  };
}
