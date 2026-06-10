"use client";

import { DocumentActions } from "@/components/document-actions";
import { DocumentDetailsSkeleton } from "@/components/document-details-skeleton";
import { FileViewer } from "@/components/file-viewer";
import { VaultRelatedFiles } from "@/components/vault/vault-related-files";
import { useDocumentParams } from "@/hooks/use-document-params";
import { useTRPC } from "@/trpc/client";
import { formatSize } from "@/utils/format";
import { formatDate } from "@orbis/utils/format";
import { SheetHeader } from "@orbis/ui/sheet";
import { useQuery, useQueryClient } from "@tanstack/react-query";

export function DocumentDetails() {
  const trpc = useTRPC();
  const queryClient = useQueryClient();
  const { params } = useDocumentParams();

  const isOpen = Boolean(params.filePath || params.documentId);
  const fullView = Boolean(params.documentId);

  const { data, isLoading } = useQuery({
    ...trpc.documents.getById.queryOptions({
      filePath: params.filePath!,
      id: params.documentId!,
    }),
    enabled: isOpen,
    staleTime: 0, 
    initialData: () => {
      const pages = queryClient
        .getQueriesData({ queryKey: trpc.documents.get.infiniteQueryKey() })
        // @ts-expect-error
        .flatMap(([, data]) => data?.pages ?? [])
        .flatMap((page) => page.data ?? []);

      return pages.find(
        (d) =>
          d.id === params.documentId ||
          d.path_tokens?.join("/") === params.filePath,
      );
    },
  });

  if (isLoading) {
    return <DocumentDetailsSkeleton />;
  }

  return (
    <div className="flex flex-col flex-grow min-h-0 relative h-full w-full">
      <SheetHeader className="mb-4 flex justify-between items-center flex-row">
        <div className="min-w-0 flex-1 max-w-[70%] flex flex-row gap-2 items-end">
          <h2 className="text-lg truncate flex-0 font-serif">
            {data?.title ?? data?.name?.split("/").at(-1)}
          </h2>
          <span className="text-sm text-muted-foreground whitespace-nowrap">
            {data?.size && formatSize(data?.size)}
          </span>
        </div>

        <DocumentActions showDelete={fullView} filePath={data?.pathTokens as string[]} id={data?.id} />
      </SheetHeader>

      <div className="h-full max-h-[763px] p-0 pb-4 overflow-hidden">
        <div className="flex flex-col flex-grow min-h-0 relative h-full w-full items-center justify-center">
          <FileViewer
            url={`${process.env.NEXT_PUBLIC_API_URL}/files/proxy?filePath=vault/${(data?.pathTokens as string[])?.join("/")}`}
            mimeType={data?.mimetype!}
            maxWidth={565}
          />
        </div>
      </div>

      <div>
        {data?.createdAt && (
          <p className="text-sm text-[#878787] mb-4 line-clamp-2">
            {formatDate(data.createdAt)}
          </p>
        )}

        {fullView && <VaultRelatedFiles />}
      </div>
    </div>
  );
}
