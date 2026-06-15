"use client";

import { Button } from "@orbis/ui/button";
import { useToast } from "@orbis/ui/use-toast";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { Trash } from "lucide-react";
import { useReportStore } from "@/store/report";
import { useTRPC } from "@/ipc/query-adapter";
import { BottomBar as CoreBottomBar } from "../core/bottom-bar";

type Props = {
  data: any[];
};

export function BottomBar({ data }: Props) {
  const { rowSelection, setRowSelection } = useReportStore();
  const trpc = useTRPC();
  const queryClient = useQueryClient();
  const { toast } = useToast();

  const selectedIds = Object.keys(rowSelection).filter(
    (id) => rowSelection[id],
  );
  const count = selectedIds.length;

  // Bulk delete mutation
  const deleteMutation = useMutation(
    trpc.certificateTemplates.delete.mutationOptions({
      onSuccess: () => {
        queryClient.invalidateQueries({
          queryKey: trpc.certificateTemplates.list.queryKey(),
        });
        queryClient.invalidateQueries({
          queryKey: trpc.certificateTemplates.summary.queryKey(),
        });
        setRowSelection({});
        toast({
          title: `${count} templates deleted`,
          variant: "success",
        });
      },
    }),
  );

  const handleDelete = async () => {
    try {
      await Promise.all(
        selectedIds.map((id) => deleteMutation.mutateAsync({ id })),
      );
    } catch (error) {
      toast({
        title: "Error deleting templates",
        variant: "destructive",
      });
    }
  };

  if (count === 0) return null;

  return (
    <CoreBottomBar selectedCount={count} onDeselect={() => setRowSelection({})}>
      <Button
        variant="ghost"
        size="sm"
        className="text-destructive hover:text-destructive hover:bg-destructive/10 h-8"
        onClick={handleDelete}
        disabled={deleteMutation.isPending}
      >
        <Trash className="w-4 h-4 mr-2" />
        Delete
      </Button>
    </CoreBottomBar>
  );
}
