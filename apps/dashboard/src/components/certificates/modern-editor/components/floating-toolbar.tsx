"use client";

import React from "react";
import { 
  MousePointer2, 
  Hash, 
  Square, 
  Circle,
  PenTool, 
  Type, 
  Grid2X2, 
  MessageSquare,
  ChevronDown,
  Hand,
  Plus
} from "lucide-react";
import { useEditorStore } from "../store/use-editor-store";
import { cn } from "@midday/ui/cn";
import { 
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@midday/ui/tooltip";

export function FloatingToolbar() {
  const { addElement } = useEditorStore();

  const tools = [
    { id: 'select', icon: MousePointer2, label: 'Move (V)' },
    { id: 'frame', icon: Hash, label: 'Frame (F)', action: () => addElement({ type: 'frame', name: 'Frame', width: 800, height: 600, style: { backgroundColor: '#ffffff' } }) },
    { id: 'rect', icon: Square, label: 'Rectangle (R)', action: () => addElement({ type: 'shape', name: 'Rectangle', width: 100, height: 100, style: { backgroundColor: '#3b82f6' } }) },
    { id: 'pen', icon: PenTool, label: 'Pen (P)' },
    { id: 'text', icon: Type, label: 'Text (T)', action: () => addElement({ type: 'text', content: 'New Text', width: 200, height: 40, style: { fontSize: 24, fontWeight: '700' } }) },
    { id: 'ellipse', icon: Circle, label: 'Ellipse (O)' },
    { id: 'assets', icon: Grid2X2, label: 'Resources' },
    { id: 'hand', icon: Hand, label: 'Hand Tool (H)' },
    { id: 'comment', icon: MessageSquare, label: 'Comment (C)' },
  ];

  return (
    <TooltipProvider delayDuration={400}>
      <div className="fixed bottom-6 left-1/2 -translate-x-1/2 z-[9999]">
        <div className="flex items-center gap-0.5 bg-[#2c2c2c] border border-white/10 p-1 rounded-[12px] shadow-2xl backdrop-blur-md">
           {tools.map((tool, idx) => (
              <React.Fragment key={tool.id}>
                 <Tooltip>
                    <TooltipTrigger asChild>
                       <button 
                          onClick={tool.action}
                          className={cn(
                             "h-8 w-8 flex items-center justify-center rounded-[8px] transition-all group relative",
                             tool.id === 'select' ? "bg-[#0c8ce9] text-white" : "text-white/40 hover:text-white hover:bg-white/5"
                          )}
                       >
                          <tool.icon className={cn("w-4 h-4", tool.id === 'select' && "fill-current")} />
                          {['frame', 'rect', 'ellipse', 'pen'].includes(tool.id) && (
                             <ChevronDown className="w-2 h-2 absolute bottom-0.5 right-0.5 opacity-40" />
                          )}
                       </button>
                    </TooltipTrigger>
                    <TooltipContent side="top" className="bg-[#1e1e1e] border-[#333] text-white text-[10px] font-bold p-1 px-2 rounded-[4px]">
                       {tool.label}
                    </TooltipContent>
                 </Tooltip>
                 {(idx === 0 || idx === 8) && <div className="w-[1px] h-4 bg-white/10 mx-1" />}
              </React.Fragment>
           ))}
        </div>
      </div>
    </TooltipProvider>
  );
}
