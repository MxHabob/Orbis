"use client";

import React from "react";
import { 
  Undo2, 
  Redo2, 
  Plus,
  Minus,
  Maximize,
  MousePointer2,
  Type as TextIcon,
  Square,
  Circle,
  Database,
  Image as ImageIcon
} from "lucide-react";
import { Button } from "@midday/ui/button";
import { useEditorStore } from "../store/use-editor-store";
import { cn } from "@midday/ui/cn";
import { Separator } from "@midday/ui/separator";

export function Toolbar() {
  const { undo, redo, canvasConfig, setCanvasConfig, addElement } = useEditorStore();

  const handleZoom = (delta: number) => {
    setCanvasConfig({ zoom: Math.max(0.1, Math.min(3, canvasConfig.zoom + delta)) });
  };

  return (
    <div className="absolute bottom-8 left-1/2 -translate-x-1/2 z-50 flex items-center gap-1.5 bg-background backdrop-blur-sm rounded-2xl p-1.5 px-3 min-w-max animate-in fade-in slide-in-from-bottom-4 duration-500">
      
      {/* Select Tool */}
      <Button variant="ghost" size="icon" className="h-9 w-9 rounded-xl bg-primary/10 text-primary border border-primary/20">
         <MousePointer2 className="w-4 h-4 fill-current" />
      </Button>

      <Separator orientation="vertical" className="h-4 mx-1" />

      {/* Creation Tools */}
      <div className="flex items-center gap-1">
        <Button 
          variant="ghost" size="icon" className="h-9 w-9 rounded-xl hover:bg-accent"
          onClick={() => addElement({ type: "text", content: "Double Click to Edit", style: { fontSize: 24, color: "#000000" } })}
        >
          <TextIcon className="w-4 h-4" />
        </Button>
        <Button 
          variant="ghost" size="icon" className="h-9 w-9 rounded-xl hover:bg-accent"
          onClick={() => addElement({ type: "shape", width: 150, height: 150, style: { backgroundColor: "#3b82f6" } })}
        >
          <Square className="w-4 h-4" />
        </Button>
        <Button 
          variant="ghost" size="icon" className="h-9 w-9 rounded-xl hover:bg-accent"
          onClick={() => addElement({ type: "shape", width: 150, height: 150, style: { backgroundColor: "#ef4444", borderRadius: 100 } })}
        >
          <Circle className="w-4 h-4" />
        </Button>
        <Button 
          variant="ghost" size="icon" className="h-9 w-9 rounded-xl hover:bg-accent"
          onClick={() => addElement({ type: "variable", variableKey: "student_name", content: "{{Student Name}}", style: { color: "#3b82f6", fontWeight: "bold" } })}
        >
          <Database className="w-4 h-4" />
        </Button>
      </div>

      <Separator orientation="vertical" className="h-4 mx-1" />

      {/* History */}
      <div className="flex items-center gap-0.5">
          <Button variant="ghost" size="icon" className="h-9 w-9 rounded-xl hover:bg-accent" onClick={undo}>
            <Undo2 className="w-4 h-4" />
          </Button>
          <Button variant="ghost" size="icon" className="h-9 w-9 rounded-xl hover:bg-accent" onClick={redo}>
            <Redo2 className="w-4 h-4" />
          </Button>
      </div>

      <Separator orientation="vertical" className="h-4 mx-1" />

      {/* Zoom Controls (Compact) */}
      <div className="flex items-center gap-1 bg-accent/30 rounded-xl px-1.5 py-0.5">
        <Button variant="ghost" size="icon" className="h-7 w-7 rounded-lg" onClick={() => handleZoom(-0.1)}>
          <Minus className="w-3 h-3" />
        </Button>
        <button 
          className="text-[10px] font-mono w-10 text-center hover:bg-accent rounded transition-colors h-7"
          onClick={() => setCanvasConfig({ zoom: 1 })}
        >
          {Math.round(canvasConfig.zoom * 100)}%
        </button>
        <Button variant="ghost" size="icon" className="h-7 w-7 rounded-lg" onClick={() => handleZoom(0.1)}>
          <Plus className="w-3 h-3" />
        </Button>
      </div>

    </div>
  );
}
