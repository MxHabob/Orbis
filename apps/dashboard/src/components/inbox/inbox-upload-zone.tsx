"use client";

import { useTRPC } from "@/trpc/client";
import { cn } from "@midday/ui/cn";
import { useToast } from "@midday/ui/use-toast";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { type ReactNode, useEffect, useState } from "react";
import { useDropzone } from "react-dropzone";
import { authClient } from "@midday/auth/auth-client";

type Props = {
  children: ReactNode;
  onUploadComplete?: (inboxId?: string) => void;
};

export function UploadZone({ children, onUploadComplete }: Props) {
  const trpc = useTRPC();
  const { data: session } = authClient.useSession();
  const user = session?.user;
  const queryClient = useQueryClient();
  const [progress, setProgress] = useState(0);
  const [showProgress, setShowProgress] = useState(false);
  const [toastId, setToastId] = useState<string | undefined>(undefined);
  const { toast, dismiss, update } = useToast();
  
  useEffect(() => {
    if (!toastId && showProgress) {
      const { id } = toast({
        title: "جاري رفع الملفات...",
        progress,
        variant: "progress",
        description: "يرجى عدم إغلاق المتصفح حتى اكتمال الرفع",
        duration: Number.POSITIVE_INFINITY,
      });

      if (id) {
        setToastId(id);
      }
    } else if (toastId) {
      update(toastId, {
        id: toastId,
        progress,
        title: "جاري رفع الملفات...",
      });
    }
  }, [showProgress, progress, toastId]);

  const onDrop = async (files: File[]) => {
    if (!files.length) return;

    setShowProgress(true);
    setProgress(10);

    try {
      const results = await Promise.all(
        files.map(async (file: File) => {
          const formData = new FormData();
          formData.append("file", file);
          formData.append("bucket", "vault"); // Using vault bucket for inbox too for now, or unified storage
          formData.append("path", `inbox/${user?.id || "anonymous"}/${file.name}`);

          const response = await fetch("/api/upload", {
            method: "POST",
            body: formData,
          });

          if (!response.ok) throw new Error("Upload failed");

          const data = await response.json();
          return {
            filename: file.name,
            file,
            path: data.path,
          };
        }),
      );

      setProgress(80);



      setProgress(100);
      toast({
        title: "تم الرفع بنجاح",
        variant: "success",
        duration: 2000,
      });

      setShowProgress(false);
      setToastId(undefined);
      if (toastId) dismiss(toastId);
      onUploadComplete?.();
    } catch (error) {

      setShowProgress(false);
      setToastId(undefined);
      if (toastId) dismiss(toastId);

      toast({
        duration: 2500,
        variant: "error",
        title: "حدث خطأ أثناء الرفع، يرجى المحاولة مرة أخرى",
      });
    }
  };

  const { getRootProps, getInputProps, isDragActive } = useDropzone({
    onDrop,
    maxSize: 5000000,
    maxFiles: 25,
    accept: {
      "image/*": [".jpg", ".jpeg", ".png", ".webp", ".heic", ".heif", ".avif"],
      "application/pdf": [".pdf"],
    },
  });

  return (
    <div
      {...getRootProps({ onClick: (evt) => evt.stopPropagation() })}
      className="relative h-full"
    >
      <div className="absolute top-0 bottom-0 right-0 left-0 z-[51] pointer-events-none">
        <div
          className={cn(
            "bg-background h-full flex items-center justify-center text-center invisible",
            isDragActive && "visible",
          )}
        >
          <input {...getInputProps()} id="upload-files" />
          <div className="flex flex-col items-center justify-center gap-2">
            <p className="text-xs">
              أسقط الملفات هنا للرفع <br />
              بحد أقصى 25 ملفاً في المرة الواحدة
            </p>
          </div>
        </div>
      </div>

      {children}
    </div>
  );
}
