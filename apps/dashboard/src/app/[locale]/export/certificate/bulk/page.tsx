"use client";

import { useSearchParams } from "next/navigation";
import { useTRPC } from "@/trpc/client";
import { CertificateViewer } from "@/components/certificates/certificate-viewer";
import { Spinner } from "@midday/ui/spinner";
import { useEffect, useState, useMemo, useCallback, useRef } from "react";
import { useQuery } from "@tanstack/react-query";

/**
 * Bulk Export Page for Puppeteer (2026 - Production Grade)
 *
 * This page supports TWO modes:
 * 1. INJECTED MODE (mode=injected): Worker pre-fetches data from DB and injects
 *    it into window.__EXPORT_DATA__. No tRPC calls needed. No auth required.
 * 2. FALLBACK MODE: Uses tRPC to fetch data (for testing in browser).
 *
 * The injected mode completely bypasses tRPC authentication issues that
 * were causing blank PDF pages.
 */

const HIJRI_MONTHS_NAMES: Record<number, string> = {
  1: "محرم", 2: "صفر", 3: "ربيع أول", 4: "ربيع ثاني",
  5: "جمادى أول", 6: "جمادى ثاني", 7: "رجب", 8: "شعبان",
  9: "رمضان", 10: "شوال", 11: "ذو القعدة", 12: "ذو الحجة",
};

const mapStudentToVariables = (student: any, type?: string, month?: number) => {
  if (!student) return {};

  const stats = (student.stats as any) || {};
  const currentYear = student.currentClass?.yearName || "2024-2025";
  
  return {
    ...student,
    student_name: student.name || "",
    name: student.name || "",
    academic_number: student.studentNumber || "",
    national_id: student.secretNumber || "",
    department: stats.department || "",
    section: stats.section || "",
    class_name: student.currentClass?.name || "",
    total_grade: stats.total_grade || "",
    total_grade_text: stats.total_grade_text || "",
    grade_label: stats.grade_label || "",
    final_result: stats.final_result || "",
    rank: stats.rank || "1",
    org_name: "مدرسة صنعاء النموذجية",
    admin_name: stats.admin_name || "مدير المدرسة",
    vp_name: stats.vp_name || "وكيل المدرسة",
    issue_date: new Date().toLocaleDateString('ar-YE', { year: 'numeric', month: 'long', day: 'numeric' }),
    report_type: type === "half-yearly" ? "النصفي" : (type === "final" ? "النهائي" : `شهري - لشهر ${month ? (HIJRI_MONTHS_NAMES[month] || `شهر ${month}`) : ""}`),
    month_name: month ? (HIJRI_MONTHS_NAMES[month] || `شهر ${month}`) : "",
    academic_year: currentYear,
    studentId: student.userId,
  };
};

/**
 * Wait for all images on the page to finish loading.
 */
function waitForAllImages(): Promise<void> {
  return new Promise((resolve) => {
    const images = document.querySelectorAll("img");
    if (images.length === 0) {
      resolve();
      return;
    }

    let loaded = 0;
    const total = images.length;

    const checkDone = () => {
      loaded++;
      if (loaded >= total) resolve();
    };

    images.forEach((img) => {
      if (img.complete && img.naturalHeight > 0) {
        checkDone();
      } else {
        img.addEventListener("load", checkDone, { once: true });
        img.addEventListener("error", checkDone, { once: true });
      }
    });

    // Safety timeout
    setTimeout(() => resolve(), 15000);
  });
}

/**
 * Get injected data from window.__EXPORT_DATA__ (set by Puppeteer worker)
 */
function getInjectedData(): {
  students: any[];
  template: any;
  certificateType: string;
  selectedMonth: number;
} | null {
  if (typeof window !== "undefined" && (window as any).__EXPORT_DATA__) {
    return (window as any).__EXPORT_DATA__;
  }
  return null;
}

export default function BulkExportCertificatePage() {
  const searchParams = useSearchParams();
  const isInjectedMode = searchParams.get("mode") === "injected";
  const readySignalSent = useRef(false);

  // ─── INJECTED MODE: Use data from window.__EXPORT_DATA__ ───
  const [injectedData, setInjectedData] = useState<any>(null);

  useEffect(() => {
    if (isInjectedMode) {
      const data = getInjectedData();
      if (data) {
        console.log(`[ExportPage] Injected mode: ${data.students?.length} students, template: ${data.template?.name}`);
        setInjectedData(data);
      } else {
        console.error("[ExportPage] Injected mode but no data found on window.__EXPORT_DATA__");
      }
    }
  }, [isInjectedMode]);

  // ─── FALLBACK MODE: Use tRPC (for browser testing) ───
  const studentIds = useMemo(
    () => isInjectedMode ? [] : (searchParams.get("studentIds")?.split(",").filter(Boolean) || []),
    [searchParams, isInjectedMode]
  );
  const templateId = searchParams.get("templateId");
  const fallbackType = searchParams.get("type") || "monthly";
  const fallbackMonth = parseInt(searchParams.get("month") || "1");
  const trpc = useTRPC();

  const { data: trpcStudents, isLoading: loadingStudents } = useQuery(
    trpc.students.getByUserIds.queryOptions(
      { userIds: studentIds },
      { enabled: !isInjectedMode && studentIds.length > 0 }
    ));

  const { data: trpcTemplate, isLoading: loadingTemplate } = useQuery(
    trpc.certificateTemplates.getById.queryOptions(
      { id: templateId! },
      { enabled: !isInjectedMode && !!templateId }
    ));

  // ─── Resolve final data source ───
  const students = isInjectedMode ? injectedData?.students : trpcStudents;
  const template = isInjectedMode ? injectedData?.template : trpcTemplate;
  const type = isInjectedMode ? (injectedData?.certificateType || "monthly") : fallbackType;
  const month = isInjectedMode ? (injectedData?.selectedMonth || 1) : fallbackMonth;

  const [loadedCount, setLoadedCount] = useState(0);

  // Signal Puppeteer when ALL resources are ready
  useEffect(() => {
    if (readySignalSent.current) return;
    if (!students || students.length === 0) return;
    if (loadedCount < students.length) return;

    readySignalSent.current = true;
    console.log(`[ExportPage] All ${students.length} certificates loaded. Waiting for images and fonts...`);

    Promise.all([
      waitForAllImages(),
      document.fonts ? document.fonts.ready : Promise.resolve(),
    ])
      .then(() => {
        console.log("[ExportPage] All resources loaded. Signaling ready.");
        setTimeout(() => {
          (window as any).isReadyToCapture = true;
          console.log("[ExportPage] ✅ isReadyToCapture = true");
        }, 1000);
      })
      .catch(() => {
        console.warn("[ExportPage] Resource loading error, proceeding anyway.");
        (window as any).isReadyToCapture = true;
      });
  }, [loadedCount, students]);

  const handleCertificateLoad = useCallback(() => {
    setLoadedCount((prev) => prev + 1);
  }, []);

  // ─── Loading state ───
  const isLoading = isInjectedMode
    ? !injectedData
    : (loadingStudents || loadingTemplate || !students || !template);

  // Compute page dimensions from template for CSS @page
  const size = (template?.size || "a4") as string;
  const orientation = template?.orientation || "portrait";

  const PAPER_MM: Record<string, { w: number; h: number }> = {
    a4: { w: 210, h: 297 },
    a5: { w: 148, h: 210 },
    letter: { w: 216, h: 279 },
  };
  const dims = PAPER_MM[size] || PAPER_MM.a4!;
  const pageW = orientation === "portrait" ? dims.w : dims.h;
  const pageH = orientation === "portrait" ? dims.h : dims.w;

  if (isLoading) {
    return (
      <div className="flex items-center justify-center min-h-screen bg-white">
        <Spinner />
        <span className="mr-2">جاري تجهيز البيانات...</span>
      </div>
    );
  }

  if (!students || students.length === 0 || !template) {
    return (
      <div className="flex items-center justify-center min-h-screen bg-white text-red-500">
        <span>لا توجد بيانات للتصدير</span>
      </div>
    );
  }

  return (
    <div className="bg-white p-0 m-0">
      {students.map((student: any, index: number) => (
        <div
          key={student.userId || index}
          className={`${index > 0 ? "break-before-page" : ""}`}
          style={{
            display: "flex",
            justifyContent: "center",
            alignItems: "flex-start",
            width: `${pageW}mm`,
            minHeight: `${pageH}mm`,
            margin: "0 auto",
            padding: 0,
            boxSizing: "border-box",
            overflow: "hidden",
          }}
        >
          <CertificateViewer
            template={template as any}
            data={mapStudentToVariables(student, type, month)}
            certificateType={type}
            month={month}
            preview={false}
            className="shadow-none border-none"
            onLoad={handleCertificateLoad}
          />
        </div>
      ))}

      <style jsx global>{`
        * {
          animation: none !important;
          transition: none !important;
        }

        @page {
          margin: 0 !important;
          padding: 0 !important;
          size: ${pageW}mm ${pageH}mm;
        }

        @media print {
          .break-before-page {
            break-before: page;
          }
          * {
            -webkit-print-color-adjust: exact !important;
            print-color-adjust: exact !important;
            color-adjust: exact !important;
          }
        }

        html, body {
          background-color: white !important;
          margin: 0 !important;
          padding: 0 !important;
          width: 100%;
          direction: rtl;
        }

        img {
          image-rendering: auto;
          -webkit-print-color-adjust: exact !important;
          print-color-adjust: exact !important;
        }
      `}</style>
    </div>
  );
}
