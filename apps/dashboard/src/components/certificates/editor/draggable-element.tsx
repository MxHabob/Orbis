"use client";

import { memo } from "react";
import { cn } from "@midday/ui/cn";
import { Button } from "@midday/ui/button";
import { Sparkles, Trash2 } from "lucide-react";
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
        "absolute target flex items-center justify-center",
        isPreview ? "cursor-default select-text" : "cursor-pointer select-none",
        isSelected && "z-50"
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
        width: el.width ? `${el.width}%` : "auto",
        height: (el.type === "shape") ? `${el.height}px` : "auto",
      }}
    >
      {isSelected && !isPreview && (
        <div className="absolute -top-12 left-1/2 -translate-x-1/2 flex items-center gap-1 bg-background border shadow-xl p-1 rounded-lg z-[100] animate-in fade-in zoom-in duration-200">
           <Button
              variant="ghost"
              size="icon"
              className="h-8 w-8 text-destructive hover:bg-destructive/10"
              onClick={(e) => {
                e.stopPropagation();
                onDelete(el.id);
              }}
            >
              <Trash2 className="w-4 h-4" />
            </Button>
            <div className="w-px h-4 bg-border mx-1" />
            <div className="px-2 text-[10px] font-medium text-muted-foreground whitespace-nowrap">
              {el.x.toFixed(0)}%, {el.y.toFixed(0)}%
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
        />
       ) : el.type === "image" ? (
        <img src={content} className="w-full h-auto shadow-sm" draggable={false} alt="asset" />
      ) : (
        <div 
          className={cn(
            "whitespace-nowrap font-bold transition-all",
            el.variableKey && !isPreview && "px-3 py-1 bg-primary/10 text-primary border border-primary/20 rounded-lg flex items-center gap-2"
          )}
          style={{
            fontSize: `${el.fontSize}px`,
            fontWeight: el.fontWeight,
            color: el.color,
            fontFamily: el.fontFamily,
          }}
        >
          {el.variableKey && !isPreview && <Sparkles className="w-3 h-3" />}
          {content}
        </div>
      )}
    </div>
  );
});

DraggableElement.displayName = "DraggableElement";
