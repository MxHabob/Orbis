import { useEffect, useState } from "react";
import { useSearchParams } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import { Spinner } from "@orbis/ui/spinner";
import { CertificateViewer } from "@/components/certificates/certificate-viewer";
import { api } from "@/lib/tauri-api";

const HIJRI_MONTHS_NAMES: Record<number, string> = {
  1: "محرم",
  2: "صفر",
  3: "ربيع أول",
  4: "ربيع ثاني",
  5: "جمادى أول",
  6: "جمادى ثاني",
  7: "رجب",
  8: "شعبان",
  9: "رمضان",
  10: "شوال",
  11: "ذو القعدة",
  12: "ذو الحجة",
};

const mapStudentToVariables = (student: any, type?: string, month?: number) => {
  if (!student) return {};

  const stats = (student.stats as any) || {};
  const currentYear = student.currentClass?.yearName || "2024-2025";

  return {
    ...student,
    student_name: student.name || "",
    seat_number: student.studentNumber || "",
    secret_number: student.secretNumber || "",
    student_id: student.academicId || student.studentNumber || "",
    department: stats.department || "",
    section: stats.section || "",
    class_name: student.currentClass?.name || "",
    total_grade: stats.total_grade || "",
    total_grade_text: stats.total_grade_text || "",
    grade_label: stats.grade_label || "",
    final_result: stats.final_result || "",
    rank: stats.rank || "",
    org_name: "مدرسة صنعاء النموذجية",
    admin_name: stats.admin_name || "",
    vp_name: stats.vp_name || "",
    issue_date: new Date().toLocaleDateString("ar-YE", {
      year: "numeric",
      month: "long",
      day: "numeric",
    }),
    certificate_type:
      type === "half-yearly"
        ? "النصفي"
        : type === "final"
          ? "النهائي"
          : "الشهري",
    report_type:
      type === "half-yearly"
        ? "النصفي"
        : type === "final"
          ? "النهائي"
          : `الشهري - لشهر ${month ? HIJRI_MONTHS_NAMES[month] || `شهر ${month}` : ""}`,
    school_year: currentYear,
    studentId: student.userId,
    userId: student.userId,
    id: student.userId,
  };
};

export default function ExportCertificatePage() {
  const [searchParams] = useSearchParams();
  const studentId = searchParams.get("studentId");
  const templateId = searchParams.get("templateId");
  const type = searchParams.get("type") || "monthly";
  const month = parseInt(searchParams.get("month") || "1");

  const [isReady, setIsReady] = useState(false);

  const { data: exportData, isLoading } = useQuery({
    queryKey: ["export-certificate-data", studentId, templateId, type, month],
    queryFn: async () => {
      // 1. Fetch enrollment by studentId (userId)
      const enrollmentsList = await api.enrollments.getAll("", { student_id: studentId! });
      const enrollment = enrollmentsList?.[0];
      if (!enrollment) throw new Error("Enrollment not found");

      // 2. Fetch related details in parallel
      const [userResponse, classesList, templateResponse] = await Promise.all([
        api.users.getUser(enrollment.student_id),
        api.classes.getAll(""),
        api.certificates.getById(templateId!),
      ]);

      const user = userResponse as any;
      const classObj = classesList?.find((c: any) => c.id === enrollment.class_id) as any;
      const template = templateResponse as any;

      // Map to combined student structure
      const student = {
        ...enrollment,
        name: user?.full_name || "",
        userId: enrollment.student_id,
        currentClass: {
          id: classObj?.id,
          name: classObj?.name,
          yearName: classObj?.cycle_id || "2024-2025",
          academicYearId: classObj?.cycle_id,
        },
        stats: enrollment.meta ? JSON.parse(enrollment.meta) : {},
      };

      return { student, template };
    },
    enabled: !!studentId && !!templateId,
  });

  useEffect(() => {
    if (exportData?.student && exportData?.template) {
      const timer = setTimeout(() => {
        setIsReady(true);
        (window as any).isReadyToCapture = true;
      }, 1500);
      return () => clearTimeout(timer);
    }
  }, [exportData]);

  if (isLoading || !exportData?.student || !exportData?.template) {
    return (
      <div className="flex items-center justify-center min-h-screen bg-white">
        <Spinner />
      </div>
    );
  }

  const { student, template } = exportData;
  const studentVariables = mapStudentToVariables(student, type, month);

  return (
    <>
      <div className="inline-block bg-white p-0 m-0">
        <CertificateViewer
          template={template as any}
          data={studentVariables}
          certificateType={type}
          month={month}
          className="shadow-none border-none"
        />
      </div>
    </>
  );
}
