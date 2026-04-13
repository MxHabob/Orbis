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
  Trash2,
  LayoutGrid,
  Layers,
  Grid2X2
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
    <div className="h-full flex flex-col bg-background select-none w-72 z-20 border-l border-border">
      {/* Premium Header */}
      <div className="p-4 flex items-center justify-between shrink-0 border-b border-border">
          <div className="flex -space-x-1.5">
            {[1, 2, 3].map(i => (
                <Avatar key={i} className="h-6 w-6 border-2 border-background ring-1 ring-border">
                    <AvatarFallback className={cn("text-[8px] font-bold text-white", i === 1 ? "bg-primary" : "bg-purple-600")}>{i === 1 ? 'JD' : 'AS'}</AvatarFallback>
                </Avatar>
            ))}
          </div>

          <div className="flex items-center gap-2">
             <Button variant="ghost" size="icon" className="h-8 w-8 text-muted-foreground hover:text-foreground">
                <Share2 className="w-4 h-4" />
             </Button>
             <Button 
                onClick={onSave}
                disabled={isSaving}
                size="sm"
                className="h-8 text-[11px] font-bold px-4 rounded-lg"
             >
                {isSaving ? "Saving..." : "Deploy"}
             </Button>
          </div>
      </div>

      {/* Tabs / Modes */}
      <div className="px-4 py-3">
          <div className="flex p-1 bg-accent/50 rounded-lg gap-1 border border-border">
             <button 
                onClick={() => setMode('design')}
                className={cn(
                    "flex-1 py-1.5 rounded-md text-[11px] font-bold transition-all",
                    mode === 'design' ? "bg-background text-foreground shadow-sm border border-border" : "text-muted-foreground hover:text-foreground"
                )}
             >
                Design
             </button>
             <button 
                onClick={() => setMode('prototype')}
                className={cn(
                    "flex-1 py-1.5 rounded-md text-[11px] font-bold transition-all",
                    mode === 'prototype' ? "bg-background text-foreground shadow-sm border border-border" : "text-muted-foreground hover:text-foreground"
                )}
             >
                Proto
             </button>
          </div>
      </div>

      <ScrollArea className="flex-1">
          <div className="px-4 pb-10">
              {mode === 'prototype' ? (
                 <div className="space-y-6 mt-2">
                    {!selectedElement ? (
                        <div className="flex flex-col items-center justify-center py-20 text-center opacity-30">
                            <Zap className="w-8 h-8 mb-4 stroke-1" />
                            <p className="text-[10px] font-medium uppercase tracking-widest leading-6">Select element to<br/>add interaction</p>
                        </div>
                    ) : (
                        <div className="space-y-6">
                            <SectionHeader title="Interactions" icon={Plus} />
                            {selectedElement.interactions?.length === 0 ? (
                                <button 
                                    onClick={() => handleUpdate({ interactions: [{ trigger: 'onClick', action: 'openUrl', value: 'https://' }] })}
                                    className="w-full p-4 border border-dashed border-border rounded-xl hover:bg-accent/50 transition-all text-center group"
                                >
                                    <p className="text-[10px] text-muted-foreground font-medium group-hover:text-foreground">Add interaction</p>
                                </button>
                            ) : (
                                <div className="space-y-3">
                                    {selectedElement.interactions?.map((int, i) => (
                                         <div key={i} className="bg-accent/30 p-3 rounded-xl space-y-3 border border-border relative group/int">
                                             <div className="flex items-center justify-between">
                                                 <div className="px-2 py-0.5 rounded-md bg-primary/10 text-primary text-[9px] font-bold uppercase tracking-wider">Click</div>
                                                 <button className="opacity-0 group-hover/int:opacity-100 transition-opacity text-muted-foreground hover:text-destructive" onClick={() => handleUpdate({ interactions: [] })}>
                                                     <Trash2 className="w-3 h-3" />
                                                 </button>
                                             </div>
                                             <div className="space-y-1">
                                                 <Label className="text-[9px] text-muted-foreground uppercase font-bold tracking-tight">Navigate to</Label>
                                                 <Input 
                                                     className="h-7 bg-background border-border text-[10px] rounded-lg"
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
                 <div className="space-y-6 mt-2">
                    {!selectedElement ? (
                        <div className="space-y-6">
                            <div>
                                <SectionHeader title="Design Settings" />
                                <div className="mt-3 space-y-4">
                                    <div className="space-y-1.5">
                                        <Label className="text-[10px] font-bold text-muted-foreground uppercase tracking-tight">Canvas Color</Label>
                                        <ColorControl 
                                            value={canvasConfig.backgroundColor || "#121212"} 
                                            onChange={(c) => setCanvasConfig({ backgroundColor: c })}
                                        />
                                    </div>
                                    <div className="grid grid-cols-2 gap-2">
                                        <div className="bg-accent/30 p-2.5 rounded-lg border border-border">
                                             <Label className="text-[9px] text-muted-foreground uppercase font-bold">Grid</Label>
                                             <div className="mt-1 flex items-center justify-between">
                                                <span className="text-[11px] font-medium">{canvasConfig.gridSize}px</span>
                                                <div className={cn("w-2.5 h-2.5 rounded-full", canvasConfig.showGrid ? "bg-primary" : "bg-border")} />
                                             </div>
                                        </div>
                                        <div className="bg-accent/30 p-2.5 rounded-lg border border-border">
                                             <Label className="text-[9px] text-muted-foreground uppercase font-bold">Rulers</Label>
                                             <div className="mt-1 flex items-center justify-between">
                                                <span className="text-[11px] font-medium">{canvasConfig.showRulers ? 'ON' : 'OFF'}</span>
                                                <div className={cn("w-2.5 h-2.5 rounded-full", canvasConfig.showRulers ? "bg-primary" : "bg-border")} />
                                             </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            <Separator className="bg-border" />
                        </div>
                    ) : (
                        <div className="space-y-6">
                            <div>
                                <SectionHeader title="Transform" />
                                <div className="grid grid-cols-2 gap-x-4 gap-y-3 mt-3 bg-accent/30 p-3 rounded-xl border border-border">
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
                                    <div className="col-span-2 pt-1 border-t border-border">
                                        <PropertyField 
                                            label="°" 
                                            value={Math.round(selectedElement.rotation)} 
                                            onChange={(v) => handleUpdate({ rotation: v })}
                                        />
                                    </div>
                                </div>
                            </div>
                            
                            <Separator className="bg-border" />
                            
                            {selectedElement.type === 'frame' && (
                                <>
                                    <div className="space-y-3">
                                        <SectionHeader title="Grid Layout" icon={LayoutGrid} />
                                        <div className="space-y-3">
                                            <div className="grid grid-cols-2 gap-1.5">
                                                <Button 
                                                    variant="outline" 
                                                    size="sm" 
                                                    className="h-7 text-[9px] font-bold gap-1 px-1.5 border-primary/20 bg-primary/5 hover:bg-primary/10 text-primary"
                                                    onClick={() => {
                                                        const a4Width = 794;
                                                        const a4Height = 1123;
                                                        handleUpdate({ width: a4Width, height: Math.floor(a4Height / 2) });
                                                        setCanvasConfig({ layoutConfig: { rows: 2, columns: 1, gap: 0 } });
                                                    }}
                                                >
                                                    <Layers className="w-3 h-3" />
                                                    2-UPS (Half A4)
                                                </Button>
                                                <Button 
                                                    variant="outline" 
                                                    size="sm" 
                                                    className="h-7 text-[9px] font-bold gap-1 px-1.5 border-primary/20 bg-primary/5 hover:bg-primary/10 text-primary"
                                                    onClick={() => {
                                                        const a4Width = 794;
                                                        const a4Height = 1123;
                                                        handleUpdate({ width: Math.floor(a4Width / 2), height: Math.floor(a4Height / 3) });
                                                        setCanvasConfig({ layoutConfig: { rows: 3, columns: 2, gap: 0 } });
                                                    }}
                                                >
                                                    <Grid2X2 className="w-3 h-3" />
                                                    6-UPS (Cards)
                                                </Button>
                                            </div>

                                            <div className="bg-accent/30 p-3 rounded-xl border border-border space-y-3">
                                                <div className="grid grid-cols-2 gap-4">
                                                    <PropertyField 
                                                        label="Rows" 
                                                        value={canvasConfig.layoutConfig?.rows || 1} 
                                                        onChange={(v) => setCanvasConfig({ layoutConfig: { ...canvasConfig.layoutConfig, rows: v, columns: canvasConfig.layoutConfig?.columns || 1, gap: canvasConfig.layoutConfig?.gap || 0 } })}
                                                    />
                                                    <PropertyField 
                                                        label="Cols" 
                                                        value={canvasConfig.layoutConfig?.columns || 1} 
                                                        onChange={(v) => setCanvasConfig({ layoutConfig: { ...canvasConfig.layoutConfig, columns: v, rows: canvasConfig.layoutConfig?.rows || 1, gap: canvasConfig.layoutConfig?.gap || 0 } })}
                                                    />
                                                </div>
                                                <div className="pt-2 border-t border-border/50">
                                                    <PropertyField 
                                                        label="Gap" 
                                                        value={canvasConfig.layoutConfig?.gap || 0} 
                                                        onChange={(v) => setCanvasConfig({ layoutConfig: { ...canvasConfig.layoutConfig, gap: v, rows: canvasConfig.layoutConfig?.rows || 1, columns: canvasConfig.layoutConfig?.columns || 1 } })}
                                                    />
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                    <Separator className="bg-border" />
                                </>
                            )}

                            {selectedElement.type === 'text' && (
                                <>
                                    <div className="space-y-3">
                                        <SectionHeader title="Typography" icon={Settings2} />
                                        <div className="space-y-3">
                                            <div className="bg-accent/30 p-3 rounded-lg border border-border">
                                                <span className="text-[10px] text-muted-foreground uppercase font-bold block mb-2">Font Size</span>
                                                <div className="flex items-center gap-3">
                                                    <Slider 
                                                        value={[selectedElement.style.fontSize || 32]} 
                                                        max={200} 
                                                        step={1} 
                                                        onValueChange={([v]) => handleUpdate({ style: { ...selectedElement.style, fontSize: v } })}
                                                    />
                                                    <span className="text-[11px] font-bold w-6 text-right">{selectedElement.style.fontSize}</span>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                    <Separator className="bg-border" />
                                </>
                            )}

                            <div className="space-y-3">
                                <SectionHeader title="Appearance" icon={Plus} />
                                <div className="space-y-3">
                                    <div className="space-y-1.5">
                                        <Label className="text-[10px] font-bold text-muted-foreground uppercase tracking-tight">Fill</Label>
                                        <ColorControl 
                                            value={selectedElement.style.color || selectedElement.style.backgroundColor || "#000000"} 
                                            onChange={(color) => handleUpdate({ 
                                                style: { [selectedElement.type === 'shape' || selectedElement.type === 'frame' ? 'backgroundColor' : 'color']: color } 
                                            })}
                                        />
                                    </div>
                                    <div className="bg-accent/30 p-3 rounded-lg border border-border">
                                        <div className="flex items-center justify-between mb-2">
                                            <span className="text-[10px] text-muted-foreground uppercase font-bold">Opacity</span>
                                            <span className="text-[11px] font-bold">{Math.round((selectedElement.opacity ?? 1) * 100)}%</span>
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
        <div className="flex items-center justify-between group cursor-pointer transition-all hover:translate-x-0.5">
            <span className="text-[11px] font-bold text-foreground tracking-tight">{title}</span>
            {Icon && <Icon className="w-3 h-3 text-muted-foreground group-hover:text-foreground transition-colors" />}
        </div>
    )
}

function PropertyField({ label, value, onChange }: { label: string, value: number, onChange: (v: number) => void }) {
    return (
        <div className="flex items-center gap-2">
            <span className="text-[10px] font-bold text-muted-foreground w-3 truncate">{label}</span>
            <input 
                type="number" 
                value={value}
                onChange={(e) => onChange(Number(e.target.value))}
                className="bg-transparent border-none text-[11px] text-foreground font-bold p-0 focus:ring-0 w-full [appearance:textfield] transition-colors focus:text-primary"
            />
        </div>
    )
}

function ColorControl({ value, onChange }: { value: string, onChange: (v: string) => void }) {
    return (
        <div className="flex items-center gap-2 bg-background p-2 rounded-lg border border-border group hover:border-primary/50 transition-all">
            <div className="relative h-5 w-8 shrink-0">
                <ColorPicker value={value} onSelect={onChange} />
            </div>
            <input 
                value={value.toUpperCase()} 
                onChange={(e) => onChange(e.target.value)}
                className="bg-transparent border-none text-[10px] text-muted-foreground font-mono font-bold w-full focus:ring-0 uppercase tracking-tight"
            />
        </div>
    );
}
