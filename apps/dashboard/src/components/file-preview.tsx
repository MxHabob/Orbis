"use client";

import { FilePreviewIcon } from "@/components/file-preview-icon";
import { useFileUrl } from "@/hooks/use-file-url";
import { cn } from "@midday/ui/cn";
import { Icons } from "@midday/ui/icons";
import { Skeleton } from "@midday/ui/skeleton";
import Image from "next/image";
import { useState } from "react";

type Props = {
  documentId?: string;
  mimeType: string;
  filePath: string;
  lazy?: boolean;
  fixedSize?: { width: number; height: number };
};

function ErrorPreview() {
  return (
    <div className="w-full h-full flex items-center justify-center bg-primary/10">
      <div className="flex flex-col items-center justify-center">
        <Icons.BrokenImage className="size-4" />
      </div>
    </div>
  );
}

export function FilePreview({
  documentId,
  mimeType,
  filePath,
  lazy = false,
  fixedSize,
}: Props) {
  const [imageError, setImageError] = useState(false);
  const [imageLoading, setImageLoading] = useState(true);

  const {
    url: src,
    isLoading,
  } = useFileUrl(
    documentId
      ? { documentId }
      : { url: `/api/files/preview?filePath=vault/${filePath}` }
  );

  const isPreviewSupported = mimeType.startsWith("image/") || mimeType === "application/pdf";

  if (!isPreviewSupported) {
    return <FilePreviewIcon mimetype={mimeType} />;
  }

  if (isLoading || !src) {
    return <Skeleton className="w-full h-full" />;
  }

  return (
    <div className="relative w-full h-full flex items-center justify-center">
      {imageLoading && !imageError && (
        <Skeleton className="absolute inset-0 w-full h-full" />
      )}

      {imageError && <ErrorPreview />}

      {!imageError && (
        <Image
          src={src}
          alt="File Preview"
          {...(fixedSize
            ? {
                width: fixedSize.width,
                height: fixedSize.height,
                sizes: `${fixedSize.width}px`,
                unoptimized: true,
              }
            : {
                fill: true,
              })}
          className={cn(
            "object-cover object-top border border-border dark:border-none w-full h-full",
            imageLoading ? "opacity-0" : "opacity-100",
          )}
          loading={lazy ? "lazy" : "eager"}
          priority={!lazy}
          onLoadingComplete={() => setImageLoading(false)}
          onError={() => {
            setImageError(true);
            setImageLoading(false);
          }}
        />
      )}
    </div>
  );
}
