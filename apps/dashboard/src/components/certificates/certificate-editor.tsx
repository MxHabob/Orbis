
"use client";

import { useState, useRef } from "react";
import { useRouter } from "next/navigation";
import { useUpload } from "@/hooks/use-upload";
import { Button } from "@midday/ui/button";
import { Input } from "@midday/ui/input";
import { Label } from "@midday/ui/label";
import { Slider } from "@midday/ui/slider";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@midday/ui/tabs";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@midday/ui/select";
import { useTRPC } from "@/trpc/client";
import { useMutation } from "@tanstack/react-query";
import { toast } from "@midday/ui/use-toast";
import { Plus, Type, Upload, ArrowLeft, Loader2, Image as ImageIcon } from "lucide-react";

import { cn } from "@midday/ui/cn";
import { useHotkeys } from "react-hotkeys-hook";
import { Icons } from "@midday/ui/icons";

type ElementType = "text" | "variable";

interface EditorElement {
  id: string;
  type: ElementType;
  content: string;
  variableKey?: string; // For dynamic data like {{student_name}}
  x: number; // percentage 0-100
  y: number; // percentage 0-100
  fontSize: number;
  fontWeight: string;
  color: string;
  align: "left" | "center" | "right";
  width?: number; // max-width in percent
}

const VARIABLES = [
  { label: "اسم الطالب", value: "{{student_name}}" },
  { label: "اسم المدرسة", value: "{{school_name}}" },
  { label: "الصف الدراسي", value: "{{class_name}}" },
  { label: "العام الدراسي", value: "{{academic_year}}" },
  { label: "المجموع الكلي", value: "{{total_grade}}" },
  { label: "النسبة المئوية", value: "{{percentage}}" },
  { label: "تاريخ اليوم", value: "{{date}}" },
  { label: "التقدير", value: "{{grade_label}}" },
  { label: "رقم الجلوس", value: "{{student_number}}" },
];

export function CertificateEditor({
  initialData,
  mode = "create",
}: {
  initialData?: any;
  mode?: "create" | "edit";
}) {
  const router = useRouter();
  const trpc = useTRPC();
  const [isSaving, setIsSaving] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);
  
  const createTemplate = useMutation(trpc.certificateTemplate.create.mutationOptions({}));
  const updateTemplate = useMutation(trpc.certificateTemplate.upsert.mutationOptions({}));
  
  // State
  const [name, setName] = useState(initialData?.name || "New Template");
  const [backgroundImage, setBackgroundImage] = useState<string | null>(initialData?.backgroundImageUrl || null);
  const [elements, setElements] = useState<EditorElement[]>(
    initialData?.headerContent?.elements || []
  );
  const [selectedElementId, setSelectedElementId] = useState<string | null>(null);
  
  // Tools
  const { uploadFile, isLoading: isUploading } = useUpload();
  
  const handleBackgroundUpload = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;
    
    const { url } = await uploadFile({
      bucket: "apps",
      path: ["certificates", "backgrounds", `${crypto.randomUUID()}-${file.name}`],
      file,
    });
    
    if (url) {
      setBackgroundImage(url);
    }
  };

  const addElement = (type: ElementType, content: string, variableKey?: string) => {
    const newElement: EditorElement = {
      id: crypto.randomUUID(),
      type,
      content,
      variableKey,
      x: 50, // Center
      y: 50, // Center
      fontSize: 24,
      fontWeight: "normal",
      color: "#000000",
      align: "center",
    };
    
    setElements([...elements, newElement]);
    setSelectedElementId(newElement.id);
  };

  const updateElement = (id: string, updates: Partial<EditorElement>) => {
    setElements(elements.map(el => el.id === id ? { ...el, ...updates } : el));
  };
  
  const removeElement = (id: string) => {
    setElements(elements.filter(el => el.id !== id));
    if (selectedElementId === id) setSelectedElementId(null);
  };

  // Drag Logic (Simplified for React State)
  const handleMouseDown = (e: React.MouseEvent, id: string) => {
    e.stopPropagation();
    setSelectedElementId(id);
    
    const startX = e.clientX;
    const startY = e.clientY;
    const element = elements.find(el => el.id === id);
    if (!element || !containerRef.current) return;
    
    const startElX = element.x;
    const startElY = element.y;
    const containerRect = containerRef.current.getBoundingClientRect();
    
    const handleMouseMove = (moveEvent: MouseEvent) => {
      const deltaX = moveEvent.clientX - startX;
      const deltaY = moveEvent.clientY - startY;
      
      const percentX = (deltaX / containerRect.width) * 100;
      const percentY = (deltaY / containerRect.height) * 100;
      
      updateElement(id, {
        x: Math.min(100, Math.max(0, startElX + percentX)),
        y: Math.min(100, Math.max(0, startElY + percentY))
      });
    };
    
    const handleMouseUp = () => {
      document.removeEventListener("mousemove", handleMouseMove);
      document.removeEventListener("mouseup", handleMouseUp);
    };
    
    document.addEventListener("mousemove", handleMouseMove);
    document.addEventListener("mouseup", handleMouseUp);
  };

  const handleSave = async () => {
    setIsSaving(true);
    try {
      const payload = {
        name,
        backgroundImageUrl: backgroundImage,
        headerContent: { elements }, // Store elements in headerContent JSON
        // Assume default A4/Portrait for now
      };
      
      if (mode === "edit" && initialData?.id) {
        await updateTemplate.mutateAsync({
          id: initialData.id,
          ...payload,
        });
        toast({ title: "Template updated", variant: "success" });
      } else {
        await createTemplate.mutateAsync(payload);
        toast({ title: "Template created", variant: "success" });
        router.push("/certificates");
      }
    } catch (e) {
      toast({ title: "Failed to save template", variant: "error" });
      console.error(e);
    } finally {
      setIsSaving(false);
    }
  };

  useHotkeys("delete", () => {
    if (selectedElementId) removeElement(selectedElementId);
  }, [selectedElementId]);

  const selectedElement = elements.find(el => el.id === selectedElementId);

  return (
    <div className="flex h-[calc(100vh-80px)] overflow-hidden">
      {/* Sidebar Controls */}
      <div className="w-80 bg-background border-r p-4 flex flex-col gap-6 overflow-y-auto z-10 shrink-0">
        <div className="flex items-center gap-2 mb-4">
          <Button variant="ghost" size="icon" onClick={() => router.back()}>
            <ArrowLeft className="w-4 h-4" />
          </Button>
          <Input 
            value={name} 
            onChange={(e) => setName(e.target.value)} 
            className="font-bold border-none shadow-none focus-visible:ring-0 px-0 text-lg"
          />
        </div>

        <Tabs defaultValue="add">
          <TabsList className="w-full">
            <TabsTrigger value="add" className="flex-1">Add</TabsTrigger>
            <TabsTrigger value="edit" className="flex-1" disabled={!selectedElement}>Edit</TabsTrigger>
            <TabsTrigger value="settings" className="flex-1">Settings</TabsTrigger>
          </TabsList>
          
          <TabsContent value="add" className="space-y-4 mt-4">
            <div className="space-y-2">
              <Label>Background Image</Label>
              <div className="border border-dashed rounded-lg p-4 text-center hover:bg-muted/50 transition-colors relative">
                {isUploading ? (
                  <div className="flex flex-col items-center gap-2">
                    <Loader2 className="w-6 h-6 animate-spin text-muted-foreground" />
                    <span className="text-xs text-muted-foreground">Uploading...</span>
                  </div>
                ) : (
                  <>
                    <input 
                      type="file" 
                      accept="image/*" 
                      className="absolute inset-0 opacity-0 cursor-pointer" 
                      onChange={handleBackgroundUpload}
                    />
                    <div className="flex flex-col items-center gap-2">
                      <Upload className="w-6 h-6 text-muted-foreground" />
                      <span className="text-sm font-medium">Upload from Canva/device</span>
                      <span className="text-xs text-muted-foreground">PNG, JPG (A4 Recommended)</span>
                    </div>
                  </>
                )}
              </div>
              <div className="flex justify-center">
                 <a href="https://www.canva.com/create/certificates/" target="_blank" rel="noopener noreferrer" className="text-xs text-primary hover:underline flex items-center">
                   Open Canva to Design <Icons.ExternalLink className="w-3 h-3 ml-1" />
                 </a>
              </div>
            </div>

            <div className="space-y-2">
              <Label>Dynamic Fields</Label>
              <div className="grid grid-cols-1 gap-2">
                {VARIABLES.map((v) => (
                  <Button 
                    key={v.value} 
                    variant="outline" 
                    className="justify-start text-xs h-9"
                    onClick={() => addElement("variable", v.label, v.value)}
                  >
                    <Plus className="w-3 h-3 mr-2 text-muted-foreground" />
                    {v.label}
                  </Button>
                ))}
              </div>
            </div>

            <div className="space-y-2">
              <Label>Static Text</Label>
              <Button onClick={() => addElement("text", "نص ثابت")} className="w-full">
                <Type className="w-4 h-4 mr-2" /> Add Text
              </Button>
            </div>
          </TabsContent>

          <TabsContent value="edit" className="space-y-4 mt-4">
            {selectedElement ? (
              <>
                 <div className="space-y-2">
                   <Label>Content</Label>
                   <Input 
                     value={selectedElement.content} 
                     onChange={(e) => updateElement(selectedElement.id, { content: e.target.value })}
                     disabled={selectedElement.type === "variable"}
                   />
                 </div>
                 
                 <div className="space-y-2">
                   <Label>Font Size ({selectedElement.fontSize}px)</Label>
                   <Slider 
                     value={[selectedElement.fontSize]} 
                     min={8} 
                     max={120} 
                     step={1}
                     onValueChange={([val]) => updateElement(selectedElement.id, { fontSize: val })}
                   />
                 </div>

                 <div className="grid grid-cols-2 gap-4">
                    <div className="space-y-2">
                       <Label>Color</Label>
                       <div className="flex items-center gap-2">
                         <div className="w-6 h-6 rounded border shadow-sm" style={{ background: selectedElement.color }} />
                         <Input 
                            type="color" 
                            className="w-full h-8 p-1"
                            value={selectedElement.color}
                            onChange={(e) => updateElement(selectedElement.id, { color: e.target.value })}
                         />
                       </div>
                    </div>
                    <div className="space-y-2">
                       <Label>Weight</Label>
                       <Select 
                         value={selectedElement.fontWeight} 
                         onValueChange={(val) => updateElement(selectedElement.id, { fontWeight: val })}
                        >
                         <SelectTrigger><SelectValue /></SelectTrigger>
                         <SelectContent>
                           <SelectItem value="normal">Normal</SelectItem>
                           <SelectItem value="bold">Bold</SelectItem>
                         </SelectContent>
                       </Select>
                    </div>
                 </div>
                 
                 <div className="space-y-2">
                   <Label>Alignment</Label>
                   <div className="flex border rounded-md overflow-hidden">
                     {["left", "center", "right"].map((align) => (
                       <button
                         key={align}
                         className={cn(
                           "flex-1 p-2 text-xs hover:bg-muted capitalize",
                           selectedElement.align === align && "bg-secondary font-medium"
                          )}
                         onClick={() => updateElement(selectedElement.id, { align: align as any })}
                       >
                         {align}
                       </button>
                     ))}
                   </div>
                 </div>
                 
                 <Button variant="destructive" onClick={() => removeElement(selectedElement.id)} className="w-full mt-4">
                   Remove Element
                 </Button>
              </>
            ) : (
              <div className="text-center py-10 text-muted-foreground">Select an element to edit</div>
            )}
          </TabsContent>
          
          <TabsContent value="settings" className="space-y-4 text-sm text-muted-foreground mt-4">
            <p>
              Size: A4 (210mm x 297mm) <br/>
              Orientation: Portrait
            </p>
          </TabsContent>
        </Tabs>
        
        <div className="mt-auto pt-4 border-t">
          <Button onClick={handleSave} disabled={isSaving} className="w-full">
            {isSaving && <Loader2 className="w-4 h-4 mr-2 animate-spin" />}
            Save Template
          </Button>
        </div>
      </div>

      {/* Main Canvas Area */}
      <div className="flex-1 bg-grid-slate-100 dark:bg-grid-slate-900/10 bg-secondary/30 p-8 flex items-center justify-center overflow-auto relative">
         <div 
           ref={containerRef}
           className="bg-white shadow-xl relative overflow-hidden transition-all duration-300" 
           style={{ 
             width: "595px", // A4 at 72dpi approx, simplified for screen
             height: "842px",
             aspectRatio: "210/297",
             transform: "scale(0.8)", // Initial zoom
           }}
           onClick={() => setSelectedElementId(null)}
         >
            {/* Background */}
            {backgroundImage ? (
              <img 
                src={backgroundImage} 
                alt="Certificate Background" 
                className="w-full h-full object-cover absolute inset-0 pointer-events-none select-none"
              />
            ) : (
              <div className="w-full h-full flex flex-col items-center justify-center text-muted-foreground/30 border-2 border-dashed border-muted m-4 rounded-xl">
                <ImageIcon className="w-16 h-16 mb-4" />
                <p>Upload a background image</p>
              </div>
            )}

            {/* Elements */}
            {elements.map((el) => (
              <div
                key={el.id}
                onMouseDown={(e) => handleMouseDown(e, el.id)}
                className={cn(
                  "absolute cursor-move select-none p-1 border border-transparent hover:border-blue-300 transition-colors",
                  selectedElementId === el.id && "border-primary ring-1 ring-primary"
                )}
                style={{
                  left: `${el.x}%`,
                  top: `${el.y}%`,
                  transform: "translate(-50%, -50%)",
                  fontSize: `${el.fontSize}px`,
                  fontWeight: el.fontWeight,
                  color: el.color,
                  textAlign: el.align,
                  whiteSpace: "nowrap",
                }}
              >
                {el.variableKey ? <span className="opacity-80 font-mono text-sm bg-yellow-100/50 px-1 border border-yellow-200 rounded text-black">{el.content}</span> : el.content}
              </div>
            ))}
         </div>
      </div>
    </div>
  );
}
