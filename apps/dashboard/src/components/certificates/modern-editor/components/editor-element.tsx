"use client";

import React, { useState, useRef, useEffect } from "react";
import { type EditorElement as IEditorElement } from "../types";
import { cn } from "@midday/ui/cn";
import { useEditorStore } from "../store/use-editor-store";

interface Props {
  element: IEditorElement;
}

export function EditorElement({ element }: Props) {
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
              className="w-full h-full bg-transparent border-none outline-none resize-none overflow-hidden p-0 m-0 no-pan"
              style={{
                color: style.color || "#000000",
                fontSize: `${style.fontSize || 16}px`,
                fontWeight: style.fontWeight || "normal",
                fontFamily: style.fontFamily || "Inter, sans-serif",
                textAlign: style.textAlign || "left",
                lineHeight: style.lineHeight || 1.2,
                letterSpacing: `${style.letterSpacing || 0}px`,
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
            className="w-full h-full flex items-center select-none cursor-text"
            style={{
              color: style.color || "#000000",
              fontSize: `${style.fontSize || 16}px`,
              fontWeight: style.fontWeight || "normal",
              fontFamily: style.fontFamily || "Inter, sans-serif",
              textAlign: style.textAlign || "left",
              justifyContent: style.textAlign === "left" ? "flex-start" : style.textAlign === "right" ? "flex-end" : "center",
              lineHeight: style.lineHeight || 1.2,
              letterSpacing: `${style.letterSpacing || 0}px`,
            }}
          >
            {element.content || element.variableKey || "Double click to edit"}
          </div>
        );
      case "image":
        return (
          <div className="w-full h-full relative overflow-hidden">
             <img
              src={element.content}
              alt=""
              className="w-full h-full object-contain pointer-events-none"
            />
          </div>
        );
      case "shape":
        return (
          <div
            style={{
              backgroundColor: style.backgroundColor || "#3b82f6",
              width: "100%",
              height: "100%",
              borderRadius: `${style.borderRadius || 0}px`,
              border: `${style.borderWidth || 0}px solid ${style.borderColor || "transparent"}`,
            }}
          />
        );
      case "frame":
        return (
          <div
            className="w-full h-full shadow-[0_0_50px_rgba(0,0,0,0.1)] relative"
            style={{
              backgroundColor: style.backgroundColor || "#ffffff",
              backgroundImage: element.content ? `url(${element.content})` : "none",
              backgroundSize: "cover",
              backgroundPosition: "center",
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
        "absolute cursor-move outline-none editor-element-container",
        element.type === "variable" && "ring-1 ring-blue-500/30 ring-dashed hover:ring-blue-500/60",
        isEditing && "ring-2 ring-primary bg-white shadow-2xl z-[99999]",
        element.locked && "pointer-events-none opacity-80"
      )}
      style={{
        left: element.x,
        top: element.y,
        width: element.width,
        height: element.height,
        transform: `rotate(${element.rotation}deg)`,
        opacity: element.opacity,
        zIndex: element.zIndex,
      }}
    >
      {renderContent()}
    </div>
  );
}
