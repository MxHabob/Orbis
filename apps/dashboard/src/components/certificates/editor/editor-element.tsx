"use client";

import React, { useState, useRef, useEffect, memo } from "react";
import { type EditorElement as IEditorElement } from "@/types/certificate-editor";
import { cn } from "@midday/ui/cn";
import { useEditorStore } from "@/store/certificate-editor";
import { GradesTable } from "../grades-table";

interface Props {
  element: IEditorElement;
}

export const EditorElement = memo(function EditorElement({ element }: Props) {
  const { updateElement } = useEditorStore();
  const [isEditing, setIsEditing] = useState(false);
  const [tempContent, setTempContent] = useState(element.content || "");
  const inputRef = useRef<HTMLTextAreaElement>(null);
  
  const style = element.style || {};

  useEffect(() => {
    if (isEditing && inputRef.current) {
      inputRef.current.focus();
      inputRef.current.select();
    }
  }, [isEditing]);

  const handleBlur = () => {
    setIsEditing(false);
    if (tempContent !== element.content) {
      updateElement(element.id, { content: tempContent });
    }
  };

  if (!element.visible) return null;

  const renderContent = () => {
    switch (element.type) {
      case "text":
      case "variable":
        if (isEditing) {
          return (
            <textarea
              ref={inputRef}
              className="w-full h-full bg-transparent border-none outline-none resize-none overflow-hidden p-0 m-0 no-pan scroll-none"
              style={{
                color: style.color || "#000000",
                fontSize: `${style.fontSize || 16}px`,
                fontWeight: style.fontWeight || "normal",
                fontFamily: style.fontFamily || "Inter, sans-serif",
                textAlign: style.textAlign || "left",
                lineHeight: style.lineHeight || 1.2,
                letterSpacing: `${style.letterSpacing || 0}px`,
                direction: style.direction || "ltr",
              }}
              value={tempContent}
              onChange={(e) => setTempContent(e.target.value)}
              onBlur={handleBlur}
              onKeyDown={(e) => {
                if (e.key === "Enter" && !e.shiftKey) {
                  e.preventDefault();
                  handleBlur();
                }
              }}
            />
          );
        }
        return (
          <div
            onDoubleClick={() => !element.locked && element.type === "text" && setIsEditing(true)}
            className={cn(
                "w-full h-full flex items-center select-none",
                element.type === 'text' ? "cursor-text" : "cursor-default"
            )}
            style={{
              color: style.color || "#000000",
              fontSize: `${style.fontSize || 16}px`,
              fontWeight: style.fontWeight || "normal",
              fontFamily: style.fontFamily || "Inter, sans-serif",
              textAlign: style.textAlign || "left",
              justifyContent: style.textAlign === "left" ? "flex-start" : style.textAlign === "right" ? "flex-end" : "center",
              lineHeight: style.lineHeight || 1.2,
              letterSpacing: `${style.letterSpacing || 0}px`,
              textShadow: style.shadow || "none",
              direction: style.direction || "ltr",
            }}
          >
            {element.type === 'variable' ? (
                <div className="flex items-center pointer-events-none">
                    <span className="opacity-40" style={{ fontSize: '0.8em' }}>{'{ '}</span>
                    <span>{element.variableKey}</span>
                    <span className="opacity-40" style={{ fontSize: '0.8em' }}>{' }'}</span>
                </div>
            ) : (
                element.content || "Double click to edit"
            )}
          </div>
        );
      case "image":
        return (
          <div className="w-full h-full relative group/img transition-transform duration-500">
             <img
              src={element.content}
              alt=""
              className="w-full h-full object-contain pointer-events-none"
              style={{ 
                  opacity: element.opacity,
                  filter: style.shadow ? `drop-shadow(${style.shadow})` : "none"
              }}
            />
          </div>
        );
      case "table":
        return (
            <GradesTable 
                isPlaceholder={true} 
                className="w-full h-full" 
                fontSize={style.fontSize || 10}
            />
        );
      case "shape":
        return (
          <div
            className="w-full h-full shadow-lg transition-all duration-300"
            style={{
              backgroundColor: style.backgroundColor || "#3b82f6",
              borderRadius: `${style.borderRadius || 0}px`,
              border: `${style.borderWidth || 0}px solid ${style.borderColor || "transparent"}`,
              boxShadow: style.shadow || "none",
            }}
          />
        );
      case "frame":
        return (
          <div
            className="w-full h-full shadow-2xl relative overflow-hidden"
            style={{
              backgroundColor: style.backgroundColor || "#ffffff",
              backgroundImage: element.content ? `url(${element.content})` : "none",
              backgroundSize: "cover",
              backgroundPosition: "center",
              borderRadius: `${style.borderRadius || 0}px`,
            }}
          />
        );
      default:
        return null;
    }
  };

  return (
    <div
      data-element-id={element.id}
      className={cn(
        "absolute outline-none editor-element-container",
        !element.locked && "hover:outline hover:outline-2 hover:outline-primary/40 hover:outline-offset-2",
        isEditing && "outline outline-2 outline-primary outline-offset-4 bg-white/5 backdrop-blur-sm shadow-2xl z-[99999] rounded-sm",
        element.locked && "opacity-60 cursor-not-allowed"
      )}
      style={{
        left: element.x,
        top: element.y,
        width: element.width,
        height: element.height,
        transform: `rotate(${element.rotation}deg)`,
        opacity: isEditing ? 1 : element.opacity,
        zIndex: element.zIndex,
      }}
    >
      {renderContent()}
    </div>
  );
});
