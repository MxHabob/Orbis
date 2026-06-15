import { Spinner } from "@orbis/ui/spinner";
import { useQuery } from "@tanstack/react-query";
import { useSearchParams } from "react-router-dom";
import { Suspense, useCallback, useEffect, useMemo, useState } from "react";
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

  let resolvedImage = student.image || "";
  if (
    resolvedImage &&
    !resolvedImage.startsWith("http") &&
    !resolvedImage.includes("/")
  ) {
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
    issue_date: new Date().toLocaleDateString("ar-YE", {
      year: "numeric",
      month: "long",
      day: "numeric",
    }),
    report_type:
      type === "half-yearly"
        ? "النصفي"
        : type === "final"
          ? "النهائي"
          : `الشهري - لشهر ${month ? HIJRI_MONTHS_NAMES[month] || `شهر ${month}` : ""}`,
    month_name: month ? HIJRI_MONTHS_NAMES[month] || `شهر ${month}` : "",
    academic_year: currentYear,
    studentId: student.userId,
    student_image: resolvedImage,
  };
};

export default function BulkExportCertificatePage() {
  return (
    <Suspense
      fallback={
        <div className="flex flex-col items-center justify-center min-h-screen bg-white">
          <Spinner className="w-8 h-8 opacity-20" />
          <span className="mt-4 text-xs font-bold uppercase tracking-widest opacity-40">
            Initializing PDF Pipeline...
          </span>
        </div>
      }
    >
      <BulkExportContent />
    </Suspense>
  );
}

function BulkExportContent() {
  const [searchParams] = useSearchParams();
  const studentIds = useMemo(
    () => searchParams.get("studentIds")?.split(",").filter(Boolean) || [],
    [searchParams],
  );
  const templateId = searchParams.get("templateId");
  const type = searchParams.get("type") || "monthly";
  const month = parseInt(searchParams.get("month") || "1");

  const [finishedPages, setFinishedPages] = useState<Set<number>>(new Set());
  const rankingPeriod =
    type === "half-yearly"
      ? "half-yearly"
      : type === "final"
        ? "final"
        : "monthly";
  const rankingMonth = rankingPeriod === "monthly" ? month : undefined;

  const { data: exportData, isLoading } = useQuery({
    queryKey: ["bulk-export-certificate-data", studentIds, templateId, type, month],
    queryFn: async () => {
      // 1. Fetch template
      const template = await api.certificates.getById(templateId!);

      // 2. Fetch all enrollments in parallel
      const enrollments = await Promise.all(
        studentIds.map(async (id) => {
          const list = await api.enrollments.getAll("", { student_id: id });
          return list?.[0];
        })
      );
      const activeEnrollments = enrollments.filter(Boolean);

      // 3. Fetch user info, classes and analytics in parallel
      const classesList = await api.classes.getAll("");
      
      const mappedStudents = await Promise.all(
        activeEnrollments.map(async (enrollment) => {
          const user = await api.users.getUser(enrollment.student_id);
          const classObj = classesList?.find((c: any) => c.id === enrollment.class_id);
          
          return {
            ...enrollment,
            name: (user as any)?.full_name || "",
            userId: enrollment.student_id,
            currentClass: {
              id: classObj?.id,
              name: classObj?.name,
              yearName: classObj?.cycle_id || "2024-2025",
              academicYearId: classObj?.cycle_id,
            },
            stats: enrollment.meta ? JSON.parse(enrollment.meta) : {},
          };
        })
      );

      return { students: mappedStudents, template };
    },
    enabled: studentIds.length > 0 && !!templateId,
  });

  const students = exportData?.students;
  const certificateTemplate = exportData?.template;

  const canvasConfig = (certificateTemplate as any)?.content?.canvasConfig;
  const layoutConfig = canvasConfig?.layoutConfig ||
    (certificateTemplate as any)?.content?.layoutConfig || {
      rows: 1,
      columns: 1,
      gap: 0,
    };
  const totalSlots = layoutConfig.rows * layoutConfig.columns;

  const studentChunks = useMemo(() => {
    if (!students) return [];
    const chunks = [];
    for (let i = 0; i < students.length; i += totalSlots) {
      chunks.push(students.slice(i, i + totalSlots));
    }
    return chunks;
  }, [students, totalSlots]);

  useEffect(() => {
    if (
      studentChunks.length > 0 &&
      finishedPages.size >= studentChunks.length
    ) {
      requestAnimationFrame(() => {
        requestAnimationFrame(() => {
          const delay = Math.max(5000, studentChunks.length * 300);
          const timer = setTimeout(() => {
            (window as any).isReadyToCapture = true;
            console.info(
              `[Export] PDF Pipeline Ready. Final delay: ${delay}ms`,
            );
          }, delay);
          return () => clearTimeout(timer);
        });
      });
    }
  }, [finishedPages, studentChunks]);

  const handlePageReady = useCallback((index: number) => {
    setFinishedPages((prev) => new Set([...prev, index]));
  }, []);

  if (isLoading || !students || !certificateTemplate) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen bg-white">
        <Spinner className="w-8 h-8 opacity-20" />
        <span className="mt-4 text-xs font-bold uppercase tracking-widest opacity-40">
          Initializing PDF Pipeline...
        </span>
      </div>
    );
  }

  return (
    <div className="bg-white p-0 m-0 overflow-hidden">
      <div
        id="pdf-metadata"
        style={{ display: "none" }}
        data-total-pages={studentChunks.length}
      />

      {studentChunks.map((chunk, chunkIndex) => (
        <div
          key={`sheet-${chunkIndex}`}
          className={`${chunkIndex > 0 ? "break-before-page" : ""} flex justify-center`}
        >
          <CertificateViewer
            template={certificateTemplate as any}
            data={chunk.map((student) => {
              // Standard stats rank mapping
              return {
                ...mapStudentToVariables(student, type, month),
                rank: (student.stats as any)?.rank || "",
                rank_percentage: (student.stats as any)?.percentage ?? "",
                rank_total: (student.stats as any)?.totalScore ?? "",
                rank_max: (student.stats as any)?.maxScore ?? "",
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
    </div>
  );
}
