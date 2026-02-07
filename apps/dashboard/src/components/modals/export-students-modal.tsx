"use client";

import { Button } from "@midday/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@midday/ui/dialog";
import { Icons } from "@midday/ui/icons";
import { Label } from "@midday/ui/label";
import { RadioGroup, RadioGroupItem } from "@midday/ui/radio-group";
import { Spinner } from "@midday/ui/spinner";
import { useTRPC } from "@/trpc/client";
import { useMutation } from "@tanstack/react-query";
import { useState } from "react";
import { useToast } from "@midday/ui/use-toast";

interface ExportStudentsModalProps {
  isOpen: boolean;
  onOpenChange: (open: boolean) => void;
  classId: string;
  className: string;
}

export function ExportStudentsModal({
  isOpen,
  onOpenChange,
  classId,
  className,
}: ExportStudentsModalProps) {
  const [format, setFormat] = useState<"xlsx" | "csv">("xlsx");
  const { toast } = useToast();
  const trpc = useTRPC();

  const exportMutation = useMutation(
    trpc.students.exportByClassId.mutationOptions({
      onSuccess: (data) => {
        // Convert base64 to blob and download
        const byteCharacters = atob(data.data);
        const byteNumbers = new Array(byteCharacters.length);
        for (let i = 0; i < byteCharacters.length; i++) {
          byteNumbers[i] = byteCharacters.charCodeAt(i);
        }
        const byteArray = new Uint8Array(byteNumbers);
        const blob = new Blob([byteArray], { type: data.mimeType });

        // Create download link
        const url = window.URL.createObjectURL(blob);
        const link = document.createElement("a");
        link.href = url;
        link.download = data.filename;
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
        window.URL.revokeObjectURL(url);

        toast({
          title: "Export Successful",
          description: `Students data has been exported successfully as ${format.toUpperCase()}.`,
        });

        onOpenChange(false);
      },
      onError: (error) => {
        toast({
          title: "Export Failed",
          description:
            error.message || "An error occurred while exporting students data.",
          variant: "destructive",
        });
      },
    })
  );

  const handleExport = () => {
    exportMutation.mutate({
      classId,
      format,
    });
  };

  return (
    <Dialog open={isOpen} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-[400px]">
        <div className="p-4">
          <DialogHeader className="mb-6">
            <DialogTitle>Export Students</DialogTitle>
            <DialogDescription>
              Export students from <strong>{className}</strong> to a file.
            </DialogDescription>
          </DialogHeader>

          <div className="space-y-4">
            <div className="space-y-3">
              <Label className="text-sm font-medium">Export Format</Label>
              <RadioGroup value={format} onValueChange={(v) => setFormat(v as "xlsx" | "csv")}>
                <div className="flex items-center space-x-2 p-3 border rounded-lg hover:bg-accent/50 cursor-pointer transition">
                  <RadioGroupItem value="xlsx" id="xlsx" />
                  <Label
                    htmlFor="xlsx"
                    className="flex-1 cursor-pointer font-normal"
                  >
                    <div className="flex items-center gap-2">
                      <div>
                        <div className="font-medium">Excel (XLSX)</div>
                        <div className="text-xs text-muted-foreground">
                          Formatted spreadsheet with colors and statistics
                        </div>
                      </div>
                    </div>
                  </Label>
                </div>

                <div className="flex items-center space-x-2 p-3 border rounded-lg hover:bg-accent/50 cursor-pointer transition">
                  <RadioGroupItem value="csv" id="csv" />
                  <Label
                    htmlFor="csv"
                    className="flex-1 cursor-pointer font-normal"
                  >
                    <div className="flex items-center gap-2">
                      <div>
                        <div className="font-medium">CSV</div>
                        <div className="text-xs text-muted-foreground">
                          Plain text format compatible with all systems
                        </div>
                      </div>
                    </div>
                  </Label>
                </div>
              </RadioGroup>
            </div>

            <div className="flex justify-end gap-2 pt-4">
              <Button
                type="button"
                variant="outline"
                onClick={() => onOpenChange(false)}
                disabled={exportMutation.isPending}
              >
                Cancel
              </Button>
              <Button
                type="button"
                onClick={handleExport}
                disabled={exportMutation.isPending}
              >
                {exportMutation.isPending ? (
                  <div className="flex items-center space-x-2">
                    <Spinner className="size-4" />
                    <span>Exporting...</span>
                  </div>
                ) : (
                  <div className="flex items-center space-x-2">
                    <Icons.ArrowDownward className="size-4" />
                    <span>Export</span>
                  </div>
                )}
              </Button>
            </div>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
