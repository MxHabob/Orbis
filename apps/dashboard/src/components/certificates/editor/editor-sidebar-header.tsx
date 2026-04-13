"use client";

import React, { useState, useEffect } from "react";
import { 
  ChevronDown, 
  History, 
  Share2, 
  FileDown, 
  GitBranch, 
  Palette, 
  Trash2,
  Copy,
  ChevronRight,
  Edit2
} from "lucide-react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
  DropdownMenuSeparator,
  DropdownMenuSub,
  DropdownMenuSubTrigger,
  DropdownMenuSubContent,
  DropdownMenuPortal,
} from "@midday/ui/dropdown-menu";
import { Badge } from "@midday/ui/badge";
import { useEditorStore } from "@/store/certificate-editor";

export function EditorSidebarHeader() {
  const { name, setName } = useEditorStore();
  const [isEditing, setIsEditing] = useState(false);
  const [tempName, setTempName] = useState(name);

  useEffect(() => {
    setTempName(name);
  }, [name]);

  const handleRename = () => {
    if (tempName.trim()) {
        setName(tempName);
    }
    setIsEditing(false);
  };

  return (
    <div className="px-3 py-4 space-y-3 bg-[#1e1e1e] border-b border-white/5">
      <div className="flex items-start justify-between">
        <div className="space-y-1 w-full flex flex-col">
          <div className="flex items-center gap-1 group w-full">
            {isEditing ? (
              <input 
                  autoFocus
                  className="bg-[#2c2c2c] text-white text-[14px] font-bold px-2 py-0.5 rounded border border-blue-500/50 outline-none w-full max-w-[140px]"
                  value={tempName}
                  onChange={(e) => setTempName(e.target.value)}
                  onBlur={handleRename}
                  onKeyDown={(e) => {
                      if (e.key === 'Enter') handleRename();
                      if (e.key === 'Escape') {
                          setTempName(name);
                          setIsEditing(false);
                      }
                  }}
              />
            ) : (
              <div 
                onDoubleClick={() => setIsEditing(true)}
                className="flex items-center gap-1 cursor-text group/title max-w-[160px]"
              >
                <span className="text-[14px] font-bold text-white tracking-tight truncate">
                  {name}
                </span>
              </div>
            )}
            
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <button className="w-5 h-5 rounded flex items-center justify-center bg-blue-500/20 text-blue-400 hover:bg-blue-500/30 transition-colors outline-none shrink-0 ml-auto">
                  <ChevronDown className="w-3.5 h-3.5" />
                </button>
              </DropdownMenuTrigger>
              
              <DropdownMenuContent className="w-64 bg-[#2c2c2c] border-white/10 text-white shadow-2xl">
                <DropdownMenuItem className="text-xs gap-3 py-2">
                  <History className="w-3.5 h-3.5 opacity-60" />
                  Show version history
                  <span className="ml-auto text-[10px] opacity-40 italic">Ctrl+Alt+H</span>
                </DropdownMenuItem>
                <DropdownMenuItem className="text-xs gap-3 py-2 opacity-30 cursor-not-allowed">
                  Publish library...
                </DropdownMenuItem>
                <DropdownMenuItem className="text-xs gap-3 py-2">
                  <FileDown className="w-3.5 h-3.5 opacity-60" />
                  Export...
                  <span className="ml-auto text-[10px] opacity-40">Ctrl+Shift+E</span>
                </DropdownMenuItem>
                
                <DropdownMenuSeparator className="bg-white/5" />
                
                <DropdownMenuItem onClick={() => setIsEditing(true)} className="text-xs gap-3 py-2">
                  <Edit2 className="w-3.5 h-3.5 opacity-60" />
                  Rename...
                </DropdownMenuItem>

                <DropdownMenuSub>
                  <DropdownMenuSubTrigger className="text-xs gap-3 py-2">
                    Add to sidebar
                  </DropdownMenuSubTrigger>
                  <DropdownMenuPortal>
                    <DropdownMenuSubContent className="bg-[#2c2c2c] border-white/10 text-white">
                      <DropdownMenuItem className="text-xs">Favorites</DropdownMenuItem>
                      <DropdownMenuItem className="text-xs">Recents</DropdownMenuItem>
                    </DropdownMenuSubContent>
                  </DropdownMenuPortal>
                </DropdownMenuSub>

                <DropdownMenuItem className="text-xs gap-3 py-2">
                  <GitBranch className="w-3.5 h-3.5 opacity-60" />
                  Create branch...
                </DropdownMenuItem>

                <DropdownMenuSub>
                  <DropdownMenuSubTrigger className="text-xs gap-3 py-2">
                    File color profile
                  </DropdownMenuSubTrigger>
                  <DropdownMenuPortal>
                      <DropdownMenuSubContent className="bg-[#2c2c2c] border-white/10 text-white">
                          <DropdownMenuItem className="text-xs">sRGB</DropdownMenuItem>
                          <DropdownMenuItem className="text-xs">Display P3</DropdownMenuItem>
                      </DropdownMenuSubContent>
                  </DropdownMenuPortal>
                </DropdownMenuSub>

                <DropdownMenuSeparator className="bg-white/5" />

                <DropdownMenuItem className="text-xs gap-3 py-2">
                  <Copy className="w-3.5 h-3.5 opacity-60" />
                  Duplicate
                </DropdownMenuItem>
                <DropdownMenuItem className="text-xs gap-3 py-2 opacity-50">
                  Move file...
                </DropdownMenuItem>
                <DropdownMenuItem className="text-xs gap-3 py-2 text-red-400 hover:text-red-300">
                  <Trash2 className="w-3.5 h-3.5" />
                  Move to trash
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </div>

          <div className="flex items-center gap-2">
            <span className="text-[11px] text-[#919191] font-medium">Drafts</span>
            <Badge variant="secondary" className="bg-[#2b3548] text-[#8ea7ff] hover:bg-[#2b3548] border-none text-[9px] px-1.5 h-4.5 rounded uppercase font-bold tracking-wider leading-none">
                Free
            </Badge>
          </div>
        </div>
      </div>
    </div>
  );
}
