"use client";

import { useI18n } from "@/locales/client";
import { Badge } from "@orbis/ui/badge";
import { createColumnHelper } from "@tanstack/react-table";
import { format } from "date-fns";
import { User, ArrowRight } from "lucide-react";
import React from "react";
import { ActionsMenu } from "./actions-menu";

export interface PromotionHistory {
  id: string | number;
  studentId: string;
  studentName: string | null;
  studentNumber: string | null;
  fromClassId: number;
  toClassId: number | null;
  fromAcademicYearId: number;
  toAcademicYearId: number;
  finalScore: string | null;
  status: string;
  decision: string;
  notes: string | null;
  promotedAt: Date | null;
  fromClassName: string | null;
  toClassName: string | null;
  fromYearName: string | null;
  toYearName: string | null;
}

const columnHelper = createColumnHelper<PromotionHistory>();

export const columns = (
  t: ReturnType<typeof useI18n>,
  onRollback: (studentIds: string[]) => void,
  onOverride: (history: PromotionHistory) => void,
  isRollingBack?: boolean
) => [
  columnHelper.accessor("studentName", {
    header: t("students.full_name"),
    cell: (info) => (
      <div className="flex items-center gap-3">
        <div className="size-6 bg-accent/50 flex items-center justify-center text-[#606060] border border-border/50">
          <User size={12} />
        </div>
        <div className="flex flex-col min-w-0">
           <span className="text-[11px] font-medium truncate">{info.getValue()}</span>
           {info.row.original.studentNumber && (
             <span className="text-[9px] text-[#606060] font-mono leading-none mt-0.5">
               {info.row.original.studentNumber}
             </span>
           )}
        </div>
      </div>
    ),
  }),
  columnHelper.accessor("fromClassName", {
    header: t("promotions.current_year"),
    cell: (info) => (
      <div className="text-[10px] font-medium text-[#606060]">{info.getValue()}</div>
    ),
  }),
  columnHelper.accessor("toClassName", {
    header: t("promotions.target_year"),
    cell: (info) => (
      <div className="flex items-center gap-2 text-[10px]">
        <ArrowRight size={10} className="text-[#606060] flex-shrink-0" />
        <span className="font-medium text-[#606060]">
          {info.getValue() || <span className="italic text-[#606060]">-</span>}
        </span>
      </div>
    ),
  }),
  columnHelper.accessor("status", {
    header: t("promotions.decision"),
    cell: (info) => {
      const status = info.getValue();
      switch (status) {
        case "promoted":
          return <Badge variant="success" className="rounded-none text-[9px] px-1 h-4">{t("promotions.status_promoted")}</Badge>;
        case "retained":
          return <Badge variant="destructive" className="rounded-none bg-red-500/10 text-red-500 hover:bg-red-500/20 text-[9px] px-1 h-4">{t("promotions.status_retained")}</Badge>;
        case "graduated":
          return <Badge variant="secondary" className="rounded-none bg-blue-500/10 text-blue-500 hover:bg-blue-500/20 text-[9px] px-1 h-4">{t("promotions.status_graduated")}</Badge>;
        case "conditional":
          return <Badge variant="outline" className="rounded-none text-[9px] px-1 h-4">{t("promotions.status_conditional")}</Badge>;
        default:
          return <Badge variant="outline" className="rounded-none text-[9px] px-1 h-4 uppercase">{status}</Badge>;
      }
    },
  }),
  columnHelper.accessor("promotedAt", {
    header: (t as any)("promotions.promoted_at") || "Date",
    cell: (info) => {
      const date = info.getValue();
      return (
        <div className="text-[10px] text-[#606060]">
          {date ? format(new Date(date), "MMM d, HH:mm") : "-"}
        </div>
      );
    },
  }),
  columnHelper.display({
    id: "actions",
    header: "",
    cell: (info) => (
      <div className="flex items-center justify-end">
        <ActionsMenu
          record={info.row.original}
          onRollback={onRollback}
          onOverride={onOverride}
          isRollingBack={isRollingBack}
        />
      </div>
    ),
  }),
];
