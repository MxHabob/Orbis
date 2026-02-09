
"use client";

import { cn } from "@midday/ui/cn";
import { useMemo } from "react";
import { ShieldCheck, Award, BadgeAlert, Check } from "lucide-react";

interface Element {
  id: string;
  type: "text" | "variable" | "image" | "badge" | "table" | "grades-table" | "shape";
  content: string;
  variableKey?: string;
  x: number;
  y: number;
  fontSize?: number;
  fontWeight?: string;
  color?: string;
  align?: "left" | "center" | "right";
  width?: number;
  height?: number;
  opacity?: number;
  rotation?: number;
  fontFamily?: string;
  backgroundColor?: string;
  columns?: Array<{ id: string; header: string; width: number; variableKey?: string }>;
}

interface CertificateViewerProps {
  template: {
    backgroundImageUrl?: string | null;
    headerContent?: any;
    mainContent?: any;
    footerContent?: any;
    size?: string;
    orientation?: string;
  };
  data?: Record<string, string | number>;
  preview?: boolean;
  className?: string;
}

const PAPER_SIZES = {
  a4: { pxWidth: 794, pxHeight: 1123 },
  a5: { pxWidth: 559, pxHeight: 794 },
  letter: { pxWidth: 816, pxHeight: 1056 },
};

const STAMP_ICONS = {
  Approved: ShieldCheck,
  Excellent: Award,
  Urgent: BadgeAlert,
  Verified: Check,
  "Approved / معتمد": ShieldCheck,
  "Excellent / ممتاز": Award,
  "Urgent / عاجل": BadgeAlert,
  "Verified / موثق": Check,
};

export function CertificateViewer({
  template,
  data = {},
  preview = false,
  className,
}: CertificateViewerProps) {
  const elements = useMemo(() => {
    const h = template.headerContent?.elements || [];
    const m = template.mainContent?.elements || [];
    const f = template.footerContent?.elements || [];
    return [...h, ...m, ...f] as Element[];
  }, [template]);

  const size = (template.size || "a4") as keyof typeof PAPER_SIZES;
  const orientation = template.orientation || "portrait";
  
  const currentSize = PAPER_SIZES[size] || PAPER_SIZES.a4;
  const canvasWidth = orientation === "portrait" ? currentSize.pxWidth : currentSize.pxHeight;
  const canvasHeight = orientation === "portrait" ? currentSize.pxHeight : currentSize.pxWidth;

  const replaceVariables = (text: string) => {
    if (!text) return "";
    return text.replace(/\{\{(.*?)\}\}/g, (match, key) => {
      const value = data[key.trim()];
      if (value !== undefined) return String(value);
      return preview ? match : "";
    });
  };

  return (
    <div 
      className={cn("bg-white shadow-2xl relative overflow-hidden select-none print:shadow-none print:m-0", className)}
      style={{
        width: `${canvasWidth}px`,
        height: `${canvasHeight}px`,
        aspectRatio: `${canvasWidth}/${canvasHeight}`,
      }}
      data-canvas-content
    >
       {/* Background */}
       {template.backgroundImageUrl && (
         <img 
            src={template.backgroundImageUrl} 
            alt="Background" 
            className="w-full h-full object-cover absolute inset-0 pointer-events-none"
         />
       )}

       {/* Elements */}
       {elements.map((el) => (
         <div
            key={el.id}
            className="absolute transition-all"
            style={{
              left: `${el.x}%`,
              top: `${el.y}%`,
              transform: `translate(-50%, -50%) rotate(${el.rotation || 0}deg)`,
              textAlign: el.align as any,
              opacity: el.opacity ?? 1,
              width: (el.type === "image" || el.type === "badge" || el.type === "table" || el.type === "grades-table" || el.type === "shape") ? `${el.width}%` : "auto",
              height: el.type === "shape" ? `${el.height}px` : "auto",
              zIndex: (el.type === "image" || el.type === "badge") ? 5 : 10,
            }}
         >
           {el.type === "image" ? (
             <img src={el.content} className="w-full h-auto pointer-events-none" alt="logo" />
           ) : el.type === "badge" ? (
              <div className="w-full aspect-square flex items-center justify-center rounded-full bg-white shadow-xl border-4" style={{ borderColor: el.color }}>
                 {(() => {
                   const StampIcon = (STAMP_ICONS as any)[el.content] || ShieldCheck;
                   return <StampIcon className="w-[60%] h-[60%]" style={{ color: el.color }} />;
                 })()}
              </div>
           ) : el.type === "shape" ? (
             <div 
                className="w-full h-full" 
                style={{ backgroundColor: el.backgroundColor }} 
             />
           ) : el.type === "grades-table" ? (
             <div className="w-full bg-white border-2 border-black text-[10px]" dir="rtl">
                <div 
                  className="grid bg-gray-100 border-b-2 border-black font-bold text-center"
                  style={{ gridTemplateColumns: `repeat(${el.columns?.length || 5}, 1fr)` }}
                >
                  {el.columns?.map(col => (
                    <div key={col.id} className="border-l border-black py-1 px-1">{col.header}</div>
                  ))}
                </div>
                {/* Normally we'd render rows here based on data, for now we show dummy or partial implementation */}
                {/* This part should be dynamic based on the actual student's results in the future */}
                <div className="p-2 text-center text-muted-foreground italic">بيانات الدرجات الديناميكية</div>
             </div>
           ) : (
             <div 
                style={{
                  fontSize: `${el.fontSize}px`,
                  fontWeight: el.fontWeight,
                  color: el.color,
                  fontFamily: el.fontFamily,
                  whiteSpace: "nowrap",
                }}
             >
                {el.variableKey ? replaceVariables(el.variableKey) : replaceVariables(el.content)}
             </div>
           )}
         </div>
       ))}
    </div>
  );
}
