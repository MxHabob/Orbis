"use client";

import { useSearchParams } from "next/navigation";
import { useTRPC } from "@/trpc/client";
import { CertificateViewer } from "@/components/certificates/certificate-viewer";
import { Spinner } from "@orbis/ui/spinner";
import { useEffect, useState, useMemo, useCallback, Suspense } from "react";
import { useQuery } from "@tanstack/react-query";

/**
 * Bulk Export Page for Puppeteer (2026 - Production Grade)
 *
 * This page is rendered headlessly by Puppeteer for bulk certificate generation.
 * Each certificate gets its own page via CSS `break-before: page`.
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
  
  // Resolve image path: if it's just a filename, prepend the standard student folder
  let resolvedImage = student.image || "";
  if (resolvedImage && !resolvedImage.startsWith('http') && !resolvedImage.includes('/')) {
    resolvedImage = `students/${student.userId}/${resolvedImage}`;
  }

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
    rank: stats.rank || "",
    org_name: "مدرسة صنعاء النموذجية",
    admin_name: stats.admin_name || "مدير المدرسة",
    vp_name: stats.vp_name || "وكيل المدرسة",
    issue_date: new Date().toLocaleDateString('ar-YE', { year: 'numeric', month: 'long', day: 'numeric' }),
    report_type: type === "half-yearly" ? "النصفي" : (type === "final" ? "النهائي" : `الشهري - لشهر ${month ? (HIJRI_MONTHS_NAMES[month] || `شهر ${month}`) : ""}`),
    month_name: month ? (HIJRI_MONTHS_NAMES[month] || `شهر ${month}`) : "",
    academic_year: currentYear,
    studentId: student.userId,
    student_image: resolvedImage,
  };
};

export default function BulkExportCertificatePage() {
  return (
    <Suspense fallback={
      <div className="flex flex-col items-center justify-center min-h-screen bg-white">
        <Spinner className="w-8 h-8 opacity-20" />
        <span className="mt-4 text-xs font-bold uppercase tracking-widest opacity-40">Initializing PDF Pipeline...</span>
      </div>
    }>
      <BulkExportContent />
    </Suspense>
  );
}

function BulkExportContent() {
  const searchParams = useSearchParams();
  const studentIds = useMemo(
    () => searchParams.get("studentIds")?.split(",").filter(Boolean) || [],
    [searchParams]
  );
  const templateId = searchParams.get("templateId");
  const type = searchParams.get("type") || "monthly";
  const month = parseInt(searchParams.get("month") || "1");
  const trpc = useTRPC();

  const { data: students, isLoading: loadingStudents } = useQuery(
    trpc.students.getByUserIds.queryOptions(
      { userIds: studentIds },
      { enabled: studentIds.length > 0 }
    ));

  const { data: certificateTemplate, isLoading: loadingTemplate } = useQuery(
    trpc.certificateTemplates.getById.queryOptions(
      { id: templateId! },
      { enabled: !!templateId }
    ));

  const [finishedPages, setFinishedPages] = useState<Set<number>>(new Set());
  const rankingPeriod = type === "half-yearly" ? "half-yearly" : type === "final" ? "final" : "monthly";
  const rankingMonth = rankingPeriod === "monthly" ? month : undefined;
  
  const classId = students?.[0]?.currentClass?.id;
  const academicYearId = students?.[0]?.currentClass?.academicYearId;

  const { data: topStudents } = useQuery(
    trpc.grades.getTopStudents.queryOptions(
      {
        classId: classId || 0,
        academicYearId: academicYearId || 0,
        period: rankingPeriod as "monthly" | "half-yearly" | "final",
        month: rankingMonth,
        limit: 10,
      },
      {
        enabled: !!classId && !!academicYearId && (rankingPeriod !== "monthly" || !!rankingMonth),
      }
    )
  );

  const canvasConfig = (certificateTemplate?.content as any)?.canvasConfig;
  const layoutConfig = canvasConfig?.layoutConfig || (certificateTemplate?.content as any)?.layoutConfig || { rows: 1, columns: 1, gap: 0 };
  const totalSlots = layoutConfig.rows * layoutConfig.columns;

  const studentChunks = useMemo(() => {
    if (!students) return [];
    const chunks = [];
    for (let i = 0; i < students.length; i += totalSlots) {
       chunks.push(students.slice(i, i + totalSlots));
    }
    return chunks;
  }, [students, totalSlots]);

  // Readiness logic (Production Grade 2026)
  useEffect(() => {
    if (studentChunks.length > 0 && finishedPages.size >= studentChunks.length) {
      requestAnimationFrame(() => {
        requestAnimationFrame(() => {
          const delay = Math.max(5000, studentChunks.length * 300);
          const timer = setTimeout(() => {
            (window as any).isReadyToCapture = true;
            console.info(`[Export] PDF Pipeline Ready. Final delay: ${delay}ms`);
          }, delay);
          return () => clearTimeout(timer);
        });
      });
    }
  }, [finishedPages, studentChunks]);

  const handlePageReady = useCallback((index: number) => {
    setFinishedPages(prev => new Set([...prev, index]));
  }, []);

  const isLoading = loadingStudents || loadingTemplate || !students || !certificateTemplate;

  if (isLoading) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen bg-white">
        <Spinner className="w-8 h-8 opacity-20" />
        <span className="mt-4 text-xs font-bold uppercase tracking-widest opacity-40">Initializing PDF Pipeline...</span>
      </div>
    );
  }

  return (
    <div className="bg-white p-0 m-0 overflow-hidden">
      <div id="pdf-metadata" style={{ display: 'none' }} data-total-pages={studentChunks.length} />

      {studentChunks.map((chunk, chunkIndex) => (
        <div
          key={`sheet-${chunkIndex}`}
          className={`${chunkIndex > 0 ? "break-before-page" : ""} flex justify-center`}
        >
          <CertificateViewer
            template={certificateTemplate as any}
            data={chunk.map(student => {
                const rankEntry = topStudents?.find((item: any) => item.studentId === student.userId);
                return {
                    ...mapStudentToVariables(student, type, month),
                    rank: rankEntry?.rankLabel || ((student.stats as any)?.rank || ""),
                    rank_percentage: rankEntry?.percentage ?? "",
                    rank_total: rankEntry?.totalScore ?? "",
                    rank_max: rankEntry?.maxScore ?? "",
                };
            })}
            certificateType={type}
            month={month}
            preview={false}
            className="shadow-none border-none outline-none"
            onLoad={() => handlePageReady(chunkIndex)}
          />
        </div>
      ))}

      <style jsx global>{`
        * {
          animation: none !important;
          transition: none !important;
          print-color-adjust: exact !important;
          -webkit-print-color-adjust: exact !important;
        }

        @page { margin: 0 !important; }

        @media print {
          .break-before-page {
            break-before: page;
            page-break-before: always;
          }
        }

        html, body {
          background-color: white !important;
          margin: 0 !important;
          padding: 0 !important;
          width: 100%;
          direction: rtl;
        }
      `}</style>
    </div>
  );
}