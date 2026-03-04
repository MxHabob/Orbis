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
    issue_date: "",
    // Keep camelCase aliases for backwards compatibility
    studentName: student.name || "",
    className: student.currentClass?.name || "",
    academicYear: "",
    // Include IDs for grades table lookup
    studentId: student.userId,
    userId: student.userId,
    id: student.userId,
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
