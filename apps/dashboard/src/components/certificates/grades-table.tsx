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
  const totalColumns = 1 + columns.length + 1 + (hasGradeLabel ? 1 : 0); 
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
        <table className="w-full border-collapse table-fixed">
          <thead>
            <tr 
              className="border-b-2 border-zinc-300 bg-zinc-100"
              style={{ fontSize: headerSize }}
            >
              <th 
                className="py-2 px-3 text-right text-zinc-600 font-bold border-l border-zinc-200"
                style={{ width: "25%" }}
              >
                المادة
              </th>
              {columns.map((col: string) => (
                <th 
                  key={col} 
                  className="py-2 px-1 text-center border-l border-zinc-200 text-zinc-600 font-semibold"
                >
                  {col}
                </th>
              ))}
              <th 
                className="py-2 px-1 text-center font-bold text-zinc-900 bg-zinc-200/60 border-l border-zinc-200"
                style={{ width: "12%" }}
              >
                المجموع
              </th>
              {hasGradeLabel && (
                <th 
                  className="py-2 px-1 text-center font-bold text-zinc-900 bg-zinc-200/60"
                  style={{ width: "15%" }}
                >
                  التقدير
                </th>
              )}
            </tr>
          </thead>

          <tbody className="divide-y divide-zinc-100">
            {rows.map((row: any, i: number) => (
              <tr 
                key={i} 
                className="text-center items-center hover:bg-zinc-50/50 transition-colors group"
                style={{ fontSize: bodySize }}
              >
                <td className="py-1.5 px-3 text-right font-bold text-zinc-800 border-l border-zinc-100">
                  {row.subject}
                </td>
                
                {columns.map((col: string) => (
                  <td key={col} className="py-1.5 px-1 border-l border-zinc-100 text-zinc-600 text-center">
                    {row.grades[col] != null ? (
                      <span className="font-medium">{row.grades[col]}</span>
                    ) : (
                      <span className="text-zinc-300">-</span>
                    )}
                  </td>
                ))}
                
                <td className="py-1.5 px-1 font-bold text-zinc-900 bg-zinc-50/50 group-hover:bg-zinc-100/50 transition-colors border-l border-zinc-100 text-center">
                  {row.total || 0}
                </td>
                
                {hasGradeLabel && (
                  <td className="py-1.5 px-1 text-center">
                    {row.gradeLabel ? (
                      <span className={cn(
                        "px-1.5 py-0.5 rounded font-bold whitespace-nowrap inline-block",
                        GRADE_COLORS[row.gradeLabel] || "text-zinc-600 bg-zinc-100"
                      )} style={{ fontSize: badgeSize }}>
                        {row.gradeLabel}
                      </span>
                    ) : (
                      <span className="text-zinc-300">-</span>
                    )}
                  </td>
                )}
              </tr>
            ))}
          </tbody>

          <tfoot>
            <tr className="border-t-2 border-zinc-800 bg-zinc-800 text-white font-bold">
              <td className="py-2 px-3 text-right" style={{ fontSize: footerLabelSize }}>
                المجموع الكلي
              </td>
              <td 
                className="py-1.5 px-3 text-center tracking-tight"
                colSpan={totalColumns - 1}
                style={{ fontSize: footerValueSize }}
              >
                {totalFinalScore}
                {totalMaxScore > 0 && (
                  <span className="text-zinc-400 mr-2" style={{ fontSize: headerSize }}>
                    / {totalMaxScore}
                  </span>
                )}
              </td>
            </tr>
          </tfoot>
        </table>
      </div>
    </div>
  );
}
