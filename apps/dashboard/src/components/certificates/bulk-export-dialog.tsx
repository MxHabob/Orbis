"use client";

import { useState, useEffect } from "react";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from "@midday/ui/dialog";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@midday/ui/select";
import { Button } from "@midday/ui/button";
import { Label } from "@midday/ui/label";
import { useTRPC } from "@/trpc/client";
import { Progress } from "@midday/ui/progress";
import { Check, Download, Loader2, FileText, AlertCircle } from "lucide-react";
import { toast } from "@midday/ui/use-toast";
import { cn } from "@midday/utils";

interface BulkExportDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

const MONTHS = [
  { value: "1", label: "سبتمبر (1)" },
  { value: "2", label: "أكتوبر (2)" },
  { value: "3", label: "نوفمبر (3)" },
  { value: "4", label: "ديسمبر (4)" },
  { value: "5", label: "يناير (5)" },
  { value: "6", label: "فبراير (6)" },
  { value: "7", label: "مارس (7)" },
  { value: "8", label: "أبريل (8)" },
  { value: "9", label: "مايو (9)" },
  { value: "10", label: "يونيو (10)" },
];

const CERT_TYPES = [
  { value: "monthly", label: "شهادة شهرية" },
  { value: "midterm_1", label: "اختبار منتصف الفصل الأول" },
  { value: "final_1", label: "نهاية الفصل الأول" },
  { value: "midterm_2", label: "اختبار منتصف الفصل الثاني" },
  { value: "final_2", label: "نهاية العام (الفصل الثاني)" },
];

export function BulkExportDialog({ open, onOpenChange }: BulkExportDialogProps) {
  const trpc = useTRPC();
  
  // Form State
  const [selectedClassId, setSelectedClassId] = useState<string>("");
  const [selectedTemplateId, setSelectedTemplateId] = useState<string>("");
  const [selectedMonth, setSelectedMonth] = useState<string>("1");
  const [certificateType, setCertificateType] = useState<string>("monthly");
  
  // Job State
  const [jobId, setJobId] = useState<string | null>(null);
  const [progress, setProgress] = useState<number>(0);
  const [status, setStatus] = useState<"idle" | "processing" | "completed" | "failed">("idle");
  const [result, setResult] = useState<any>(null);

  // Queries
  const { data: classesResponse } = trpc.classes.getAll.useQuery();
  const classes = classesResponse?.data;
  const { data: templates } = trpc.certificateTemplates.list.useQuery();
  
  // Mutations
  const startExport = trpc.certificateTemplates.startExport.useMutation({
    onSuccess: (data) => {
      setJobId(data.id);
      setStatus("processing");
    },
    onError: (error) => {
      toast({
        title: "خطأ في بدء التصدير",
        description: error.message,
        variant: "destructive",
      });
      setStatus("idle");
    },
  });

  // Polling for job status
  const jobStatus = trpc.jobs.getStatus.useQuery(
    { jobId: jobId! },
    {
      enabled: !!jobId && status === "processing",
      refetchInterval: (query) => {
        const data = query.state.data;
        if (data?.status === "completed" || data?.status === "failed") {
          return false;
        }
        return 2000;
      },
    }
  );

  useEffect(() => {
    if (jobStatus.data) {
      setProgress(jobStatus.data.progress || 0);
      
      if (jobStatus.data.status === "completed") {
        setStatus("completed");
        setResult(jobStatus.data.result);
        toast({
          title: "تم توليد الشهائد بنجاح",
          description: "ملف PDF جاهز للتحميل",
        });
      } else if (jobStatus.data.status === "failed") {
        setStatus("failed");
        toast({
          title: "فشل التصدير",
          description: jobStatus.data.error || "حدث خطأ غير معروف",
          variant: "destructive",
        });
      }
    }
  }, [jobStatus.data]);

  const handleStart = () => {
    if (!selectedClassId) {
      toast({ title: "يرجى اختيار الفصل", variant: "destructive" });
      return;
    }
    
    startExport.mutate({
      classIds: [selectedClassId],
      templateId: selectedTemplateId || undefined,
      certificateType,
      selectedMonth: parseInt(selectedMonth),
    });
  };

  const reset = () => {
    setJobId(null);
    setProgress(0);
    setStatus("idle");
    setResult(null);
  };

  const handleOpenChange = (newOpen: boolean) => {
    if (!newOpen && status === "processing") return; // Prevent closing while processing
    onOpenChange(newOpen);
    if (!newOpen) setTimeout(reset, 500);
  };

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle className="text-right">توليد الشهائد الجماعي</DialogTitle>
          <DialogDescription className="text-right">
            اختر الفصل والشهر لتوليد ملف PDF واحد يحتوي على جميع الشهائد.
          </DialogDescription>
        </DialogHeader>

        <div className="grid gap-4 py-4 direction-rtl" dir="rtl">
          {status === "idle" ? (
            <>
              <div className="grid gap-2">
                <Label htmlFor="class">الفصل الدراسي</Label>
                <Select value={selectedClassId} onValueChange={setSelectedClassId}>
                  <SelectTrigger id="class">
                    <SelectValue placeholder="اختر الفصل" />
                  </SelectTrigger>
                  <SelectContent>
                    {classes?.map((c) => (
                      <SelectItem key={c.id} value={c.id.toString()}>
                        {c.name}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>

              <div className="grid gap-2">
                <Label htmlFor="template">قالب الشهادة</Label>
                <Select value={selectedTemplateId} onValueChange={setSelectedTemplateId}>
                  <SelectTrigger id="template">
                    <SelectValue placeholder="القالب الافتراضي" />
                  </SelectTrigger>
                  <SelectContent>
                    {templates?.map((t) => (
                      <SelectItem key={t.id} value={t.id}>
                        {t.name} {t.isDefault ? "(افتراضي)" : ""}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>

              <div className="grid grid-cols-2 gap-4">
                <div className="grid gap-2">
                  <Label htmlFor="type">النوع</Label>
                  <Select value={certificateType} onValueChange={setCertificateType}>
                    <SelectTrigger id="type">
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      {CERT_TYPES.map((t) => (
                        <SelectItem key={t.value} value={t.value}>
                          {t.label}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
                <div className="grid gap-2">
                  <Label htmlFor="month">الشهر</Label>
                  <Select 
                    value={selectedMonth} 
                    onValueChange={setSelectedMonth}
                    disabled={certificateType !== "monthly"}
                  >
                    <SelectTrigger id="month">
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      {MONTHS.map((m) => (
                        <SelectItem key={m.value} value={m.value}>
                          {m.label}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
              </div>
            </>
          ) : (
            <div className="flex flex-col items-center justify-center py-8 space-y-6">
              {status === "processing" && (
                <>
                  <div className="relative flex items-center justify-center">
                    <Loader2 className="h-12 w-12 text-primary animate-spin" />
                    <span className="absolute text-[10px] font-bold">{progress}%</span>
                  </div>
                  <div className="w-full space-y-2">
                    <Progress value={progress} className="h-2" />
                    <p className="text-center text-sm text-muted-foreground">
                      جاري توليد ملف الـ PDF... قد يستغرق ذلك بضع دقائق
                    </p>
                  </div>
                </>
              )}

              {status === "completed" && (
                <>
                  <div className="h-16 w-16 bg-green-100 rounded-full flex items-center justify-center">
                    <Check className="h-8 w-8 text-green-600" />
                  </div>
                  <div className="text-center space-y-2">
                    <h3 className="font-bold text-lg">اكتملت العملية!</h3>
                    <p className="text-sm text-muted-foreground">
                      تم توليد {result?.totalItems || "جميع"} الشهائد بنجاح.
                    </p>
                  </div>
                  <Button asChild className="w-full" variant="default">
                    <a href={result?.url} download={result?.fileName}>
                      <Download className="ml-2 h-4 w-4" />
                      تحميل الملف الآن
                    </a>
                  </Button>
                </>
              )}

              {status === "failed" && (
                <>
                  <div className="h-16 w-16 bg-red-100 rounded-full flex items-center justify-center">
                    <AlertCircle className="h-8 w-8 text-red-600" />
                  </div>
                  <div className="text-center space-y-2">
                    <h3 className="font-bold text-lg text-red-600">فشلت العملية</h3>
                    <p className="text-xs text-muted-foreground max-w-[250px]">
                      {jobStatus.data?.error || "حدث خطأ غير متوقع أثناء معالجة الشهائد."}
                    </p>
                  </div>
                  <Button onClick={reset} variant="outline" className="w-full">
                    حاول مرة أخرى
                  </Button>
                </>
              )}
            </div>
          )}
        </div>

        <DialogFooter className="sm:justify-start">
          {status === "idle" && (
            <Button 
              type="button" 
              className="w-full" 
              onClick={handleStart}
              disabled={startExport.isPending}
            >
              {startExport.isPending ? (
                <>
                  <Loader2 className="ml-2 h-4 w-4 animate-spin" />
                  جاري البدء...
                </>
              ) : (
                <>
                  <FileText className="ml-2 h-4 w-4" />
                  بدء التوليد
                </>
              )}
            </Button>
          )}
          {status === "completed" && (
            <Button variant="outline" className="w-full" onClick={() => handleOpenChange(false)}>
              إغلاق
            </Button>
          )}
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
