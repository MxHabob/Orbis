"use client";

import { useTRPC } from "@/trpc/client";
import { useQuery } from "@tanstack/react-query";
import { Loader2 } from "lucide-react";
import { cn } from "@midday/ui/cn";
import { useEffect } from "react";

interface GradesTableProps {
  studentId: string;
  type?: string;
  month?: number;
  className?: string;
  isPreview?: boolean;
  onLoad?: () => void;
  fontSize?: number;
}

// Grade label colors for visual distinction
const GRADE_COLORS: Record<string, string> = {
  "ممتاز": "text-emerald-700 bg-emerald-50",
  "جيد جداً": "text-blue-700 bg-blue-50",
  "جيد": "text-amber-700 bg-amber-50",
  "مقبول": "text-orange-700 bg-orange-50",
  "راسب": "text-red-700 bg-red-50",
};

export function GradesTable({ 
  studentId, 
  type, 
  month, 
  className, 
  isPreview, 
  onLoad,
  fontSize = 10
}: GradesTableProps) {
  const trpc = useTRPC();

  const { data, isLoading } = useQuery(
    trpc.grades.getReportData.queryOptions(
      { studentId, type, month },
      { 
        enabled: !!studentId,
        staleTime: 1000 * 60 * 5 // 5 minutes cache
      }
    )
  );

  useEffect(() => {
    if (!isLoading) {
      onLoad?.();
    }
  }, [isLoading, data, onLoad]);

  if (isLoading) {
    return (
      <div className="flex items-center justify-center p-8 bg-white border border-black/10 rounded">
        <Loader2 className="w-5 h-5 animate-spin text-muted-foreground" />
        <span className="ml-2 text-xs text-muted-foreground">جاري تحميل الدرجات...</span>
      </div>
    );
  }

  const columns = data?.columns || [];
  const rows = data?.rows || [];
  const certificateType = data?.type || type || "monthly";

  if (rows.length === 0) {
    return (
      <div className="p-8 text-center bg-white border border-dashed border-gray-300 rounded text-muted-foreground text-xs" dir="rtl">
        لا توجد بيانات درجات مسجلة لهذا الطالب في هذا الوقت.
      </div>
    );
  }

  // Columns: assessmentItems (or aggregated columns) + المجموع + التقدير
  const hasGradeLabel = rows.some((row: any) => row.gradeLabel);
  const totalColumns = columns.length + (hasGradeLabel ? 2 : 1); 
  const gridTemplateColumns = `minmax(100px, 2fr) repeat(${columns.length}, 1fr) minmax(50px, 1fr)${hasGradeLabel ? ' minmax(60px, 1fr)' : ''}`;
  
  const totalFinalScore = rows.reduce((acc: number, curr: any) => acc + (curr.total || 0), 0);
  const totalMaxScore = rows.reduce((acc: number, curr: any) => acc + (curr.maxTotal || 0), 0);

  // Dynamic font sizes based on the base fontSize prop
  const headerSize = `${Math.max(6, fontSize - 1)}px`;
  const bodySize = `${fontSize}px`;
  const badgeSize = `${Math.max(6, fontSize - 2)}px`;
  const footerLabelSize = `${fontSize}px`;
  const footerValueSize = `${Math.max(10, fontSize + 2)}px`;

  return (
    <div className={cn("w-full text-zinc-900 flex flex-col gap-2", className)} dir="rtl">
      <div className="w-full bg-white rounded-lg border border-zinc-200 shadow-sm overflow-hidden">
        {/* Header Row  */}
        <div 
          className="grid border-b-2 border-zinc-300 font-bold uppercase tracking-wider text-center bg-zinc-100"
          style={{ gridTemplateColumns, fontSize: headerSize }}
        >
          <div className="py-2 px-3 text-right text-zinc-600 font-bold border-l border-zinc-200">المادة</div>
          {columns.map((col: string) => (
            <div key={col} className="py-2 px-1 flex items-center justify-center border-l border-zinc-200 text-zinc-600 font-semibold">
              {col}
            </div>
          ))}
          <div className="py-2 px-1 flex items-center justify-center font-bold text-zinc-900 bg-zinc-200/60 border-l border-zinc-200">
            المجموع
          </div>
          {hasGradeLabel && (
            <div className="py-2 px-1 flex items-center justify-center font-bold text-zinc-900 bg-zinc-200/60">
              التقدير
            </div>
          )}
        </div>

        {/* Data Rows */}
        <div className="divide-y divide-zinc-100">
          {rows.map((row: any, i: number) => (
            <div 
              key={i} 
              className="grid text-center items-center hover:bg-zinc-50/50 transition-colors group"
              style={{ gridTemplateColumns, fontSize: bodySize }}
            >
              <div className="py-1.5 px-3 text-right font-bold text-zinc-800 border-l border-zinc-100">
                {row.subject}
              </div>
              
              {columns.map((col: string) => (
                <div key={col} className="py-1.5 px-1 flex items-center justify-center border-l border-zinc-100 text-zinc-600">
                  {row.grades[col] != null ? (
                    <span className="font-medium">{row.grades[col]}</span>
                  ) : (
                    <span className="text-zinc-300">-</span>
                  )}
                </div>
              ))}
              
              <div className="py-1.5 px-1 flex items-center justify-center font-bold text-zinc-900 bg-zinc-50/50 group-hover:bg-zinc-100/50 transition-colors border-l border-zinc-100">
                {row.total || 0}
              </div>
              
              {hasGradeLabel && (
                <div className="py-1.5 px-1 flex items-center justify-center">
                  {row.gradeLabel ? (
                    <span className={cn(
                      "px-1.5 py-0.5 rounded font-bold whitespace-nowrap",
                      GRADE_COLORS[row.gradeLabel] || "text-zinc-600 bg-zinc-100"
                    )} style={{ fontSize: badgeSize }}>
                      {row.gradeLabel}
                    </span>
                  ) : (
                    <span className="text-zinc-300">-</span>
                  )}
                </div>
              )}
            </div>
          ))}
        </div>

        {/* Grand Total Footer */}
        <div 
          className="grid border-t-2 border-zinc-800 font-bold items-center bg-zinc-800 text-white"
          style={{ gridTemplateColumns }}
        >
          <div className="py-2 px-3 text-right font-bold" style={{ fontSize: footerLabelSize }}>المجموع الكلي</div>
          <div 
            className="py-1.5 px-3 text-center tracking-tight font-bold"
            style={{ 
              gridColumn: `span ${totalColumns - 1}`,
              fontSize: footerValueSize
            }}
          >
            {totalFinalScore}
            {totalMaxScore > 0 && (
              <span className="text-zinc-400 mr-1" style={{ fontSize: headerSize }}>/ {totalMaxScore}</span>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
