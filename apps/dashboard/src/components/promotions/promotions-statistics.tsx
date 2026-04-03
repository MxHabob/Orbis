"use client";

import { useI18n } from "@/locales/client";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@midday/ui/card";
import { Skeleton } from "@midday/ui/skeleton";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@midday/ui/table";
import { Users, TrendingUp, Award, GraduationCap } from "lucide-react";
import { cn } from "@midday/ui/cn";

interface PromotionStatisticsData {
  total: number;
  promoted: number;
  retained: number;
  graduated: number;
  conditional: number;
  passRate: number;
  averageScore: number;
  highestScore: number;
  lowestScore: number;
  classBreakdown: Array<{
    className: string;
    total: number;
    promoted: number;
    retained: number;
    graduated: number;
  }>;
}

interface PromotionsStatisticsProps {
  data?: PromotionStatisticsData;
  isLoading: boolean;
}

export function PromotionsStatistics({ data, isLoading }: PromotionsStatisticsProps) {
  const t = useI18n();

  if (isLoading || !data) {
    return (
      <div className="flex flex-col gap-6 w-full">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          {[...Array(4)].map((_, i) => (
            <Card key={i} className="rounded-none border-border shadow-none">
              <CardHeader className="p-4 pb-2">
                <Skeleton className="h-3 w-20" />
              </CardHeader>
              <CardContent className="p-4 pt-0">
                <Skeleton className="h-6 w-12" />
              </CardContent>
            </Card>
          ))}
        </div>
      </div>
    );
  }

  if (data.total === 0) {
    return (
      <div className="flex flex-col items-center justify-center py-10 px-6 border border-dashed rounded-xl bg-muted/5 text-muted-foreground animate-in fade-in duration-500">
        <div className="h-16 w-16 rounded-full bg-muted/10 flex items-center justify-center mb-6 text-3xl">
          📈
        </div>
        <h2 className="font-medium text-lg text-foreground mb-2">
          {t("common.no_data")}
        </h2>
        <p className="text-sm text-[#606060] max-w-sm text-center">
          {t("grades.no_results_description")}
        </p>
      </div>
    );
  }


  const kpis = [
    {
      title: "إجمالي الترفيعات",
      value: data.total,
      description: "طالب",
    },
    {
      title: "نسبة النجاح",
      value: `${data.passRate}%`,
      description: "طلاب ناجحون",
    },
    {
      title: "متوسط الدرجات",
      value: `${data.averageScore}%`,
      description: `الأعلى: ${data.highestScore}%`,
    },
    {
      title: "الخريجين",
      value: data.graduated,
      description: "طالب متخرج",
    },
  ];

  return (
    <div className="flex flex-col gap-6 w-full">
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        {kpis.map((kpi, i) => (
          <Card key={i} className="rounded-none border-border shadow-none bg-background">
            <CardHeader className="p-4 pb-1">
              <span className="text-[10px] uppercase tracking-wider text-[#606060] font-bold">{kpi.title}</span>
            </CardHeader>
            <CardContent className="p-4 pt-0">
              <div className="text-2xl font-bold font-mono tracking-tight">{kpi.value}</div>
              <div className="text-[10px] text-[#606060] mt-1">{kpi.description}</div>
            </CardContent>
          </Card>
        ))}
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Status Breakdown Table */}
        <Card className="col-span-1 lg:col-span-3 rounded-none border-border shadow-none">
           <CardHeader className="p-4 border-b">
             <CardTitle className="text-sm font-bold uppercase tracking-tight">تفاصيل الصفوف</CardTitle>
           </CardHeader>
           <CardContent className="p-0">
             <Table>
               <TableHeader>
                 <TableRow className="hover:bg-transparent bg-muted/30">
                   <TableHead className="text-xs uppercase font-bold tracking-tight">الصف</TableHead>
                   <TableHead className="text-center text-xs uppercase font-bold tracking-tight">الإجمالي</TableHead>
                   <TableHead className="text-center text-xs uppercase font-bold tracking-tight">ترفيع</TableHead>
                   <TableHead className="text-center text-xs uppercase font-bold tracking-tight">إعادة</TableHead>
                   <TableHead className="text-center text-xs uppercase font-bold tracking-tight">خريجين</TableHead>
                 </TableRow>
               </TableHeader>
               <TableBody>
                 {data.classBreakdown.map((c, i) => (
                   <TableRow key={i} className="hover:bg-muted/30 transition-colors h-10 border-b">
                     <TableCell className="font-medium text-[11px]">{c.className}</TableCell>
                     <TableCell className="text-center text-[11px] font-mono">{c.total}</TableCell>
                     <TableCell className="text-center text-[11px] font-mono text-emerald-600">{c.promoted}</TableCell>
                     <TableCell className="text-center text-[11px] font-mono text-rose-600">{c.retained}</TableCell>
                     <TableCell className="text-center text-[11px] font-mono text-blue-600">{c.graduated}</TableCell>
                   </TableRow>
                 ))}
               </TableBody>
             </Table>
           </CardContent>
        </Card>
      </div>
    </div>
  );
}
