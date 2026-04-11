import type { Job } from "bullmq";
import type { ReportGenerationData } from "../queue";
import { logger } from "@midday/logger";
import { getClassBulkReportData, getClassesV4, getStudentsParentsData } from "@midday/db/queries";
import { 
  generateClassBulkReportExport, 
  generateAttendanceSheetExport, 
  generateStudentsParentsExport,
  generateAttendanceFilename,
  generateParentsReportFilename,
  generateBulkReportFilename,
  generateGradesFilename,
  generateControlStaffExport,
  generateSecretGradesExport
} from "@midday/export";
import { uploadFile } from "@midday/storage";
import { createDocument } from "@midday/db/queries/documents";

export async function handleReportGeneration(job: Job<ReportGenerationData>) {
  const startTime = Date.now();
  const { classId, type, month, monthText, format, userId } = job.data;
  let subjectNameFound: string | undefined;
  
  logger.info(`[ReportGen] Job ${job.id} started. Class: ${classId}, Type: ${type}`);
  
  try {
    await job.updateProgress(10);
    
    const allClasses = await getClassesV4({ flat: true, limit: 0 });
    const classData = allClasses.find(c => c.id === classId);
    if (!classData) {
      throw new Error(`لم يتم العثور على الفصل الدراسي`);
    }

    await job.updateProgress(30);

    let buffer: Buffer;

    if (type === "attendance_sheet") {
      const { getAttendanceSheetData } = await import("@midday/db/queries");
      const studentsData = await getAttendanceSheetData(classId);

      await job.updateProgress(50);

      buffer = await generateAttendanceSheetExport(studentsData, {
        className: classData.name,
        classColor: classData.color,
        format: format,
      });
    } else if (type === "students_parents_data") {
      const { getStudentsParentsData } = await import("@midday/db/queries");
      const studentsData = await getStudentsParentsData(classId);

      await job.updateProgress(50);

      buffer = await generateStudentsParentsExport(studentsData, {
        className: classData.name,
        classColor: classData.color,
        format: format,
      });
    } else if (type === "grades_grid") {
      const { getGradeGridData } = await import("@midday/db/queries");
      const { generateGradesExport, generateGradesFilename } = await import("@midday/export");

      const data = await getGradeGridData({
        classId: classId,
        subjectId: job.data.subjectId!,
        termId: job.data.termId!,
        month: job.data.month!,
        academicYearId: job.data.academicYearId!,
      });

      const exportData = data.students.map((student: any) => {
        const studentGrades: Record<number, number | string | null> = {};
        data.items.forEach((item: any) => {
          const grade = data.grades.find(
            (g: any) => g.studentId === student.studentId && g.assessmentItemId === item.id
          );
          studentGrades[item.id] = grade?.score ?? null;
        });

        return {
          studentNumber: student.studentNumber,
          name: student.name || "N/A",
          grades: studentGrades,
        };
      });

      await job.updateProgress(50);

      subjectNameFound = data.subjectName;

      buffer = await generateGradesExport(exportData, {
        className: classData.name,
        classColor: classData.color,
        academicYear: undefined,
        subjectName: data.subjectName,
        assessmentItems: data.items.map((i: any) => ({
          id: i.id,
          title: i.title,
          month: i.month,
          maxScore: i.maxScore,
        })),
        format: format,
      });
    } else if (type === "control_staff") {
      const studentsData = await getStudentsParentsData(classId);

      await job.updateProgress(50);

      const exportData = studentsData.map((s: any) => ({
        studentNumber: s.studentNumber,
        name: s.studentName || "N/A",
        seatingNumber: s.seatingNumber,
        secretNumber: s.secretNumber,
      }));

      buffer = await generateControlStaffExport(exportData, {
        className: classData.name,
        classColor: classData.color,
        format: format,
      });
    } else if (type === "secret_final" || type === "secret_midterm") {
      const reportType = type === "secret_final" ? "final" : "half-yearly";
      const bulkData = (await getClassBulkReportData(classId, reportType, month)) as any;

      await job.updateProgress(50);

      buffer = await generateSecretGradesExport(bulkData, {
        className: classData.name,
        classColor: classData.color,
        reportType,
        format: format,
      });
    } else {
      const bulkData = await getClassBulkReportData(classId, type as any, month);
      
      await job.updateProgress(50);

      buffer = await generateClassBulkReportExport(bulkData as any, {
        className: classData.name,
        classColor: classData.color,
        reportType: type,
        month: month,
        monthText: monthText,
        format: format,
      });
    }

    await job.updateProgress(80);

    let filename: string;
    let pdfTitle: string;

    if (type === "attendance_sheet") {
      filename = generateAttendanceFilename(classData.name, format);
      pdfTitle = `كشف الحضور والغياب - ${classData.name}`;
    } else if (type === "students_parents_data") {
      filename = generateParentsReportFilename(classData.name, format);
      pdfTitle = `بيانات الطلاب والأهالي - ${classData.name}`;
    } else if (type === "grades_grid") {
      filename = generateGradesFilename(classData.name, subjectNameFound, format);
      pdfTitle = `كشف الدرجات - ${classData.name}${subjectNameFound ? ` - ${subjectNameFound}` : ""}`;
    } else if (type === "control_staff") {
      filename = `موظفي_الكنترول_${classData.name}_${new Date().getTime()}.${format}`;
      pdfTitle = `موظفي الكنترول - ${classData.name}`;
    } else if (type === "secret_final" || type === "secret_midterm") {
      filename = type === "secret_final" 
        ? `درجات_التصفية_${classData.name}.${format}`
        : `درجات_نصفية_${classData.name}.${format}`;
      pdfTitle = type === "secret_final" ? "كشف درجات التصفية" : "كشف الدرجات النصفية";
    } else {
      filename = generateBulkReportFilename(classData.name, type, format, monthText);
      const typeText = type === "monthly" ? "شهري" : type === "half-yearly" ? "نصفي" : type === "final" ? "نهائي" : "درجات";
      pdfTitle = `تقرير درجات ${typeText} - ${classData.name}`;
    }

    const mimeType = format === "xlsx" 
      ? "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" 
      : "text/csv";

    // Upload to Vault storage replacing synchronous buffer responses
    const path = `exports/${userId}/${Date.now()}_${filename}`;
    const uploadResult = await uploadFile({ 
      bucket: "vault", 
      key: path, 
      file: buffer, 
      contentType: mimeType 
    });

    // Save metadata in vault documents if appropriate
    await createDocument({
      name: filename,
      title: pdfTitle,
      size: buffer.length,
      mimetype: mimeType,
      extension: format,
      key: path,
      userId: userId,
      type: "file",
      pathTokens: ["exports", "reports", userId, filename],
    });

    await job.updateProgress(100);
    const duration = ((Date.now() - startTime) / 1000).toFixed(1);
    logger.info(`[ReportGen] ✅ Job ${job.id} completed in ${duration}s. Size: ${(buffer.length / 1024).toFixed(0)}KB`);

    return {
      success: true,
      url: uploadResult.url,
      fileName: filename,
      fullPath: path,
      totalItems: 1, // Represents 1 exported file/report
    };

  } catch (error) {
    const errMsg = error instanceof Error ? error.message : "Unknown error";
    logger.error(`[ReportGen] ❌ Job ${job.id} failed: ${errMsg}`);
    throw error;
  }
}
