"use client";

import { cn } from "@midday/ui/cn";
import { useMemo, useEffect } from "react";
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
  onLoad?: () => void;
}

const PAPER_SIZES: Record<string, { pxWidth: number; pxHeight: number; mmWidth: number; mmHeight: number }> = {
  a4: { pxWidth: 595, pxHeight: 842, mmWidth: 210, mmHeight: 297 },
  a5: { pxWidth: 420, pxHeight: 595, mmWidth: 148, mmHeight: 210 },
  letter: { pxWidth: 612, pxHeight: 792, mmWidth: 216, mmHeight: 279 },
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
  onLoad,
}: CertificateViewerProps) {
  const elements = useMemo(() => {
    return (template.content?.elements || []) as Element[];
  }, [template]);

  useEffect(() => {
    const hasTable = elements.some(
      (el) =>
        el.type === "table" ||
        el.type === "grades-table" ||
        (el.type === "variable" && el.variableKey === "table")
    );

    if (!hasTable && onLoad) {
      const timer = setTimeout(() => {
        onLoad();
      }, 300); // 300ms to allow fonts to render
      return () => clearTimeout(timer);
    }
  }, [elements, onLoad]);

  const size = (template.size || "a4") as keyof typeof PAPER_SIZES;
  const orientation = template.orientation || "portrait";
  
  const currentSize = PAPER_SIZES[size] ?? PAPER_SIZES.a4!;
  const canvasWidth = orientation === "portrait" ? currentSize!.pxWidth : currentSize!.pxHeight;
  const canvasHeight = orientation === "portrait" ? currentSize!.pxHeight : currentSize!.pxWidth;

  const replaceVariables = (text: string) => {
    if (!text) return "";
    return text.replace(/\{\{(.*?)\}\}/g, (match, key) => {
      const trimmedKey = key.trim();
      const value = data[trimmedKey];
      if (value !== undefined && value !== null) return String(value);
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
        boxSizing: 'border-box',
        direction: 'rtl',
      }}
      data-canvas-content
    >
       {/* Background */}
       {template.backgroundImageUrl && (
         <img 
            src={template.backgroundImageUrl} 
            alt="Background" 
            className="w-full h-full object-fill absolute inset-0 pointer-events-none"
         />
       )}

       {/* Elements */}
       {elements.map((el) => {
         const elementContent = el.variableKey ? replaceVariables(el.variableKey) : replaceVariables(el.content);
         const isText = el.type === "text" || el.type === "variable";
         
         return (
          <div
            key={el.id}
            className={cn(
              "absolute flex items-center",
              el.align === "center" ? "justify-center" : el.align === "left" ? "justify-end" : "justify-start"
            )}
            style={{
              left: `${el.x}%`,
              top: `${el.y}%`,
              transform: `rotate(${el.rotation || 0}deg)`,
              textAlign: el.align as any,
              opacity: el.opacity ?? 1,
              width: el.width ? `${el.width}%` : isText ? "fit-content" : "auto",
              height: el.type === "shape" ? `${el.height}px` : "auto",
              zIndex: (el.type === "image" || el.type === "badge") ? 5 : 10,
              minWidth: isText ? "50px" : "auto",
              boxSizing: 'border-box',
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
                 onLoad={onLoad}
                 fontSize={el.fontSize}
              />
           ) : (
             <div 
                className="font-bold w-full"
                style={{
                  fontSize: `${el.fontSize}px`,
                  fontWeight: el.fontWeight,
                  color: el.color,
                  fontFamily: el.fontFamily,
                  whiteSpace: "pre-wrap",
                  wordBreak: "break-word",
                  overflow: "visible",
                  textAlign: el.align as any,
                }}
             >
                {elementContent}
             </div>
           )}
          </div>
         );
       })}
    </div>
  );
}
