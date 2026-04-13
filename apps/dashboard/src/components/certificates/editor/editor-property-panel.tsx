"use client";

import React from "react";
import { useEditorStore } from "@/store/certificate-editor";
import { Input } from "@midday/ui/input";
import { Label } from "@midday/ui/label";
import { Slider } from "@midday/ui/slider";
import { 
  Plus,
  Share2,
  Settings2,
  Zap,
  Trash2
} from "lucide-react";
import { Button } from "@midday/ui/button";
import { ColorPicker } from "@/components/color-picker";
import { cn } from "@midday/ui/cn";
import { ScrollArea } from "@midday/ui/scroll-area";
import { Separator } from "@midday/ui/separator";
import { Avatar, AvatarFallback } from "@midday/ui/avatar";

interface PropertyPanelProps {
  onSave?: () => void;
  isSaving?: boolean;
}

export function EditorPropertyPanel({ onSave, isSaving }: PropertyPanelProps) {
  const { elements, selectedElementIds, updateElement, canvasConfig, setCanvasConfig, mode, setMode } = useEditorStore();

  const selectedElement = elements.find((el) => el.id === selectedElementIds[0]);

  const handleUpdate = (updates: any) => {
    if (selectedElement) {
       updateElement(selectedElement.id, updates);
    }
  };

  return (
    <div className="h-full flex flex-col glass-panel select-none w-[280px] z-20 border-l-0 shadow-[-10px_0_30px_rgba(0,0,0,0.3)]">
      {/* Premium Header */}
      <div className="p-5 flex items-center justify-between shrink-0">
          <div className="flex -space-x-2">
            {[1, 2, 3].map(i => (
                <Avatar key={i} className="h-7 w-7 border-2 border-[#121212] ring-1 ring-white/10">
                    <AvatarFallback className={cn("text-[9px] font-bold text-white", i === 1 ? "bg-blue-600" : "bg-purple-600")}>{i === 1 ? 'JD' : 'AS'}</AvatarFallback>
                </Avatar>
            ))}
            <div className="h-7 w-7 rounded-full bg-white/5 border-2 border-[#121212] flex items-center justify-center text-[9px] font-bold text-white/40 ring-1 ring-white/10">+2</div>
          </div>

          <div className="flex items-center gap-2">
             <Button variant="ghost" size="icon" className="h-9 w-9 text-white/40 hover:text-white hover:bg-white/5 rounded-xl transition-all">
                <Share2 className="w-4 h-4" />
             </Button>
             <Button 
                onClick={onSave}
                disabled={isSaving}
                className="h-9 bg-blue-600 hover:bg-blue-500 text-white text-xs font-bold px-5 rounded-xl shadow-lg shadow-blue-600/20 truncate"
             >
                {isSaving ? "Saving..." : "Deploy"}
             </Button>
          </div>
      </div>

      {/* Tabs / Modes */}
      <div className="px-5 mb-4">
          <div className="flex p-1 bg-white/[0.03] rounded-xl gap-1 border border-white/[0.05]">
             <button 
                onClick={() => setMode('design')}
                className={cn(
                    "flex-1 py-2 rounded-lg text-xs font-bold transition-all",
                    mode === 'design' ? "bg-white/[0.07] text-white shadow-xl ring-1 ring-white/10" : "text-white/30 hover:text-white/50"
                )}
             >
                Design
             </button>
             <button 
                onClick={() => setMode('prototype')}
                className={cn(
                    "flex-1 py-2 rounded-lg text-xs font-bold transition-all",
                    mode === 'prototype' ? "bg-white/[0.07] text-white shadow-xl ring-1 ring-white/10" : "text-white/30 hover:text-white/50"
                )}
             >
                Proto
             </button>
          </div>
      </div>

      <ScrollArea className="flex-1">
          <div className="px-5 pb-10">
              {mode === 'prototype' ? (
                 <div className="space-y-8 mt-2">
                    {!selectedElement ? (
                        <div className="flex flex-col items-center justify-center py-20 text-center opacity-20">
                            <Zap className="w-10 h-10 mb-4" strokeWidth={1} />
                            <p className="text-[11px] font-medium uppercase tracking-widest leading-6">Select element to<br/>create interaction</p>
                        </div>
                    ) : (
                        <div className="space-y-6">
                            <SectionHeader title="Interactions" icon={Plus} />
                            {selectedElement.interactions?.length === 0 ? (
                                <button 
                                    onClick={() => handleUpdate({ interactions: [{ trigger: 'onClick', action: 'openUrl', value: 'https://' }] })}
                                    className="w-full p-4 border border-dashed border-white/10 rounded-2xl hover:bg-white/5 transition-all text-center group"
                                >
                                    <p className="text-[11px] text-white/30 font-medium group-hover:text-white/60">Tap to add interaction</p>
                                </button>
                            ) : (
                                <div className="space-y-4">
                                    {selectedElement.interactions?.map((int, i) => (
                                        <div key={i} className="bg-white/[0.03] p-4 rounded-2xl space-y-4 border border-white/10 shadow-xl relative group/int">
                                            <div className="flex items-center justify-between">
                                                <div className="px-2 py-1 rounded-md bg-blue-500/10 text-blue-400 text-[10px] font-bold uppercase tracking-wider">Click</div>
                                                <button className="opacity-0 group-hover/int:opacity-100 transition-opacity text-white/20 hover:text-red-400" onClick={() => handleUpdate({ interactions: [] })}>
                                                    <Trash2 className="w-3.5 h-3.5" />
                                                </button>
                                            </div>
                                            <div className="space-y-1.5">
                                                <Label className="text-[9px] text-white/20 uppercase font-black tracking-widest">Navigate to</Label>
                                                <Input 
                                                    className="h-8 bg-black/20 border-white/5 text-[11px] rounded-lg focus:ring-blue-500"
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
                 <div className="space-y-8 mt-2">
                    {!selectedElement ? (
                        <div className="space-y-8">
                            <div>
                                <SectionHeader title="Design Settings" />
                                <div className="mt-4 space-y-4">
                                    <div className="space-y-2">
                                        <Label className="text-[10px] font-bold text-white/20 uppercase tracking-widest font-black">Canvas Color</Label>
                                        <ColorControl 
                                            value={canvasConfig.backgroundColor || "#121212"} 
                                            onChange={(c) => setCanvasConfig({ backgroundColor: c })}
                                        />
                                    </div>
                                    <div className="grid grid-cols-2 gap-3">
                                        <div className="bg-white/[0.03] p-3 rounded-xl border border-white/[0.05]">
                                             <Label className="text-[9px] text-white/20 uppercase font-black">Grid</Label>
                                             <div className="mt-1 flex items-center justify-between">
                                                <span className="text-xs font-bold text-white/60">{canvasConfig.gridSize}px</span>
                                                <div className={cn("w-3 h-3 rounded-full", canvasConfig.showGrid ? "bg-blue-500 shadow-[0_0_8px_rgba(59,130,246,0.5)]" : "bg-white/10")} />
                                             </div>
                                        </div>
                                        <div className="bg-white/[0.03] p-3 rounded-xl border border-white/[0.05]">
                                             <Label className="text-[9px] text-white/20 uppercase font-black">Rulers</Label>
                                             <div className="mt-1 flex items-center justify-between">
                                                <span className="text-xs font-bold text-white/60">{canvasConfig.showRulers ? 'ON' : 'OFF'}</span>
                                                <div className={cn("w-3 h-3 rounded-full", canvasConfig.showRulers ? "bg-white/40" : "bg-white/10")} />
                                             </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            <Separator className="bg-white/5" />
                            <div className="opacity-40 grayscale group cursor-not-allowed">
                                <SectionHeader title="Styles" icon={Plus} />
                                <p className="text-[10px] mt-2 text-white/30">No global styles defined</p>
                            </div>
                        </div>
                    ) : (
                        <div className="space-y-8">
                            <div>
                                <SectionHeader title="Transform" />
                                <div className="grid grid-cols-2 gap-x-6 gap-y-5 mt-4 bg-white/[0.02] p-4 rounded-2xl border border-white/[0.05]">
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
                                    <div className="col-span-2 pt-1 border-t border-white/5">
                                        <PropertyField 
                                            label="°" 
                                            value={Math.round(selectedElement.rotation)} 
                                            onChange={(v) => handleUpdate({ rotation: v })}
                                        />
                                    </div>
                                </div>
                            </div>
                            
                            <Separator className="bg-white/5" />
                            
                            {selectedElement.type === 'text' && (
                                <>
                                    <div className="space-y-4">
                                        <SectionHeader title="Typography" icon={Settings2} />
                                        <div className="space-y-4">
                                            <div className="bg-white/[0.03] p-3 rounded-xl border border-white/5">
                                                <span className="text-[10px] text-white/20 uppercase font-black block mb-2">Font Size</span>
                                                <div className="flex items-center gap-4">
                                                    <Slider 
                                                        value={[selectedElement.style.fontSize || 32]} 
                                                        max={200} 
                                                        step={1} 
                                                        onValueChange={([v]) => handleUpdate({ style: { ...selectedElement.style, fontSize: v } })}
                                                    />
                                                    <span className="text-xs font-bold w-8 text-right">{selectedElement.style.fontSize}</span>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                    <Separator className="bg-white/5" />
                                </>
                            )}

                            <div className="space-y-4">
                                <SectionHeader title="Appearance" icon={Plus} />
                                <div className="space-y-4">
                                    <div className="space-y-2">
                                        <Label className="text-[10px] font-bold text-white/20 uppercase tracking-widest font-black">Fill</Label>
                                        <ColorControl 
                                            value={selectedElement.style.color || selectedElement.style.backgroundColor || "#000000"} 
                                            onChange={(color) => handleUpdate({ 
                                                style: { [selectedElement.type === 'shape' || selectedElement.type === 'frame' ? 'backgroundColor' : 'color']: color } 
                                            })}
                                        />
                                    </div>
                                    <div className="bg-white/[0.03] p-3 rounded-xl border border-white/5">
                                        <div className="flex items-center justify-between mb-2">
                                            <span className="text-[10px] text-white/20 uppercase font-black">Opacity</span>
                                            <span className="text-xs font-bold">{Math.round((selectedElement.opacity ?? 1) * 100)}%</span>
                                        </div>
                                        <Slider 
                                            value={[(selectedElement.opacity ?? 1) * 100]} 
                                            max={100} 
                                            onValueChange={([v]) => handleUpdate({ opacity: v / 100 })}
                                        />
                                    </div>
                                </div>
                            </div>
                        </div>
                    )}
                 </div>
              )}
          </div>
      </ScrollArea>
    </div>
  );
}

function SectionHeader({ title, icon: Icon }: { title: string, icon?: any }) {
    return (
        <div className="flex items-center justify-between group cursor-pointer transition-all hover:translate-x-1">
            <span className="text-xs font-bold text-white/90 tracking-tight">{title}</span>
            {Icon && <Icon className="w-4 h-4 text-white/20 group-hover:text-white transition-colors" />}
        </div>
    )
}

function PropertyField({ label, value, onChange }: { label: string, value: number, onChange: (v: number) => void }) {
    return (
        <div className="flex items-center gap-3 group">
            <span className="text-[10px] font-black text-white/20 w-3">{label}</span>
            <input 
                type="number" 
                value={value}
                onChange={(e) => onChange(Number(e.target.value))}
                className="bg-transparent border-none text-xs text-white/80 font-bold p-0 focus:ring-0 w-full [appearance:textfield] transition-colors focus:text-blue-400"
            />
        </div>
    )
}

function ColorControl({ value, onChange }: { value: string, onChange: (v: string) => void }) {
    return (
        <div className="flex items-center gap-3 bg-black/20 p-2 rounded-xl border border-white/5 group hover:border-white/20 transition-all">
            <div className="relative h-6 w-10 shrink-0">
                <ColorPicker value={value} onSelect={onChange} />
            </div>
            <input 
                value={value.toUpperCase()} 
                onChange={(e) => onChange(e.target.value)}
                className="bg-transparent border-none text-[11px] text-white/60 font-mono font-bold w-full focus:ring-0 uppercase tracking-wider"
            />
        </div>
    );
}

