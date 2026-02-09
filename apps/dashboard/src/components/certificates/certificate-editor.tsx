"use client";

import { useState, useCallback, useRef, useEffect } from "react";
import { useRouter } from "next/navigation";
import { useTRPC } from "@/trpc/client";
import { Button } from "@midday/ui/button";
import { Input } from "@midday/ui/input";
import { Label } from "@midday/ui/label";
import { Slider } from "@midday/ui/slider";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@midday/ui/tabs";
import { SheetContent } from "@midday/ui/sheet";
import { ScrollArea } from "@midday/ui/scroll-area";
import { Portal } from "@/components/portal";
import { useCertificateParams } from "@/hooks/use-certificate-params";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@midday/ui/select";
import { useToast } from "@midday/ui/use-toast";
import { motion, AnimatePresence } from "framer-motion";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import {
  Type,
  ImageIcon,
  Layers,
  Upload,
  Plus,
  Trash2,
  Sparkles,
  Maximize,
  ZoomIn,
  ZoomOut,
  RotateCcw,
  Eye,
  EyeOff,
  ArrowLeft,
  Loader2,
  Save,
  Download,
  Square,
  Minus,
  Table as TableIcon,
  AlignLeft,
  AlignCenter,
  AlignRight,
  ChevronUp,
  ChevronDown,
  X,
} from "lucide-react";

import { cn } from "@midday/ui/cn";
import { Icons } from "@midday/ui/icons";
import { TransformWrapper, TransformComponent } from "react-zoom-pan-pinch";

type ElementType = "text" | "variable" | "image" | "table" | "shape";

interface TableColumn {
  id: string;
  header: string;
  width: number;
}

interface EditorElement {
  id: string;
  type: ElementType;
  content: string;
  variableKey?: string;
  x: number; // percentage
  y: number; // percentage
  fontSize?: number;
  fontWeight?: string;
  color?: string;
  align?: "left" | "center" | "right";
  width?: number; // percentage of canvas
  height?: number; // percentage of canvas
  opacity?: number;
  rotation?: number;
  fontFamily?: string;
  borderColor?: string;
  borderWidth?: number;
  backgroundColor?: string;
  columns?: TableColumn[]; // For table type
}

const PAPER_SIZES = {
  a4: { width: 595, height: 842 },
  letter: { width: 612, height: 792 },
};

const VARIABLES = [
  { group: "بيانات الطالب", items: [
    { label: "اسم الطالب", value: "{{student_name}}" },
    { label: "الرقم الأكاديمي", value: "{{academic_number}}" },
    { label: "الرقم القومي", value: "{{national_id}}" },
    { label: "القسم / التخصص", value: "{{department}}" },
    { label: "الشعبة", value: "{{section}}" },
  ]},
  { group: "النتائج الدراسية", items: [
    { label: "جدول الدرجات", value: "table", isTable: true },
    { label: "الصف / المستوى", value: "{{class_name}}" },
    { label: "المجموع الكلي (رقماً)", value: "{{total_grade}}" },
    { label: "المجموع الكلي (كتابة)", value: "{{total_grade_text}}" },
    { label: "التقدير العام", value: "{{grade_label}}" },
    { label: "النتيجة النهائية", value: "{{final_result}}" },
    { label: "الترتيب", value: "{{rank}}" },
  ]},
  { group: "بيانات المؤسسة", items: [
    { label: "اسم المنظمة", value: "{{org_name}}" },
    { label: "إدارة المدرسة", value: "{{admin_name}}" },
    { label: "وكيل المدرسة", value: "{{vp_name}}" },
    { label: "تاريخ التقرير", value: "{{issue_date}}" },
  ]}
];

// SAMPLE_DATA was removed to use real data from the database.


const FONTS = [
  { label: "الخط الافتراضي", value: "inherit" },
  { label: "خط كوفي", value: "'Kufam', sans-serif" },
  { label: "خط أميري (رسمي)", value: "'Amiri', serif" },
  { label: "خط مروان", value: "'Cairo', sans-serif" },
];

export function CertificateEditor({
  initialData,
  mode = "create",
  isSheet,
}: {
  initialData?: any;
  mode?: "create" | "edit";
  isSheet?: boolean;
}) {
  const { setParams } = useCertificateParams();
  const router = useRouter();
  const trpc = useTRPC();
  const [isSaving, setIsSaving] = useState(false);
  const { toast } = useToast();
  const queryClient = useQueryClient();

  const [name, setName] = useState(initialData?.name || "قالب جديد");
  const [paperSize, setPaperSize] = useState<"a4" | "letter">(initialData?.size || "a4");
  const [orientation, setOrientation] = useState<"portrait" | "landscape">(initialData?.orientation || "portrait");
  const [backgroundImage, setBackgroundImage] = useState(initialData?.backgroundImageUrl || "");
  const [elements, setElements] = useState<EditorElement[]>(initialData?.headerContent?.elements || []);
  const [selectedElementId, setSelectedElementId] = useState<string | null>(null);
  const [isPreview, setIsPreview] = useState(false);
  const [previewStudentId, setPreviewStudentId] = useState<string | null>(null);
  const { data: students } = useQuery(trpc.students.getAll.queryOptions());

  const [isDragging, setIsDragging] = useState(false);
  const [dragOffset, setDragOffset] = useState({ x: 0, y: 0 });
  const canvasRef = useRef<HTMLDivElement>(null);

  const selectedPreviewStudent = students?.find(s => s.userId === previewStudentId) || students?.[0];

  const getPreviewData = (student: any) => {
    if (!student) return {};
    const stats = (student.stats as any) || {};
    return {
      student_name: student.name || "اسم الطالب",
      name: student.name || "اسم الطالب",
      academic_number: student.studentNumber || "20240001",
      national_id: student.secretNumber || "123456789",
      department: stats.department || "علمي",
      section: stats.section || "أ",
      class_name: student.currentClass?.name || "الأول أساسي",
      total_grade: stats.total_grade || "0",
      total_grade_text: stats.total_grade_text || "",
      grade_label: stats.grade_label || "",
      final_result: stats.final_result || "",
      rank: stats.rank || "1",
      org_name: "مدرسة النصر النموذجية",
      admin_name: "مدير المدرسة",
      vp_name: "مسجل الطلاب",
      issue_date: "2024/2023",
    };
  };

  const previewData = getPreviewData(selectedPreviewStudent);


  const createTemplate = useMutation(
    trpc.certificateTemplates.upsert.mutationOptions({
      onSuccess: () => {
        queryClient.invalidateQueries({
          queryKey: trpc.certificateTemplates.list.queryKey(),
        });
      },
    }),
  );

  const updateTemplate = useMutation(
    trpc.certificateTemplates.upsert.mutationOptions({
      onSuccess: () => {
        queryClient.invalidateQueries({
          queryKey: trpc.certificateTemplates.list.queryKey(),
        });
      },
    }),
  );

  const canvasWidth = orientation === "portrait" ? PAPER_SIZES[paperSize].width : PAPER_SIZES[paperSize].height;
  const canvasHeight = orientation === "portrait" ? PAPER_SIZES[paperSize].height : PAPER_SIZES[paperSize].width;

  const addElement = (type: ElementType, content: string, variableKey?: string) => {
    const newElement: EditorElement = {
      id: `el-${Date.now()}`,
      type,
      content,
      variableKey,
      x: 50,
      y: 50,
      opacity: 1,
      rotation: 0,
      ...(type === "image" ? { width: 20, height: 20 } : 
         type === "shape" ? { width: 30, height: 0.5, backgroundColor: "#000000" } :
         type === "table" ? { 
            width: 80, 
            height: 20, 
            columns: [
              { id: "1", header: "المادة", width: 40 },
              { id: "2", header: "المحصلة", width: 30 },
              { id: "3", header: "المجموع", width: 30 }
            ] 
         } : {
        fontSize: 18,
        fontWeight: "bold",
        color: "#000000",
        align: "right",
        fontFamily: "inherit"
      })
    };
    
    setElements([...elements, newElement]);
    setSelectedElementId(newElement.id);
  };

  const updateElement = (id: string, updates: Partial<EditorElement>) => {
    setElements(elements.map(el => el.id === id ? { ...el, ...updates } : el));
  };

  const deleteElement = (id: string) => {
    setElements(elements.filter(el => el.id !== id));
    setSelectedElementId(null);
  };

  const handleMouseDown = useCallback((e: React.MouseEvent, elementId: string) => {
    if (isPreview) return;
    
    e.stopPropagation();
    setSelectedElementId(elementId);
    setIsDragging(true);
    
    const element = elements.find(el => el.id === elementId);
    if (!element || !canvasRef.current) return;

    const canvas = canvasRef.current;
    const rect = canvas.getBoundingClientRect();
    const elementX = (element.x / 100) * rect.width;
    const elementY = (element.y / 100) * rect.height;
    
    setDragOffset({
      x: e.clientX - rect.left - elementX,
      y: e.clientY - rect.top - elementY,
    });
  }, [isPreview, elements]);

  const handleMouseMove = useCallback((e: React.MouseEvent) => {
    if (!isDragging || !selectedElementId || !canvasRef.current) return;

    const canvas = canvasRef.current;
    const rect = canvas.getBoundingClientRect();
    
    const x = ((e.clientX - rect.left - dragOffset.x) / rect.width) * 100;
    const y = ((e.clientY - rect.top - dragOffset.y) / rect.height) * 100;

    setElements(prev => prev.map(el => 
      el.id === selectedElementId 
        ? { ...el, x: Math.max(0, Math.min(100, x)), y: Math.max(0, Math.min(100, y)) }
        : el
    ));
  }, [isDragging, selectedElementId, dragOffset]);

  const handleMouseUp = useCallback(() => {
    setIsDragging(false);
  }, []);

  useEffect(() => {
    if (isDragging) {
      const handleGlobalMouseUp = () => setIsDragging(false);
      window.addEventListener('mouseup', handleGlobalMouseUp);
      return () => window.removeEventListener('mouseup', handleGlobalMouseUp);
    }
  }, [isDragging]);

  const handleFileUpload = async (e: React.ChangeEvent<HTMLInputElement>, elementType: "background" | "image") => {
    const file = e.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = (event) => {
      const dataUrl = event.target?.result as string;
      if (elementType === "background") {
        setBackgroundImage(dataUrl);
      } else {
        addElement("image", dataUrl);
      }
    };
    reader.readAsDataURL(file);
  };

  const handleSave = async () => {
    setIsSaving(true);
    try {
      const payload = {
        name,
        backgroundImageUrl: backgroundImage,
        headerContent: { elements },
        size: paperSize,
        orientation,
      };
      
      if (mode === "edit" && initialData?.id) {
        await updateTemplate.mutateAsync({
          id: initialData.id,
          ...payload,
        });
        toast({ title: "تم حفظ القالب بنجاح", variant: "success" });
        if (isSheet) setParams(null);
      } else {
        await createTemplate.mutateAsync(payload);
        toast({ title: "تم إنشاء القالب بنجاح", variant: "success" });
        if (isSheet) {
          setParams(null);
        } else {
          router.push("/certificates");
        }
      }
    } catch (e) {
      toast({ title: "فشل حفظ القالب", variant: "destructive" });
    } finally {
      setIsSaving(false);
    }
  };

  const selectedElement = elements.find(el => el.id === selectedElementId);

  const EditorContent = (
    <div className="relative h-full flex w-full overflow-hidden">
      <div className="w-[320px] bg-background border-r flex flex-col z-20 shrink-0">
        <div className="p-6 border-b">
          <div className="flex items-center gap-3 mb-4">
            {!isSheet && (
              <Button variant="ghost" size="icon" onClick={() => router.back()} className="h-8 w-8">
                <ArrowLeft className="w-4 h-4" />
              </Button>
            )}
            {isSheet && (
              <Button variant="ghost" size="icon" onClick={() => setParams(null)} className="h-8 w-8">
                <X className="w-4 h-4" />
              </Button>
            )}
            <div className="flex-1 min-w-0">
              <Input 
                value={name} 
                onChange={(e) => setName(e.target.value)} 
                className="font-semibold text-sm border-0 shadow-none focus-visible:ring-0 px-0 bg-transparent"
                placeholder="اسم القالب..."
                dir="rtl"
              />
            </div>
          </div>
        </div>

        <Tabs defaultValue="add" className="flex-1 flex flex-col overflow-hidden">
          <TabsList className="w-full rounded-none bg-transparent border-b px-6">
            <TabsTrigger value="add" className="text-xs">إضافة</TabsTrigger>
            <TabsTrigger value="layers" className="text-xs">الطبقات</TabsTrigger>
            <TabsTrigger value="settings" className="text-xs">الإعدادات</TabsTrigger>
          </TabsList>
          
          <ScrollArea className="flex-1 p-6" hideScrollbar>
            <TabsContent value="add" className="space-y-6 mt-0">
              <div className="space-y-3">
                <Label className="text-xs text-muted-foreground">الخلفية والشعارات</Label>
                <div className="grid grid-cols-2 gap-3">
                  <div className="relative group overflow-hidden rounded-lg border-2 border-dashed aspect-square hover:border-primary/50 transition-colors cursor-pointer bg-accent/50 flex items-center justify-center">
                    {backgroundImage ? (
                      <img src={backgroundImage} className="w-full h-full object-cover" alt="BG" />
                    ) : (
                      <div className="flex flex-col items-center gap-2">
                        <ImageIcon className="w-5 h-5 text-muted-foreground" />
                        <span className="text-xs text-muted-foreground">الخلفية</span>
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
                    <span className="text-xs text-muted-foreground">شعار</span>
                    <input id="add-logo-input" type="file" className="hidden" onChange={(e) => handleFileUpload(e, "image")} />
                  </Button>
                </div>
              </div>

              <div className="space-y-3">
                <Label className="text-xs text-muted-foreground">المتغيرات الديناميكية</Label>
                <div className="space-y-4">
                  {VARIABLES.map((group) => (
                    <div key={group.group} className="space-y-2">
                      <span className="text-xs font-medium text-muted-foreground/60">{group.group}</span>
                      <div className="space-y-1">
                        {group.items.map((v) => (
                          <Button 
                            key={v.value} 
                            variant="ghost"
                            className="w-full justify-start text-xs h-8"
                            onClick={() => addElement("variable", v.label, v.value)}
                          >
                            <Plus className="w-3 h-3 mr-2" />
                            {v.label}
                          </Button>
                        ))}
                      </div>
                    </div>
                  ))}
                </div>
              </div>

              <div className="space-y-3">
                <Label className="text-xs text-muted-foreground">النصوص والجداول</Label>
                <div className="grid grid-cols-2 gap-2">
                  <Button onClick={() => addElement("text", "نص جديد")} variant="outline" className="text-xs h-9">
                    <Type className="w-3 h-3 mr-2" /> 
                    نص ثابت
                  </Button>
                  <Button onClick={() => addElement("table", "table")} variant="outline" className="text-xs h-9">
                    <TableIcon className="w-3 h-3 mr-2" /> 
                    جدول
                  </Button>
                </div>
              </div>

              <div className="space-y-3">
                <Label className="text-xs text-muted-foreground">الأشكال والخطوط</Label>
                <div className="grid grid-cols-2 gap-2">
                  <Button onClick={() => addElement("shape", "line")} variant="outline" className="text-xs h-9">
                    <Minus className="w-3 h-3 mr-2" /> 
                    خط فاصل
                  </Button>
                  <Button onClick={() => addElement("shape", "box")} variant="outline" className="text-xs h-9">
                    <Square className="w-3 h-3 mr-2" /> 
                    مربع
                  </Button>
                </div>
              </div>


            </TabsContent>

            <TabsContent value="layers" className="space-y-2 mt-0">
              {elements.length === 0 ? (
                <div className="text-center py-12 text-muted-foreground text-sm">
                  <Layers className="w-8 h-8 mx-auto mb-2 opacity-20" />
                  <p className="text-xs">لا توجد طبقات</p>
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
                <Label className="text-xs text-muted-foreground">حجم الورقة</Label>
                <Select value={paperSize} onValueChange={(v: any) => setPaperSize(v)}>
                  <SelectTrigger><SelectValue /></SelectTrigger>
                  <SelectContent>
                    <SelectItem value="a4">A4 (210×297 mm)</SelectItem>
                    <SelectItem value="letter">Letter (8.5×11 in)</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              <div className="space-y-3">
                <Label className="text-xs text-muted-foreground">الاتجاه</Label>
                <Select value={orientation} onValueChange={(v: any) => setOrientation(v)}>
                  <SelectTrigger><SelectValue /></SelectTrigger>
                  <SelectContent>
                    <SelectItem value="portrait">طولي (Portrait)</SelectItem>
                    <SelectItem value="landscape">عرضي (Landscape)</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              {selectedElement && (
                <div className="space-y-4 pt-4 border-t">
                  <Label className="text-xs text-muted-foreground">خصائص العنصر المحدد</Label>
                  
                  {(selectedElement.type === "text" || selectedElement.type === "variable") && (
                    <>
                      <div className="space-y-2">
                        <Label className="text-xs">حجم الخط: {selectedElement.fontSize}px</Label>
                        <Slider 
                          value={[selectedElement.fontSize || 18]} 
                          onValueChange={([v]) => updateElement(selectedElement.id, { fontSize: v })}
                          min={8} max={120} step={1}
                        />
                      </div>

                      <div className="space-y-2">
                        <Label className="text-xs">نوع الخط</Label>
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
                        <Label className="text-xs">المحاذاة</Label>
                        <div className="flex items-center gap-1 bg-accent p-1 rounded-md">
                          <Button variant="ghost" size="icon" className={cn("h-7 flex-1", selectedElement.align === "right" && "bg-background")} onClick={() => updateElement(selectedElement.id, { align: "right" })}><AlignRight className="w-3 h-3" /></Button>
                          <Button variant="ghost" size="icon" className={cn("h-7 flex-1", selectedElement.align === "center" && "bg-background")} onClick={() => updateElement(selectedElement.id, { align: "center" })}><AlignCenter className="w-3 h-3" /></Button>
                          <Button variant="ghost" size="icon" className={cn("h-7 flex-1", selectedElement.align === "left" && "bg-background")} onClick={() => updateElement(selectedElement.id, { align: "left" })}><AlignLeft className="w-3 h-3" /></Button>
                        </div>
                      </div>
                    </>
                  )}

                  {(selectedElement.type === "image" || selectedElement.type === "shape" || selectedElement.type === "table") && (
                    <div className="space-y-2">
                      <Label className="text-xs">حجم العنصر: {selectedElement.width}%</Label>
                      <Slider 
                        value={[selectedElement.width || 20]} 
                        onValueChange={([v]) => updateElement(selectedElement.id, { width: v })}
                        min={5} max={100} step={1}
                      />
                    </div>
                  )}

                  <div className="space-y-2">
                    <Label className="text-xs">اللون</Label>
                    <div className="flex items-center gap-2">
                      <input 
                        type="color" 
                        className="w-10 h-10 rounded border cursor-pointer"
                        value={selectedElement.type === 'shape' ? selectedElement.backgroundColor : selectedElement.color}
                        onChange={(e) => updateElement(selectedElement.id, selectedElement.type === 'shape' ? { backgroundColor: e.target.value } : { color: e.target.value })}
                      />
                      <span className="text-xs font-mono">{(selectedElement.type === 'shape' ? selectedElement.backgroundColor : selectedElement.color)?.toUpperCase()}</span>
                    </div>
                  </div>
                </div>
              )}
            </TabsContent>
          </ScrollArea>

          <AnimatePresence>
            {!selectedElement && (
              <div className="p-6 border-t bg-background">
                <Button onClick={handleSave} disabled={isSaving} className="w-full">
                  {isSaving ? <Loader2 className="w-4 h-4 ml-2 animate-spin" /> : <Save className="w-4 h-4 ml-2" />}
                  {mode === "create" ? "إنشاء القالب" : "حفظ التغييرات"}
                </Button>
              </div>
            )}
          </AnimatePresence>
        </Tabs>
      </div>

      {/* Main Canvas Area (نفس تصميم invoice) */}
      <div className="flex-1 flex flex-col min-w-0 overflow-hidden bg-[#fcfcfc] dark:bg-[#0f0f0f]">
        <div className="h-16 border-b flex items-center justify-between px-6 bg-background shrink-0">
          <div className="flex items-center gap-4">
            <div className="flex items-center gap-2 text-xs text-muted-foreground">
              <Maximize className="w-3 h-3" />
              {paperSize.toUpperCase()} · {orientation === 'portrait' ? 'طولي' : 'عرضي'}
            </div>

            {isPreview && (
              <div className="flex items-center gap-2 border-r pr-4 mr-4" dir="rtl">
                <TableIcon className="w-3 h-3 text-muted-foreground" />
                <Select dir="rtl" value={previewStudentId || students?.[0]?.userId || ""} onValueChange={setPreviewStudentId}>
                  <SelectTrigger className="h-7 w-[200px] text-[10px] bg-accent/50 border-0 shadow-none focus:ring-0">
                    <SelectValue placeholder="اختر سجل للمعاينة..." />
                  </SelectTrigger>
                  <SelectContent className="max-h-[400px]">
                    {students?.map((s: any) => (
                      <SelectItem key={s.userId} value={s.userId} className="text-xs">
                        <div className="flex flex-col items-start gap-0.5" dir="rtl">
                          <span className="font-bold">{s.name || "بدون اسم"}</span>
                          <span className="text-[9px] text-muted-foreground opacity-70">{s.studentNumber}</span>
                        </div>
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
            )}
          </div>


          <div className="flex items-center gap-2">
            <Button 
              variant={!isPreview ? "secondary" : "ghost"} 
              size="sm"
              onClick={() => setIsPreview(false)}
            >
              <Type className="w-4 h-4 mr-2" />
              تحرير
            </Button>
            <Button 
              variant={isPreview ? "secondary" : "ghost"} 
              size="sm"
              onClick={() => setIsPreview(true)}
            >
              <Eye className="w-4 h-4 mr-2" />
              معاينة
            </Button>
          </div>
        </div>

        <div className="flex-1 overflow-hidden p-8 flex items-center justify-center">
          <TransformWrapper
            key={`${orientation}-${paperSize}`}
            initialScale={orientation === "landscape" ? 0.45 : 0.6}
            minScale={0.2}
            maxScale={2}
            centerOnInit
          >
            {({ zoomIn, zoomOut, resetTransform }) => (
              <>
                <div className="absolute bottom-6 left-6 z-30 flex items-center gap-2 bg-background border rounded-lg shadow-lg p-1">
                  <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => zoomOut()}>
                    <ZoomOut className="w-4 h-4" />
                  </Button>
                  <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => resetTransform()}>
                    <RotateCcw className="w-4 h-4" />
                  </Button>
                  <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => zoomIn()}>
                    <ZoomIn className="w-4 h-4" />
                  </Button>
                </div>

                <div className="w-full h-full">
                  <TransformComponent wrapperClass="!w-full !h-full" contentClass="!w-full !h-full flex items-center justify-center">
                    <motion.div
                      ref={canvasRef}
                      layout
                      className={cn(
                        "relative bg-white shadow-2xl origin-center",
                        isDragging && "select-none"
                      )}
                      style={{ width: canvasWidth, height: canvasHeight }}
                      onMouseMove={handleMouseMove}
                      onMouseUp={handleMouseUp}
                      onClick={(e) => {
                        if (e.target === e.currentTarget && !isPreview) {
                          setSelectedElementId(null);
                        }
                      }}
                    >
                      {backgroundImage && (
                        <img src={backgroundImage} className="absolute inset-0 w-full h-full object-cover" alt="Background" />
                      )}

                      <motion.div className="absolute inset-0" layout>
                        {elements.map((el) => {
                          const isSelected = selectedElementId === el.id;
                          const content = isPreview && el.variableKey 
                            ? (previewData[el.variableKey.replace(/[{}]/g, "") as keyof typeof previewData] || el.content)
                            : el.content;


                          return (
                            <div
                              key={el.id}
                              onMouseDown={(e) => handleMouseDown(e, el.id)}
                              onClick={() => !isPreview && setSelectedElementId(el.id)}
                              className={cn(
                                "absolute transition-all",
                                isPreview ? "cursor-default select-text" : "cursor-move select-none",
                                isSelected && "z-50",
                                isDragging && isSelected && "cursor-grabbing"
                              )}
                              style={{
                                left: `${el.x}%`,
                                top: `${el.y}%`,
                                transform: `translate(-50%, -50%) rotate(${el.rotation || 0}deg)`,
                                textAlign: el.align as any,
                                opacity: el.opacity,
                                width: (el.type === "image" || el.type === "shape" || el.type === "table") ? `${el.width}%` : "auto",
                                height: (el.type === "shape") ? `${el.height}px` : "auto",
                              }}
                            >
                              {isSelected && (
                                <div className="absolute -inset-1 ring-2 ring-primary rounded" />
                              )}

                              {el.type === "shape" ? (
                                <div 
                                  className={cn("w-full h-full", el.content === "box" && "border-2")} 
                                  style={{ 
                                    backgroundColor: el.backgroundColor,
                                    borderColor: el.content === "box" ? el.backgroundColor : "transparent",
                                  }} 
                                />
                              ) : el.type === "table" ? (
                                <div className="w-full bg-white border border-black text-xs" dir="rtl">
                                  <div className="grid grid-cols-5 bg-gray-100 border-b border-black font-bold py-1 text-center">
                                    <div className="border-l border-black col-span-2">المادة</div>
                                    <div className="border-l border-black">الشهر 1</div>
                                    <div className="border-l border-black">نصف العام</div>
                                    <div>المجموع</div>
                                  </div>
                                  {[1, 2, 3, 4, 5, 6].map((i) => (
                                    <div key={i} className="grid grid-cols-5 border-b border-gray-200 text-xs py-1 text-center">
                                      <div className="border-l border-gray-200 col-span-2 text-right pr-2">مادة {i}</div>
                                      <div className="border-l border-gray-200">20</div>
                                      <div className="border-l border-gray-200">30</div>
                                      <div>50</div>
                                    </div>
                                  ))}
                                  <div className="grid grid-cols-5 bg-gray-50 font-bold py-1 text-center border-t border-black">
                                    <div className="col-span-2 text-right pr-2">المجموع</div>
                                    <div className="col-span-3">480 / 500</div>
                                  </div>
                                </div>
                              ) : el.type === "image" ? (
                                <img src={content} className="w-full h-auto shadow-sm" draggable={false} alt="asset" />

                              ) : (
                                <div 
                                  className={cn(
                                    "whitespace-nowrap font-bold",
                                    el.variableKey && !isPreview && "px-3 py-1 bg-primary/10 text-primary border border-primary/20 rounded-lg flex items-center gap-2"
                                  )}
                                  style={{
                                    fontSize: `${el.fontSize}px`,
                                    fontWeight: el.fontWeight,
                                    color: el.color,
                                    fontFamily: el.fontFamily,
                                  }}
                                >
                                  {el.variableKey && !isPreview && <Sparkles className="w-3 h-3" />}
                                  {content}
                                </div>
                              )}
                            </div>
                          );
                        })}
                      </motion.div>
                    </motion.div>
                  </TransformComponent>
                </div>
              </>
            )}
          </TransformWrapper>
        </div>
      </div>
    </div>
  );

  if (isSheet) {
    return (
      <SheetContent
        style={{ maxWidth: 1400 }}
        className="bg-white dark:bg-[#080808] p-0 [&>button]:hidden"
      >
        {EditorContent}
      </SheetContent>
    );
  }

  return EditorContent;
}
