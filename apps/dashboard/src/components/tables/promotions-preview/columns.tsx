"use client";

import { Badge } from "@orbis/ui/badge";
import { cn } from "@orbis/ui/cn";
import { createColumnHelper } from "@tanstack/react-table";
import { GraduationCap, School, User } from "lucide-react";
import React from "react";
import type { useI18n } from "@/locales/client";

export interface PromotionPreview {
  studentId: string;
  name: string;
  finalScore: number | null;
  currentStatus: string;
  suggestedStatus: string;
  targetClassId: number | null;
  targetClassName: string | null;
}

const columnHelper = createColumnHelper<PromotionPreview>();

export const columns = (t: ReturnType<typeof useI18n>) => [
  columnHelper.accessor("name", {
    header: t("students.full_name"),
    cell: (info) => (
      <div className="flex items-center gap-3">
        <div className="size-6 bg-accent/50 flex items-center justify-center text-[#606060] border border-border/50">
          <User size={12} />
        </div>
        <span className="text-[11px] font-medium">{info.getValue()}</span>
      </div>
    ),
  }),
  columnHelper.accessor("finalScore", {
    header: t("promotions.final_score"),
    cell: (info) => {
      const score = info.getValue();
      return score !== null ? (
        <span
          className={cn(
            "text-[10px] font-mono font-bold",
            score < 50 ? "text-red-500" : "text-emerald-500",
          )}
        >
          {score.toFixed(0)}%
        </span>
      ) : (
        <span className="text-[#606060] italic text-[10px]">-</span>
      );
    },
  }),
  columnHelper.accessor("suggestedStatus", {
    header: t("promotions.decision"),
    cell: (info) => {
      const status = info.getValue();
      switch (status) {
        case "promoted":
          return (
            <Badge
              variant="success"
              className="rounded-none text-[9px] px-1 h-4"
            >
              {t("promotions.status_promoted")}
            </Badge>
          );
        case "retained":
          return (
            <Badge
              variant="destructive"
              className="rounded-none bg-red-500/10 text-red-500 hover:bg-red-500/20 text-[9px] px-1 h-4"
            >
              {t("promotions.status_retained")}
            </Badge>
          );
        case "graduated":
          return (
            <Badge
              variant="secondary"
              className="rounded-none bg-blue-500/10 text-blue-500 hover:bg-blue-500/20 text-[9px] px-1 h-4"
            >
              {t("promotions.status_graduated")}
            </Badge>
          );
        case "conditional":
          return (
            <Badge
              variant="outline"
              className="rounded-none text-[9px] px-1 h-4"
            >
              {t("promotions.status_conditional")}
            </Badge>
          );
        default:
          return (
            <Badge
              variant="outline"
              className="rounded-none text-[9px] px-1 h-4 uppercase"
            >
              {status}
            </Badge>
          );
      }
    },
  }),
  columnHelper.accessor("targetClassName", {
    header: t("promotions.target_class"),
    cell: (info) => {
      const name = info.getValue();
      const row = info.row.original;
      if (name) {
        return (
          <div className="flex items-center gap-2 text-[10px] font-medium text-[#606060]">
            <School size={12} />
            {name}
          </div>
        );
      }
      if (row.suggestedStatus === "graduated") {
        return (
          <span className="flex items-center gap-2 text-[10px] text-blue-500 font-bold uppercase">
            <GraduationCap size={12} /> {t("students.graduated")}
          </span>
        );
      }
      return <span className="text-[#606060] text-[10px] italic">-</span>;
    },
  }),
];
