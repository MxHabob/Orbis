import ExcelJS from "exceljs";

interface ClassStatistics {
  totalCount: number;
  passedCount: number;
  failedCount: number;
  averageGrade: number;
}

interface StudentExportData {
  studentNumber: string;
  name: string;
  email?: string | null;
  phone?: string | null;
  sex?: string | null;
  average?: number | null;
  status?: string | null;
}

interface ExportOptions {
  className: string;
  academicYear?: string;
  statistics?: ClassStatistics | null;
  format: "xlsx" | "csv";
}

/**
 * Generate Excel/CSV export for students in a class
 * Following 2026 best practices:
 * - Modern ExcelJS library
 * - Type-safe implementation
 * - Rich formatting for XLSX
 * - Clean CSV output
 */
export async function generateStudentExport(
  students: StudentExportData[],
  options: ExportOptions,
): Promise<Buffer> {
  const { className, academicYear, statistics, format } = options;

  const workbook = new ExcelJS.Workbook();
  workbook.creator = "Mulboi Education System";
  workbook.created = new Date();
  workbook.modified = new Date();

  const worksheet = workbook.addWorksheet("Students", {
    properties: { tabColor: { argb: "FF4A90E2" } },
  });

  // Add header section
  const titleRow = worksheet.addRow([
    `Class Report: ${className}${academicYear ? ` - ${academicYear}` : ""}`,
  ]);
  titleRow.font = { size: 16, bold: true };
  titleRow.height = 25;

  // Add statistics if available
  if (statistics) {
    worksheet.addRow([]);
    const statsRow = worksheet.addRow([
      "Total Students:",
      statistics.totalCount || 0,
      "Passed:",
      statistics.passedCount || 0,
      "Failed:",
      statistics.failedCount || 0,
      "Average:",
      statistics.averageGrade ? `${statistics.averageGrade.toFixed(2)}%` : "N/A",
    ]);
    statsRow.font = { bold: true };
    worksheet.addRow([]);
  } else {
    worksheet.addRow([]);
  }

  // Define columns
  const columns = [
    { header: "Student Number", key: "studentNumber", width: 20 },
    { header: "Name", key: "name", width: 30 },
    { header: "Email", key: "email", width: 30 },
    { header: "Phone", key: "phone", width: 20 },
    { header: "Gender", key: "sex", width: 15 },
    { header: "Average", key: "average", width: 12 },
    { header: "Status", key: "status", width: 15 },
  ];

  // Add column headers
  const headerRow = worksheet.addRow(columns.map((col) => col.header));
  headerRow.font = { bold: true, color: { argb: "FFFFFFFF" } };
  headerRow.fill = {
    type: "pattern",
    pattern: "solid",
    fgColor: { argb: "FF4A90E2" },
  };
  headerRow.alignment = { vertical: "middle", horizontal: "center" };
  headerRow.height = 20;

  // Set column widths
  worksheet.columns = columns;

  // Add student data
  for (const student of students) {
    const row = worksheet.addRow({
      studentNumber: student.studentNumber,
      name: student.name,
      email: student.email || "-",
      phone: student.phone || "-",
      sex: student.sex
        ? student.sex.charAt(0).toUpperCase() + student.sex.slice(1)
        : "-",
      average: student.average ? `${student.average.toFixed(2)}%` : "-",
      status: student.status
        ? student.status.charAt(0).toUpperCase() + student.status.slice(1)
        : "-",
    });

    // Conditional formatting for status
    if (student.status === "passed") {
      row.getCell("status").font = { color: { argb: "FF28A745" } };
    } else if (student.status === "failed") {
      row.getCell("status").font = { color: { argb: "FFDC3545" } };
    }

    // Conditional formatting for average
    if (student.average) {
      const avgCell = row.getCell("average");
      if (student.average >= 90) {
        avgCell.font = { color: { argb: "FF28A745" }, bold: true };
      } else if (student.average >= 50) {
        avgCell.font = { color: { argb: "FF4A90E2" } };
      } else {
        avgCell.font = { color: { argb: "FFDC3545" } };
      }
    }
  }

  // Add borders to all cells
  worksheet.eachRow((row, rowNumber) => {
    if (rowNumber > 3) {
      // Skip title and stats rows
      row.eachCell((cell) => {
        cell.border = {
          top: { style: "thin", color: { argb: "FFD3D3D3" } },
          left: { style: "thin", color: { argb: "FFD3D3D3" } },
          bottom: { style: "thin", color: { argb: "FFD3D3D3" } },
          right: { style: "thin", color: { argb: "FFD3D3D3" } },
        };
        cell.alignment = { vertical: "middle" };
      });
    }
  });

  // Add footer
  worksheet.addRow([]);
  const footerRow = worksheet.addRow([
    `Generated on ${new Date().toLocaleString("en-US", {
      dateStyle: "full",
      timeStyle: "short",
    })}`,
  ]);
  footerRow.font = { italic: true, size: 9, color: { argb: "FF999999" } };

  // Generate buffer based on format
  if (format === "csv") {
    const csvData = await workbook.csv.writeBuffer({
      formatterOptions: {
        // Only include data rows for CSV (skip formatting)
        headers: columns.map((col) => col.header),
      },
    });
    return Buffer.from(csvData);
  }

  // XLSX format
  const buffer = await workbook.xlsx.writeBuffer();
  return Buffer.from(buffer);
}

/**
 * Generate filename for export
 */
export function generateExportFilename(
  className: string,
  format: "xlsx" | "csv",
): string {
  const timestamp = new Date()
    .toISOString()
    .replace(/[:.]/g, "-")
    .slice(0, -5);
  const safeName = className.replace(/[^a-zA-Z0-9-_]/g, "_");
  return `${safeName}_students_${timestamp}.${format}`;
}

export interface GradesExportOptions extends ExportOptions {
  assessmentItems: { id: number; title: string }[];
}

export interface StudentGradeRow {
  studentNumber: string;
  name: string;
  grades: Record<number, number | string | null>;
}

/**
 * Generate Excel/CSV export for grades
 */
export async function generateGradesExport(
  data: StudentGradeRow[],
  options: GradesExportOptions,
): Promise<Buffer> {
  const { className, academicYear, assessmentItems, format } = options;

  const workbook = new ExcelJS.Workbook();
  const worksheet = workbook.addWorksheet("Grades");

  // Title
  const titleRow = worksheet.addRow([
    `Grades Report: ${className}${academicYear ? ` - ${academicYear}` : ""}`,
  ]);
  titleRow.font = { size: 16, bold: true };
  worksheet.addRow([]);

  // Columns definition
  const columns = [
    { header: "Student Number", key: "studentNumber", width: 20 },
    { header: "Name", key: "name", width: 30 },
    ...assessmentItems.map((item) => ({
      header: item.title,
      key: `item_${item.id}`,
      width: 15,
    })),
  ];

  // Header row
  const headerRow = worksheet.addRow(columns.map((col) => col.header));
  headerRow.font = { bold: true, color: { argb: "FFFFFFFF" } };
  headerRow.fill = {
    type: "pattern",
    pattern: "solid",
    fgColor: { argb: "FF4A90E2" },
  };
  headerRow.alignment = { horizontal: "center" };

  worksheet.columns = columns;

  // Data rows
  for (const student of data) {
    const rowData: any = {
      studentNumber: student.studentNumber,
      name: student.name,
    };

    assessmentItems.forEach((item) => {
      rowData[`item_${item.id}`] = student.grades[item.id] ?? "-";
    });

    const row = worksheet.addRow(rowData);
    
    // Add some alignment
    row.alignment = { vertical: "middle" };
  }

  // Generate buffer
  if (format === "csv") {
    const csvData = await workbook.csv.writeBuffer();
    return Buffer.from(csvData);
  }

  const buffer = await workbook.xlsx.writeBuffer();
  return Buffer.from(buffer);
}

export function generateGradesFilename(
  className: string,
  format: "xlsx" | "csv",
): string {
  const timestamp = new Date()
    .toISOString()
    .replace(/[:.]/g, "-")
    .slice(0, -5);
  const safeName = className.replace(/[^a-zA-Z0-9-_]/g, "_");
  return `${safeName}_grades_${timestamp}.${format}`;
}

export interface ClassBulkReportStudent {
  studentId: string;
  studentNumber: string;
  name: string | null;
  reportData: {
    rows: any[];
    columns: string[];
    type: string;
  };
}

export async function generateClassBulkReportExport(
  studentsData: ClassBulkReportStudent[],
  options: {
    className: string;
    reportType: string;
    month?: number;
    monthText?: string;
    format: "xlsx" | "csv";
  }
): Promise<Buffer> {
  const { className, reportType, format } = options;

  const workbook = new ExcelJS.Workbook();
  const worksheet = workbook.addWorksheet("Report", { rightToLeft: true }); // Arabi structure

  // Header Title
  const titleText =
    reportType === "monthly"
      ? `تقرير درجات شهري - ${className}${options.monthText ? ` - ${options.monthText}` : ""}`
      : `تقرير درجات نصفي - ${className}`;

  const titleRow = worksheet.addRow([titleText]);
  titleRow.font = { size: 16, bold: true };
  worksheet.addRow([]);

  if (studentsData.length === 0) {
    worksheet.addRow(["لايوجد بيانات"]);
    return format === "csv" ? Buffer.from(await workbook.csv.writeBuffer()) : Buffer.from(await workbook.xlsx.writeBuffer());
  }

  // Extract columns and subjects from the first student (they all share the same structure ideally)
  const columnsRef = studentsData[0]?.reportData?.columns || [];
  const columnMaxScoresRef = (studentsData[0]?.reportData as any)?.columnMaxScores || {};
  const allSubjects = studentsData[0]?.reportData?.rows.map(r => r.subject) || [];

  // Row 1: Subject headers spanning sub-columns
  const row1 = ["م", "رقم الطالب", "اسم الطالب"];
  const subColCount = columnsRef.length + 1; // assessments columns + المجموع 
  
  for (const subject of allSubjects) {
     row1.push(subject);
     for (let i = 1; i < subColCount; i++) row1.push(""); // empty cells to merge
  }
  
  const header1 = worksheet.addRow(row1);
  header1.font = { bold: true, size: 12 };
  header1.alignment = { horizontal: "center", vertical: "middle" };
  header1.fill = { type: 'pattern', pattern: 'solid', fgColor: { argb: 'FFC6E0B4' } }; // Light green background from image

  // Merge cells for subjects in row1
  let colIndex = 4;
  for (const subject of allSubjects) {
    if (subColCount > 1) {
      worksheet.mergeCells(header1.number, colIndex, header1.number, colIndex + subColCount - 1);
    }
    colIndex += subColCount;
  }

  // Row 2: sub-columns (assessments & total)
  const row2 = ["", "", ""];
  for (const subject of allSubjects) {
    for (let i = columnsRef.length - 1; i >= 0; i--) {
        const colTitle = columnsRef[i];
        const maxScore = columnMaxScoresRef[colTitle] ? `Max ${columnMaxScoresRef[colTitle]}` : "Max Score";
        row2.push(`${colTitle}\n${maxScore}`);
    }
    // "المجموع" follows the assessments
    row2.push("المجموع");
  }

  const header2 = worksheet.addRow(row2);
  header2.font = { bold: true, color: { argb: "FFFFFFFF" } };
  header2.alignment = { horizontal: "center", vertical: "middle", wrapText: true };
  header2.fill = { type: 'pattern', pattern: 'solid', fgColor: { argb: 'FF4A90E2' } };
  header2.height = 30;

  worksheet.mergeCells(header1.number, 1, header2.number, 1); // Merge "م"
  worksheet.mergeCells(header1.number, 2, header2.number, 2); // Merge "student number"
  worksheet.mergeCells(header1.number, 3, header2.number, 3); // Merge "اسم الطالب"

  worksheet.getColumn(1).width = 5;
  worksheet.getColumn(2).width = 15;
  worksheet.getColumn(3).width = 30;

  // Add borders for headers
  [header1, header2].forEach(row => {
    row.eachCell({ includeEmpty: true }, cell => {
      cell.border = {
        top: { style: 'thin' }, left: { style: 'thin' },
        bottom: { style: 'thin' }, right: { style: 'thin' }
      };
    });
  });

  // Data rows
  studentsData.forEach((student, sIdx) => {
    const rowData = [
      sIdx + 1,
      student.studentNumber,
      student.name || ""
    ];

    const reportRows = student.reportData.rows;

    for (const subject of allSubjects) {
      const subjectRow = reportRows.find((r: any) => r.subject === subject);
      if (subjectRow) {
         for (let i = columnsRef.length - 1; i >= 0; i--) {
           const colName = columnsRef[i];
           const g = subjectRow.grades[colName];
           rowData.push(g !== null && g !== undefined ? g : "-");
         }
         rowData.push(subjectRow.total); // المجموع
      } else {
         for (let i = 0; i < columnsRef.length; i++) rowData.push("-");
         rowData.push("-");
      }
    }

    const dRow = worksheet.addRow(rowData);
    dRow.alignment = { horizontal: "center", vertical: "middle" };
    dRow.eachCell({ includeEmpty: true }, cell => {
       cell.border = {
         top: { style: 'thin' }, left: { style: 'thin' },
         bottom: { style: 'thin' }, right: { style: 'thin' }
       };
    });
  });

  if (format === "csv") {
    const csvData = await workbook.csv.writeBuffer();
    return Buffer.from(csvData);
  }

  const buffer = await workbook.xlsx.writeBuffer();
  return Buffer.from(buffer);
}
