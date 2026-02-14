"use client";

import { useTRPC } from "@/trpc/client";
import { cn } from "@midday/ui/cn";
import { useToast } from "@midday/ui/use-toast";
import { useMutation } from "@tanstack/react-query";
import { type ReactNode, useEffect, useRef, useState } from "react";
import { type FileRejection, useDropzone } from "react-dropzone";
import { authClient } from "@midday/auth/auth-client";

type UploadResult = {
  filename: string;
  file: File;
};

type Props = {
  children: ReactNode;
  onUpload?: (
    results: {
      file_path: string[];
      mimetype: string;
      size: number;
    }[],
  ) => void;
};

export function VaultUploadZone({ onUpload, children }: Props) {
  const trpc = useTRPC();
  const { data: session } = authClient.useSession();
  const [progress, setProgress] = useState(0);
  const [showProgress, setShowProgress] = useState(false);
  const [toastId, setToastId] = useState<string | null>(null);
  const uploadProgress = useRef<number[]>([]);
  const { toast, dismiss, update } = useToast();

  const processDocumentMutation = useMutation(
    trpc.documents.processDocument.mutationOptions(),
  );

  useEffect(() => {
    if (!toastId && showProgress) {
      const { id } = toast({
        title: `جاري رفع الملفات...`,
        progress,
        variant: "progress",
        description: "يرجى عدم إغلاق المتصفح حتى اكتمال الرفع",
        duration: Number.POSITIVE_INFINITY,
      });

      setToastId(id);
    } else if (toastId) {
      update(toastId, {
        id: toastId,
        title: `جاري رفع الملفات...`,
        progress,
        variant: "progress",
      });
    }
  }, [showProgress, progress, toastId]);

  const onDrop = async (files: File[]) => {
    if (!files.length) return;

    setShowProgress(true);
    setProgress(10); // Start progress

    try {
      const results = await Promise.all(
        files.map(async (file: File) => {
          const formData = new FormData();
          formData.append("file", file);
          formData.append("bucket", "vault");
          formData.append("path", `${session?.user.id || "anonymous"}/${file.name}`);

          const response = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/api/upload`, {
            method: "POST",
            body: formData,
            credentials: "include",
          });

          if (!response.ok) throw new Error("Upload failed");

          const data = await response.json();
          return {
            filename: file.name,
            file,
            path: data.path, // The key returned by storage
          };
        })
      );

      setProgress(80);

      // Trigger the metadata processing in DB
      await processDocumentMutation.mutateAsync(
        results.map((result) => ({
          filePath: result.path.split("/"),
          mimetype: result.file.type,
          size: result.file.size,
        }))
      );

      setProgress(100);
      toast({
        title: "تم رفع الملفات بنجاح",
        variant: "success",
        duration: 2000,
      });

      setShowProgress(false);
      setToastId(null);
      if (toastId) dismiss(toastId);
    } catch (error) {
      toast({
        duration: 2500,
        variant: "error",
        title: "حدث خطأ أثناء الرفع، يرجى المحاولة مرة أخرى",
      });
      setShowProgress(false);
      setToastId(null);
    }
  };

  const { getRootProps, getInputProps, isDragActive } = useDropzone({
    onDrop,
    onDropRejected: ([reject]: FileRejection[]) => {
      if (reject?.errors.find(({ code }) => code === "file-too-large")) {
        toast({
          duration: 2500,
          variant: "error",
          title: "File size to large.",
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
    maxSize: 50000000, // 50MB
    maxFiles: 25,
    accept: {
      "image/*": [".jpg", ".jpeg", ".png", ".webp", ".heic", ".heif", ".avif"],
      "application/pdf": [".pdf"],
      "application/msword": [".doc"],
      "application/vnd.openxmlformats-officedocument.wordprocessingml.document":
        [".docx"],
      "application/vnd.oasis.opendocument.text": [".odt"],
      "application/vnd.ms-excel": [".xls"],
      "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet": [
        ".xlsx",
      ],
      "application/vnd.oasis.opendocument.spreadsheet": [".ods"],
      "application/vnd.ms-powerpoint": [".ppt"],
      "application/vnd.openxmlformats-officedocument.presentationml.presentation":
        [".pptx"],
      "application/vnd.oasis.opendocument.presentation": [".odp"],
      "text/plain": [".txt"],
      "text/csv": [".csv"],
      "text/markdown": [".md"],
      "application/rtf": [".rtf"],
      "application/zip": [".zip"],
    },
  });

  return (
    <div
      className="relative h-full"
      {...getRootProps({ onClick: (evt) => evt.stopPropagation() })}
    >
      <div className="absolute top-0 right-0 left-0 z-[51] w-full pointer-events-none h-[calc(100vh-150px)]">
        <div
          className={cn(
            "bg-background h-full w-full flex items-center justify-center text-center",
            isDragActive ? "visible" : "invisible",
          )}
        >
          <input {...getInputProps()} id="upload-files" />

          <div className="flex flex-col items-center justify-center gap-2">
            <p className="text-sm">
              قم بسحب وإفلات الملفات هنا. <br />
              بحد أقصى 25 ملفاً في المرة الواحدة.
            </p>

            <span className="text-xs text-[#878787]">الحد الأقصى لحجم الملف 5 ميجابايت</span>
          </div>
        </div>
      </div>

      {children}
    </div>
  );
}
