"use client";

import { useI18n } from "@/locales/client";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@midday/ui/table";
import { Skeleton } from "@midday/ui/skeleton";

export function HistoryTableSkeleton() {
  const t = useI18n();

  return (
    <div className="w-full">
      <Table>
        <TableHeader>
          <TableRow className="bg-muted/30 border-b border-border h-10">
            <TableHead className="w-[250px] text-[10px] font-bold uppercase text-[#606060]">{t("students.full_name")}</TableHead>
            <TableHead className="text-[10px] font-bold uppercase text-[#606060]">{t("promotions.current_year")}</TableHead>
            <TableHead className="text-[10px] font-bold uppercase text-[#606060]">{t("promotions.target_year")}</TableHead>
            <TableHead className="text-center text-[10px] font-bold uppercase text-[#606060]">{t("promotions.decision")}</TableHead>
            <TableHead className="text-[10px] font-bold uppercase text-[#606060]">{(t as any)("promotions.promoted_at") || "Date"}</TableHead>
            <TableHead className="text-right text-[10px] font-bold uppercase text-[#606060]">{(t as any)("common.actions")}</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {[...Array(5)].map((_, index) => (
            <TableRow key={index} className="h-10 border-b border-border">
              <TableCell>
                <div className="flex items-center gap-3">
                  <Skeleton className="size-6 rounded-none" />
                  <Skeleton className="h-3 w-[120px]" />
                </div>
              </TableCell>
              <TableCell><Skeleton className="h-3 w-24" /></TableCell>
              <TableCell><Skeleton className="h-3 w-24" /></TableCell>
              <TableCell className="text-center">
                 <Skeleton className="h-4 w-16 mx-auto rounded-none" />
              </TableCell>
              <TableCell><Skeleton className="h-3 w-24" /></TableCell>
              <TableCell className="text-right">
                <Skeleton className="h-6 w-6 ml-auto rounded-none" />
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  );
}
