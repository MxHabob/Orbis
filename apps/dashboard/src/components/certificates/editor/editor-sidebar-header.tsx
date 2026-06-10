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
} from "@orbis/ui/dropdown-menu";
import { Badge } from "@orbis/ui/badge";
import { useEditorStore } from "@/store/certificate-editor";
import { cn } from "@orbis/ui/cn";
import { Input } from "@orbis/ui/input";

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
    <div className="px-3 py-4 space-y-3 bg-background border-b border-border">
      <div className="flex items-start justify-between">
        <div className="space-y-1 w-full flex flex-col">
          <div className="flex items-center gap-1 group w-full">
            {isEditing ? (
              <Input 
                  autoFocus
                  className="h-7 text-[14px] font-bold px-2 py-0.5 w-full max-w-[140px] bg-secondary border-primary/20"
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
                <span className="text-[14px] font-bold text-foreground tracking-tight truncate">
                  {name}
                </span>
                <Edit2 className="w-3 h-3 text-muted-foreground opacity-0 group-hover/title:opacity-40 transition-opacity shrink-0" />
              </div>
            )}
            
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <button className="w-5 h-5 rounded flex items-center justify-center bg-primary/10 text-primary hover:bg-primary/20 transition-colors outline-none shrink-0 ml-auto focus:ring-1 focus:ring-ring">
                  <ChevronDown className="w-3.5 h-3.5" />
                </button>
              </DropdownMenuTrigger>
              
              <DropdownMenuContent className="w-64">
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
                
                <DropdownMenuSeparator />
                
                <DropdownMenuItem onClick={() => setIsEditing(true)} className="text-xs gap-3 py-2">
                  <Edit2 className="w-3.5 h-3.5 opacity-60" />
                  Rename...
                </DropdownMenuItem>

                <DropdownMenuSub>
                  <DropdownMenuSubTrigger className="text-xs gap-3 py-2">
                    Add to sidebar
                  </DropdownMenuSubTrigger>
                  <DropdownMenuPortal>
                    <DropdownMenuSubContent>
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
                      <DropdownMenuSubContent>
                          <DropdownMenuItem className="text-xs">sRGB</DropdownMenuItem>
                          <DropdownMenuItem className="text-xs">Display P3</DropdownMenuItem>
                      </DropdownMenuSubContent>
                  </DropdownMenuPortal>
                </DropdownMenuSub>

                <DropdownMenuSeparator />

                <DropdownMenuItem className="text-xs gap-3 py-2">
                  <Copy className="w-3.5 h-3.5 opacity-60" />
                  Duplicate
                </DropdownMenuItem>
                <DropdownMenuItem className="text-xs gap-3 py-2 opacity-50">
                  Move file...
                </DropdownMenuItem>
                <DropdownMenuItem className="text-xs gap-3 py-2 text-destructive focus:text-destructive">
                  <Trash2 className="w-3.5 h-3.5" />
                  Move to trash
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </div>

          <div className="flex items-center gap-2">
            <span className="text-[11px] text-muted-foreground font-medium">Drafts</span>
            <Badge variant="secondary" className="text-primary hover:bg-secondary/80 border-none text-[9px] px-1.5 h-4.5 rounded uppercase font-bold tracking-wider leading-none">
                Free
            </Badge>
          </div>
        </div>
      </div>
    </div>
  );
}
