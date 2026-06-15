import { Skeleton } from "@orbis/ui/skeleton";
import { lazyNamed, withSuspense } from "@/lib/lazy-component";
import { useFileUrl } from "@/hooks/use-file-url";
import { FilePreviewIcon } from "./file-preview-icon";

const DynamicImageViewer = withSuspense(
  lazyNamed(
    () => import("@/components/image-viewer"),
    "ImageViewer",
  ),
  <Skeleton className="h-full w-full" />,
);

const DynamicPdfViewer = withSuspense(
  lazyNamed(() => import("@/components/pdf-viewer"), "PdfViewer"),
  <Skeleton className="h-full w-full" />,
);

type Props = {
  mimeType: string | null;
  url: string;
  maxWidth?: number;
};

export function FileViewer({ mimeType, url, maxWidth }: Props) {
  const needsAuth = url.includes("/files/proxy");
  const isLocalPreview = url.includes("/api/files/preview");

  const {
    url: finalUrl,
    isLoading,
    hasFileKey,
  } = useFileUrl(
    needsAuth && !isLocalPreview
      ? {
          type: "url",
          url,
        }
      : null,
  );

  if (needsAuth && !isLocalPreview && (isLoading || !hasFileKey)) {
    return <Skeleton className="h-full w-full" />;
  }

  const displayUrl = finalUrl || url;
  const isPreviewImage = isLocalPreview;

  if (
    (mimeType === "application/pdf" ||
      mimeType === "application/octet-stream") &&
    !isPreviewImage
  ) {
    return (
      <DynamicPdfViewer url={displayUrl} key={displayUrl} maxWidth={maxWidth} />
    );
  }

  if (mimeType?.startsWith("image/") || isPreviewImage) {
    return <DynamicImageViewer url={displayUrl} key={displayUrl} />;
  }

  return (
    <div className="size-16">
      <FilePreviewIcon mimetype={mimeType} />
    </div>
  );
}
