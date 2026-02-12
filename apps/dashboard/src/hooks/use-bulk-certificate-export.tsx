"use client";

import { useCallback } from "react";
import { useTRPC } from "@/trpc/client";
import { useToast } from "@midday/ui/use-toast";
import { useExportStore } from "@/store/export";
import { generateBulkPdfFromElements, createOffscreenContainer } from "@midday/export";
import { createRoot } from "react-dom/client";
import { CertificateViewer } from "@/components/certificates/certificate-viewer";
import { TRPCProvider } from "@/trpc/client";
import { QueryClient, QueryClientProvider, useQueryClient } from "@tanstack/react-query";
import { createTRPCClient, httpBatchLink } from "@trpc/client";
import superjson from "superjson";
import { AppRouter } from "@midday/api/trpc/routers/_app";

interface ExportConfig {
  classIds: string[];
  templateId?: string;
  certificateType: string;
  selectedMonth: number;
}

export function useBulkCertificateExport() {
  const trpc = useTRPC();
  const queryClient = useQueryClient(); // استخدام QueryClient من TanStack
  const { toast } = useToast();
  const { setExportData, setIsExporting, setProgress, setStatus } = useExportStore();

  const exportCertificates = useCallback(
    async (config: ExportConfig) => {
      const { classIds, templateId, certificateType, selectedMonth } = config;
      
      setIsExporting(true);
      setStatus("processing");
      setProgress(5);

      try {
        // 1. Fetch Template using TanStack Query Client
        // استخدام queryOptions يضمن توافق tRPC مع TanStack Query بشكل كامل
        const template = await queryClient.fetchQuery(
          trpc.certificateTemplates.getById.queryOptions({ id: templateId! })
        );

        if (!template) throw new Error("لم يتم العثور على القالب");
        
        setProgress(10);

        // 2. Fetch All Students for all classes
        // تنفيذ الطلبات بالتوازي باستخدام fetchQuery
        const studentPromises = classIds.map(id => 
          queryClient.fetchQuery(
            trpc.students.getByClassId.queryOptions({ id, limit: 1000 })
          )
        );
        
        const studentResults = await Promise.all(studentPromises);
        const students = studentResults.flatMap(res => res.data);

        if (students.length === 0) {
          throw new Error("لا يوجد طلاب في الفصول المختارة");
        }

        setProgress(15);

        // 3. Setup Offscreen Environment
        const { container, cleanup } = createOffscreenContainer();
        
        // إنشاء QueryClient جديد للبيئة المعزولة للتصدير
        const offscreenQueryClient = new QueryClient({
          defaultOptions: {
            queries: {
              staleTime: 1000 * 60,
              retry: false,
            },
          },
        });

        // إنشاء عميل tRPC متوافق مع البيئة المعزولة
        const trpcClient = createTRPCClient<AppRouter>({
          links: [
            httpBatchLink({
              url: `${process.env.NEXT_PUBLIC_API_URL || "http://localhost:3002"}/trpc`,
              transformer: superjson,
              fetch(url, options) {
                return fetch(url, {
                  ...options,
                  credentials: "include",
                });
              },
            }),
          ],
        });

        const root = createRoot(container);
        
        const mapStudentToVariables = (student: any) => ({
          ...student,
          studentName: student.name,
          className: student.currentClass?.name || "",
          academicYear: student.yearName || "", 
        });

        // 4. Rendering React tree
        root.render(
          <QueryClientProvider client={offscreenQueryClient}>
            <TRPCProvider trpcClient={trpcClient} queryClient={offscreenQueryClient}>
              <div id="bulk-export-wrapper" style={{ direction: 'rtl' }}>
                {students.map((student) => (
                  <div 
                    key={student.userId} 
                    className="certificate-item" 
                    style={{ marginBottom: '100px', display: 'inline-block' }}
                  >
                    <CertificateViewer
                      template={template as any}
                      data={mapStudentToVariables(student)}
                      certificateType={certificateType}
                      month={selectedMonth}
                    />
                  </div>
                ))}
              </div>
            </TRPCProvider>
          </QueryClientProvider>
        );

        // 5. الانتظار لتحميل البيانات الداخلية للشهادات
        for (let i = 0; i < 8; i++) {
          await new Promise(resolve => setTimeout(resolve, 500));
          setProgress(15 + (i + 1) * 4); 
        }

        const renderedElements = Array.from(container.querySelectorAll(".certificate-item")) as HTMLElement[];
        
        if (renderedElements.length === 0) {
          throw new Error("فشل في تهيئة القوالب للطباعة");
        }

        setProgress(50);

        // 6. PDF Generation
        const blob = await generateBulkPdfFromElements(renderedElements, {
          orientation: (template.orientation as any) || "portrait",
          quality: 0.95,
          scale: 2,
          onProgress: (current, total) => {
            const p = 50 + Math.round((current / total) * 45);
            setProgress(p);
          }
        });

        // 7. Instant Download
        const url = URL.createObjectURL(blob);
        const link = document.createElement("a");
        link.href = url;
        link.download = `شهادات_${template.name}_${new Date().toLocaleDateString('ar-SA').replace(/\//g, '-')}.pdf`;
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
        URL.revokeObjectURL(url);

        setProgress(100);
        setStatus("completed");
        
        setTimeout(() => {
          setIsExporting(false);
          setStatus("idle");
          setProgress(0);
          cleanup();
          root.unmount();
        }, 1500);

        return { success: true };
      } catch (error: any) {
        console.error("Export error:", error);
        toast({
          title: "فشل التصدير",
          description: error.message || "حدث خطأ غير متوقع",
          variant: "error",
        });
        setStatus("failed");
        setIsExporting(false);
        return { success: false };
      }
    },
    [trpc, queryClient, toast, setIsExporting, setProgress, setStatus]
  );

  return { 
    exportCertificates, 
    isLoading: false
  };
}
