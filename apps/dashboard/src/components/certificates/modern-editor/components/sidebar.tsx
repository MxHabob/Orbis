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
  Circle,
  ChevronDown,
  ArrowLeft,
  Layout,
  Plus,
  MousePointer2,
  Hash,
  PenTool,
  Grid2X2,
  MessageSquare,
  FileText,
  Package,
  Upload,
  MoreVertical,
  Cloud,
  Loader2,
  GripVertical
} from "lucide-react";
import { Button } from "@midday/ui/button";
import { Input } from "@midday/ui/input";
import { ScrollArea } from "@midday/ui/scroll-area";
import { Separator } from "@midday/ui/separator";
import { useEditorStore } from "../store/use-editor-store";
import { cn } from "@midday/ui/cn";
import { useDropzone } from "react-dropzone";
import { useTRPC } from "@/trpc/client";
import { useMutation, useQuery } from "@tanstack/react-query";
import { useToast } from "@midday/ui/use-toast";
import { authClient } from "@midday/auth/auth-client";

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

export function Sidebar({ templateName }: SidebarProps) {
  const { elements, selectedElementIds, setSelectedElements, addElement, reorderElements } = useEditorStore();
  const [activeTab, setActiveTab] = useState<"layers" | "assets">("layers");
  const { toast } = useToast();
  const trpc = useTRPC();
  const { data: session } = authClient.useSession();
  const [isUploading, setIsUploading] = useState(false);

  // DnD Sensors
  const sensors = useSensors(
    useSensor(PointerSensor),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    })
  );

  // Fetch Global Assets
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
  }, [session, processDocumentMutation, refetchAssets]);

  const { getRootProps, getInputProps } = useDropzone({
    onDrop: handleUpload,
    accept: { "image/*": [".png", ".jpg", ".jpeg", ".svg", ".webp"] }
  });

  // Layer management - In Design tools, first array element is bottom, last is top.
  // Sidebar displays Top to Bottom.
  const activeLayers = [...elements].reverse(); 

  const handleDragEnd = (event: DragEndEvent) => {
    const { active, over } = event;
    if (active.id !== over?.id) {
       const oldIndex = activeLayers.findIndex((item) => item.id === active.id);
       const newIndex = activeLayers.findIndex((item) => item.id === over?.id);
       
       // Sync back to elements array (which is bottom-up)
       const actualOldIndex = elements.length - 1 - oldIndex;
       const actualNewIndex = elements.length - 1 - newIndex;
       
       reorderElements(actualOldIndex, actualNewIndex);
    }
  };

  return (
    <div className="flex flex-col h-full bg-[#1e1e1e] border-r border-[#333333] select-none w-64 shadow-xl z-20">
      {/* Header */}
      <div className="p-4 space-y-1">
         <div className="flex items-center justify-between group">
            <div className="flex items-center gap-2">
                <div className="w-6 h-6 rounded bg-[#2c2c2c] border border-[#444] flex items-center justify-center">
                    <Icons.Figma className="w-3.5 h-3.5 text-white/80" />
                </div>
                <div className="flex flex-col">
                    <span className="text-[11px] font-bold text-white/90 truncate max-w-[120px]">{templateName}</span>
                    <div className="flex items-center gap-1">
                        <span className="text-[9px] text-white/40">Cloud Workspace</span>
                        <div className="px-1 py-0.5 rounded-[2px] bg-emerald-500/10 border border-emerald-500/20 text-emerald-400 text-[8px] font-bold uppercase tracking-widest">PRO</div>
                    </div>
                </div>
            </div>
            <Button variant="ghost" size="icon" className="h-6 w-6 text-white/40 hover:text-white">
                <MoreVertical className="w-4 h-4" />
            </Button>
         </div>
      </div>

      {/* Tabs */}
      <div className="px-3 flex items-center gap-4 border-b border-[#333333] pb-2">
         <button onClick={() => setActiveTab('layers')} className={cn("text-[11px] font-bold transition-colors", activeTab === 'layers' ? "text-white border-b-2 border-[#0c8ce9] pb-1" : "text-white/40 hover:text-white/60 pb-1")}>Layers</button>
         <button onClick={() => setActiveTab('assets')} className={cn("text-[11px] font-bold transition-colors", activeTab === 'assets' ? "text-white border-b-2 border-[#0c8ce9] pb-1" : "text-white/40 hover:text-white/60 pb-1")}>Assets</button>
      </div>

      <ScrollArea className="flex-1">
          {activeTab === 'layers' ? (
             <div className="flex flex-col py-2">
                <div className="px-4 py-2 flex items-center justify-between group cursor-pointer hover:bg-white/5 uppercase tracking-widest opacity-40">
                    <span className="text-[10px] font-bold text-white/90">Main Design</span>
                    <ChevronDown className="w-3 h-3 text-white/40" />
                </div>
                <div className="px-2 mt-2">
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
                </div>
             </div>
          ) : (
             <div className="p-4 space-y-6">
                <div className="flex flex-col gap-4">
                    <div {...getRootProps()} className="relative group">
                        <input {...getInputProps()} />
                        <Button disabled={isUploading} className="w-full bg-[#2c2c2c] border border-white/5 hover:bg-[#383838] h-10 gap-2 text-white/90 text-[11px] font-bold shadow-lg">
                           {isUploading ? <Loader2 className="w-4 h-4 animate-spin" /> : <Upload className="w-4 h-4 text-[#0c8ce9]" />}
                           Upload to Cloud
                        </Button>
                    </div>
                    <div className="space-y-4">
                        <div className="flex items-center justify-between uppercase tracking-tighter">
                            <span className="text-[10px] font-bold text-white/40">Global Library</span>
                            <Cloud className="w-3 h-3 text-emerald-400" />
                        </div>
                        <div className="grid grid-cols-2 gap-3">
                           {globalAssets.filter((a: any) => a.mimetype?.startsWith('image/')).map((asset: any) => {
                              const imageUrl = `${process.env.NEXT_PUBLIC_API_URL}/files/proxy?filePath=vault/${asset.key}`;
                              return (
                                <div key={asset.id} onClick={() => addElement({ type: 'image', content: imageUrl, width: 400, height: 400, name: asset.name })} className="group relative aspect-square bg-[#121212] rounded-[8px] border border-white/5 overflow-hidden hover:border-[#0c8ce9]/50 transition-all cursor-pointer shadow-md">
                                   <img src={imageUrl} alt={asset.name} className="w-full h-full object-cover transition-transform group-hover:scale-110" />
                                   <div className="absolute inset-0 bg-black/40 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center"><Plus className="w-5 h-5 text-white" /></div>
                                   <div className="absolute bottom-0 left-0 right-0 p-2 bg-gradient-to-t from-black/80 to-transparent"><p className="text-[8px] text-white/80 truncate font-medium">{asset.name}</p></div>
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
    transition,
    zIndex: isDragging ? 100 : 1,
    opacity: isDragging ? 0.5 : 1
  };

  return (
    <div
      ref={setNodeRef}
      style={style}
      onClick={onClick}
      className={cn(
        "group flex items-center gap-2 px-4 py-1.5 rounded-[4px] cursor-default transition-all duration-75 mx-1 mb-0.5",
        selected ? "bg-[#0c8ce9] text-white shadow-sm" : "hover:bg-white/5 text-white/40 hover:text-white/90"
      )}
    >
      <div {...attributes} {...listeners} className="cursor-grab active:cursor-grabbing opacity-0 group-hover:opacity-60 hover:opacity-100 p-0.5">
          <GripVertical className="w-3 h-3" />
      </div>
      <LayerIcon type={element.type} />
      <span className={cn("text-[11px] truncate flex-1", selected ? "font-bold" : "font-medium")}>
          {element.name || element.type}
      </span>
      <div className="flex items-center opacity-0 group-hover:opacity-100 gap-2 shrink-0">
          <Lock className={cn("w-3 h-3 transition-opacity", element.locked ? "opacity-100" : "opacity-40 hover:opacity-100")} />
          <Eye className={cn("w-3 h-3 transition-opacity", !element.visible ? "opacity-100" : "opacity-40 hover:opacity-100")} />
      </div>
    </div>
  );
}

function LayerIcon({ type }: { type: string }) {
    switch (type) {
        case 'frame': return <Hash className="w-3.5 h-3.5" />;
        case 'text': return <TextIcon className="w-3.5 h-3.5" />;
        case 'image': return <ImageIcon className="w-3.5 h-3.5" />;
        case 'shape': return <Square className="w-3.5 h-3.5" />;
        case 'variable': return <Database className="w-3.5 h-3.5 text-blue-400" />;
        default: return <Grid2X2 className="w-3.5 h-3.5" />;
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
