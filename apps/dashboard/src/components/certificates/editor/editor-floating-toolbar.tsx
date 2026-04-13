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
  Hand
} from "lucide-react";
import { useEditorStore } from "@/store/certificate-editor";
import { cn } from "@midday/ui/cn";
import { 
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@midday/ui/tooltip";

export function EditorFloatingToolbar() {
  const { addElement } = useEditorStore();

  const tools = [
    { id: 'select', icon: MousePointer2, label: 'Move (V)' },
    { id: 'frame', icon: Hash, label: 'Frame (F)', action: () => addElement({ type: 'frame', name: 'Frame' }) },
    { id: 'rect', icon: Square, label: 'Rectangle (R)', action: () => addElement({ type: 'shape', name: 'Rectangle', width: 200, height: 200 }) },
    { id: 'pen', icon: PenTool, label: 'Pen (P)' },
    { id: 'text', icon: Type, label: 'Text (T)', action: () => addElement({ type: 'text', content: 'Design Text' }) },
    { id: 'ellipse', icon: Circle, label: 'Ellipse (O)' },
    { id: 'assets', icon: Grid2X2, label: 'Resources' },
    { id: 'hand', icon: Hand, label: 'Hand Tool (H)' },
    { id: 'comment', icon: MessageSquare, label: 'Comment (C)' },
  ];

  return (
    <TooltipProvider delayDuration={400}>
      <div className="fixed bottom-10 left-1/2 -translate-x-1/2 z-[40] pointer-events-auto">
        <div className="flex items-center gap-1.5 glass-panel p-2 rounded-2xl border-white/10 shadow-[0_20px_50px_rgba(0,0,0,0.5)] ring-1 ring-white/10">
           {tools.map((tool, idx) => (
              <React.Fragment key={tool.id}>
                 <Tooltip>
                    <TooltipTrigger asChild>
                       <button 
                          onClick={tool.action}
                          className={cn(
                             "h-10 w-10 flex items-center justify-center rounded-xl transition-all group relative duration-200",
                             tool.id === 'select' 
                                ? "bg-blue-600 text-white shadow-lg shadow-blue-500/30" 
                                : "text-white/40 hover:text-white hover:bg-white/5"
                          )}
                       >
                          <tool.icon className={cn("w-4 h-4", tool.id === 'select' && "fill-current")} strokeWidth={2.5} />
                          {['frame', 'rect', 'ellipse', 'pen'].includes(tool.id) && (
                             <div className="absolute bottom-1 right-1 opacity-20 group-hover:opacity-40 transition-opacity">
                                <ChevronDown className="w-2 h-2" />
                             </div>
                          )}
                       </button>
                    </TooltipTrigger>
                    <TooltipContent side="top" sideOffset={12} className="bg-black/80 backdrop-blur-xl border-white/10 text-white text-[10px] font-black p-2 px-3 rounded-lg uppercase tracking-widest">
                       {tool.label}
                    </TooltipContent>
                 </Tooltip>
                 {(idx === 0 || idx === 8) && <div className="w-[1px] h-5 bg-white/10 mx-1" />}
              </React.Fragment>
           ))}
        </div>
      </div>
    </TooltipProvider>
  );
}
