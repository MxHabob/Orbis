"use client";

import { useSearchParams } from "next/navigation";
import { useTRPC } from "@/trpc/client";
import { CertificateViewer } from "@/components/certificates/certificate-viewer";
import { Spinner } from "@midday/ui/spinner";
import { useEffect, useState, useMemo } from "react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

/**
 * Bulk Export page for Puppeteer
 * Renders many certificates in one go for efficient multi-page PDF generation.
 */

const mapStudentToVariables = (student: any) => {
  if (!student) return {};
  
  return {
    ...student,
    studentName: student.name,
    className: student.currentClass?.name || "",
    academicYear: student.yearName || "",
  };
};

export default function BulkExportCertificatePage() {
  const searchParams = useSearchParams();
  const studentIds = useMemo(() => searchParams.get("studentIds")?.split(",") || [], [searchParams]);
  const templateId = searchParams.get("templateId");
  const type = searchParams.get("type") || "monthly";
  const month = parseInt(searchParams.get("month") || "1");
  const trpc = useTRPC();

  // Fetch all students data
  // We use a single query that might be better in the future, but for now we'll do it or rely on the caller
  // Actually, we can just fetch them here
  const [students, setStudents] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function loadData() {
      if (studentIds.length === 0) return;
      
      try {
        const promises = studentIds.map(id => trpc.students.getStudentByUserId.query({ userId: id }));
        const results = await Promise.all(promises);
        setStudents(results.filter(Boolean));
      } catch (error) {
        console.error("Error loading students for bulk export:", error);
      } finally {
        setLoading(false);
      }
    }
    loadData();
  }, [studentIds, trpc]);

  const { data: template, isLoading: loadingTemplate } = trpc.certificateTemplates.getById.useQuery(
    { id: templateId! },
    { enabled: !!templateId }
  );

  useEffect(() => {
    if (!loading && !loadingTemplate && students.length > 0 && template) {
      // Allow some time for GradesTable inside CertificateViewer to fetch data
      const timer = setTimeout(() => {
        (window as any).isReadyToCapture = true;
      }, 5000); // 5 seconds buffer for high volume
      return () => clearTimeout(timer);
    }
  }, [loading, loadingTemplate, students, template]);

  if (loading || loadingTemplate || students.length === 0 || !template) {
    return (
      <div className="flex items-center justify-center min-h-screen bg-white">
        <Spinner />
        <span className="ml-2">جاري تجهيز البيانات... ({students.length}/{studentIds.length})</span>
      </div>
    );
  }

  return (
    <div className="bg-white p-0 m-0 flex flex-col items-center">
      {students.map((student, index) => (
        <div key={student.userId} className={`${index > 0 ? 'break-before-page' : ''} print:m-0`}>
          <CertificateViewer
            template={template as any}
            data={mapStudentToVariables(student)}
            certificateType={type}
            month={month}
            className="shadow-none border-none"
          />
        </div>
      ))}
      <style jsx global>{`
        @media print {
          .break-before-page {
            break-before: page;
          }
        }
        body {
          background-color: white !important;
        }
      `}</style>
    </div>
  );
}
