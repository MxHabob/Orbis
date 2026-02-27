"use client";

import { useSearchParams } from "next/navigation";
import { useTRPC } from "@/trpc/client";
import { useQuery } from "@tanstack/react-query";
import { CertificateViewer } from "@/components/certificates/certificate-viewer";
import { Spinner } from "@midday/ui/spinner";
import { useEffect, useState } from "react";

/**
 * Specialized Export Proxy Page
 * Used by the background worker to render high-quality certificates.
 */

const mapStudentToVariables = (student: any) => {
  if (!student) return {};
  
  return {
    ...student,
    studentName: student.name,
    className: student.class?.name || "",
    academicYear: student.academicYear?.name || "",
  };
};

export default function ExportCertificatePage() {
  const searchParams = useSearchParams();
  const studentId = searchParams.get("studentId");
  const templateId = searchParams.get("templateId");
  const type = searchParams.get("type") || "monthly";
  const month = parseInt(searchParams.get("month") || "1");
  const trpc = useTRPC();

  const { data: student, isLoading: loadingStudent } = useQuery(
    trpc.students.getStudentByUserId.queryOptions(
      { userId: studentId! },
      { enabled: !!studentId }
    )
  );

  const { data: template, isLoading: loadingTemplate } = useQuery(
    trpc.certificateTemplates.getById.queryOptions(
      { id: templateId! },
      { enabled: !!templateId }
    )
  );

  const [isReady, setIsReady] = useState(false);

  useEffect(() => {
    if (student && template) {
      // Small delay to ensure all nested components (like GradesTable) are hydrated
      const timer = setTimeout(() => {
        setIsReady(true);
        // Signal to Puppeteer that we are ready to capture
        (window as any).isReadyToCapture = true;
      }, 1500);
      return () => clearTimeout(timer);
    }
  }, [student, template]);

  if (loadingStudent || loadingTemplate || !student || !template) {
    return (
      <div className="flex items-center justify-center min-h-screen bg-white">
        <Spinner />
      </div>
    );
  }

  const studentVariables = mapStudentToVariables(student as any);

  return (
    <div className="inline-block bg-white p-0 m-0">
      <CertificateViewer
        template={template as any}
        data={studentVariables}
        certificateType={type}
        month={month}
        className="shadow-none border-none"
      />
    </div>
  );
}
