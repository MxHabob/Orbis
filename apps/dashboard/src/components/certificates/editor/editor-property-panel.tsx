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
import { PAPER_PRESETS } from "./constants";

interface PropertyPanelProps {
  onSave?: () => void;
  isSaving?: boolean;
}

export function EditorPropertyPanel({ onSave, isSaving }: PropertyPanelProps) {
  const { elements, selectedElementIds, updateElement, canvasConfig, setCanvasConfig, addElement } = useEditorStore();

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
    <div className="h-full flex flex-col bg-[#0d0d0d] text-[#e0e0e0] select-none w-[240px] z-20 border-l border-white/5 shadow-2xl transition-all duration-300">
      {/* Header / Actions Row */}
      <div className="px-2 py-2 flex items-center gap-1.5 shrink-0 border-b border-white/[0.04] bg-[#0d0d0d]">
          <Button 
            onClick={onSave}
            disabled={isSaving}
            className="h-7 text-[10px] font-bold px-3 rounded-md bg-white text-black hover:bg-white/90 border-none shrink-0 gap-1.5 shadow-[0_0_15px_rgba(255,255,255,0.1)] transition-all hover:scale-[1.02] active:scale-95"
          >
            <Plus className="w-3.5 h-3.5 fill-current" />
            حفظ
          </Button>
          
          <Button 
            variant="outline"
            className="h-7 text-[10px] font-bold px-3 rounded-md border-white/10 bg-white/[0.03] hover:bg-white/[0.08] text-white shrink-0 gap-1.5 transition-all"
          >
            <Maximize2 className="w-3.5 h-3.5 opacity-60" />
            استخراج
          </Button>

          <Button variant="ghost" size="icon" className="h-7 w-7 text-[#919191] hover:text-white hover:bg-white/5 transition-colors shrink-0">
            <MoreVertical className="w-3.5 h-3.5" />
          </Button>

          <Separator orientation="vertical" className="h-4 bg-white/10 mx-0.5" />
          
          <span className="text-[10px] font-bold text-[#919191] truncate flex-1 pl-1 opacity-60 tracking-tight">
            {elementName}
          </span>
      </div>

      <ScrollArea className="flex-1">
          <div className="flex flex-col pb-20">
              {!selectedElement ? (
                  <div className="divide-y divide-white/[0.04]">
                      {/* Creation Tools moved here since they were removed from sidebar */}
                      <Section title="Tools">
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
                                   className="flex flex-col items-center gap-1 p-2 rounded-lg hover:bg-white/5 transition-colors group"
                                 >
                                     <tool.icon className="w-4 h-4 text-[#919191] group-hover:text-white transition-colors" />
                                     <span className="text-[8px] text-[#666] font-bold group-hover:text-[#919191] uppercase">{tool.label}</span>
                                 </button>
                             ))}
                          </div>
                      </Section>

                      <Section title="Document">
                          <div className="space-y-4">
                              <div className="space-y-2">
                                 <Label className="text-[9px] text-[#666] font-black uppercase tracking-tighter">Background Color</Label>
                                 <ColorControl 
                                    value={canvasConfig.backgroundColor || "#ffffff"} 
                                    onChange={(c) => setCanvasConfig({ backgroundColor: c })}
                                 />
                              </div>
                          </div>
                      </Section>

                      {/* Page Presets moved here */}
                      <Section title="Page Presets">
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
                                     className="w-full flex items-center justify-between px-3 py-2 rounded-lg hover:bg-white/[0.03] transition-all group border border-transparent hover:border-white/[0.05]"
                                  >
                                      <span className="text-[11px] font-bold text-[#919191] group-hover:text-white">{preset.name}</span>
                                      <span className="text-[9px] text-[#666] font-mono group-hover:text-[#919191]">{preset.width}×{preset.height}</span>
                                  </button>
                              ))}
                          </div>
                      </Section>
                  </div>
              ) : (
                  <div className="divide-y divide-white/[0.04]">
                      {/* Section: Frame / Transform */}
                      <Section title="Layout & Transform">
                         <div className="flex items-center gap-0.5 bg-white/[0.02] border border-white/[0.05] rounded-lg p-0.5 mb-3">
                            <AlignButton icon={AlignLeft} />
                            <AlignButton icon={AlignCenter} />
                            <AlignButton icon={AlignRight} />
                            <Separator orientation="vertical" className="h-3 bg-white/10 mx-0.5" />
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
                            <div className="flex items-center gap-0.5 bg-white/[0.02] border border-white/[0.05] rounded p-0.5">
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
                                    <SelectTrigger className="h-7 text-[10px] bg-white/[0.02] border-white/[0.08] text-white focus:ring-0 px-2 font-medium hover:bg-white/[0.05] transition-colors">
                                        <SelectValue />
                                    </SelectTrigger>
                                    <SelectContent className="bg-[#121212] border-white/10 text-white">
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
                                        <SelectTrigger className="h-7 text-[10px] bg-white/[0.02] border-white/[0.08] text-white focus:ring-0 px-2 font-medium">
                                            <SelectValue />
                                        </SelectTrigger>
                                        <SelectContent className="bg-[#121212] border-white/10 text-white">
                                            <SelectItem value="300">Light</SelectItem>
                                            <SelectItem value="400">Regular</SelectItem>
                                            <SelectItem value="600">Semi Bold</SelectItem>
                                            <SelectItem value="700">Bold</SelectItem>
                                        </SelectContent>
                                    </Select>
                                    <div className="bg-white/[0.02] border border-white/[0.08] rounded h-7 flex items-center px-1.5 gap-1 pt-0.5 focus-within:ring-1 focus-within:ring-white/20 transition-all">
                                        <input 
                                            type="number" 
                                            value={selectedElement.style.fontSize || 16}
                                            onChange={(e) => handleStyleUpdate({ fontSize: Number(e.target.value) })}
                                            className="bg-transparent border-none text-[10px] text-white font-bold p-0 focus:ring-0 w-full"
                                        />
                                        <ChevronDown className="w-3 h-3 text-[#919191] ml-auto shrink-0 opacity-40" />
                                    </div>
                                </div>

                                <div className="flex items-center gap-0.5 bg-white/[0.02] border border-white/[0.05] rounded p-0.5">
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
                                    <Separator orientation="vertical" className="h-3 bg-white/10 mx-0.5" />
                                    <button 
                                        onClick={() => handleStyleUpdate({ direction: 'ltr' })}
                                        className={cn("flex-1 text-[9px] font-black h-6 rounded transition-all", (selectedElement.style.direction === 'ltr' || !selectedElement.style.direction) ? "bg-white/10 text-white shadow-lg shadow-white/5" : "text-[#919191]")}
                                    >
                                        LTR
                                    </button>
                                    <button 
                                        onClick={() => handleStyleUpdate({ direction: 'rtl' })}
                                        className={cn("flex-1 text-[9px] font-black h-6 rounded transition-all", selectedElement.style.direction === 'rtl' ? "bg-white/10 text-white shadow-lg shadow-white/5" : "text-[#919191]")}
                                    >
                                        RTL
                                    </button>
                                </div>
                             </div>
                          </Section>
                      )}

                      {/* Fill Section with Integrated Opacity */}
                      <Section title="Fill">
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

                      {/* Stroke Section */}
                      <Section title="Stroke">
                         <div className="flex items-center justify-between opacity-20 hover:opacity-100 transition-opacity cursor-pointer py-1">
                            <span className="text-[10px] text-[#919191]">No stroke</span>
                         </div>
                      </Section>

                      {/* Effects Section */}
                      <Section title="Effects">
                         <div className="flex items-center justify-between opacity-20 hover:opacity-100 transition-opacity cursor-pointer py-1">
                            <span className="text-[10px] text-[#919191]">No shadow or blur</span>
                         </div>
                      </Section>
                  </div>
              )}
          </div>
      </ScrollArea>
      
      {/* Zoom / Info Bar */}
      <div className="p-3 border-t border-white/[0.04] bg-[#0d0d0d] flex items-center justify-between shrink-0">
          <div className="bg-white/5 rounded-full p-1.5 hover:bg-white/10 transition-colors cursor-pointer border border-white/5">
             <Plus className="w-3 h-3 text-[#919191]" />
          </div>
          <div className="text-[9px] font-black text-[#919191] bg-white/5 w-5 h-5 flex items-center justify-center rounded-full hover:bg-white/10 transition-colors cursor-pointer border border-white/5">
             <span>?</span>
          </div>
      </div>
    </div>
  );
}

function Section({ title, children }: { title: string, children: React.ReactNode }) {
    return (
        <div className="p-3 space-y-3">
            <div className="flex items-center justify-between group">
                <span className="text-[11px] font-bold text-[#666]">{title}</span>
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
                "flex-1 h-6 flex items-center justify-center rounded transition-all duration-200 hover:bg-white/5",
                active ? "bg-white/10 text-white shadow-inner" : "text-[#919191]"
            )}
        >
            <Icon className="w-3.5 h-3.5" />
        </button>
    )
}

function PropertyInput({ label, value, onChange, icon: Icon }: { label: string, value: number, onChange: (v: number) => void, icon?: any }) {
    return (
        <div className="flex items-center gap-1.5 bg-white/[0.01] border border-white/[0.05] p-1.5 rounded h-7 focus-within:bg-white/[0.03] focus-within:border-white/10 transition-all flex-1">
            {Icon ? <Icon className="w-3 h-3 text-[#666] shrink-0" /> : <span className="text-[9px] font-black text-[#666] w-2 flex justify-center uppercase tracking-tighter shrink-0">{label}</span>}
            <input 
                type="number" 
                value={value}
                onChange={(e) => onChange(Number(e.target.value))}
                className="bg-transparent border-none text-[10px] text-white font-bold p-0 focus:ring-0 w-full [appearance:textfield]"
            />
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
        <div className="flex items-center gap-2 bg-white/[0.01] border border-white/[0.05] pr-1.5 pl-1 rounded h-8 group hover:bg-white/[0.04] transition-all flex-1">
            <div className="h-5 w-8 shrink-0 relative rounded overflow-hidden border border-white/10">
                <ColorPicker value={value} onSelect={onChange} />
            </div>
            <input 
                value={value.toUpperCase().replace('#', '')} 
                onChange={(e) => onChange(`#${e.target.value}`)}
                className="bg-transparent border-none text-[10px] text-white font-mono font-black w-14 focus:ring-0 uppercase tracking-tight shrink-0"
            />
            
            <Separator orientation="vertical" className="h-3 bg-white/10 mx-0.5" />
            
            <div className="flex items-center gap-1 flex-1">
                <input 
                    type="number" 
                    value={opacity ?? 100}
                    onChange={(e) => onOpacityChange?.(Number(e.target.value))}
                    className="bg-transparent border-none text-[10px] text-white font-black p-0 focus:ring-0 w-full text-right [appearance:textfield]"
                />
                <span className="text-[9px] text-[#666] font-black opacity-40">%</span>
            </div>
        </div>
    );
}
