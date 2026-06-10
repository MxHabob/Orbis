"use client";

import { usePendingUploadsStore } from "@/store/pending-uploads";
import { useTRPC } from "@/trpc/client";
import { resumableUpload } from "@/utils/upload";
import { authClient } from "@orbis/auth/auth-client";
import { cn } from "@orbis/ui/cn";
import { useToast } from "@orbis/ui/use-toast";
import { stripSpecialCharacters } from "@orbis/utils";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { type ReactNode, useEffect, useRef, useState } from "react";
import { useDropzone } from "react-dropzone";

// Shared toast ID for coordinating between upload zone and data table
export const PROCESSING_TOAST_ID = "transactions-processing";

type UploadResult = {
  filename: string;
  file: File;
};

type ProcessAttachmentInput = {
  filePath: string[];
  mimetype: string;
  size: number;
};

type Props = {
  children: ReactNode;
};

export function GradesUploadZone({ children }: Props) {
  const trpc = useTRPC();
  const { data: session } = authClient.useSession();
  const user = session?.user;
  const queryClient = useQueryClient();
  const [progress, setProgress] = useState(0);
  const [showProgress, setShowProgress] = useState(false);
  const [toastId, setToastId] = useState<string | undefined>(undefined);
  const uploadProgress = useRef<number[]>([]);
  const { toast, dismiss, update } = useToast();
  const addPending = usePendingUploadsStore((state) => state.addPending);

  useEffect(() => {
    if (!toastId && showProgress) {
      const { id } = toast({
        title: `Uploading ${uploadProgress.current.length} files`,
        progress,
        variant: "progress",
        description: "Files will be matched to transactions automatically",
        duration: Number.POSITIVE_INFINITY,
      });

      if (id) {
        setToastId(id);
      }
    } else if (toastId) {
      update(toastId, {
        id: toastId,
        progress,
        title: `Uploading ${uploadProgress.current.length} files`,
      });
    }
  }, [showProgress, progress, toastId]);

  const onDrop = async (files: File[]) => {
    // NOTE: If onDropRejected
    if (!files.length) {
      return;
    }

    // Guard against missing teamId to prevent uploads to incorrect paths
    if (!user?.id) {
      toast({
        duration: 2500,
        variant: "error",
        title: "Unable to upload. Please try again.",
      });
      return;
    }

    // Set default progress
    uploadProgress.current = files.map(() => 0);

    setShowProgress(true);

    const path = [user.id, "inbox"];

    try {
      // First, create inbox items immediately for instant feedback
      const inboxItems = await Promise.all(
        files.map(async (file: File) => {
          // Use the same filename processing as resumableUpload
          const processedFilename = stripSpecialCharacters(file.name);
          const filePath = [...path, processedFilename];
        }),
      );





      // Reset once done
      uploadProgress.current = [];

      setProgress(0);
      setShowProgress(false);
      dismiss(toastId);
      setToastId(undefined);

      // Show persistent processing toast - will be updated by data-table when matches arrive
      const fileCount = files.length;
      toast({
        id: PROCESSING_TOAST_ID,
        title: `Processing ${fileCount} ${fileCount === 1 ? "receipt" : "receipts"}...`,
        description: "Looking for matching transactions",
        variant: "spinner",
        duration: Number.POSITIVE_INFINITY, // Persistent until updated
      });
    } catch (error) {


      setShowProgress(false);
      setToastId(undefined);
      dismiss(toastId);

      toast({
        duration: 2500,
        variant: "error",
        title: "Something went wrong please try again.",
      });
    }
  };

  const { getRootProps, getInputProps, isDragActive } = useDropzone({
    onDrop,
    onDropRejected: ([reject]) => {
      if (reject?.errors.find(({ code }) => code === "file-too-large")) {
        toast({
          duration: 2500,
          variant: "error",
          title: "File size too large.",
        });
      }

      if (reject?.errors.find(({ code }) => code === "file-invalid-type")) {
        toast({
          duration: 2500,
          variant: "error",
          title: "File type not supported.",
        });
      }
    },
    maxSize: 5000000, // 5MB
    maxFiles: 25,
    noClick: true, // Don't open file dialog on click - only drag & drop
    accept: {
      "image/*": [".jpg", ".jpeg", ".png", ".webp", ".heic", ".heif", ".avif"],
      "application/pdf": [".pdf"],
    },
  });

  return (
    <div {...getRootProps()} className="relative">
      <div className="absolute top-0 bottom-0 right-0 left-0 z-[51] pointer-events-none">
        <div
          className={cn(
            "bg-background h-full flex items-center justify-center text-center invisible",
            isDragActive && "visible",
          )}
        >
          <input {...getInputProps()} id="upload-transaction-files" />
          <p className="text-xs">
            Drop your receipts here. <br />
            They will be matched to transactions automatically.
            <br />
            <span className="text-[#878787]">
              Maximum of 25 files at a time.
            </span>
          </p>
        </div>
      </div>

      {children}
    </div>
  );
}
