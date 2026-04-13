"use client";

import React, { useState, useCallback } from "react";
import { 
  Square, 
  Image as ImageIcon, 
  Database, 
  Layers,
  Eye,
  Lock,
  Type as TextIcon,
  Hash,
  Grid2X2,
  Plus,
  Upload,
  MoreVertical,
  Loader2,
  GripVertical,
  User,
  Fingerprint,
  Table as TableIcon,
  Trophy,
  Award,
  Search,
  LayoutGrid
} from "lucide-react";
import { Button } from "@midday/ui/button";
import { ScrollArea } from "@midday/ui/scroll-area";
import { Separator } from "@midday/ui/separator";
import { useEditorStore } from "@/store/certificate-editor";
import { cn } from "@midday/ui/cn";
import { useDropzone } from "react-dropzone";
import { useTRPC } from "@/trpc/client";
import { useMutation, useQuery } from "@tanstack/react-query";
import { useToast } from "@midday/ui/use-toast";
import { authClient } from "@midday/auth/auth-client";
import { Label } from "@midday/ui/label";
import { PAPER_PRESETS } from "./constants";
import { Input } from "@midday/ui/input";

// DnD Kit Imports
import {
  DndContext, 
  closestCenter,
  KeyboardSensor,
  PointerSensor,
  useSensor,
  useSensors,
  DragEndEvent,
} from '@dnd-kit/core';
import {
  SortableContext,
  sortableKeyboardCoordinates,
  verticalListSortingStrategy,
  useSortable
} from '@dnd-kit/sortable';
import { CSS } from '@dnd-kit/utilities';

interface SidebarProps {
  templateName: string;
}

export function EditorSidebar({ templateName }: SidebarProps) {
  const { 
    elements, 
    selectedElementIds, 
    setSelectedElements, 
    addElement, 
    reorderElements,
    canvasConfig,
    setCanvasConfig
  } = useEditorStore();
  
  const [activeTab, setActiveTab] = useState<"layers" | "assets" | "variables">("layers");
  const [searchQuery, setSearchQuery] = useState("");
  const { toast } = useToast();
  const trpc = useTRPC();
  const { data: session } = authClient.useSession();
  const [isUploading, setIsUploading] = useState(false);

  const sensors = useSensors(
    useSensor(PointerSensor),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    })
  );

  const { data: globalAssetsData, refetch: refetchAssets } = useQuery(
    trpc.documents.get.queryOptions({
        pageSize: 50,
    })
  );
  const globalAssets = globalAssetsData?.data || [];
  const processDocumentMutation = useMutation(trpc.documents.processDocument.mutationOptions());

  const handleUpload = useCallback(async (files: File[]) => {
    if (!files.length) return;
    setIsUploading(true);
    try {
      await Promise.all(
        files.map(async (file) => {
          const formData = new FormData();
          formData.append("file", file);
          formData.append("bucket", "vault");
          formData.append("path", `${session?.user.id || "anonymous"}/assets/${file.name}`);
          const res = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/api/upload`, {
            method: "POST",
            body: formData,
            credentials: "include",
          });
          const data = await res.json();
          await processDocumentMutation.mutateAsync([{
            filePath: data.path.split("/"),
            mimetype: file.type,
            size: file.size,
          }]);
        })
      );
      refetchAssets();
      toast({ title: "Assets synchronized", variant: "success" });
    } catch (error) {
      toast({ title: "Sync failed", variant: "destructive" });
    } finally {
      setIsUploading(false);
    }
  }, [session, processDocumentMutation, refetchAssets, toast]);

  const { getRootProps, getInputProps } = useDropzone({
    onDrop: handleUpload,
    accept: { "image/*": [".png", ".jpg", ".jpeg", ".svg", ".webp"] }
  });

  const activeLayers = [...elements].reverse(); 

  const handleDragEnd = (event: DragEndEvent) => {
    const { active, over } = event;
    if (active.id !== over?.id) {
       const oldIndex = activeLayers.findIndex((item) => item.id === active.id);
       const newIndex = activeLayers.findIndex((item) => item.id === over?.id);
       const actualOldIndex = elements.length - 1 - oldIndex;
       const actualNewIndex = elements.length - 1 - newIndex;
       reorderElements(actualOldIndex, actualNewIndex);
    }
  };

  const databaseVariables = [
    { key: "student_name", label: "اسم الطالب الكامل", icon: User },
    { key: "seat_number", label: "رقم الجلوس العام", icon: Hash },
    { key: "secret_number", label: "الرقم السري للمغلف", icon: Fingerprint },
    { key: "student_id", label: "الرقم الأكاديمي", icon: Hash },
    { key: "class_name", label: "اسم الفصل / الصف", icon: Grid2X2 },
    { key: "rank", label: "الترتيب العام (المركز)", icon: Trophy },
    { key: "certificate_type", label: "نوع الشهادة / التقرير", icon: Award },
    { key: "school_year", label: "العام الدراسي", icon: Hash },
    { key: "grades_table", label: "جدول كشف الدرجات", icon: TableIcon, isTable: true },
  ].filter(v => v.label.includes(searchQuery) || v.key.includes(searchQuery));

  return (
    <div className="flex flex-col h-full bg-background select-none w-64 z-20 border-r border-border">
      {/* Header */}
      <div className="p-4 space-y-4">
         <div className="flex items-center justify-between">
            <div className="flex items-center gap-2.5">
                <div className="w-7 h-7 rounded-lg bg-primary flex items-center justify-center shadow-lg shadow-primary/20">
                    <Grid2X2 className="w-4 h-4 text-primary-foreground" />
                </div>
                <div className="flex flex-col">
                    <span className="text-[11px] font-bold text-foreground tracking-tight truncate max-w-[120px]">{templateName}</span>
                    <span className="text-[9px] text-muted-foreground font-medium uppercase">Edit Template</span>
                </div>
            </div>
            <Button variant="ghost" size="icon" className="h-7 w-7 text-muted-foreground hover:text-foreground">
                <MoreVertical className="w-3.5 h-3.5" />
            </Button>
         </div>

         {/* Creation Tools */}
         <div className="grid grid-cols-4 gap-1">
            {[
                { type: 'frame', icon: Hash, label: 'Page' },
                { type: 'text', icon: TextIcon, label: 'Text' },
                { type: 'image', icon: ImageIcon, label: 'Image' },
                { type: 'shape', icon: Square, label: 'Shape' }
            ].map(tool => (
                <button 
                  key={tool.label}
                  onClick={() => addElement({ type: tool.type as any })}
                  className="flex flex-col items-center gap-1 p-1.5 rounded-lg hover:bg-accent transition-colors group"
                >
                    <tool.icon className="w-4 h-4 text-muted-foreground group-hover:text-foreground transition-colors" />
                    <span className="text-[9px] text-muted-foreground font-medium group-hover:text-foreground">{tool.label}</span>
                </button>
            ))}
         </div>

         <Separator className="bg-border/50" />

         {/* Page Presets */}
         <div className="space-y-1.5 pt-1">
             <Label className="text-[10px] uppercase tracking-wider font-bold text-muted-foreground pl-1">Page Formats</Label>
             <div className="space-y-0.5">
                 {PAPER_PRESETS.map(preset => (
                     <button
                        key={preset.id}
                        onClick={() => addElement({ 
                            type: 'frame', 
                            width: preset.width, 
                            height: preset.height,
                            name: preset.name
                        })}
                        className="w-full flex items-center justify-between px-2.5 py-1.5 rounded-md hover:bg-accent transition-all text-left group"
                     >
                         <span className="text-[11px] font-medium text-foreground/70 group-hover:text-foreground">{preset.name}</span>
                         <span className="text-[10px] text-muted-foreground/50 font-mono group-hover:text-muted-foreground">{preset.width}×{preset.height}</span>
                     </button>
                 ))}
             </div>
         </div>
      </div>

      <Separator className="bg-border mx-4 w-auto" />

      {/* Tabs Navigation */}
      <div className="px-4 flex items-center gap-4 mt-4 mb-2">
         {["layers", "assets", "variables"].map((tab) => (
             <button 
                key={tab}
                onClick={() => setActiveTab(tab as any)} 
                className={cn(
                    "text-[11px] font-bold transition-all relative pb-2 capitalize", 
                    activeTab === tab ? "text-primary border-b-2 border-primary" : "text-muted-foreground hover:text-foreground"
                )}
             >
                {tab}
             </button>
         ))}
      </div>

      <ScrollArea className="flex-1">
          {activeTab === 'layers' ? (
             <div className="flex flex-col py-2 px-2">
                {activeLayers.length === 0 ? (
                    <div className="flex flex-col items-center justify-center py-10 px-6 text-center opacity-30">
                        <Layers className="w-6 h-6 mb-3" />
                        <p className="text-[10px] font-medium leading-relaxed">Workspace is empty</p>
                    </div>
                ) : (
                <DndContext 
                    sensors={sensors}
                    collisionDetection={closestCenter}
                    onDragEnd={handleDragEnd}
                >
                    <SortableContext 
                        items={activeLayers.map(el => el.id)}
                        strategy={verticalListSortingStrategy}
                    >
                        {activeLayers.map((el) => (
                            <SortableLayerItem 
                                key={el.id} 
                                element={el} 
                                selected={selectedElementIds.includes(el.id)} 
                                onClick={() => setSelectedElements([el.id])} 
                            />
                        ))}
                    </SortableContext>
                </DndContext>
                )}
             </div>
          ) : activeTab === 'assets' ? (
             <div className="p-4 space-y-4">
                <div {...getRootProps()} className="relative group">
                    <input {...getInputProps()} />
                    <Button disabled={isUploading} size="sm" className="w-full gap-2 text-[10px] font-bold rounded-lg transition-all border border-border bg-background hover:bg-accent text-foreground">
                       {isUploading ? <Loader2 className="w-3.5 h-3.5 animate-spin" /> : <Upload className="w-3.5 h-3.5" />}
                       Upload Asset
                    </Button>
                </div>
                
                <div className="grid grid-cols-2 gap-2 pb-6">
                   {globalAssets.filter((a: any) => a.mimetype?.startsWith('image/')).map((asset: any) => {
                      const imageUrl = `${process.env.NEXT_PUBLIC_API_URL}/files/proxy?filePath=vault/${asset.key}`;
                      return (
                        <div 
                          key={asset.id} 
                          onClick={() => addElement({ type: 'image', content: imageUrl, width: 400, height: 400, name: asset.name })} 
                          className="group relative aspect-square bg-accent/30 rounded-lg border border-border overflow-hidden hover:border-primary transition-all cursor-pointer"
                        >
                           <img src={imageUrl} alt={asset.name} className="w-full h-full object-cover transition-transform duration-300 group-hover:scale-110" />
                           <div className="absolute inset-0 bg-primary/10 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center">
                              <Plus className="w-4 h-4 text-primary" />
                           </div>
                        </div>
                      );
                   })}
                </div>
             </div>
          ) : (
             <div className="p-4 space-y-4">
                <div className="relative">
                    <Search className="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-muted-foreground" />
                    <Input 
                        placeholder="Search variables..." 
                        className="h-8 pl-8 text-[11px] bg-accent/50 border-border rounded-lg"
                        value={searchQuery}
                        onChange={(e) => setSearchQuery(e.target.value)}
                    />
                </div>

                <div className="space-y-1">
                    {databaseVariables.map(v => (
                        <button
                            key={v.key}
                            onClick={() => addElement({ 
                                type: v.isTable ? "table" : "variable", 
                                variableKey: v.key,
                                name: v.label,
                                width: v.isTable ? 600 : 250,
                                height: v.isTable ? 400 : 40,
                                style: { 
                                    fontFamily: 'Inter',
                                    fontSize: v.isTable ? 10 : 24,
                                    color: v.isTable ? '#000000' : 'hsl(var(--primary))',
                                    fontWeight: 'bold',
                                    textAlign: 'center'
                                }
                            })}
                            className="w-full flex items-center gap-3 px-3 py-2.5 rounded-xl hover:bg-accent group transition-all"
                        >
                            <div className="w-8 h-8 rounded-lg bg-accent/50 flex items-center justify-center transition-colors group-hover:bg-primary/10">
                                <v.icon className={cn("w-4 h-4 transition-colors", activeTab === 'variables' ? "text-primary/60 group-hover:text-primary" : "text-muted-foreground")} />
                            </div>
                            <div className="flex flex-col text-left overflow-hidden">
                                <span className="text-[11px] font-bold text-foreground/80 group-hover:text-foreground tracking-tight truncate">{v.label}</span>
                                <span className="text-[9px] text-muted-foreground font-mono">{v.key}</span>
                            </div>
                            <Plus className="w-3.5 h-3.5 ml-auto opacity-0 group-hover:opacity-40 transition-opacity shrink-0" />
                        </button>
                    ))}
                </div>
             </div>
          )}
      </ScrollArea>
    </div>
  );
}

function SortableLayerItem({ element, selected, onClick }: { element: any, selected: boolean, onClick: () => void }) {
  const { updateElement } = useEditorStore();
  const [isEditing, setIsEditing] = useState(false);
  const [tempName, setTempName] = useState(element.name || element.type);

  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging
  } = useSortable({id: element.id});

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    zIndex: isDragging ? 100 : 1,
    opacity: isDragging ? 0.6 : 1
  };

  const handleRename = () => {
    if (tempName.trim()) {
        updateElement(element.id, { name: tempName });
    }
    setIsEditing(false);
  };

  return (
    <div
      ref={setNodeRef}
      style={style}
      onClick={onClick}
      className={cn(
        "group flex items-center gap-2 px-2 py-1.5 rounded-md cursor-default transition-colors mb-0.5",
        selected 
            ? "bg-primary text-primary-foreground" 
            : "hover:bg-accent text-muted-foreground hover:text-foreground font-medium"
      )}
    >
      <div {...attributes} {...listeners} className="cursor-grab active:cursor-grabbing opacity-0 group-hover:opacity-50 transition-opacity">
          <GripVertical className="w-3 h-3" />
      </div>
      <LayerIcon type={element.type} selected={selected} />
      
      {isEditing ? (
          <input 
              autoFocus
              className="flex-1 bg-background text-foreground text-[11px] px-1 py-0 rounded border-none outline-none focus:ring-1 focus:ring-primary-foreground h-4"
              value={tempName}
              onChange={(e) => setTempName(e.target.value)}
              onBlur={handleRename}
              onKeyDown={(e) => {
                  if (e.key === 'Enter') handleRename();
                  if (e.key === 'Escape') {
                      setTempName(element.name || element.type);
                      setIsEditing(false);
                  }
              }}
          />
      ) : (
          <span 
              onDoubleClick={() => setIsEditing(true)}
              className={cn("text-[11px] truncate flex-1 tracking-tight", selected ? "font-bold" : "font-medium")}
          >
              {element.name || element.type}
          </span>
      )}

      <div className="flex items-center opacity-0 group-hover:opacity-100 gap-1.5 transition-opacity">
          <Lock className={cn("w-3 h-3 cursor-pointer", element.locked ? "text-primary-foreground" : "opacity-40 hover:opacity-100")} />
          <Eye className={cn("w-3 h-3 cursor-pointer", !element.visible ? "text-primary-foreground" : "opacity-40 hover:opacity-100")} />
      </div>
    </div>
  );
}

function LayerIcon({ type, selected }: { type: string, selected?: boolean }) {
    const className = cn("w-3.5 h-3.5", selected ? "text-primary-foreground" : "text-muted-foreground group-hover:text-foreground");
    switch (type) {
        case 'text': return <TextIcon className={className} />;
        case 'image': return <ImageIcon className={className} />;
        case 'shape': return <Square className={className} />;
        case 'variable': return <Database className={cn("w-3.5 h-3.5", selected ? "text-primary-foreground" : "text-primary")} />;
        case 'table': return <TableIcon className={cn("w-3.5 h-3.5", selected ? "text-primary-foreground" : "text-primary")} />;
        default: return <Grid2X2 className={className} />;
    }
}
