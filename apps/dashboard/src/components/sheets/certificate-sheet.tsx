"use client";

import { useCertificateParams } from "@/hooks/use-certificate-params";
import { useTRPC } from "@/trpc/client";
import { Sheet, SheetContent } from "@midday/ui/sheet";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import React from "react";
import { CertificateEditor } from "../certificates/certificate-editor";
import type { PaperSize, EditorElement } from "../certificates/editor/types";

export function CertificateSheet() {
  const trpc = useTRPC();
  const queryClient = useQueryClient();
  const { setParams, type, templateId } = useCertificateParams();
  const isOpen = type === "create" || type === "edit";

  const { data, isLoading } = useQuery(
    trpc.certificateTemplates.getById.queryOptions(
      {
        id: templateId!,
      },
      {
        enabled: !!templateId,
      },
    ),
  );

  const handleOnOpenChange = (open: boolean) => {
    if (!open) {
      queryClient.invalidateQueries({
        queryKey: trpc.certificateTemplates.list.queryKey(),
      });
    }

    setParams(null);
  };

  const initialData = data ? {
    id: data.id,
    name: data.name,
    size: data.size as PaperSize,
    orientation: data.orientation as "portrait" | "landscape",
    backgroundImageUrl: data.backgroundImageUrl ?? undefined,
    content: (data.content as any) || undefined,
  } : undefined;

  return (
    <Sheet open={isOpen} onOpenChange={handleOnOpenChange}>
      {type === "edit" && isLoading ? (
        <SheetContent
          style={{ maxWidth: 1400 }}
          className="bg-white dark:bg-[#080808] p-0 [&>button]:hidden"
        >
          <div className="flex h-full w-full items-center justify-center">
            <div className="h-6 w-6 animate-spin rounded-full border-2 border-gray-900 border-t-transparent dark:border-gray-50" />
          </div>
        </SheetContent>
      ) : (
        <CertificateEditor
          key={templateId || "create"}
          initialData={initialData}
          mode={type as "create" | "edit"}
          isSheet
        />
      )}
    </Sheet>
  );
}
