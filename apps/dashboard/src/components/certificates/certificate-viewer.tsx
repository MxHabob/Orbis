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

  const layoutConfig = template.content?.canvasConfig?.layoutConfig || template.content?.layoutConfig || { rows: 1, columns: 1, gap: 0 };
  const [bgLoaded, setBgLoaded] = useState(false);
  const [imagesLoadedCount, setImagesLoadedCount] = useState(0);
  const onLoadCalledRef = useRef(false);

  const imageElements = useMemo(() => elements.filter(el => el.type === "image" && el.visible), [elements]);
  const dataArray = Array.isArray(data) ? data : [data];
  const totalSlots = layoutConfig.rows * layoutConfig.columns;
  const currentData = preview ? dataArray : dataArray.slice(0, totalSlots);
  const totalImages = imageElements.length * currentData.length;

  // Find the base frame (which defines the overall sheet size if subdivided, or individual record size)
  const baseFrame = useMemo(() => elements.find(el => el.type === 'frame'), [elements]);
  const sheetWidth = baseFrame?.width || 794; 
  const sheetHeight = baseFrame?.height || 1123;

  // Internal Subdivision Math (2026 Print Standards)
  const slotWidth = (sheetWidth - (layoutConfig.columns - 1) * (layoutConfig.gap || 0)) / layoutConfig.columns;
  const slotHeight = (sheetHeight - (layoutConfig.rows - 1) * (layoutConfig.gap || 0)) / layoutConfig.rows;

  const hasTable = useMemo(() => elements.some(el => el.type === "table"), [elements]);

  useEffect(() => {
    if (onLoadCalledRef.current) return;
    
    const backgroundReady = !template.backgroundImageUrl || bgLoaded;
    const imagesReady = imagesLoadedCount >= totalImages;

    if (backgroundReady && imagesReady) {
        // Still give half-yearly/final tables a bit more time if they exist
        if (hasTable) {
             const timer = setTimeout(() => {
                onLoadCalledRef.current = true;
                onLoad?.();
             }, 1000);
             return () => clearTimeout(timer);
        } else {
            onLoadCalledRef.current = true;
            onLoad?.();
        }
    }
  }, [bgLoaded, template.backgroundImageUrl, hasTable, onLoad, imagesLoadedCount, totalImages]);

  const renderSingleRecord = (recordData: Record<string, any>, recordIndex: number) => {
    const row = Math.floor(recordIndex / layoutConfig.columns);
    const col = recordIndex % layoutConfig.columns;
    
    const offsetX = col * (slotWidth + (layoutConfig.gap || 0));
    const offsetY = row * (slotHeight + (layoutConfig.gap || 0));

    // If no data for this slot, don't render content but keep the slot layout in preview
    if (!recordData || Object.keys(recordData).length === 0) return null;

    return (
      <div 
        key={`record-${recordIndex}`}
        className="absolute overflow-hidden"
        style={{
            left: `${offsetX}px`,
            top: `${offsetY}px`,
            width: `${slotWidth}px`,
            height: `${slotHeight}px`,
            // Background is usually on the master sheet (frame), but can be repeated if desired
            // For now, we assume the master frame handles the sheet background
        }}
      >
        {elements.map((el) => {
            if (!el.visible || el.id === baseFrame?.id) return null;

            const style = el.style || {};
            // We assume design was done relative to the first slot's relative coordinates
            // (el.x - baseFrame.x) is the relative position within slot 0
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
                        <img 
                          src={el.variableKey ? recordData[el.variableKey] : el.content} 
                          className="w-full h-full object-contain" 
                          alt={el.name} 
                          crossOrigin="anonymous"
                          onLoad={() => setImagesLoadedCount(prev => prev + 1)}
                          onError={() => setImagesLoadedCount(prev => prev + 1)}
                        />
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
        width: `${sheetWidth}px`,
        height: `${sheetHeight}px`,
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
