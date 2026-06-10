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
  Copy,
  Trash2,
  ArrowUp,
  ArrowDown,
  ChevronDown,
  FileText
} from "lucide-react";
import { Button } from "@orbis/ui/button";
import { ScrollArea } from "@orbis/ui/scroll-area";
import { Separator } from "@orbis/ui/separator";
import { useEditorStore } from "@/store/certificate-editor";
import { cn } from "@orbis/ui/cn";
import { useDropzone } from "react-dropzone";
import { useTRPC } from "@/trpc/client";
import { useMutation, useQuery } from "@tanstack/react-query";
import { useToast } from "@orbis/ui/use-toast";
import { authClient } from "@orbis/auth/auth-client";
import { Input } from "@orbis/ui/input";
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
  ContextMenuSeparator,
  ContextMenuShortcut,
} from "@orbis/ui/context-menu";
import { EditorSidebarHeader } from "./editor-sidebar-header";

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

interface SidebarProps {}

export function EditorSidebar({}: SidebarProps) {
  const { 
    elements, 
    selectedElementIds, 
    setSelectedElements, 
    addElement, 
    reorderElements,
    name
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

  const [collapsedGroups, setCollapsedGroups] = useState<Set<string>>(new Set());

  const toggleGroup = (id: string, e: React.MouseEvent) => {
    e.stopPropagation();
    const newCollapsed = new Set(collapsedGroups);
    if (newCollapsed.has(id)) newCollapsed.delete(id);
    else newCollapsed.add(id);
    setCollapsedGroups(newCollapsed);
  };

  // Build a hierarchical tree for rendering
  const buildLayerTree = (items: typeof elements, parentId?: string, depth = 0): any[] => {
    const result: any[] = [];
    const children = items
        .filter(el => el.parentId === parentId)
        .sort((a, b) => b.zIndex - a.zIndex); // Top zIndex first in layers list

    children.forEach(child => {
        result.push({ ...child, depth });
        if (child.type === 'group' && !collapsedGroups.has(child.id)) {
            result.push(...buildLayerTree(items, child.id, depth + 1));
        }
    });

    return result;
  };

  const activeLayers = buildLayerTree(elements); 

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
    { key: "student_image", label: "صورة الطالب", icon: ImageIcon },
    { key: "grades_table", label: "جدول كشف الدرجات", icon: TableIcon, isTable: true },
  ].filter(v => v.label.includes(searchQuery) || v.key.includes(searchQuery));

  return (
    <div className="flex flex-col h-full bg-background select-none w-64 z-20 border-r border-border">
      <EditorSidebarHeader />

      {/* Tabs / Tool Bar */}
      <div className="px-3 flex items-center gap-4 mt-2 mb-2">
         {["layers", "assets", "variables"].map((tab) => (
             <button 
                key={tab}
                onClick={() => setActiveTab(tab as any)} 
                className={cn(
                    "text-[11px] font-bold transition-all relative pb-2 capitalize", 
                    activeTab === tab ? "text-foreground border-b-2 border-primary" : "text-muted-foreground hover:text-foreground"
                )}
             >
                {tab === 'layers' ? 'File' : tab}
             </button>
         ))}
      </div>

      <ScrollArea className="flex-1">
          {activeTab === 'layers' ? (
             <div className="flex flex-col">
                {/* Pages Section */}
                <div className="px-3 py-2">
                    <div className="flex items-center justify-between mb-2 px-1">
                        <span className="text-[11px] font-bold text-foreground opacity-90">Pages</span>
                        <Plus className="w-3.5 h-3.5 text-muted-foreground cursor-pointer hover:text-foreground" />
                    </div>
                    <div className="flex items-center gap-2 px-2 py-1.5 rounded bg-muted text-foreground">
                        <span className="text-[11px] font-medium">Page 1</span>
                    </div>
                </div>

                <Separator className="mx-3 w-auto my-2" />

                {/* Layers Section */}
                <div className="px-3 py-2">
                    <div className="flex items-center justify-between mb-2 px-1">
                        <span className="text-[11px] font-bold text-foreground opacity-90">Layers</span>
                    </div>
                    <div className="flex flex-col gap-0.5">
                        {activeLayers.length === 0 ? (
                            <div className="flex flex-col items-center justify-center py-10 px-6 text-center opacity-10">
                                <Layers className="w-6 h-6 mb-3" />
                                <p className="text-[10px] font-bold uppercase tracking-widest text-foreground">Empty</p>
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
                                        depth={el.depth}
                                        isCollapsed={collapsedGroups.has(el.id)}
                                        onToggleGroup={(e) => toggleGroup(el.id, e)}
                                        selected={selectedElementIds.includes(el.id)} 
                                        onClick={() => setSelectedElements([el.id])} 
                                    />
                                ))}
                            </SortableContext>
                        </DndContext>
                        )}
                    </div>
                </div>
             </div>
          ) : activeTab === 'assets' ? (
             <div className="p-4 space-y-4">
                <div {...getRootProps()} className="relative group">
                    <input {...getInputProps()} />
                    <Button disabled={isUploading} size="sm" className="w-full gap-2">
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
                          className="group relative aspect-square bg-muted rounded border border-border overflow-hidden hover:border-primary/20 transition-all cursor-pointer"
                        >
                           <img src={imageUrl} alt={asset.name} className="w-full h-full object-cover transition-transform duration-300 group-hover:scale-110" />
                        </div>
                      );
                   })}
                </div>
             </div>
          ) : (
             <div className="p-3 space-y-4">
                <div className="relative">
                    <Search className="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-muted-foreground" />
                    <Input 
                        placeholder="Search variables..." 
                        className="h-8 pl-8 text-[11px] bg-muted border-border"
                        value={searchQuery}
                        onChange={(e) => setSearchQuery(e.target.value)}
                    />
                </div>

                <div className="space-y-0.5">
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
                            className="w-full flex items-center gap-3 px-3 py-2 rounded hover:bg-muted group transition-all"
                        >
                            <v.icon className="w-3.5 h-3.5 text-muted-foreground group-hover:text-primary transition-colors" />
                            <div className="flex flex-col text-left overflow-hidden">
                                <span className="text-[10px] font-bold text-foreground group-hover:text-primary tracking-tight truncate">{v.label}</span>
                                <span className="text-[8px] text-muted-foreground font-mono uppercase">{v.key}</span>
                            </div>
                        </button>
                    ))}
                </div>
             </div>
          )}
      </ScrollArea>
    </div>
  );
}

function SortableLayerItem({ element, selected, onClick, depth = 0, isCollapsed, onToggleGroup }: { 
    element: any, 
    selected: boolean, 
    onClick: () => void,
    depth?: number,
    isCollapsed?: boolean,
    onToggleGroup?: (e: React.MouseEvent) => void
}) {
  const { updateElement, deleteElements, duplicateElements, bringToFront, sendToBack } = useEditorStore();
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
    opacity: isDragging ? 0.6 : 1,
    paddingLeft: `${(depth * 12) + 8}px`
  };

  const handleRename = () => {
    if (tempName.trim()) {
        updateElement(element.id, { name: tempName });
    }
    setIsEditing(false);
  };

  return (
    <ContextMenu>
      <ContextMenuTrigger asChild>
        <div
          ref={setNodeRef}
          style={style}
          onClick={onClick}
          className={cn(
            "group flex items-center gap-2 px-2 py-1 rounded cursor-default transition-all mb-0.5",
            selected 
                ? "bg-accent text-accent-foreground" 
                : "hover:bg-muted text-foreground/70 hover:text-foreground"
          )}
        >
          <div {...attributes} {...listeners} className="cursor-grab active:cursor-grabbing opacity-0 group-hover:opacity-40 transition-opacity">
              <GripVertical className="w-3 h-3 text-muted-foreground" />
          </div>

          {element.type === 'group' && (
              <button onClick={onToggleGroup} className="p-0.5 hover:bg-muted-foreground/10 rounded transition-transform">
                  <ChevronDown className={cn("w-3 h-3 transition-transform", isCollapsed && "-rotate-90")} />
              </button>
          )}

          <LayerIcon type={element.type} selected={selected} />
          
          {isEditing ? (
              <Input 
                  autoFocus
                  className="flex-1 h-5 text-[10px] px-1 py-0 rounded border-primary/30 font-bold"
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
                  className={cn("text-[11px] truncate flex-1 tracking-tight", selected ? "opacity-100 font-bold" : "opacity-70 group-hover:opacity-100")}
              >
                  {element.name || element.type}
              </span>
          )}

          <div className="flex items-center opacity-0 group-hover:opacity-60 gap-1.5 transition-opacity text-muted-foreground">
              <button className="hover:text-foreground" onClick={(e) => { e.stopPropagation(); updateElement(element.id, { locked: !element.locked }); }}>
                 {element.locked ? <Lock className="w-3 h-3" /> : <Lock className="w-3 h-3 opacity-20" />}
              </button>
              <button className="hover:text-foreground" onClick={(e) => { e.stopPropagation(); updateElement(element.id, { visible: !element.visible }); }}>
                 {element.visible ? <Eye className="w-3 h-3" /> : <Eye className="w-3 h-3 opacity-20" />}
              </button>
          </div>
        </div>
      </ContextMenuTrigger>

      <ContextMenuContent className="w-48">
          <ContextMenuItem onClick={() => setIsEditing(true)} className="text-xs font-medium gap-2">
              Rename
              <ContextMenuShortcut>F2</ContextMenuShortcut>
          </ContextMenuItem>
          <ContextMenuItem onClick={() => duplicateElements([element.id])} className="text-xs font-medium gap-2">
              Duplicate
              <ContextMenuShortcut>⌘D</ContextMenuShortcut>
          </ContextMenuItem>
          <ContextMenuSeparator />
          <ContextMenuItem onClick={() => bringToFront(element.id)} className="text-xs font-medium gap-2">
              Bring to Front
          </ContextMenuItem>
          <ContextMenuItem onClick={() => sendToBack(element.id)} className="text-xs font-medium gap-2">
              Send to Back
          </ContextMenuItem>
          <ContextMenuSeparator />
          <ContextMenuItem onClick={() => deleteElements([element.id])} className="text-xs font-medium gap-2 text-destructive focus:text-destructive">
              Delete
              <ContextMenuShortcut>⌫</ContextMenuShortcut>
          </ContextMenuItem>
      </ContextMenuContent>
    </ContextMenu>
  );
}

function LayerIcon({ type, selected }: { type: string, selected?: boolean }) {
    const className = cn("w-3.5 h-3.5 transition-colors", selected ? "text-primary-foreground" : "text-muted-foreground group-hover:text-foreground");
    switch (type) {
        case 'text': return <TextIcon className={className} />;
        case 'image': return <ImageIcon className={className} />;
        case 'shape': return <Square className={className} />;
        case 'variable': return <Database className={cn("w-3.5 h-3.5", selected ? "text-primary-foreground" : "text-primary")} />;
        case 'table': return <TableIcon className={cn("w-3.5 h-3.5", selected ? "text-primary-foreground" : "text-primary")} />;
        case 'group': return <Layers className={className} />;
        default: return <Grid2X2 className={className} />;
    }
}
