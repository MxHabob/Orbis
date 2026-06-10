"use client";

import { useI18n } from "@/locales/client";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@orbis/ui/table";
import { Skeleton } from "@orbis/ui/skeleton";

export function PreviewTableSkeleton() {
  const t = useI18n();

  return (
    <div className="w-full border-x border-b border-border">
      <Table>
        <TableHeader>
          <TableRow className="hover:bg-transparent bg-muted/30 border-b h-10">
            <TableHead className="w-[300px] text-[10px] font-bold uppercase py-2 h-auto text-[#606060]">{t("students.full_name")}</TableHead>
            <TableHead className="text-center text-[10px] font-bold uppercase py-2 h-auto text-[#606060]">{t("promotions.final_score")}</TableHead>
            <TableHead className="text-center text-[10px] font-bold uppercase py-2 h-auto text-[#606060]">{t("promotions.decision")}</TableHead>
            <TableHead className="text-[10px] font-bold uppercase py-2 h-auto text-[#606060]">{t("promotions.target_class")}</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {[...Array(5)].map((_, index) => (
            <TableRow key={index} className="h-10 border-b">
              <TableCell>
                <div className="flex items-center gap-3">
                  <Skeleton className="size-6 rounded-none" />
                  <Skeleton className="h-3 w-[120px]" />
                </div>
              </TableCell>
              <TableCell className="text-center">
                 <Skeleton className="h-6 w-10 mx-auto rounded-none" />
              </TableCell>
              <TableCell className="text-center">
                 <Skeleton className="h-4 w-16 mx-auto rounded-none" />
              </TableCell>
              <TableCell>
                 <Skeleton className="h-3 w-32" />
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  );
}
