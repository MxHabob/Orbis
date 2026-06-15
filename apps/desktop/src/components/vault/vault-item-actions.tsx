"use client";

import { Button } from "@orbis/ui/button";
import { Icons } from "@orbis/ui/icons";
import { useMutation } from "@tanstack/react-query";
import { useState } from "react";
import { useCopyToClipboard } from "usehooks-ts";
import { useFileUrl } from "@/hooks/use-file-url";
import { downloadFile } from "@/lib/download";
import { getFileProxyUrl } from "@/utils/files";
import { useTRPC } from "@/ipc/query-adapter";
import { DeleteVaultFileDialog } from "./delete-vault-file-dialog";

type Props = {
  id: string;
  filePath: string[];
  hideDelete?: boolean;
};

export function VaultItemActions({ id, filePath, hideDelete }: Props) {
  const [, copy] = useCopyToClipboard();
  const [isCopied, setIsCopied] = useState(false);
  const [showDeleteDialog, setShowDeleteDialog] = useState(false);
  const trpc = useTRPC();

  const filename = filePath?.at(-1);
  const { url: downloadUrl } = useFileUrl(
    id || filePath
      ? {
          type: "download",
          documentId: id as string,
          url:
            !id && filePath
              ? getFileProxyUrl(`vault/${filePath.join("/")}`)
              : undefined,
        }
      : null,
  );

  const handleShare = () => {
    if (downloadUrl) {
      copy(downloadUrl);
      setIsCopied(true);
      setTimeout(() => setIsCopied(false), 2000);
    }
  };

  return (
    <div className="flex flex-row gap-2">
      <Button
        variant="outline"
        size="icon"
        className="rounded-full size-7 bg-background"
        onClick={() => {
          if (downloadUrl && filename) {
            const url = new URL(downloadUrl, window.location.origin);
            url.searchParams.set("download", "true");
            downloadFile(url.toString(), filename);
          }
        }}
        disabled={!downloadUrl}
      >
        <Icons.ArrowCoolDown className="size-3.5" />
      </Button>

      <Button
        variant="outline"
        size="icon"
        type="button"
        onClick={handleShare}
        className="rounded-full size-7 bg-background"
      >
        {isCopied ? (
          <Icons.Check className="size-3.5 -mt-0.5" />
        ) : (
          <Icons.Copy className="size-3.5" />
        )}
      </Button>

      {!hideDelete && (
        <Button
          variant="outline"
          size="icon"
          className="rounded-full size-7 bg-background"
          onClick={() => setShowDeleteDialog(true)}
        >
          <Icons.Delete className="size-3.5" />
        </Button>
      )}

      <DeleteVaultFileDialog
        id={id}
        filePath={filePath}
        isOpen={showDeleteDialog}
        onOpenChange={setShowDeleteDialog}
      />
    </div>
  );
}
