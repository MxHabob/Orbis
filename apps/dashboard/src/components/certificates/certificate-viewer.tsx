"use client";

import { cn } from "@midday/ui/cn";
import { useMemo, useEffect, useState, useRef } from "react";
import { GradesTable } from "./grades-table";
import { type EditorElement as IEditorElement } from "@/types/certificate-editor";

interface CertificateViewerProps {
  template: {
    backgroundImageUrl?: string | null;
    content?: {
        elements: IEditorElement[];
        layoutConfig?: {
            rows: number;
            columns: number;
            gap: number;
        };
    } | any;
    size?: string;
    orientation?: string;
  };
  data?: Record<string, string | number> | Array<Record<string, string | number>>;
  certificateType?: string;
  month?: number;
  preview?: boolean;
  className?: string;
  onLoad?: () => void;
}

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
    const rawElements = (template.content?.elements || []) as IEditorElement[];
    return rawElements.sort((a, b) => a.zIndex - b.zIndex);
  }, [template]);

  const layoutConfig = template.content?.layoutConfig || { rows: 1, columns: 1, gap: 0 };
  const [bgLoaded, setBgLoaded] = useState(false);
  const onLoadCalledRef = useRef(false);

  // Find the base frame (which defines the individual record size)
  const baseFrame = useMemo(() => elements.find(el => el.type === 'frame'), [elements]);
  const recordWidth = baseFrame?.width || 794; 
  const recordHeight = baseFrame?.height || 1123;

  // Calculate overall canvas size based on layout
  const canvasWidth = (recordWidth * layoutConfig.columns) + (layoutConfig.gap * (layoutConfig.columns - 1));
  const canvasHeight = (recordHeight * layoutConfig.rows) + (layoutConfig.gap * (layoutConfig.rows - 1));

  const hasTable = useMemo(() => elements.some(el => el.type === "table"), [elements]);

  useEffect(() => {
    if (onLoadCalledRef.current) return;
    if (bgLoaded || !template.backgroundImageUrl || hasTable) {
        onLoadCalledRef.current = true;
        onLoad?.();
    }
  }, [bgLoaded, template.backgroundImageUrl, hasTable, onLoad]);

  const renderSingleRecord = (recordData: Record<string, any>, recordIndex: number) => {
    const row = Math.floor(recordIndex / layoutConfig.columns);
    const col = recordIndex % layoutConfig.columns;
    
    const offsetX = col * (recordWidth + layoutConfig.gap);
    const offsetY = row * (recordHeight + layoutConfig.gap);

    return (
      <div 
        key={`record-${recordIndex}`}
        className="absolute overflow-hidden"
        style={{
            left: `${offsetX}px`,
            top: `${offsetY}px`,
            width: `${recordWidth}px`,
            height: `${recordHeight}px`,
            backgroundColor: baseFrame?.style?.backgroundColor || "#ffffff",
            backgroundImage: baseFrame?.content ? `url(${baseFrame.content})` : "none",
            backgroundSize: "cover",
            backgroundPosition: "center",
            // Add a subtle separator if previewing multiple on one page
            border: preview && (layoutConfig.rows > 1 || layoutConfig.columns > 1) ? '1px dashed #e5e7eb' : 'none'
        }}
      >
        {elements.map((el) => {
            if (!el.visible || el.id === baseFrame?.id) return null;

            const style = el.style || {};
            const left = baseFrame ? el.x - baseFrame.x : el.x;
            const top = baseFrame ? el.y - baseFrame.y : el.y;

            let content = el.content || "";
            if (el.type === "variable" && el.variableKey) {
                content = String(recordData[el.variableKey] || (preview ? `{${el.name}}` : ""));
            }

            return (
                <div
                    key={`${recordIndex}-${el.id}`}
                    className={cn(
                        "absolute flex",
                        style.textAlign === "center" ? "justify-center" : style.textAlign === "right" ? "justify-end" : "justify-start"
                    )}
                    style={{
                        left: `${left}px`,
                        top: `${top}px`,
                        width: `${el.width}px`,
                        height: `${el.height}px`,
                        transform: `rotate(${el.rotation || 0}deg)`,
                        opacity: el.opacity ?? 1,
                        zIndex: el.zIndex,
                        pointerEvents: "none",
                    }}
                >
                    {el.type === "image" ? (
                        <img src={el.content} className="w-full h-full object-contain" alt={el.name} />
                    ) : el.type === "shape" ? (
                        <div 
                            className="w-full h-full" 
                            style={{ 
                                backgroundColor: style.backgroundColor,
                                borderRadius: `${style.borderRadius || 0}px`,
                                border: `${style.borderWidth || 0}px solid ${style.borderColor || "transparent"}`,
                                boxShadow: style.shadow || "none",
                            }} 
                        />
                    ) : el.type === "table" ? (
                        <GradesTable 
                            studentId={recordData.studentId as string} 
                            type={certificateType}
                            month={month}
                            isPreview={preview}
                            onLoad={onLoad}
                            fontSize={style.fontSize || 10}
                        />
                    ) : (
                        <div 
                            className="w-full h-full"
                            style={{
                                fontSize: `${style.fontSize}px`,
                                fontWeight: style.fontWeight || "bold",
                                color: style.color || "#000000",
                                fontFamily: style.fontFamily,
                                textAlign: style.textAlign || "center",
                                lineHeight: style.lineHeight || 1.2,
                                whiteSpace: "pre-wrap",
                                wordBreak: "break-word",
                            }}
                        >
                            {content}
                        </div>
                    )}
                </div>
            );
        })}
      </div>
    );
  };

  const dataArray = Array.isArray(data) ? data : [data];
  // Fill the remaining grid slots with empty objects in preview mode
  const totalSlots = layoutConfig.rows * layoutConfig.columns;
  const displayData = preview ? [...dataArray, ...Array(Math.max(0, totalSlots - dataArray.length)).fill({})] : dataArray;

  return (
    <div 
      className={cn("relative bg-white print:shadow-none", className)}
      style={{
        width: `${canvasWidth}px`,
        height: `${canvasHeight}px`,
        direction: 'rtl',
      }}
      data-canvas-content
    >
       {/* Support for old legacy background image if needed (now at canvas level) */}
       {template.backgroundImageUrl && (
         <img 
            src={template.backgroundImageUrl} 
            alt="Background" 
            crossOrigin="anonymous"
            className="w-full h-full object-fill absolute inset-0 pointer-events-none"
            onLoad={() => setBgLoaded(true)}
            style={{ zIndex: -1 }}
         />
       )}

       {/* Render Records in Grid */}
       {displayData.slice(0, totalSlots).map((recordData, idx) => renderSingleRecord(recordData, idx))}
    </div>
  );
}
