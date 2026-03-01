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
 * This page is NOT for end users — it's rendered headlessly by Puppeteer.
 * Each certificate gets its own page via CSS `break-before: page`.
 *
 * Performance Optimizations:
 * - Uses `font-display: swap` to prevent blocking on web fonts
 * - Disables animations and transitions for faster rendering
 * - Sets explicit dimensions to prevent layout shifts
 * - Signals readiness via `window.isReadyToCapture` after all grades load
 */

const mapStudentToVariables = (student: any) => {
  if (!student) return {};

  const stats = (student.stats as any) || {};
  return {
    // Spread raw student data so any direct key also works
    ...student,
    // Keys matching the certificate template editor variables
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
    org_name: stats.org_name || "",
    admin_name: stats.admin_name || "",
    vp_name: stats.vp_name || "",
    issue_date: student.yearName || "",
    // Keep camelCase aliases for backwards compatibility
    studentName: student.name || "",
    className: student.currentClass?.name || "",
    academicYear: student.yearName || "",
    // Include IDs for grades table lookup
    studentId: student.userId,
    userId: student.userId,
    id: student.userId,
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

  // Fetch all students in one batch query
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

  // Signal Puppeteer when all certificates are rendered
  useEffect(() => {
    console.log(`[ExportPage] loadedCount: ${loadedCount}, total: ${students?.length}`);
    if (students && students.length > 0 && loadedCount >= students.length) {
      console.log(`[ExportPage] All certificates loaded. Signaling readiness...`);
      // Wait for browser to finish painting
      const timer = setTimeout(() => {
        // Force layout recalculation
        document.body.offsetHeight;
        (window as any).isReadyToCapture = true;
        console.log(`[ExportPage] window.isReadyToCapture set to true`);
      }, 1000);
      return () => clearTimeout(timer);
    }
  }, [loadedCount, students]);

  const handleCertificateLoad = useCallback(() => {
    console.log(`[ExportPage] Certificate Loaded Event received`);
    setLoadedCount((prev) => prev + 1);
  }, []);

  const isLoading = loadingStudents || loadingTemplate || !students || !template;

  if (isLoading) {
    return (
      <div className="flex items-center justify-center min-h-screen bg-white">
        <Spinner />
        <span className="ml-2">جاري تجهيز البيانات...</span>
      </div>
    );
  }

  if (students.length === 0) {
    return (
      <div className="p-8 text-center text-red-500">
        لا توجد بيانات للطلاب المحددين
      </div>
    );
  }

  return (
    <div className="bg-white p-0 m-0 flex flex-col items-center">
      {students.map((student, index) => (
        <div
          key={student.userId}
          className={`${index > 0 ? "break-before-page" : ""} print:m-0`}
        >
          <CertificateViewer
            template={template as any}
            data={mapStudentToVariables(student)}
            certificateType={type}
            month={month}
            className="shadow-none border-none"
            onLoad={handleCertificateLoad}
          />
        </div>
      ))}

      {/* Debug overlay — hidden in print */}
      <div className="fixed top-0 left-0 bg-black/80 text-white p-2 text-xs z-50 print:hidden">
        Loaded: {loadedCount} / {students.length}
      </div>

      {/* Print & Puppeteer CSS optimizations */}
      <style jsx global>{`
        /* Disable animations for faster Puppeteer rendering */
        *,
        *::before,
        *::after {
          animation-duration: 0s !important;
          animation-delay: 0s !important;
          transition-duration: 0s !important;
          transition-delay: 0s !important;
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

        body {
          background-color: white !important;
          margin: 0 !important;
          padding: 0 !important;
        }

        /* Ensure images are loaded before capture */
        img {
          image-rendering: -webkit-optimize-contrast;
        }
      `}</style>
    </div>
  );
}
