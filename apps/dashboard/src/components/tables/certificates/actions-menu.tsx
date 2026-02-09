"use client";

import { useTRPC } from "@/trpc/client";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@midday/ui/dropdown-menu";
import { Button } from "@midday/ui/button";
import { DotsHorizontalIcon } from "@radix-ui/react-icons";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useToast } from "@midday/ui/use-toast";
import { useRouter } from "next/navigation";
import { Star, Trash, Edit } from "lucide-react";
import type { CertificateTemplate } from "./types";

type Props = {
  row: CertificateTemplate;
};

export function ActionsMenu({ row }: Props) {
  const trpc = useTRPC();
  const queryClient = useQueryClient();
  const { toast } = useToast();
  const router = useRouter();

  const deleteMutation = useMutation(
    trpc.certificateTemplates.delete.mutationOptions({
      onSuccess: () => {
        queryClient.invalidateQueries({
          queryKey: trpc.certificateTemplates.list.queryKey(),
        });
        toast({
          title: "Template deleted",
          variant: "success",
        });
      },
    })
  );

  const setDefaultMutation = useMutation(
    trpc.certificateTemplates.setDefault.mutationOptions({
      onSuccess: () => {
         queryClient.invalidateQueries({
          queryKey: trpc.certificateTemplates.list.queryKey(),
        });
        toast({
          title: "Default template updated",
          variant: "success",
        });
      },
    })
  );

  return (
    <div className="flex items-center justify-center w-full">
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button variant="ghost" className="h-8 w-8 p-0">
            <DotsHorizontalIcon className="h-4 w-4" />
          </Button>
        </DropdownMenuTrigger>

        <DropdownMenuContent align="end">
          <DropdownMenuItem onClick={() => router.push(`/certificates/${row.id}`)}>
            <Edit className="w-4 h-4 mr-2" />
            Edit Template
          </DropdownMenuItem>
          
          <DropdownMenuItem onClick={() => setDefaultMutation.mutate({ id: row.id })}>
            <Star className="w-4 h-4 mr-2" />
            Set as Default
          </DropdownMenuItem>
          
          <DropdownMenuItem 
            className="text-destructive focus:text-destructive" 
            onClick={() => deleteMutation.mutate({ id: row.id })}
          >
            <Trash className="w-4 h-4 mr-2" />
            Delete Template
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  );
}
