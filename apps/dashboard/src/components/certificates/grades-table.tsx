"use client";

import { useTRPC } from "@/trpc/client";
import { useQuery } from "@tanstack/react-query";
import { Loader2 } from "lucide-react";
import { cn } from "@midday/ui/cn";

interface GradesTableProps {
  studentId: string;
  type?: string;
  month?: number;
  className?: string;
  isPreview?: boolean;
}

export function GradesTable({ studentId, type, month, className, isPreview }: GradesTableProps) {
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

  if (isLoading) {
    return (
      <div className="flex items-center justify-center p-8 bg-white border border-black/10 rounded">
        <Loader2 className="w-5 h-5 animate-spin text-muted-foreground" />
        <span className="ml-2 text-xs text-muted-foreground">Fetching grades...</span>
      </div>
    );
  }

  const columns = data?.columns || [];
  const rows = data?.rows || [];

  if (rows.length === 0) {
    return (
      <div className="p-8 text-center bg-white border border-dashed border-gray-300 rounded text-muted-foreground text-xs" dir="ltr">
        No grade data recorded for this student at this time.
      </div>
    );
  }

  const gridTemplateColumns = `minmax(140px, 2fr) repeat(${columns.length}, 1fr) 1fr`;
  const totalFinalScore = rows.reduce((acc: number, curr: any) => acc + curr.total, 0);

  return (
    <div className={cn("w-full text-zinc-900 flex flex-col gap-6", className)} dir="ltr">
      {/* Modern Table Container */}
      <div className="w-full bg-white rounded-xl border border-zinc-200 shadow-sm overflow-hidden">
        {/* Header */}
        <div 
          className="grid border-b border-zinc-200 font-bold text-[10px] uppercase tracking-wider text-center bg-zinc-50/80 backdrop-blur-sm transition-colors"
          style={{ gridTemplateColumns }}
        >
          <div className="py-3 px-4 text-left text-zinc-500 font-semibold border-r border-zinc-100">Subject</div>
          {columns.map((col: string) => (
            <div key={col} className="py-3 px-1 flex items-center justify-center border-r border-zinc-100 text-zinc-500">
              {col}
            </div>
          ))}
          <div className="py-3 px-1 flex items-center justify-center font-bold text-zinc-900 bg-zinc-100/50">Total</div>
        </div>

        {/* Rows */}
        <div className="divide-y divide-zinc-100">
          {rows.map((row: any, i: number) => (
            <div 
              key={i} 
              className="grid text-center text-[11px] items-center hover:bg-zinc-50/50 transition-colors group"
              style={{ gridTemplateColumns }}
            >
              <div className="py-3 px-4 text-left font-bold text-zinc-800 border-r border-zinc-100">
                {row.subject}
              </div>
              {columns.map((col: string) => (
                <div key={col} className="py-3 px-1 flex items-center justify-center border-r border-zinc-100 text-zinc-600">
                  {row.grades[col] ?? <span className="text-zinc-300">-</span>}
                </div>
              ))}
              <div className="py-3 px-1 flex items-center justify-center font-bold text-zinc-900 bg-zinc-50/30 group-hover:bg-zinc-100/50 transition-colors">
                {row.total}
              </div>
            </div>
          ))}
        </div>

        <div 
          className="grid border-t-2 border-zinc-900 font-bold text-[11px] items-center bg-zinc-900 text-white"
          style={{ gridTemplateColumns }}
        >
          <div className="py-3 px-4 text-left">Grand Total</div>
          <div 
            className="py-3 px-4 text-center text-sm tracking-tight"
            style={{ gridColumn: `span ${columns.length + 1}` }}
          >
            {totalFinalScore}
          </div>
        </div>
      </div>
    </div>
  );
}
