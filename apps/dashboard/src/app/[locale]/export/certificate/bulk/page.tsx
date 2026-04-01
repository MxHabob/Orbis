"use client";

import { useSearchParams } from "next/navigation";
import { useTRPC } from "@/trpc/client";
import { CertificateViewer } from "@/components/certificates/certificate-viewer";
import { Spinner } from "@midday/ui/spinner";
import { useEffect, useState, useMemo, useCallback } from "react";
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
    report_type: type === "half-yearly" ? "النصفي" : (type === "final" ? "النهائي" : `شهري - لشهر ${month ? (HIJRI_MONTHS_NAMES[month] || `شهر ${month}`) : ""}`),
    month_name: month ? (HIJRI_MONTHS_NAMES[month] || `شهر ${month}`) : "",
    academic_year: currentYear,
    studentId: student.userId,
  };
};

export default function BulkExportCertificatePage() {
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

  const { data: template, isLoading: loadingTemplate } = useQuery(
    trpc.certificateTemplates.getById.queryOptions(
      { id: templateId! },
      { enabled: !!templateId }
    ));

  const [loadedCount, setLoadedCount] = useState(0);
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

  useEffect(() => {
    if (students && students.length > 0 && loadedCount >= students.length) {
      setTimeout(() => {
        (window as any).isReadyToCapture = true;
      }, 500); // Small wait for styles to settle
    }
  }, [loadedCount, students]);

  const handleCertificateLoad = useCallback(() => {
    setLoadedCount((prev) => prev + 1);
  }, []);

  const isLoading = loadingStudents || loadingTemplate || !students || !template;

  if (isLoading) {
    return (
      <div className="flex items-center justify-center min-h-screen bg-white">
        <Spinner />
        <span className="mr-2">جاري تجهيز البيانات...</span>
      </div>
    );
  }

  return (
    <div className="bg-white p-0 m-0">
      {students.map((student, index) => (
        <div
          key={student.userId}
          className={`${index > 0 ? "break-before-page" : ""} flex justify-center`}
        >
          {(() => {
            const rankEntry = topStudents?.find((item: any) => item.studentId === student.userId);
            return (
          <CertificateViewer
            template={template as any}
            data={{
              ...mapStudentToVariables(student, type, month),
              rank: rankEntry?.rankLabel || ((student.stats as any)?.rank || ""),
              rank_percentage: rankEntry?.percentage ?? "",
              rank_total: rankEntry?.totalScore ?? "",
              rank_max: rankEntry?.maxScore ?? "",
            }}
            certificateType={type}
            month={month}
            preview={false}
            className="shadow-none border-none"
            onLoad={handleCertificateLoad}
          />
            );
          })()}
        </div>
      ))}

      <style jsx global>{`
        * {
          animation: none !important;
          transition: none !important;
        }

        @page {
          margin: 0 !important;
          size: auto;
        }

        @media print {
          .break-before-page {
            break-before: page;
          }
          * {
            -webkit-print-color-adjust: exact !important;
            print-color-adjust: exact !important;
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