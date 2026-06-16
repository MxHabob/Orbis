import type { RouterOutputs } from "@/ipc/query-adapter";
import { useDocumentFilterParams } from "@/hooks/use-document-filter-params";
import { useDocumentParams } from "@/hooks/use-document-params";
import { useFileUrl } from "@/hooks/use-file-url";
import { downloadFile } from "@/lib/download";
import { useI18n } from "@/locales/client";
import { isStaleProcessing } from "@/utils/document";
import { formatSize } from "@/utils/format";

const isMimeTypeSupportedForProcessing = (mimetype: string) => {
  return ["application/pdf", "image/jpeg", "image/png", "image/webp"].includes(
    mimetype,
  );
};

import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import { Checkbox } from "@orbis/ui/checkbox";
import { cn } from "@orbis/ui/cn";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@orbis/ui/dropdown-menu";
import { Skeleton } from "@orbis/ui/skeleton";
import type { ColumnDef } from "@tanstack/react-table";
import { MoreHorizontal } from "lucide-react";
import { useState } from "react";

export type Document = RouterOutputs["documents"]["get"]["data"][number];

const Header = ({
  id,
}: {
  id: "vault.filename" | "vault.size" | "vault.actions";
}) => {
  const t = useI18n();
  return <span>{t(id)}</span>;
};

function DownloadFileMenuItem({
  documentId,
  filename,
}: {
  documentId: string;
  filename: string;
}) {
  const { url: downloadUrl } = useFileUrl(
    documentId
      ? {
          type: "download",
          documentId,
        }
      : null,
  );
  const [isDownloading, setIsDownloading] = useState(false);
  const t = useI18n();

  const handleDownload = async () => {
    if (!downloadUrl) return;

    try {
      setIsDownloading(true);
      await downloadFile(downloadUrl, filename);
    } catch (error) {
      console.error("Download failed:", error);
    } finally {
      setIsDownloading(false);
    }
  };

  return (
    <DropdownMenuItem
      onClick={handleDownload}
      disabled={!downloadUrl || isDownloading}
    >
      {t("common.download")}
    </DropdownMenuItem>
  );
}

export const columns: ColumnDef<Document>[] = [
  {
    id: "select",
    size: 50,
    minSize: 50,
    maxSize: 50,
    enableResizing: false,
    enableHiding: false,
    enableSorting: false,
    meta: {
      sticky: true,
      skeleton: { type: "checkbox" },
      className:
        "w-[50px] min-w-[50px] md:sticky md:inset-inline-start-0 bg-background group-hover:bg-muted/50 z-20 justify-center !border-inline-end !border-border p-0",
    },
    cell: ({ row }) => (
      <Checkbox
        checked={row.getIsSelected()}
        onCheckedChange={(checked) => {
          if (checked === "indeterminate") {
            row.toggleSelected();
          } else {
            row.toggleSelected(checked);
          }
        }}
      />
    ),
  },
  {
    id: "title",
    header: () => <Header id="vault.filename" />,
    accessorKey: "title",
    size: 450,
    minSize: 180,
    maxSize: 600,
    enableResizing: true,
    meta: {
      sticky: true,
      skeleton: { type: "text", width: "w-52" },
      headerLabel: "vault.filename",
      className:
        "w-[450px] min-w-[180px] md:sticky md:inset-inline-start-[50px] bg-background group-hover:bg-muted/50 z-20 !border-inline-end !border-border",
    },
    cell: ({ row }) => {
      const mimetype = row.original.metadata?.mimetype as string | undefined;
      const isSupported = mimetype
        ? isMimeTypeSupportedForProcessing(mimetype)
        : false;

      const isFailed = row.original.processingStatus === "failed";
      // Document completed but AI classification failed - title is null
      const needsClassification =
        row.original.processingStatus === "completed" && !row.original.title;
      // Get display name - fallback to filename from path
      const displayName =
        row.original.title ?? row.original.name?.split("/").at(-1);

      // Check if document is stuck in processing (pending for >10 minutes since creation)
      const staleProcessing = isStaleProcessing(
        row.original.processingStatus,
        row.original.createdAt,
      );

      // Show skeleton only for recently pending documents (not stale ones)
      const isLoading =
        row.original.processingStatus === "pending" && !staleProcessing;

      if (isLoading) {
        return <Skeleton className="w-52 h-4" />;
      }

      return (
        <span
          className={cn(
            "truncate",
            isSupported && isFailed && "text-destructive",
            isSupported &&
              (needsClassification || staleProcessing) &&
              "text-amber-600 dark:text-amber-500",
          )}
        >
          {displayName}
        </span>
      );
    },
  },
  {
    id: "size",
    header: () => <Header id="vault.size" />,
    accessorKey: "size",
    size: 100,
    minSize: 80,
    maxSize: 150,
    enableResizing: true,
    meta: {
      skeleton: { type: "text", width: "w-12" },
      headerLabel: "vault.size",
      className: "w-[100px] min-w-[80px]",
    },
    cell: ({ row }) => {
      return <span>{formatSize(row.original.metadata?.size)}</span>;
    },
  },
  {
    id: "actions",
    header: () => <Header id="vault.actions" />,
    size: 100,
    minSize: 80,
    maxSize: 100,
    enableResizing: false,
    enableHiding: false,
    meta: {
      sticky: true,
      skeleton: { type: "icon" },
      headerLabel: "vault.actions",
      className:
        "w-[100px] min-w-[80px] md:sticky md:inset-inline-end-0 bg-background group-hover:bg-muted/50 z-30 justify-center !border-inline-start !border-border",
    },
    cell: ({ row, table }) => {
      const t = useI18n();
      const { setParams } = useDocumentParams();

      const mimetype = row.original.metadata?.mimetype as string | undefined;
      const isSupported = mimetype
        ? isMimeTypeSupportedForProcessing(mimetype)
        : false;

      const isFailed = row.original.processingStatus === "failed";
      // Document completed but AI classification failed - title is null
      const needsClassification =
        row.original.processingStatus === "completed" && !row.original.title;
      // Check if document is stuck in processing (pending for >10 minutes since creation)
      const staleProcessing = isStaleProcessing(
        row.original.processingStatus,
        row.original.createdAt,
      );
      // Show retry option only for supported file types
      const showRetry =
        isSupported && (isFailed || needsClassification || staleProcessing);

      if (!table.options.meta) {
        return null;
      }

      const { handleDelete, handleShare, handleReprocess } = table.options.meta;

      return (
        <div className="flex items-center justify-center w-full">
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant="ghost" size="icon">
                <MoreHorizontal size={16} />
              </Button>
            </DropdownMenuTrigger>

            <DropdownMenuContent align="end">
              <DropdownMenuItem
                onClick={() => {
                  setParams({ documentId: row.original.id });
                }}
              >
                {t("vault.view_details")}
              </DropdownMenuItem>
              <DownloadFileMenuItem
                documentId={row.original.id}
                filename={row.original.name?.split("/").at(-1) || "download"}
              />
              <DropdownMenuItem
                onClick={() => {
                  if (row.original.pathTokens) {
                    handleShare?.(row.original.pathTokens);
                  }
                }}
                disabled={!row.original.pathTokens}
              >
                {t("vault.copy_link")}
              </DropdownMenuItem>

              {showRetry && (
                <>
                  <DropdownMenuSeparator />
                  <DropdownMenuItem
                    onClick={() => {
                      handleReprocess?.(row.original.id);
                    }}
                  >
                    {t("vault.reprocess")}
                  </DropdownMenuItem>
                </>
              )}

              <DropdownMenuSeparator />
              <DropdownMenuItem
                className="text-destructive focus:text-destructive"
                onClick={() => {
                  handleDelete?.(row.original.id);
                }}
              >
                {t("common.delete")}
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      );
    },
  },
];
