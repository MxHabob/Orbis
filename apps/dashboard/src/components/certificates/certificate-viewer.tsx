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
    } | any;
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
    // Standardize elements from the new editor structure
    const rawElements = (template.content?.elements || []) as IEditorElement[];
    return rawElements.sort((a, b) => a.zIndex - b.zIndex);
  }, [template]);

  const [bgLoaded, setBgLoaded] = useState(false);
  const onLoadCalledRef = useRef(false);

  // Find the base frame (page size)
  const baseFrame = useMemo(() => elements.find(el => el.type === 'frame'), [elements]);
  const canvasWidth = baseFrame?.width || 794; // Default to A4 pixels
  const canvasHeight = baseFrame?.height || 1123;

  const hasTable = useMemo(() => elements.some(el => el.type === "table"), [elements]);

  // Handle onLoad signaling
  useEffect(() => {
    if (onLoadCalledRef.current) return;
    if (hasTable) return; // GradesTable handles its own onLoad signaling via props if needed, 
    // but here we just wait for images.
    
    if (bgLoaded || !template.backgroundImageUrl) {
        onLoadCalledRef.current = true;
        onLoad?.();
    }
  }, [bgLoaded, template.backgroundImageUrl, hasTable, onLoad]);

  const renderElement = (el: IEditorElement) => {
    if (!el.visible) return null;

    const style = el.style || {};
    
    // Position elements relative to the base frame if one exists, otherwise absolute
    // In our new editor, elements are placed at absolute (x, y) on a 10000x10000 canvas.
    // However, for viewing, we want to crop to the frame.
    const left = baseFrame ? el.x - baseFrame.x : el.x;
    const top = baseFrame ? el.y - baseFrame.y : el.y;

    // Skip rendering the base frame itself as a child if we are using it as the container bg
    if (el.id === baseFrame?.id) return null;

    const isText = el.type === "text" || el.type === "variable";
    
    let content = el.content || "";
    if (el.type === "variable" && el.variableKey) {
        content = String(data[el.variableKey] || (preview ? `{${el.name}}` : ""));
    }

    return (
      <div
        key={el.id}
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
            src={el.content} 
            className="w-full h-full object-contain" 
            alt={el.name} 
            onLoad={() => {}} // Could track fine-grained loading
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
            studentId={(data.studentId || data.userId) as string} 
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
  };

  return (
    <div 
      className={cn("relative bg-white overflow-hidden print:shadow-none", className)}
      style={{
        width: `${canvasWidth}px`,
        height: `${canvasHeight}px`,
        backgroundColor: baseFrame?.style?.backgroundColor || "#ffffff",
        backgroundImage: baseFrame?.content ? `url(${baseFrame.content})` : "none",
        backgroundSize: "cover",
        backgroundPosition: "center",
        direction: 'rtl',
      }}
      data-canvas-content
    >
       {/* Support for old legacy background image if needed */}
       {template.backgroundImageUrl && !baseFrame?.content && (
         <img 
            src={template.backgroundImageUrl} 
            alt="Background" 
            crossOrigin="anonymous"
            className="w-full h-full object-fill absolute inset-0 pointer-events-none"
            onLoad={() => setBgLoaded(true)}
            onError={() => setBgLoaded(true)}
         />
       )}

       {/* Render all elements */}
       {elements.map(renderElement)}
    </div>
  );
}
