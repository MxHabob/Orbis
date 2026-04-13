"use client";

import React, { useState, useCallback } from "react";
import { 
  Square, 
  Image as ImageIcon, 
  Database, 
  Search,
  Layers,
  Eye,
  Lock,
  Type as TextIcon,
  ChevronDown,
  Hash,
  Grid2X2,
  Plus,
  Upload,
  MoreVertical,
  Cloud,
  Loader2,
  GripVertical
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
  arrayMove,
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
  const { elements, selectedElementIds, setSelectedElements, addElement, reorderElements } = useEditorStore();
  const [activeTab, setActiveTab] = useState<"layers" | "assets">("layers");
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

  return (
    <div className="flex flex-col h-full glass-panel select-none w-64 z-20 border-r-0">
      {/* Header */}
      <div className="p-5 space-y-4">
         <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
                <div className="w-8 h-8 rounded-xl bg-gradient-to-br from-blue-500 to-indigo-600 shadow-lg shadow-blue-500/20 flex items-center justify-center">
                    <Icons.Figma className="w-4 h-4 text-white" />
                </div>
                <div className="flex flex-col">
                    <span className="text-xs font-bold text-white tracking-tight truncate max-w-[120px]">{templateName}</span>
                    <span className="text-[10px] text-white/30 font-medium">Auto-saving...</span>
                </div>
            </div>
            <Button variant="ghost" size="icon" className="h-8 w-8 text-white/20 hover:text-white/80 hover:bg-white/5 rounded-lg">
                <MoreVertical className="w-4 h-4" />
            </Button>
         </div>

         {/* Creation Tools */}
         <div className="grid grid-cols-4 gap-2">
            {[
                { type: 'frame', icon: Hash, label: 'Page' },
                { type: 'text', icon: TextIcon, label: 'Text' },
                { type: 'image', icon: ImageIcon, label: 'Image' },
                { type: 'shape', icon: Square, label: 'Rect' }
            ].map(tool => (
                <button 
                  key={tool.label}
                  onClick={() => addElement({ type: tool.type as any })}
                  className="flex flex-col items-center gap-1.5 p-2 rounded-xl hover:bg-white/5 transition-all group"
                >
                    <div className="w-9 h-9 rounded-lg bg-white/[0.03] border border-white/[0.05] flex items-center justify-center group-hover:scale-105 group-hover:bg-white/[0.08] group-hover:border-white/[0.1] transition-all">
                        <tool.icon className={cn("w-4 h-4", tool.type === 'frame' ? "text-purple-400" : "text-white/60")} />
                    </div>
                    <span className="text-[9px] text-white/30 font-bold uppercase tracking-wider group-hover:text-white/60">{tool.label}</span>
                </button>
            ))}
         </div>

         {/* Page Presets */}
         <div className="space-y-2 mt-2">
             <Label className="text-[9px] font-black text-white/20 uppercase tracking-widest pl-1">Page Presets</Label>
             <div className="grid grid-cols-1 gap-1">
                 {PAPER_PRESETS.map(preset => (
                     <button
                        key={preset.id}
                        onClick={() => addElement({ 
                            type: 'frame', 
                            width: preset.width, 
                            height: preset.height,
                            name: preset.name
                        })}
                        className="flex items-center justify-between px-3 py-2 rounded-lg hover:bg-white/5 transition-all border border-transparent hover:border-white/5 group"
                     >
                         <span className="text-[10px] font-bold text-white/60 group-hover:text-white">{preset.name}</span>
                         <span className="text-[9px] text-white/20 font-mono">{preset.width}×{preset.height}</span>
                     </button>
                 ))}
             </div>
         </div>
      </div>

      <Separator className="bg-white/5 mx-5 w-auto" />

      {/* Tabs */}
      <div className="px-5 flex items-center gap-6 mt-4 mb-2">
         <button onClick={() => setActiveTab('layers')} className={cn("text-[11px] font-bold transition-all relative pb-2", activeTab === 'layers' ? "text-white" : "text-white/30 hover:text-white/50")}>
            Layers
            {activeTab === 'layers' && <div className="absolute bottom-0 left-0 right-0 h-0.5 bg-blue-500 rounded-full shadow-[0_0_10px_rgba(59,130,246,0.5)]" />}
         </button>
         <button onClick={() => setActiveTab('assets')} className={cn("text-[11px] font-bold transition-all relative pb-2", activeTab === 'assets' ? "text-white" : "text-white/30 hover:text-white/50")}>
            Assets
            {activeTab === 'assets' && <div className="absolute bottom-0 left-0 right-0 h-0.5 bg-blue-500 rounded-full shadow-[0_0_10px_rgba(59,130,246,0.5)]" />}
         </button>
      </div>

      <ScrollArea className="flex-1 px-2">
          {activeTab === 'layers' ? (
             <div className="flex flex-col py-2">
                <div className="px-3 mt-1">
                   {activeLayers.length === 0 ? (
                       <div className="flex flex-col items-center justify-center py-10 px-6 text-center opacity-20">
                            <Layers className="w-8 h-8 mb-3" />
                            <p className="text-[11px] font-medium leading-relaxed">Design is empty.</p>
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
             </div>
          ) : (
             <div className="p-4 space-y-6">
                <div className="flex flex-col gap-5">
                    <div {...getRootProps()} className="relative group">
                        <input {...getInputProps()} />
                        <Button disabled={isUploading} className="w-full bg-blue-600 hover:bg-blue-500 text-white border-0 h-10 gap-2 text-[11px] font-bold shadow-lg shadow-blue-600/20 rounded-xl transition-all hover:-translate-y-0.5 active:translate-y-0">
                           {isUploading ? <Loader2 className="w-4 h-4 animate-spin" /> : <Upload className="w-4 h-4 shadow-sm" />}
                           Import Assets
                        </Button>
                    </div>
                    
                    <div className="space-y-4">
                        <div className="grid grid-cols-2 gap-3 pb-6">
                           {globalAssets.filter((a: any) => a.mimetype?.startsWith('image/')).map((asset: any) => {
                              const imageUrl = `${process.env.NEXT_PUBLIC_API_URL}/files/proxy?filePath=vault/${asset.key}`;
                              return (
                                <div 
                                  key={asset.id} 
                                  onClick={() => addElement({ type: 'image', content: imageUrl, width: 400, height: 400, name: asset.name })} 
                                  className="group relative aspect-square bg-white/[0.03] rounded-xl border border-white/[0.05] overflow-hidden hover:border-blue-500/50 transition-all cursor-pointer shadow-xl active:scale-95"
                                >
                                   <img src={imageUrl} alt={asset.name} className="w-full h-full object-cover transition-transform duration-500 group-hover:scale-110" />
                                   <div className="absolute inset-0 bg-blue-600/20 opacity-0 group-hover:opacity-100 transition-all flex items-center justify-center">
                                      <Plus className="w-6 h-6 text-white drop-shadow-lg" />
                                   </div>
                                </div>
                              );
                           })}
                        </div>
                    </div>
                </div>
             </div>
          )}
      </ScrollArea>
    </div>
  );
}

function SortableLayerItem({ element, selected, onClick }: { element: any, selected: boolean, onClick: () => void }) {
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
    transition: transition || 'all 0.2s cubic-bezier(0.175, 0.885, 0.32, 1.275)',
    zIndex: isDragging ? 100 : 1,
    opacity: isDragging ? 0.6 : 1
  };

  return (
    <div
      ref={setNodeRef}
      style={style}
      onClick={onClick}
      className={cn(
        "group flex items-center gap-3 px-3 py-2.5 rounded-xl cursor-default transition-all duration-200 mb-1 active:scale-[0.98]",
        selected 
            ? "bg-blue-600 shadow-[0_4px_15px_rgba(37,99,235,0.3)] text-white" 
            : "hover:bg-white/[0.05] text-white/[0.4] hover:text-white/[0.9]"
      )}
    >
      <div {...attributes} {...listeners} className="cursor-grab active:cursor-grabbing opacity-0 group-hover:opacity-60 hover:opacity-100 transition-opacity">
          <GripVertical className="w-3.5 h-3.5" />
      </div>
      <LayerIcon type={element.type} selected={selected} />
      <span className={cn("text-[11px] truncate flex-1 tracking-tight", selected ? "font-bold" : "font-medium")}>
          {element.name || element.type}
      </span>
      <div className="flex items-center opacity-0 group-hover:opacity-100 gap-2.5 transition-all">
          <Lock className={cn("w-3.5 h-3.5 transition-all cursor-pointer", element.locked ? "text-white" : "opacity-40 hover:opacity-100")} />
          <Eye className={cn("w-3.5 h-3.5 transition-all cursor-pointer", !element.visible ? "text-white" : "opacity-40 hover:opacity-100")} />
      </div>
    </div>
  );
}

function LayerIcon({ type, selected }: { type: string, selected?: boolean }) {
    const className = cn("w-4 h-4", selected ? "text-white" : "text-white/40 group-hover:text-white/80");
    switch (type) {
        case 'text': return <TextIcon className={className} />;
        case 'image': return <ImageIcon className={className} />;
        case 'shape': return <Square className={className} />;
        case 'variable': return <Database className={cn("w-4 h-4", selected ? "text-white" : "text-blue-400")} />;
        default: return <Grid2X2 className={className} />;
    }
}

const Icons = {
    Figma: ({ className }: { className: string }) => (
        <svg viewBox="0 0 38 57" fill="none" xmlns="http://www.w3.org/2000/svg" className={className}>
            <path d="M19 28.5C19 25.9837 20.0009 23.5706 21.7825 21.7891C23.564 20.0076 25.9771 19.0067 28.4933 19.0067C31.0096 19.0067 33.4227 20.0076 35.2042 21.7891C36.9858 23.5706 37.9867 25.9837 37.9867 28.5L37.9867 38H28.4933C25.9771 38 23.564 36.9991 21.7825 35.2176C20.0009 33.436 19 31.0229 19 28.5Z" fill="#1ABCFE"/>
            <path d="M0 47.5C0 44.9837 1.00089 42.5706 2.78249 40.7891C4.56408 39.0076 6.97718 38.0067 9.49333 38.0067H19V47.5C19 50.0163 17.9991 52.4294 16.2175 54.2109C14.4359 55.9924 12.0228 56.9933 9.50667 56.9933C6.99052 56.9933 4.57741 55.9924 2.79582 54.2109C1.01423 52.4294 0.0133333 50.0163 0.0133333 47.5H0Z" fill="#0AC17E"/>
            <path d="M0 28.5C0 25.9837 1.00089 23.5706 2.78249 21.7891C4.56408 20.0076 6.97718 19.0067 9.49333 19.0067H19V38H9.49333C6.97718 38 4.56408 36.9991 2.78249 35.2176C1.00089 33.436 0 31.0229 0 28.5Z" fill="#A259FF"/>
            <path d="M0 9.50667C0 6.99052 1.00089 4.57741 2.78249 2.79582C4.56408 1.01423 6.97718 0.0133333 9.49333 0.0133333H19V19.0067H9.49333C6.97718 19.0067 4.56408 18.0058 2.78249 16.2242C1.00089 14.4426 0 12.0295 0 9.51333V9.50667Z" fill="#F24E1E"/>
            <path d="M19 0.0133333H28.5067C31.0228 0.0133333 33.4359 1.01423 35.2175 2.79582C36.9991 4.57741 38 6.99052 38 9.50667C38 12.0228 36.9991 14.4359 35.2175 16.2175C33.4359 17.9991 31.0228 19 28.5067 19H19V0.0133333Z" fill="#FF7262"/>
        </svg>
    )
}
