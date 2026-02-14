import { useTRPC } from "@/trpc/client";
import { useQuery } from "@tanstack/react-query";

type FileUrlOptions = {
  type: "download" | "preview";
  documentId?: string;
  url?: string;
};

export function useFileUrl(options: FileUrlOptions | null) {
  const trpc = useTRPC();

  const signedUrlQuery = useQuery({
    ...trpc.documents.getSignedUrl.queryOptions(
      { id: options?.documentId! },
    ),
    enabled: !!options?.documentId,
    staleTime: 3600 * 1000, // 1 hour
  });

  if (!options) {
    return {
      url: null,
      isLoading: false,
    };
  }

  if (options.url) {
    return {
      url: options.url,
      isLoading: false,
    };
  }

  return {
    url: signedUrlQuery.data?.url ?? null,
    isLoading: signedUrlQuery.isLoading,
  };
}
