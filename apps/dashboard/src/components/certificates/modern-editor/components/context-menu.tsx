"use client";

import React from "react";
import { 
  ContextMenu, 
  ContextMenuContent, 
  ContextMenuItem, 
  ContextMenuTrigger,
  ContextMenuSeparator,
  ContextMenuShortcut,
  ContextMenuSub,
  ContextMenuSubTrigger,
  ContextMenuSubContent
} from "@midday/ui/context-menu";
import { 
  Copy, 
  Trash2, 
  ArrowUp, 
  ArrowDown, 
  ChevronUp, 
  ChevronDown,
  Lock,
  EyeOff,
  Group,
  Ungroup,
  MousePointer2,
  Layers,
  Zap
} from "lucide-react";
import { useEditorStore } from "../store/use-editor-store";

interface Props {
  children: React.ReactNode;
}

export function EditorContextMenu({ children }: Props) {
  const { 
    selectedElementIds, 
    deleteElements, 
    duplicateElements, 
    bringToFront, 
    sendToBack,
    bringForward,
    sendBackward,
    elements
  } = useEditorStore();

  const selectionCount = selectedElementIds.length;
  const isSelected = selectionCount > 0;

  return (
    <ContextMenu>
      <ContextMenuTrigger asChild>
        {children}
      </ContextMenuTrigger>
      
      <ContextMenuContent className="w-64 bg-[#2c2c2c] border-[#444] text-white/90 shadow-2xl p-1 rounded-[8px]">
        {isSelected ? (
          <>
            <ContextMenuItem onClick={() => duplicateElements(selectedElementIds)} className="gap-2 focus:bg-[#0c8ce9] focus:text-white rounded-[4px] text-[11px] font-medium py-1.5 px-2">
              <Copy className="w-3.5 h-3.5" />
              Duplicate
              <ContextMenuShortcut className="text-white/40">⌘D</ContextMenuShortcut>
            </ContextMenuItem>
            
            <ContextMenuItem onClick={() => deleteElements(selectedElementIds)} className="gap-2 focus:bg-red-500/20 focus:text-red-400 text-red-400 rounded-[4px] text-[11px] font-medium py-1.5 px-2">
              <Trash2 className="w-3.5 h-3.5" />
              Delete
              <ContextMenuShortcut className="text-red-400/40">⌫</ContextMenuShortcut>
            </ContextMenuItem>

            <ContextMenuSeparator className="bg-white/10 my-1" />

            <ContextMenuSub>
              <ContextMenuSubTrigger className="gap-2 focus:bg-[#0c8ce9] focus:text-white rounded-[4px] text-[11px] font-medium py-1.5 px-2">
                <Layers className="w-3.5 h-3.5" />
                Arrange
              </ContextMenuSubTrigger>
              <ContextMenuSubContent className="bg-[#2c2c2c] border-[#444] text-white/90 p-1 rounded-[8px]">
                {(() => {
                  const firstId = selectedElementIds[0];
                  if (!firstId) return null;
                  
                  return (
                    <>
                      <ContextMenuItem onClick={() => bringToFront(firstId)} className="gap-2 focus:bg-[#0c8ce9] focus:text-white py-1.5 px-2 text-[11px]">
                        <ArrowUp className="w-3.5 h-3.5" />
                        Bring to Front
                        <ContextMenuShortcut className="text-white/40">] ]</ContextMenuShortcut>
                      </ContextMenuItem>
                      <ContextMenuItem onClick={() => bringForward(firstId)} className="gap-2 focus:bg-[#0c8ce9] focus:text-white py-1.5 px-2 text-[11px]">
                        <ChevronUp className="w-3.5 h-3.5" />
                        Bring Forward
                        <ContextMenuShortcut className="text-white/40">]</ContextMenuShortcut>
                      </ContextMenuItem>
                      <ContextMenuItem onClick={() => sendBackward(firstId)} className="gap-2 focus:bg-[#0c8ce9] focus:text-white py-1.5 px-2 text-[11px]">
                        <ChevronDown className="w-3.5 h-3.5" />
                        Send Backward
                        <ContextMenuShortcut className="text-white/40">[</ContextMenuShortcut>
                      </ContextMenuItem>
                      <ContextMenuItem onClick={() => sendToBack(firstId)} className="gap-2 focus:bg-[#0c8ce9] focus:text-white py-1.5 px-2 text-[11px]">
                        <ArrowDown className="w-3.5 h-3.5" />
                        Send to Back
                        <ContextMenuShortcut className="text-white/40">[ [</ContextMenuShortcut>
                      </ContextMenuItem>
                    </>
                  );
                })()}
              </ContextMenuSubContent>
            </ContextMenuSub>

            <ContextMenuSeparator className="bg-white/10 my-1" />

            <ContextMenuItem className="gap-2 focus:bg-[#0c8ce9] focus:text-white rounded-[4px] text-[11px] font-medium py-1.5 px-2">
              <Group className="w-3.5 h-3.5" />
              Group Selection
              <ContextMenuShortcut className="text-white/40">⌘G</ContextMenuShortcut>
            </ContextMenuItem>

            <ContextMenuItem className="gap-2 focus:bg-[#0c8ce9] focus:text-white rounded-[4px] text-[11px] font-medium py-1.5 px-2">
              <Lock className="w-3.5 h-3.5" />
              Lock Selection
              <ContextMenuShortcut className="text-white/40">⌘L</ContextMenuShortcut>
            </ContextMenuItem>

            <ContextMenuSeparator className="bg-white/10 my-1" />

            <ContextMenuItem className="gap-2 focus:bg-emerald-500/20 focus:text-emerald-400 rounded-[4px] text-[11px] font-bold py-1.5 px-2">
              <Zap className="w-3.5 h-3.5" />
              Add Interaction
              <ContextMenuShortcut className="text-emerald-400/40">⌘⇧K</ContextMenuShortcut>
            </ContextMenuItem>
          </>
        ) : (
          <>
            <ContextMenuItem className="gap-2 focus:bg-[#0c8ce9] focus:text-white rounded-[4px] text-[11px] font-medium py-1.5 px-2">
              <MousePointer2 className="w-3.5 h-3.5" />
              Select All
              <ContextMenuShortcut className="text-white/40">⌘A</ContextMenuShortcut>
            </ContextMenuItem>
            <ContextMenuSeparator className="bg-white/10 my-1" />
            <ContextMenuItem className="gap-2 focus:bg-[#0c8ce9] focus:text-white rounded-[4px] text-[11px] font-medium py-1.5 px-2">
              Paste Here
              <ContextMenuShortcut className="text-white/40">⌘V</ContextMenuShortcut>
            </ContextMenuItem>
          </>
        )}
      </ContextMenuContent>
    </ContextMenu>
  );
}
