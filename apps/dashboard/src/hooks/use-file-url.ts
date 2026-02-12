import { useTRPC } from "@/trpc/client";

type FileUrlOptions = {
  documentId?: string;
  url?: string;
};

export function useFileUrl(options: FileUrlOptions | null) {
  const trpc = useTRPC();

  const signedUrlQuery = trpc.documents.getSignedUrl.useQuery(
    { id: options?.documentId! },
    {
      enabled: !!options?.documentId,
      staleTime: 3600 * 1000, // 1 hour
    }
  );

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
