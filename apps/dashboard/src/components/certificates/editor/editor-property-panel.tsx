"use client";

import React, { useState } from "react";
import { useEditorStore } from "@/store/certificate-editor";
import { 
  Plus,
  Share2,
  Settings2,
  Zap,
  Trash2,
  LayoutGrid,
  Layers,
  Grid2X2,
  AlignCenter,
  AlignLeft,
  AlignRight,
  AlignJustify,
  AlignStartVertical,
  AlignCenterVertical,
  AlignEndVertical,
  Maximize2,
  Minimize2,
  Type,
  Palette,
  Eye,
  Droplet,
  Sun,
  Minus,
  RotateCcw,
  FlipHorizontal,
  FlipVertical,
  Box,
  CornerUpRight,
  ChevronDown,
  MoreVertical,
  ArrowRightToLine,
  ArrowLeftToLine,
  Hash,
  Square,
  Image as ImageIcon
} from "lucide-react";
import { Button } from "@midday/ui/button";
import { ColorPicker } from "@/components/color-picker";
import { cn } from "@midday/ui/cn";
import { ScrollArea } from "@midday/ui/scroll-area";
import { Separator } from "@midday/ui/separator";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@midday/ui/select";
import { Label } from "@midday/ui/label";
import { Input } from "@midday/ui/input";
import { PAPER_PRESETS } from "./constants";

interface PropertyPanelProps {
  onSave?: () => void;
  isSaving?: boolean;
}

export function EditorPropertyPanel({ onSave, isSaving }: PropertyPanelProps) {
  const { elements, selectedElementIds, updateElement, canvasConfig, setCanvasConfig, addElement, name } = useEditorStore();

  const selectedElement = elements.find((el) => el.id === selectedElementIds[0]);

  const handleUpdate = (updates: any) => {
    if (selectedElement) {
       updateElement(selectedElement.id, updates);
    }
  };

  const handleStyleUpdate = (styleUpdates: any) => {
    if (selectedElement) {
        updateElement(selectedElement.id, {
            style: { ...selectedElement.style, ...styleUpdates }
        });
    }
  };

  const elementName = selectedElement ? (selectedElement.name || selectedElement.type.charAt(0).toUpperCase() + selectedElement.type.slice(1)) : "Canvas";

  return (
    <div className="h-full flex flex-col bg-background text-foreground select-none w-64 z-20 border-l border-border transition-all duration-300">
      {/* Header / Actions Row */}
      <div className="px-2 py-2 flex items-center justify-between gap-1.5 shrink-0 border-b border-border bg-background">
          <span className="text-[12px] font-bold text-foreground/80 truncate">{elementName}</span>
        <div className="flex items-center gap-1.5">
          <Button 
            onClick={onSave}
            disabled={isSaving}
            size="sm"
            className="h-7 text-[10px] font-bold px-3 shrink-0 gap-1.5"
          >
            {isSaving ? <Loader2 className="w-3.5 h-3.5 animate-spin" /> : <Plus className="w-3.5 h-3.5" />}
          </Button>
          
          <Button 
            variant="outline"
            size="sm"
            className="h-7 text-[10px] font-bold px-3 shrink-0 gap-1.5"
            >
            <Maximize2 className="w-3.5 h-3.5 opacity-60" />
          </Button>
        </div>
      </div>
      <ScrollArea className="flex-1">
          <div className="flex flex-col pb-20">
              {!selectedElement ? (
                  <div className="divide-y divide-border">
                      {/* Creation Tools */}
                      <Section title="Design Tools">
                          <div className="grid grid-cols-4 gap-1">
                             {[
                                 { type: 'frame', icon: Hash, label: 'Page' },
                                 { type: 'text', icon: Type, label: 'Text' },
                                 { type: 'image', icon: ImageIcon, label: 'Image' },
                                 { type: 'shape', icon: Square, label: 'Shape' }
                             ].map(tool => (
                                 <button 
                                   key={tool.label}
                                   onClick={() => addElement({ type: tool.type as any })}
                                   className="flex flex-col items-center gap-1 p-2 rounded-lg hover:bg-muted transition-colors group"
                                 >
                                     <tool.icon className="w-4 h-4 text-muted-foreground group-hover:text-primary transition-colors" />
                                     <span className="text-[8px] text-muted-foreground font-bold group-hover:text-foreground uppercase tracking-tighter">{tool.label}</span>
                                 </button>
                             ))}
                          </div>
                      </Section>

                      <Section title="Canvas Settings">
                          <div className="space-y-4">
                              <div className="space-y-2">
                                 <Label className="text-[10px] text-muted-foreground font-black uppercase tracking-tighter">Wallpaper Color</Label>
                                 <ColorControl 
                                    value={canvasConfig.backgroundColor || "#ffffff"} 
                                    onChange={(c) => setCanvasConfig({ backgroundColor: c })}
                                 />
                              </div>
                          </div>
                      </Section>

                      {/* Page Presets */}
                      <Section title="Document Format">
                          <div className="space-y-1">
                              {PAPER_PRESETS.map(preset => (
                                  <button
                                     key={preset.id}
                                     onClick={() => addElement({ 
                                         type: 'frame', 
                                         width: preset.width, 
                                         height: preset.height,
                                         name: preset.name
                                     })}
                                     className="w-full flex items-center justify-between px-3 py-2 rounded-lg hover:bg-muted transition-all group border border-transparent hover:border-border"
                                  >
                                      <span className="text-[11px] font-bold text-muted-foreground group-hover:text-foreground">{preset.name}</span>
                                      <span className="text-[9px] text-muted-foreground/50 font-mono group-hover:text-muted-foreground">{preset.width}×{preset.height}</span>
                                  </button>
                              ))}
                          </div>
                      </Section>
                  </div>
              ) : (
                  <div className="divide-y divide-border">
                      {/* Section: Frame / Transform */}
                      <Section title="Position & Scale">
                         <div className="flex items-center gap-0.5 bg-muted p-0.5 mb-3">
                            <AlignButton icon={AlignLeft} />
                            <AlignButton icon={AlignCenter} />
                            <AlignButton icon={AlignRight} />
                            <Separator orientation="vertical" className="h-3 mx-0.5" />
                            <AlignButton icon={AlignStartVertical} />
                            <AlignButton icon={AlignCenterVertical} />
                            <AlignButton icon={AlignEndVertical} />
                         </div>

                         <div className="grid grid-cols-2 gap-x-2 gap-y-2">
                            <PropertyInput label="X" value={Math.round(selectedElement.x)} onChange={(v) => handleUpdate({ x: v })} />
                            <PropertyInput label="Y" value={Math.round(selectedElement.y)} onChange={(v) => handleUpdate({ y: v })} />
                            <PropertyInput label="W" value={Math.round(selectedElement.width)} onChange={(v) => handleUpdate({ width: v })} />
                            <PropertyInput label="H" value={Math.round(selectedElement.height)} onChange={(v) => handleUpdate({ height: v })} />
                            
                            <PropertyInput label="°" value={Math.round(selectedElement.rotation)} onChange={(v) => handleUpdate({ rotation: v })} />
                            <div className="flex items-center gap-0.5 bg-muted border border-border/50 rounded p-0.5">
                                <AlignButton icon={RotateCcw} />
                                <AlignButton icon={FlipHorizontal} />
                                <AlignButton icon={FlipVertical} />
                            </div>
                         </div>
                         
                         {/* Border Radius */}
                         <div className="mt-2">
                            <PropertyInput 
                                label="R" 
                                icon={Box}
                                value={selectedElement.style.borderRadius || 0} 
                                onChange={(v) => handleStyleUpdate({ borderRadius: v })} 
                            />
                         </div>
                      </Section>

                      {/* Typography (Only for Text/Variable) */}
                      {(selectedElement.type === 'text' || selectedElement.type === 'variable') && (
                          <Section title="Typography">
                             <div className="space-y-2">
                                <Select 
                                    value={selectedElement.style.fontFamily || "Inter"} 
                                    onValueChange={(v) => handleStyleUpdate({ fontFamily: v })}
                                >
                                    <SelectTrigger className="h-8 text-[11px] bg-muted border-border text-foreground hover:bg-muted/80 transition-colors">
                                        <SelectValue />
                                    </SelectTrigger>
                                    <SelectContent>
                                        <SelectItem value="Inter">Inter</SelectItem>
                                        <SelectItem value="Roboto">Roboto</SelectItem>
                                        <SelectItem value="Amiri">Amiri (Arabic)</SelectItem>
                                        <SelectItem value="Traditional Arabic">Traditional Arabic</SelectItem>
                                    </SelectContent>
                                </Select>

                                <div className="grid grid-cols-2 gap-2">
                                    <Select 
                                        value={selectedElement.style.fontWeight || "400"} 
                                        onValueChange={(v) => handleStyleUpdate({ fontWeight: v })}
                                    >
                                        <SelectTrigger className="h-8 text-[11px] bg-muted border-border">
                                            <SelectValue />
                                        </SelectTrigger>
                                        <SelectContent>
                                            <SelectItem value="300">Light</SelectItem>
                                            <SelectItem value="400">Regular</SelectItem>
                                            <SelectItem value="600">Semi Bold</SelectItem>
                                            <SelectItem value="700">Bold</SelectItem>
                                        </SelectContent>
                                    </Select>
                                    
                                    <div className="relative group/input">
                                        <Input 
                                            type="number" 
                                            value={selectedElement.style.fontSize || 16}
                                            onChange={(e) => handleStyleUpdate({ fontSize: Number(e.target.value) })}
                                            className="h-8 text-[11px] bg-muted border-border pl-6 pr-2 font-bold"
                                        />
                                        <Type className="w-3 h-3 absolute left-2 top-1/2 -translate-y-1/2 text-muted-foreground opacity-50" />
                                    </div>
                                </div>

                                <div className="flex items-center gap-0.5 bg-muted p-0.5">
                                    <AlignButton 
                                        active={selectedElement.style.textAlign === 'left'}
                                        icon={AlignLeft} 
                                        onClick={() => handleStyleUpdate({ textAlign: 'left' })} 
                                    />
                                    <AlignButton 
                                        active={selectedElement.style.textAlign === 'center'}
                                        icon={AlignCenter} 
                                        onClick={() => handleStyleUpdate({ textAlign: 'center' })} 
                                    />
                                    <AlignButton 
                                        active={selectedElement.style.textAlign === 'right'}
                                        icon={AlignRight} 
                                        onClick={() => handleStyleUpdate({ textAlign: 'right' })} 
                                    />
                                    <Separator orientation="vertical" className="h-3 mx-0.5" />
                                    <button 
                                        onClick={() => handleStyleUpdate({ direction: 'ltr' })}
                                        className={cn("flex-1 text-[10px] font-bold h-6 rounded transition-all", (selectedElement.style.direction === 'ltr' || !selectedElement.style.direction) ? "bg-background text-foreground shadow-sm" : "text-muted-foreground")}
                                    >
                                        LTR
                                    </button>
                                    <button 
                                        onClick={() => handleStyleUpdate({ direction: 'rtl' })}
                                        className={cn("flex-1 text-[10px] font-bold h-6 rounded transition-all", selectedElement.style.direction === 'rtl' ? "bg-background text-foreground shadow-sm" : "text-muted-foreground")}
                                    >
                                        RTL
                                    </button>
                                </div>
                             </div>
                          </Section>
                      )}

                      {/* Fill Section with Integrated Opacity */}
                      <Section title="Appearance">
                         <div className="flex flex-col gap-2">
                            <ColorControl 
                                value={selectedElement.style.color || selectedElement.style.backgroundColor || "#000000"} 
                                opacity={Math.round((selectedElement.opacity ?? 1) * 100)}
                                onChange={(color) => handleUpdate({ 
                                    style: { ...selectedElement.style, [selectedElement.type === 'shape' || selectedElement.type === 'frame' ? 'backgroundColor' : 'color']: color } 
                                })}
                                onOpacityChange={(opacity) => handleUpdate({ opacity: opacity / 100 })}
                            />
                         </div>
                      </Section>

                      {/* Advanced (Placeholder for now) */}
                      <Section title="Advanced">
                         <div className="flex items-center justify-between opacity-30 hover:opacity-100 transition-opacity cursor-pointer">
                            <span className="text-[10px] text-muted-foreground">Shadow & Effects</span>
                            <ChevronDown className="w-3 h-3 text-muted-foreground" />
                         </div>
                      </Section>
                  </div>
              )}
          </div>
      </ScrollArea>
      
      {/* Footer Info */}
      <div className="p-3 bg-muted/20 flex items-center justify-between">
          <div className="flex items-center gap-1.5">
             <div className="w-1.5 h-1.5 rounded-full bg-purple-800" />
             <span className="text-[9px] font-bold uppercase tracking-widest text-muted-foreground">MulverseCo. Template Editor</span>
          </div>
          <div className="text-[10px] font-bold text-muted-foreground/30">v2.4.0</div>
      </div>
    </div>
  );
}

function Section({ title, children }: { title: string, children: React.ReactNode }) {
    return (
        <div className="p-4 space-y-3">
            <div className="flex items-center justify-between group px-1">
                <span className="text-[11px] font-bold text-muted-foreground/80 tracking-wider">{title}</span>
            </div>
            {children}
        </div>
    )
}

function AlignButton({ icon: Icon, active, onClick }: { icon: any, active?: boolean, onClick?: () => void }) {
    return (
        <button 
            onClick={onClick}
            className={cn(
                "flex-1 h-7 flex items-center justify-center rounded transition-all duration-200",
                active ? "bg-background text-foreground shadow-sm" : "text-muted-foreground hover:text-foreground hover:bg-background/20"
            )}
        >
            <Icon className="w-3.5 h-3.5" />
        </button>
    )
}

function PropertyInput({ label, value, onChange, icon: Icon }: { label: string, value: number, onChange: (v: number) => void, icon?: any }) {
    return (
        <div className="relative group/prop">
            <Input 
                type="number" 
                value={value}
                onChange={(e) => onChange(Number(e.target.value))}
                className="h-8 text-[11px] bg-muted border-border pl-6 pr-2 font-bold focus:bg-background transition-colors"
            />
            <div className="absolute left-2 top-1/2 -translate-y-1/2 flex items-center justify-center">
                {Icon ? <Icon className="w-3 h-3 text-muted-foreground/40" /> : <span className="text-[9px] font-black text-muted-foreground/40 uppercase tracking-tighter">{label}</span>}
            </div>
        </div>
    )
}

function ColorControl({ value, opacity, onChange, onOpacityChange }: { 
    value: string, 
    opacity?: number, 
    onChange: (v: string) => void, 
    onOpacityChange?: (v: number) => void 
}) {
    return (
        <div className="flex items-center gap-2 bg-muted border border-border p-1 rounded-lg h-9 group transition-all">
            <div className="h-6 w-10 shrink-0 relative rounded-md overflow-hidden">
                <ColorPicker value={value} onSelect={onChange} />
            </div>
            
            <Input 
                value={value.toUpperCase().replace('#', '')} 
                onChange={(e) => onChange(`#${e.target.value}`)}
                className="h-6 bg-transparent border-none text-[11px] font-mono font-bold w-16 p-0 focus-visible:ring-0 uppercase tracking-tight"
            />
            
            <Separator orientation="vertical" className="h-4 mx-0.5 opacity-20" />
            
            <div className="flex items-center gap-1 flex-1">
                <Input 
                    type="number" 
                    value={opacity ?? 100}
                    onChange={(e) => onOpacityChange?.(Number(e.target.value))}
                    className="h-6 bg-transparent border-none text-[11px] font-bold p-0 focus-visible:ring-0 w-full text-right"
                />
                <span className="text-[10px] text-muted-foreground font-bold opacity-40">%</span>
            </div>
        </div>
    );
}

function Loader2(props: any) {
  return (
    <svg
      {...props}
      xmlns="http://www.w3.org/2000/svg"
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <path d="M21 12a9 9 0 1 1-6.219-8.56" />
    </svg>
  );
}
