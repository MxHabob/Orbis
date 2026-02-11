"use client";

import { cn } from "@midday/ui/cn";
import { useMemo } from "react";
import { ShieldCheck, Award, BadgeAlert, Check } from "lucide-react";
import { GradesTable } from "./grades-table";

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
    content?: any;
    size?: string;
    orientation?: string;
  };
  data?: Record<string, string | number>;
  certificateType?: string;
  month?: number;
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
};

export function CertificateViewer({
  template,
  data = {},
  certificateType,
  month,
  preview = false,
  className,
}: CertificateViewerProps) {
  const elements = useMemo(() => {
    return (template.content?.elements || []) as Element[];
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
           ) : (el.type === "table" || el.type === "grades-table" || (el.type === "variable" && el.variableKey === "table")) ? (
              <GradesTable 
                 studentId={(data.studentId || data.userId || data.id) as string} 
                 type={certificateType}
                 month={month}
                 isPreview={preview}
              />
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
