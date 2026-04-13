"use client";

import React from "react";
import { useEditorStore } from "../store/use-editor-store";
import { Input } from "@midday/ui/input";
import { Label } from "@midday/ui/label";
import { Slider } from "@midday/ui/slider";
import { 
  Plus,
  Play,
  Share2,
  ChevronDown,
  Eye,
  Settings2,
  Type as TypeIcon,
  MousePointer2,
  HelpCircle,
  Zap,
  Trash2
} from "lucide-react";
import { Button } from "@midday/ui/button";
import { ColorPicker } from "@/components/color-picker";
import { cn } from "@midday/ui/cn";
import { ScrollArea } from "@midday/ui/scroll-area";
import { Separator } from "@midday/ui/separator";
import { Avatar, AvatarFallback, AvatarImage } from "@midday/ui/avatar";

interface PropertyPanelProps {
  onSave?: () => void;
  isSaving?: boolean;
}

export function PropertyPanel({ onSave, isSaving }: PropertyPanelProps) {
  const { elements, selectedElementIds, updateElement, canvasConfig, setCanvasConfig, mode, setMode } = useEditorStore();

  const selectedElement = elements.find((el) => el.id === selectedElementIds[0]);

  const handleUpdate = (updates: any) => {
    if (selectedElement) {
       updateElement(selectedElement.id, updates);
    }
  };

  return (
    <div className="h-full flex flex-col bg-[#1e1e1e] border-l border-[#333333] select-none w-60 shadow-2xl">
      {/* Figma UI3 Right Header */}
      <div className="h-12 flex items-center justify-between px-3 shrink-0">
          <div className="flex items-center gap-2">
            <Avatar className="h-6 w-6 border border-[#444]">
                <AvatarFallback className="bg-emerald-600 text-[10px] text-white font-bold">M</AvatarFallback>
            </Avatar>
            <ChevronDown className="w-3 h-3 text-white/40" />
          </div>

          <div className="flex items-center gap-2">
             <Button variant="ghost" size="icon" className="h-8 w-8 text-white/60 hover:text-white">
                <Play className="w-4 h-4 fill-white/10" />
             </Button>
             <Button 
                onClick={onSave}
                disabled={isSaving}
                className="h-8 bg-[#0c8ce9] hover:bg-[#0a7bc9] text-white text-[11px] font-bold px-4 rounded-[6px] shadow-lg shadow-blue-500/10 min-w-[60px]"
             >
                {isSaving ? "Saving..." : "Save"}
             </Button>
          </div>
      </div>

      {/* Mode Switcher & Zoom */}
      <div className="flex items-center justify-between px-3 py-1 bg-accent/5">
          <div className="flex p-1 bg-[#2c2c2c] rounded-[6px] gap-0.5 shadow-inner">
             <button 
                onClick={() => setMode('design')}
                className={cn(
                    "px-3 py-1 rounded-[4px] text-[11px] font-bold transition-all",
                    mode === 'design' ? "bg-[#383838] text-white shadow-sm" : "text-white/40 hover:text-white/60"
                )}
             >
                Design
             </button>
             <button 
                onClick={() => setMode('prototype')}
                className={cn(
                    "px-3 py-1 rounded-[4px] text-[11px] font-bold transition-all",
                    mode === 'prototype' ? "bg-[#383838] text-white shadow-sm" : "text-white/40 hover:text-white/60"
                )}
             >
                Prototype
             </button>
          </div>
          <div className="flex items-center gap-1 cursor-pointer group">
             <span className="text-[11px] font-bold text-white/40 group-hover:text-white/80">{Math.round(canvasConfig.zoom * 100)}%</span>
             <ChevronDown className="w-3 h-3 text-white/20" />
          </div>
      </div>

      <ScrollArea className="flex-1">
          <div className="flex flex-col">
              {mode === 'prototype' ? (
                 <div className="p-4 space-y-6">
                    {!selectedElement ? (
                        <div className="flex flex-col items-center justify-center py-24 text-center space-y-4 opacity-30">
                            <Zap className="w-10 h-10 text-emerald-400" strokeWidth={1} />
                            <p className="text-[10px] font-medium leading-relaxed uppercase tracking-widest text-white">Select an element<br/>to add interactions</p>
                        </div>
                    ) : (
                        <div className="space-y-6">
                            <SectionHeader title="Interactions" icon={Plus} />
                            {selectedElement.interactions?.length === 0 ? (
                                <div className="p-3 border border-dashed border-white/10 rounded-[6px] text-center">
                                    <p className="text-[10px] text-white/40">No interactions yet</p>
                                    <Button 
                                        variant="link" 
                                        className="text-emerald-400 text-[10px] h-auto p-0 mt-1"
                                        onClick={() => handleUpdate({ interactions: [{ trigger: 'onClick', action: 'openUrl', value: 'https://' }] })}
                                    >
                                        + Click to add
                                    </Button>
                                </div>
                            ) : (
                                <div className="space-y-3">
                                    {selectedElement.interactions?.map((int, i) => (
                                        <div key={i} className="bg-[#2c2c2c] p-2 rounded-[6px] space-y-2 border border-white/5">
                                            <div className="flex items-center justify-between">
                                                <span className="text-[10px] font-bold text-emerald-400">On Click</span>
                                                <Button size="icon" variant="ghost" className="h-5 w-5 text-white/20 hover:text-red-400" onClick={() => handleUpdate({ interactions: [] })}>
                                                    <Trash2 className="w-3 h-3" />
                                                </Button>
                                            </div>
                                            <div className="space-y-1">
                                                <span className="text-[9px] text-white/40 uppercase font-bold">Action</span>
                                                <div className="bg-[#1e1e1e] px-2 py-1 rounded text-[10px] text-white/90">Open URL</div>
                                            </div>
                                            <div className="space-y-1">
                                                <span className="text-[9px] text-white/40 uppercase font-bold">URL</span>
                                                <input 
                                                    className="w-full bg-[#1e1e1e] border-none rounded text-[10px] p-1 text-white/90 outline-none"
                                                    value={int.value}
                                                    onChange={(e) => {
                                                        const currentInts = selectedElement.interactions;
                                                        if (!currentInts) return;
                                                        const newInts = [...currentInts];
                                                        if (newInts[i]) {
                                                            newInts[i].value = e.target.value;
                                                            handleUpdate({ interactions: newInts });
                                                        }
                                                    }}
                                                />
                                            </div>
                                        </div>
                                    ))}
                                </div>
                            )}
                        </div>
                    )}
                 </div>
              ) : (
                 <div className="p-4 space-y-6">
                    {!selectedElement ? (
                        <div className="space-y-6">
                            <SectionHeader title="Page" />
                            <div className="space-y-3">
                                <Label className="text-[9px] font-bold text-white/40 uppercase tracking-widest pl-1">Background</Label>
                                <div className="p-1 bg-[#181818] rounded border border-white/5">
                                    <ColorPicker 
                                        value={canvasConfig.backgroundColor || "#121212"} 
                                        onSelect={(color) => setCanvasConfig({ backgroundColor: color })}
                                    />
                                </div>
                            </div>
                            <Separator className="bg-[#333333]" />
                            <SectionHeader title="Variables" icon={Settings2} />
                            <Separator className="bg-[#333333]" />
                            <SectionHeader title="Styles" icon={Plus} />
                            <Separator className="bg-[#333333]" />
                            <SectionHeader title="Export" icon={Plus} />
                        </div>
                    ) : (
                        <div className="space-y-6">
                            {/* Transformations */}
                            <div className="grid grid-cols-2 gap-x-4 gap-y-3">
                                <PropertyField 
                                    label="X" 
                                    value={Math.round(selectedElement.x)} 
                                    onChange={(v) => handleUpdate({ x: v })}
                                />
                                <PropertyField 
                                    label="Y" 
                                    value={Math.round(selectedElement.y)} 
                                    onChange={(v) => handleUpdate({ y: v })}
                                />
                                <PropertyField 
                                    label="W" 
                                    value={Math.round(selectedElement.width)} 
                                    onChange={(v) => handleUpdate({ width: v })}
                                />
                                <PropertyField 
                                    label="H" 
                                    value={Math.round(selectedElement.height)} 
                                    onChange={(v) => handleUpdate({ height: v })}
                                />
                            </div>
                            
                            <Separator className="bg-[#333333]" />
                            
                            {/* Fill Section */}
                            <div className="space-y-3">
                                <SectionHeader title="Fill" icon={Plus} />
                                <div className="p-1 bg-[#181818] rounded border border-white/5">
                                    <ColorPicker 
                                        value={selectedElement.style.color || selectedElement.style.backgroundColor || "#000000"} 
                                        onSelect={(color) => handleUpdate({ 
                                            style: { [selectedElement.type === 'shape' || selectedElement.type === 'frame' ? 'backgroundColor' : 'color']: color } 
                                        })}
                                    />
                                </div>
                            </div>

                            <Separator className="bg-[#333333]" />
                            <SectionHeader title="Stroke" icon={Plus} />
                            <Separator className="bg-[#333333]" />
                            <SectionHeader title="Effects" icon={Plus} />
                        </div>
                    )}
                 </div>
              )}
          </div>
      </ScrollArea>

      <div className="p-4 mt-auto flex justify-end">
          <div className="w-7 h-7 rounded-full bg-[#2c2c2c] border border-[#444] flex items-center justify-center text-white/40 hover:text-white cursor-pointer transition-colors shadow-lg">
             <HelpCircle className="w-4 h-4" />
          </div>
      </div>
    </div>
  );
}

function SectionHeader({ title, icon: Icon }: { title: string, icon?: any }) {
    return (
        <div className="flex items-center justify-between group cursor-pointer">
            <span className="text-[11px] font-bold text-white/90">{title}</span>
            {Icon && <Icon className="w-3.5 h-3.5 text-white/40 group-hover:text-white transition-colors" />}
        </div>
    )
}

function PropertyField({ label, value, onChange }: { label: string, value: number, onChange: (v: number) => void }) {
    return (
        <div className="flex items-center gap-2 group">
            <span className="text-[11px] font-bold text-white/20 w-3">{label}</span>
            <input 
                type="number" 
                value={value}
                onChange={(e) => onChange(Number(e.target.value))}
                className="bg-transparent border-none text-[11px] text-white/90 font-medium p-0 focus:ring-0 w-full [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
            />
        </div>
    )
}
