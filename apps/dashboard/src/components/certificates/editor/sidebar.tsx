"use client";

import { memo } from "react";
import { Button } from "@midday/ui/button";
import { Input } from "@midday/ui/input";
import { Label } from "@midday/ui/label";
import { Slider } from "@midday/ui/slider";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@midday/ui/tabs";
import { ScrollArea } from "@midday/ui/scroll-area";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@midday/ui/select";
import {
  Type,
  ImageIcon,
  Layers,
  Plus,
  Trash2,
  Square,
  Minus,
  Table as TableIcon,
  AlignLeft,
  AlignCenter,
  AlignRight,
  AlignStartVertical,
  AlignCenterVertical,
  AlignEndVertical,
  AlignStartHorizontal,
  AlignCenterHorizontal,
  AlignEndHorizontal,
  Grid3X3,
  Layers2,
  ArrowLeft,
  X,
  Loader2,
} from "lucide-react";
import { cn } from "@midday/ui/cn";
import { useI18n } from "@/locales/client";
import { FONTS, VARIABLES, PAPER_SIZES, type EditorElement, type ElementType, type PaperSize } from "./types";

interface SidebarProps {
  name: string;
  setName: (name: string) => void;
  isSheet?: boolean;
  onBack: () => void;
  elements: EditorElement[];
  selectedElementId: string | null;
  setSelectedElementId: (id: string | null) => void;
  paperSize: PaperSize;
  setPaperSize: (size: PaperSize) => void;
  orientation: "portrait" | "landscape";
  setOrientation: (orientation: "portrait" | "landscape") => void;
  backgroundImage: string;
  handleFileUpload: (e: any, type: "background" | "image") => void;
  handleClearBackground: () => void;
  addElement: (type: ElementType, content: string, variableKey?: string) => void;
  updateElement: (id: string, updates: Partial<EditorElement>) => void;
  deleteElement: (id: string) => void;
  reorderElements: (newElements: EditorElement[]) => void;
  handleSave: () => void;
  isSaving: boolean;
  mode: "create" | "edit";
}

export const Sidebar = memo(({
  name,
  setName,
  isSheet,
  onBack,
  elements,
  selectedElementId,
  setSelectedElementId,
  paperSize,
  setPaperSize,
  orientation,
  setOrientation,
  backgroundImage,
  handleFileUpload,
  handleClearBackground,
  addElement,
  updateElement,
  deleteElement,
  reorderElements,
  handleSave,
  isSaving,
  mode,
}: SidebarProps) => {
  const t = useI18n();
  const selectedElement = elements.find(el => el.id === selectedElementId);

  return (
    <div className="w-[320px] bg-background border-r flex flex-col z-20 shrink-0">
      <div className="p-6 border-b">
        <div className="flex items-center gap-3 mb-4">
          <Button variant="ghost" size="icon" onClick={onBack} className="h-8 w-8">
            {isSheet ? <X className="w-4 h-4" /> : <ArrowLeft className="w-4 h-4" />}
          </Button>
          <div className="flex-1 min-w-0">
            <Input 
              value={name} 
              onChange={(e) => setName(e.target.value)} 
              className="font-semibold text-sm border-0 shadow-none focus-visible:ring-0 px-0 bg-transparent"
              placeholder={t("reports.editor.template_name")}
            />
          </div>
        </div>
      </div>

      <Tabs defaultValue="add" className="flex-1 flex flex-col overflow-hidden">
        <TabsList className="w-full rounded-none bg-transparent border-b px-6">
          <TabsTrigger value="add" className="text-xs">{t("reports.editor.add")}</TabsTrigger>
          <TabsTrigger value="layers" className="text-xs">{t("reports.editor.layers")}</TabsTrigger>
          <TabsTrigger value="settings" className="text-xs">{t("reports.editor.settings")}</TabsTrigger>
        </TabsList>
        
        <ScrollArea className="flex-1 p-6" hideScrollbar>
          <TabsContent value="add" className="space-y-6 mt-0">
            <div className="space-y-3">
              <Label className="text-xs text-muted-foreground">{t("reports.editor.background_logos")}</Label>
              <div className="grid grid-cols-2 gap-3">
                <div className="relative group overflow-hidden rounded-lg border-2 border-dashed aspect-square hover:border-primary/50 transition-colors cursor-pointer bg-accent/50 flex items-center justify-center">
                  {backgroundImage ? (
                    <>
                      <img src={backgroundImage} className="w-full h-full object-cover" alt="BG" />
                      <Button
                        variant="destructive"
                        size="icon"
                        className="absolute top-2 right-2 h-6 w-6 opacity-0 group-hover:opacity-100 transition-opacity z-10"
                        onClick={(e) => {
                          e.stopPropagation();
                          e.preventDefault();
                          handleClearBackground();
                        }}
                      >
                        <Trash2 className="w-3 h-3" />
                      </Button>
                    </>
                  ) : (
                    <div className="flex flex-col items-center gap-2">
                      <ImageIcon className="w-5 h-5 text-muted-foreground" />
                      <span className="text-xs text-muted-foreground">{t("reports.editor.background")}</span>
                    </div>
                  )}
                  <input type="file" className="absolute inset-0 opacity-0 cursor-pointer" onChange={(e) => handleFileUpload(e, "background")} />
                </div>
                <Button 
                  variant="outline" 
                  className="h-auto flex-col py-6 gap-2 border-2 border-dashed rounded-lg"
                  onClick={() => document.getElementById("add-logo-input")?.click()}
                >
                  <Plus className="w-5 h-5 text-muted-foreground" />
                  <span className="text-xs text-muted-foreground">{t("reports.editor.logo")}</span>
                  <input id="add-logo-input" type="file" className="hidden" onChange={(e) => handleFileUpload(e, "image")} />
                </Button>
              </div>
            </div>

            <div className="space-y-3">
              <Label className="text-xs text-muted-foreground">{t("reports.editor.dynamic_variables")}</Label>
              <div className="space-y-4">
                {VARIABLES.map((group) => (
                  <div key={group.group} className="space-y-2">
                    <span className="text-xs font-medium text-muted-foreground/60">
                      {t(`reports.editor.groups.${group.group.toLowerCase().replace(/\s+/g, '_')}` as any, {} as any)}
                    </span>
                    <div className="space-y-1">
                      {group.items.map((v) => (
                        <Button 
                          key={v.value} 
                          variant="ghost"
                          className="w-full justify-start text-xs h-8"
                          onClick={() => {
                            if (v.isTable) {
                              addElement("table", "table");
                            } else {
                              addElement("variable", v.label, v.value);
                            }
                          }}
                        >
                          <Plus className="w-3 h-3 mr-2" />
                          {t(`reports.editor.variables.${v.value.replace(/[{}]/g, "")}` as any, {} as any)}
                        </Button>
                      ))}
                    </div>
                  </div>
                ))}
              </div>
            </div>

            <div className="space-y-3">
              <Label className="text-xs text-muted-foreground">{t("reports.editor.fixed_text")} & {t("reports.editor.table")}</Label>
              <div className="grid grid-cols-2 gap-2">
                <Button onClick={() => addElement("text", "New Text")} variant="outline" className="text-xs h-9">
                  <Type className="w-3 h-3 mr-2" /> 
                  {t("reports.editor.fixed_text")}
                </Button>
                <Button onClick={() => addElement("table", "table")} variant="outline" className="text-xs h-9">
                  <TableIcon className="w-3 h-3 mr-2" /> 
                  {t("reports.editor.table")}
                </Button>
              </div>
            </div>

            <div className="space-y-3">
              <Label className="text-xs text-muted-foreground">Shapes & Lines</Label>
              <div className="grid grid-cols-2 gap-2">
                <Button onClick={() => addElement("shape", "line")} variant="outline" className="text-xs h-9">
                  <Minus className="w-3 h-3 mr-2" /> 
                  {t("reports.editor.divider_line")}
                </Button>
                <Button onClick={() => addElement("shape", "box")} variant="outline" className="text-xs h-9">
                  <Square className="w-3 h-3 mr-2" /> 
                  {t("reports.editor.square")}
                </Button>
              </div>
            </div>
          </TabsContent>

          <TabsContent value="layers" className="space-y-2 mt-0">
            {elements.length === 0 ? (
              <div className="text-center py-12 text-muted-foreground text-sm">
                <Layers className="w-8 h-8 mx-auto mb-2 opacity-20" />
                <p className="text-xs">{t("reports.editor.no_layers")}</p>
              </div>
            ) : (
              <div className="space-y-1">
                {[...elements].reverse().map((el) => (
                  <div 
                    key={el.id}
                    onClick={() => setSelectedElementId(el.id)}
                    className={cn(
                      "flex items-center gap-3 p-3 rounded-lg cursor-pointer transition-colors",
                      selectedElementId === el.id ? "bg-accent" : "hover:bg-accent/50"
                    )}
                  >
                    <div className="flex-1 min-w-0">
                      <p className="text-xs font-medium truncate">{el.content}</p>
                      <p className="text-xs text-muted-foreground">{el.type}</p>
                    </div>
                    <Button
                      variant="ghost"
                      size="icon"
                      className="h-7 w-7 shrink-0"
                      onClick={(e) => {
                        e.stopPropagation();
                        deleteElement(el.id);
                      }}
                    >
                      <Trash2 className="w-3 h-3" />
                    </Button>
                  </div>
                ))}
              </div>
            )}
          </TabsContent>

          <TabsContent value="settings" className="space-y-6 mt-0">
            <div className="space-y-3">
              <Label className="text-xs text-muted-foreground">{t("reports.editor.paper_size")}</Label>
              <Select value={paperSize} onValueChange={(v: PaperSize) => setPaperSize(v)}>
                <SelectTrigger><SelectValue /></SelectTrigger>
                <SelectContent>
                  {Object.entries(PAPER_SIZES).map(([key, size]) => (
                    <SelectItem key={key} value={key}>{size.label}</SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>

            <div className="space-y-3">
              <Label className="text-xs text-muted-foreground">{t("reports.editor.orientation")}</Label>
              <Select value={orientation} onValueChange={(v: "portrait" | "landscape") => setOrientation(v)}>
                <SelectTrigger><SelectValue /></SelectTrigger>
                <SelectContent>
                  <SelectItem value="portrait">{t("reports.editor.portrait")}</SelectItem>
                  <SelectItem value="landscape">{t("reports.editor.landscape")}</SelectItem>
                </SelectContent>
              </Select>
            </div>

            {/* Recommended Canva Dimensions */}
            {(() => {
              const currentPaper = PAPER_SIZES[paperSize] ?? PAPER_SIZES.a4!;
              const dpi = 300;
              const mmToInch = 25.4;
              const w = orientation === 'portrait' ? currentPaper!.mmWidth : currentPaper!.mmHeight;
              const h = orientation === 'portrait' ? currentPaper!.mmHeight : currentPaper!.mmWidth;
              const pxW = Math.round((w / mmToInch) * dpi);
              const pxH = Math.round((h / mmToInch) * dpi);
              return (
                <div className="rounded-lg border border-dashed border-blue-500/30 bg-blue-500/5 p-3 space-y-1.5">
                  <p className="text-[10px] font-semibold text-blue-500 uppercase tracking-wider">أبعاد كانفا الموصى بها</p>
                  <div className="flex items-center justify-between">
                    <span className="text-xs text-muted-foreground">الأبعاد (بكسل)</span>
                    <span className="text-xs font-mono font-semibold text-foreground">{pxW} × {pxH} px</span>
                  </div>
                  <div className="flex items-center justify-between">
                    <span className="text-xs text-muted-foreground">الأبعاد (مم)</span>
                    <span className="text-xs font-mono text-foreground">{w} × {h} mm</span>
                  </div>
                  <p className="text-[10px] text-muted-foreground/70 leading-relaxed">
                    استخدم هذه الأبعاد في كانفا لإنشاء خلفية تملأ الصفحة بدون هوامش (300 DPI)
                  </p>
                </div>
              );
            })()}

            {selectedElement && (
              <div className="space-y-4 pt-4 border-t">
                <Label className="text-xs text-muted-foreground">{t("reports.editor.selected_element")}</Label>
                
                {(selectedElement.type === "text" || selectedElement.type === "variable" || selectedElement.type === "table") && (
                  <>
                    <div className="space-y-2">
                      <div className="flex items-center justify-between">
                        <Label className="text-xs">{t("reports.editor.font_size")}</Label>
                        <div className="flex items-center gap-1.5">
                           <Input 
                             type="number" 
                             value={selectedElement.fontSize || 18} 
                             onChange={(e) => updateElement(selectedElement.id, { fontSize: parseInt(e.target.value) || 8 })}
                             className="h-7 w-14 text-[10px] text-center font-mono px-1 bg-accent/50 border-0 focus-visible:ring-1"
                           />
                           <span className="text-[10px] text-muted-foreground font-bold">px</span>
                        </div>
                      </div>
                      <Slider 
                        value={[selectedElement.fontSize || 18]} 
                        onValueChange={([v]) => updateElement(selectedElement.id, { fontSize: v })}
                        min={8} max={120} step={1}
                      />
                    </div>

                    <div className="space-y-2">
                    <Label className="text-xs">{t("reports.editor.font_family")}</Label>
                    <Select value={selectedElement.fontFamily || "inherit"} onValueChange={(v) => updateElement(selectedElement.id, { fontFamily: v })}>
                      <SelectTrigger className="h-9"><SelectValue /></SelectTrigger>
                      <SelectContent>
                        {FONTS.map(f => (
                          <SelectItem key={f.value} value={f.value} style={{ fontFamily: f.value }}>{f.label}</SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  </div>

                  <div className="space-y-2">
                    <Label className="text-xs">{t("reports.editor.alignment")}</Label>
                    <div className="flex items-center gap-1 bg-accent p-1 rounded-md">
                      <Button variant="ghost" size="icon" className={cn("h-7 flex-1", selectedElement.align === "right" && "bg-background")} onClick={() => updateElement(selectedElement.id, { align: "right" })}><AlignRight className="w-3 h-3" /></Button>
                      <Button variant="ghost" size="icon" className={cn("h-7 flex-1", selectedElement.align === "center" && "bg-background")} onClick={() => updateElement(selectedElement.id, { align: "center" })}><AlignCenter className="w-3 h-3" /></Button>
                      <Button variant="ghost" size="icon" className={cn("h-7 flex-1", selectedElement.align === "left" && "bg-background")} onClick={() => updateElement(selectedElement.id, { align: "left" })}><AlignLeft className="w-3 h-3" /></Button>
                    </div>
                  </div>
                  </>
                )}

                  <div className="space-y-3">
                    <div className="flex items-center justify-between">
                      <Label className="text-xs font-semibold">{t("reports.editor.element_size")}</Label>
                      <div className="flex items-center gap-1.5 font-mono text-[10px]">
                        <Input 
                          type="number" 
                          value={Math.round(selectedElement.width || 20)} 
                          onChange={(e) => updateElement(selectedElement.id, { width: parseFloat(e.target.value) || 1 })}
                          className="h-7 w-12 text-center px-1 bg-accent/50 border-0 focus-visible:ring-1"
                        />
                        <span className="text-muted-foreground font-bold">%</span>
                      </div>
                    </div>
                    <Slider 
                      value={[selectedElement.width || 20]} 
                      onValueChange={([v]) => updateElement(selectedElement.id, { width: v })}
                      min={1} max={100} step={0.1}
                    />
                  </div>

                  <div className="space-y-3 pt-2">
                    <Label className="text-xs font-semibold flex items-center gap-2">
                      <Grid3X3 className="w-3 h-3 text-muted-foreground" />
                      {t("reports.editor.page_alignment" as any, { defaultValue: "محاذاة الصفحة" } as any)}
                    </Label>
                    <div className="grid grid-cols-2 gap-3">
                      <div className="flex flex-col gap-1.5">
                        <span className="text-[10px] text-muted-foreground px-1 uppercase tracking-wider font-bold">أفقي</span>
                        <div className="flex items-center gap-1 bg-accent/50 p-1.5 rounded-xl border border-border/10">
                          <Button 
                            variant="ghost" 
                            size="icon" 
                            className="h-8 flex-1 bg-background shadow-sm hover:bg-background/80" 
                            onClick={() => updateElement(selectedElement.id, { x: 0 })}
                          >
                            <AlignStartHorizontal className="w-4 h-4" />
                          </Button>
                          <Button 
                            variant="ghost" 
                            size="icon" 
                            className="h-8 flex-1 bg-background shadow-sm hover:bg-background/80" 
                            onClick={() => updateElement(selectedElement.id, { x: (100 - (selectedElement.width || 20)) / 2 })}
                          >
                            <AlignCenterHorizontal className="w-4 h-4" />
                          </Button>
                          <Button 
                            variant="ghost" 
                            size="icon" 
                            className="h-8 flex-1 bg-background shadow-sm hover:bg-background/80" 
                            onClick={() => updateElement(selectedElement.id, { x: 100 - (selectedElement.width || 20) })}
                          >
                            <AlignEndHorizontal className="w-4 h-4" />
                          </Button>
                        </div>
                      </div>
                      <div className="flex flex-col gap-1.5">
                        <span className="text-[10px] text-muted-foreground px-1 uppercase tracking-wider font-bold">رأسي</span>
                        <div className="flex items-center gap-1 bg-accent/50 p-1.5 rounded-xl border border-border/10">
                          <Button 
                            variant="ghost" 
                            size="icon" 
                            className="h-8 flex-1 bg-background shadow-sm hover:bg-background/80" 
                            onClick={() => updateElement(selectedElement.id, { y: 0 })}
                          >
                            <AlignStartVertical className="w-4 h-4" />
                          </Button>
                          <Button 
                            variant="ghost" 
                            size="icon" 
                            className="h-8 flex-1 bg-background shadow-sm hover:bg-background/80" 
                            onClick={() => updateElement(selectedElement.id, { y: 45 })}
                          >
                            <AlignCenterVertical className="w-4 h-4" />
                          </Button>
                          <Button 
                            variant="ghost" 
                            size="icon" 
                            className="h-8 flex-1 bg-background shadow-sm hover:bg-background/80" 
                            onClick={() => updateElement(selectedElement.id, { y: 90 })}
                          >
                            <AlignEndVertical className="w-4 h-4" />
                          </Button>
                        </div>
                      </div>
                    </div>
                  </div>

                  <div className="space-y-3 pt-4 border-t">
                    <Label className="text-xs font-semibold flex items-center gap-2">
                      <Layers2 className="w-3 h-3 text-muted-foreground" />
                      ترتيب الطبقات
                    </Label>
                    <div className="grid grid-cols-2 gap-2">
                      <Button 
                        variant="outline" 
                        size="sm" 
                        className="text-[10px] h-8"
                        onClick={() => {
                          const index = elements.findIndex(el => el.id === selectedElementId);
                          if (index === -1) return;
                          const newElements = [...elements];
                          const [element] = newElements.splice(index, 1);
                          if (element) {
                            newElements.unshift(element); // Send to background
                            reorderElements(newElements);
                          }
                        }}
                      >
                         إرسال للخلف
                      </Button>
                      <Button 
                        variant="outline" 
                        size="sm" 
                        className="text-[10px] h-8"
                        onClick={() => {
                          const index = elements.findIndex(el => el.id === selectedElementId);
                          if (index === -1) return;
                          const newElements = [...elements];
                          const [element] = newElements.splice(index, 1);
                          if (element) {
                            newElements.push(element); // Bring to front
                            reorderElements(newElements);
                          }
                        }}
                      >
                         إحضار للمقدمة
                      </Button>
                    </div>
                  </div>

                <div className="space-y-2 pt-4 border-t">
                  <Label className="text-xs font-semibold">{t("reports.editor.color")}</Label>
                  <div className="flex items-center gap-3 p-2 border rounded-xl bg-accent/30 translate-all duration-200">
                    <div className="relative w-10 h-10 rounded-lg overflow-hidden border-2 border-background shadow-sm shrink-0">
                      <input 
                        type="color" 
                        className="absolute inset-[-100%] w-[300%] h-[300%] cursor-pointer"
                        value={selectedElement.type === 'shape' ? selectedElement.backgroundColor : selectedElement.color}
                        onChange={(e) => updateElement(selectedElement.id, selectedElement.type === 'shape' ? { backgroundColor: e.target.value } : { color: e.target.value })}
                      />
                    </div>
                    <div className="flex flex-col">
                      <span className="text-[10px] font-bold text-muted-foreground uppercase tracking-widest">Hex Code</span>
                      <span className="text-xs font-mono font-bold">{(selectedElement.type === 'shape' ? selectedElement.backgroundColor : selectedElement.color)?.toUpperCase()}</span>
                    </div>
                  </div>
                </div>
              </div>
            )}
          </TabsContent>
        </ScrollArea>

        <div className="p-6 border-t bg-background">
          <Button onClick={handleSave} disabled={isSaving} className="w-full">
            {isSaving && <Loader2 className="w-4 h-4 ml-2 animate-spin" />}
            {mode === "create" ? t("reports.editor.create_template") : t("reports.editor.save_changes")}
          </Button>
        </div>
      </Tabs>
    </div>
  );
});

Sidebar.displayName = "Sidebar";
