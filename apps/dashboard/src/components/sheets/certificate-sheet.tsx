"use client";

import { useCertificateParams } from "@/hooks/use-certificate-params";
import { useTRPC } from "@/trpc/client";
import { Sheet } from "@midday/ui/sheet";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import React from "react";
import { CertificateEditor } from "../certificates/certificate-editor";

export function CertificateSheet() {
  const trpc = useTRPC();
  const queryClient = useQueryClient();
  const { setParams, type, templateId } = useCertificateParams();
  const isOpen = type === "create" || type === "edit";

  const { data } = useQuery(
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

  return (
    <Sheet open={isOpen} onOpenChange={handleOnOpenChange}>
      <CertificateEditor key={templateId || "create"} initialData={data} mode={type as "create" | "edit"} isSheet />
    </Sheet>
  );
}
