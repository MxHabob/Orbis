"use client";

import { Button } from "@orbis/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@orbis/ui/dialog";
import { Label } from "@orbis/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@orbis/ui/select";
import { Textarea } from "@orbis/ui/textarea";
import { useToast } from "@orbis/ui/use-toast";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { Loader2 } from "lucide-react";
import { useEffect, useState } from "react";
import { useI18n } from "@/locales/client";
import { useTRPC } from "@/trpc/client";

interface PromotionHistory {
  studentId: string;
  studentName: string | null;
  fromClassId: number;
  toClassId: number | null;
  fromAcademicYearId: number;
  toAcademicYearId: number;
  status: string;
  notes: string | null;
}

interface OverridePromotionModalProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  record: PromotionHistory | null;
  classes: any[];
}

export function OverridePromotionModal({
  open,
  onOpenChange,
  record,
  classes,
}: OverridePromotionModalProps) {
  const t = useI18n();
  const trpc = useTRPC();
  const { toast } = useToast();
  const queryClient = useQueryClient();

  const [status, setStatus] = useState<string>("");
  const [toClassId, setToClassId] = useState<string>("");
  const [notes, setNotes] = useState("");

  useEffect(() => {
    if (record && open) {
      setStatus(record.status);
      setToClassId(
        record.toClassId?.toString() || record.fromClassId.toString(),
      );
      setNotes(record.notes || "");
    }
  }, [record, open]);

  const overrideMutation = useMutation(
    trpc.promotions.override.mutationOptions({
      onSuccess: () => {
        toast({ title: t("promotions.process_success") });
        queryClient.invalidateQueries({
          queryKey: trpc.promotions.historyRecords.queryKey(),
        });
        queryClient.invalidateQueries({
          queryKey: trpc.promotions.statistics.queryKey(),
        });
        onOpenChange(false);
      },
      onError: (err) => {
        toast({
          title: t("common.error"),
          description: err.message,
          variant: "destructive",
        });
      },
    }),
  );

  if (!record) return null;

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!status || (status !== "graduated" && !toClassId)) return;

    overrideMutation.mutate({
      studentId: record.studentId,
      fromClassId: record.fromClassId,
      toClassId:
        status === "graduated" ? record.fromClassId : parseInt(toClassId),
      fromAcademicYearId: record.fromAcademicYearId,
      toAcademicYearId: record.toAcademicYearId,
      status: status as any,
      notes,
    });
  };

  const showClassSelect = status !== "graduated";

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[425px] rounded-none border-border shadow-none p-0 overflow-hidden">
        <DialogHeader className="p-6 pb-2">
          <DialogTitle className="text-lg font-bold tracking-tight">
            {t("promotions.override_decision")}
          </DialogTitle>
          <DialogDescription className="text-sm">
            تعديل يدوي لقرار الترفيع للطالب{" "}
            <strong className="text-foreground">{record.studentName}</strong>.
          </DialogDescription>
        </DialogHeader>

        <form onSubmit={handleSubmit} className="p-6 pt-2 space-y-4">
          <div className="space-y-1.5">
            <Label
              htmlFor="status"
              className="text-[10px] text-[#606060] font-bold uppercase tracking-wider"
            >
              {t("promotions.decision")}
            </Label>
            <Select value={status} onValueChange={setStatus}>
              <SelectTrigger
                id="status"
                className="bg-transparent h-9 text-[11px] rounded-none border-border shadow-none focus:ring-0"
              >
                <SelectValue placeholder={t("promotions.decision")} />
              </SelectTrigger>
              <SelectContent className="rounded-none border-border">
                <SelectItem value="promoted" className="text-xs">
                  {t("promotions.status_promoted")}
                </SelectItem>
                <SelectItem value="retained" className="text-xs">
                  {t("promotions.status_retained")}
                </SelectItem>
                <SelectItem value="graduated" className="text-xs">
                  {t("promotions.status_graduated")}
                </SelectItem>
                <SelectItem value="conditional" className="text-xs">
                  {t("promotions.status_conditional")}
                </SelectItem>
              </SelectContent>
            </Select>
          </div>

          {showClassSelect && (
            <div className="space-y-1.5">
              <Label
                htmlFor="class"
                className="text-[10px] text-[#606060] font-bold uppercase tracking-wider"
              >
                {t("promotions.target_class")}
              </Label>
              <Select value={toClassId} onValueChange={setToClassId}>
                <SelectTrigger
                  id="class"
                  className="bg-transparent h-9 text-[11px] rounded-none border-border shadow-none focus:ring-0"
                >
                  <SelectValue placeholder={t("students.select_class")} />
                </SelectTrigger>
                <SelectContent className="rounded-none border-border">
                  {classes.map((c) => (
                    <SelectItem
                      key={c.id}
                      value={c.id.toString()}
                      className="text-xs"
                    >
                      {c.name} {c.division ? `- ${c.division}` : ""}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
          )}

          <div className="space-y-1.5">
            <Label
              htmlFor="notes"
              className="text-[10px] text-[#606060] font-bold uppercase tracking-wider"
            >
              {t("students.note")}
            </Label>
            <Textarea
              id="notes"
              className="resize-none h-24 bg-transparent text-[11px] rounded-none border-border shadow-none focus-visible:ring-0"
              placeholder={t("students.additional_info")}
              value={notes}
              onChange={(e) => setNotes(e.target.value)}
            />
          </div>

          <DialogFooter className="pt-2 gap-2">
            <Button
              type="button"
              variant="outline"
              onClick={() => onOpenChange(false)}
              disabled={overrideMutation.isPending}
              className="rounded-none h-9 text-[11px] flex-1 bg-transparent hover:bg-muted"
            >
              {t("common.cancel")}
            </Button>
            <Button
              type="submit"
              disabled={
                overrideMutation.isPending ||
                !status ||
                (showClassSelect && !toClassId)
              }
              className="rounded-none h-9 text-[11px] px-8 flex-1"
            >
              {overrideMutation.isPending && (
                <Loader2 className="mr-2 h-3 w-3 animate-spin" />
              )}
              {t("common.save")}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
