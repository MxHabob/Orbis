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
import { Badge } from "@midday/ui/badge";
import { Skeleton } from "@midday/ui/skeleton";
import { Icons } from "@midday/ui/icons";
import { cn } from "@midday/ui/cn";
import { PieChart, User, GraduationCap, School } from "lucide-react";

interface PromotionPreview {
  studentId: string;
  name: string;
  finalScore: number | null;
  currentStatus: string;
  suggestedStatus: string;
  targetClassId: number | null;
  targetClassName: string | null;
}

interface PromotionsPreviewTableProps {
  data: PromotionPreview[];
  isLoading: boolean;
}

export function PromotionsPreviewTable({ data, isLoading }: PromotionsPreviewTableProps) {
  const t = useI18n();

  if (isLoading) {
    return (
      <div className="w-full">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead className="w-[300px]">{t("students.full_name")}</TableHead>
              <TableHead className="text-center">{t("promotions.final_score")}</TableHead>
              <TableHead className="text-center">{t("promotions.decision")}</TableHead>
              <TableHead>{t("promotions.target_class")}</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {[...Array(5)].map((_, index) => (
              <TableRow key={index} className="h-14 hover:bg-transparent">
                <TableCell>
                  <div className="flex items-center gap-3">
                    <Skeleton className="size-8 rounded-full" />
                    <Skeleton className="h-4 w-[120px]" />
                  </div>
                </TableCell>
                <TableCell className="text-center">
                   <Skeleton className="h-8 w-12 mx-auto rounded-lg" />
                </TableCell>
                <TableCell className="text-center">
                   <Skeleton className="h-6 w-20 mx-auto rounded-full" />
                </TableCell>
                <TableCell>
                   <Skeleton className="h-4 w-32" />
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </div>
    );
  }

  if (data.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center p-20 text-center space-y-4 text-muted-foreground">
        <div>
          <h3 className="text-xl font-semibold text-foreground">{t("grades.no_students")}</h3>
          <p className="max-w-[300px] mx-auto opacity-70">{t("grades.no_students_description")}</p>
        </div>
      </div>
    );
  }

  const getStatusBadge = (status: string) => {
    switch (status) {
      case "promoted":
        return <Badge className="bg-green-500/10 text-green-500 border-green-500/20 hover:bg-green-500/20">{t("promotions.status_promoted")}</Badge>;
      case "retained":
        return <Badge variant="destructive" className="bg-red-500/10 text-red-500 border-red-500/20 hover:bg-red-500/20">{t("promotions.status_retained")}</Badge>;
      case "graduated":
        return <Badge className="bg-blue-500/10 text-blue-500 border-blue-500/20 hover:bg-blue-500/20">{t("promotions.status_graduated")}</Badge>;
      case "conditional":
        return <Badge variant="outline" className="bg-yellow-500/10 text-yellow-500 border-yellow-500/20 hover:bg-yellow-500/20">{t("promotions.status_conditional")}</Badge>;
      default:
        return <Badge variant="outline">{status}</Badge>;
    }
  };

  return (
    <div className="w-full">
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead className="w-[300px]">{t("students.full_name")}</TableHead>
            <TableHead className="text-center">{t("promotions.final_score")}</TableHead>
            <TableHead className="text-center">{t("promotions.decision")}</TableHead>
            <TableHead>{t("promotions.target_class")}</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {data.map((row) => (
            <TableRow key={row.studentId} className="h-14 hover:bg-muted/50 transition-colors group">
              <TableCell className="font-medium">
                <div className="flex items-center gap-3">
                   <div className="size-8 rounded-full bg-primary/5 flex items-center justify-center text-primary group-hover:bg-primary/10 transition-colors">
                     <User size={16} />
                   </div>
                   {row.name}
                </div>
              </TableCell>
              <TableCell className="text-center">
                {row.finalScore !== null ? (
                   <div className={cn(
                     "inline-flex items-center justify-center size-10 rounded-lg font-mono text-sm font-bold",
                     row.finalScore < 50 ? "bg-red-500/10 text-red-500 border border-red-500/20" : "bg-green-500/10 text-green-500 border border-green-500/20"
                   )}>
                     {row.finalScore.toFixed(0)}%
                   </div>
                ) : (
                  <span className="text-muted-foreground italic text-xs">-</span>
                )}
              </TableCell>
              <TableCell className="text-center">
                {getStatusBadge(row.suggestedStatus)}
              </TableCell>
              <TableCell>
                {row.targetClassName ? (
                  <div className="flex items-center gap-2 text-sm font-medium">
                     <div className="p-1 bg-muted rounded">
                        <School size={14} className="text-muted-foreground" />
                     </div>
                     {row.targetClassName}
                  </div>
                ) : row.suggestedStatus === 'graduated' ? (
                  <span className="flex items-center gap-2 text-xs text-blue-500 font-bold bg-blue-500/5 px-2 py-1 rounded w-fit">
                    <GraduationCap size={14} /> {t("students.graduated")}
                  </span>
                ) : (
                  <span className="text-muted-foreground text-xs italic">-</span>
                )}
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  );
}
