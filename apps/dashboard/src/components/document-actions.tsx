"use client";

import { Button } from "@orbis/ui/button";
import { Icons } from "@orbis/ui/icons";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useState } from "react";
import { useCopyToClipboard } from "usehooks-ts";
import { useDocumentParams } from "@/hooks/use-document-params";
import { useFileUrl } from "@/hooks/use-file-url";
import { downloadFile } from "@/lib/download";
import { useTRPC } from "@/trpc/client";

type Props = {
  showDelete?: boolean;
  filePath?: string[] | null;
  id?: string | null;
};

export function DocumentActions({ showDelete = false, filePath, id }: Props) {
  const [, copy] = useCopyToClipboard();
  const [isCopied, setIsCopied] = useState(false);
  const trpc = useTRPC();
  const queryClient = useQueryClient();
  const { setParams, params } = useDocumentParams();

  const filename = filePath?.at(-1);
  const { url: downloadUrl } = useFileUrl(
    id || params.documentId || filePath
      ? {
          type: "download",
          documentId: (id || params.documentId) as string,
          url:
            !id && !params.documentId && filePath
              ? `${process.env.NEXT_PUBLIC_API_URL}/files/proxy?filePath=vault/${filePath.join("/")}`
              : undefined,
        }
      : null,
  );

  const shortLinkMutation = useMutation(
    trpc.shortLinks.createForDocument.mutationOptions({
      onSuccess: (data) => {
        if (data?.shortUrl) {
          copy(data.shortUrl);
          setIsCopied(true);

          setTimeout(() => {
            setIsCopied(false);
          }, 3000);
        }
      },
    }),
  );

  const deleteDocumentMutation = useMutation(
    trpc.documents.delete.mutationOptions({
      onSuccess: () => {
        setParams({ documentId: null });

        queryClient.invalidateQueries({
          queryKey: trpc.documents.get.infiniteQueryKey(),
        });

        queryClient.invalidateQueries({
          queryKey: trpc.documents.get.queryKey(),
        });

        queryClient.invalidateQueries({
          queryKey: trpc.search.global.queryKey(),
        });
      },
    }),
  );

  return (
    <div className="flex flex-row">
      <Button
        variant="ghost"
        size="icon"
        onClick={() => {
          if (downloadUrl && filename) {
            const url = new URL(downloadUrl, window.location.origin);
            url.searchParams.set("download", "true");
            downloadFile(url.toString(), filename);
          }
        }}
        disabled={!downloadUrl}
      >
        <Icons.ArrowCoolDown className="size-4" />
      </Button>

      <Button
        variant="ghost"
        size="icon"
        onClick={() =>
          shortLinkMutation.mutate({
            filePath: filePath?.join("/") ?? "",
            expireIn: 60 * 60 * 24 * 30, // 30 days
          })
        }
      >
        {isCopied ? (
          <Icons.Check className="size-4" />
        ) : (
          <Icons.Copy className="size-4" />
        )}
      </Button>

      {showDelete && (
        <Button
          variant="ghost"
          size="icon"
          onClick={() =>
            deleteDocumentMutation.mutate({
              id: params.documentId!,
            })
          }
        >
          <Icons.Delete className="size-4" />
        </Button>
      )}
    </div>
  );
}
