"use client";

import { memo } from "react";
import { cn } from "@midday/ui/cn";
import { Button } from "@midday/ui/button";
import { Trash2 } from "lucide-react";
import { GradesTable } from "../grades-table";
import type { EditorElement } from "./types";

interface DraggableElementProps {
  el: EditorElement;
  isSelected: boolean;
  isPreview: boolean;
  content: string;
  onSelect: (id: string) => void;
  onDelete: (id: string) => void;
  students: any;
  previewStudentId: string | null;
  certificateType: string;
  month: number;
}

export const DraggableElement = memo(({ 
  el, 
  isSelected, 
  isPreview, 
  content, 
  onSelect,
  onDelete,
  students,
  previewStudentId,
  certificateType,
  month,
}: DraggableElementProps) => {
  return (
    <div
      id={el.id}
      data-id={el.id}
      className={cn(
        "absolute target flex items-center group",
        el.align === "center" ? "justify-center" : el.align === "left" ? "justify-start" : "justify-end",
        isPreview ? "cursor-default select-text" : "cursor-grab active:cursor-grabbing select-none hover:ring-2 hover:ring-primary/40 rounded-sm",
        isSelected && "z-50 ring-2 ring-primary/60"
      )}
      onClick={(e) => {
        e.stopPropagation();
        if (!isPreview) onSelect(el.id);
      }}
      style={{
        left: `${el.x}%`,
        top: `${el.y}%`,
        transform: `rotate(${el.rotation || 0}deg)`,
        textAlign: el.align as any,
        opacity: el.opacity,
        width: el.width ? `${el.width}%` : (el.type === "text" || el.type === "variable") ? "fit-content" : "auto",
        height: (el.type === "shape") ? `${el.height}px` : "auto",
        direction: "rtl",
        minWidth: (el.type === "text" || el.type === "variable") ? "50px" : "auto",
      }}
    >
      {isSelected && !isPreview && (
        <div className={cn(
            "absolute left-1/2 -translate-x-1/2 flex items-center gap-1 px-1.5 py-1",
            "bg-[#1a1a1a] dark:bg-white",
            "border border-white/10 dark:border-black/10",
            "shadow-[0_8px_30px_rgba(0,0,0,0.4)] rounded-xl z-[100]",
            "pointer-events-auto",
            el.y < 15 ? "top-full mt-4" : "-top-14"
          )}>
           <Button
              variant="ghost"
              size="icon"
              className="h-7 w-7 text-red-400 hover:text-red-300 hover:bg-red-500/15 rounded-lg transition-all shrink-0"
              onClick={(e) => {
                e.stopPropagation();
                onDelete(el.id);
              }}
            >
              <Trash2 className="w-3.5 h-3.5" />
            </Button>
            <div className="h-4 w-px bg-white/10 dark:bg-black/10" />
            <div className="flex items-center gap-1.5 px-2 py-0.5 font-mono text-[9px] whitespace-nowrap tabular-nums">
              <span className="text-white/40 dark:text-black/40 uppercase">X</span>
              <span className="text-white dark:text-black font-semibold">{el.x.toFixed(1)}%</span>
              <div className="w-1 h-1 rounded-full bg-white/20 dark:bg-black/20" />
              <span className="text-white/40 dark:text-black/40 uppercase">Y</span>
              <span className="text-white dark:text-black font-semibold">{el.y.toFixed(1)}%</span>
            </div>
        </div>
      )}
      {el.type === "shape" ? (
        <div 
          className={cn("w-full h-full", el.content === "box" && "border-2")} 
          style={{ 
            backgroundColor: el.backgroundColor,
            borderColor: el.content === "box" ? el.backgroundColor : "transparent",
            height: (el.type === "shape") ? `${el.height}px` : "auto",
          }} 
        />
       ) : (el.type === "table" || (el.type === "variable" && el.variableKey === "table")) ? (
        <GradesTable 
          studentId={previewStudentId || students?.[0]?.userId || ""} 
          isPreview={isPreview}
          type={certificateType}
          month={month}
          fontSize={el.fontSize}
        />
       ) : el.type === "image" ? (
        <img src={content} className="w-full h-auto shadow-sm" draggable={false} alt="asset" />
      ) : (
        <div 
          className={cn(
            "text-content font-bold",
            el.variableKey && !isPreview && "variable-pill-tag"
          )}
          style={{
            fontSize: `${el.fontSize}px`,
            fontWeight: el.fontWeight,
            color: isPreview || !el.variableKey ? el.color : undefined,
            fontFamily: el.fontFamily,
            maxWidth: "100%",
            overflow: "hidden",
            display: "inline-flex",
            alignItems: "center",
            ...(el.variableKey && !isPreview ? {
              padding: "4px 12px",
              background: "#eef2ff",
              border: "2px solid #818cf8",
              borderRadius: "99px",
              justifyContent: "center",
              color: "#4338ca",
              fontWeight: "700",
              fontSize: `${Math.max(11, el.fontSize || 14)}px`,
              boxShadow: "0 2px 8px rgba(99, 102, 241, 0.15)",
              boxSizing: "border-box",
            } : {}),
          }}
        >
          <span className="truncate">{content}</span>
        </div>
      )}
    </div>
  );
});

DraggableElement.displayName = "DraggableElement";
