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
  classColor?: string;
  academicYear?: string;
  statistics?: ClassStatistics | null;
  format: "xlsx" | "csv";
}

export function hexToArgb(hex?: string): string {
  if (!hex) return "FF4A90E2"; // default blue
  const cleanHex = hex.replace("#", "");
  if (cleanHex.length === 6) return `FF${cleanHex.toUpperCase()}`;
  if (cleanHex.length === 8) return cleanHex.toUpperCase();
  return "FF4A90E2";
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

  const primaryArgb = hexToArgb(options.classColor);

  const worksheet = workbook.addWorksheet("Students", {
    properties: { tabColor: { argb: primaryArgb } },
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
    fgColor: { argb: primaryArgb },
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
        avgCell.font = { color: { argb: primaryArgb } };
      } else {
        avgCell.font = { color: { argb: "FFDC3545" } };
      }
    }
    
    // Highlight auditors with a distinct light red style
    if (student.status === "Auditor" || student.status === "auditor") {
      row.eachCell((cell) => {
        cell.fill = {
           type: "pattern",
           pattern: "solid",
           fgColor: { argb: "FFFFEBEE" } // Light red
        };
      });
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
  const primaryArgb = hexToArgb(options.classColor);

  headerRow.font = { bold: true, color: { argb: "FFFFFFFF" } };
  headerRow.fill = {
    type: "pattern",
    pattern: "solid",
    fgColor: { argb: primaryArgb },
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
  status?: string | null;
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
    classColor?: string;
    reportType: string;
    month?: number;
    monthText?: string;
    format: "xlsx" | "csv";
  }
): Promise<Buffer> {
  const { className, reportType, format } = options;

  const workbook = new ExcelJS.Workbook();
  const primaryArgb = hexToArgb(options.classColor);
  
  const worksheet = workbook.addWorksheet("كشف الدرجات", { 
    views: [{ rightToLeft: true }],
    properties: { tabColor: { argb: primaryArgb } }
  }); 

  // Header Title
  const titleText =
    reportType === "monthly"
      ? `تقرير درجات شهري - ${className}${options.monthText ? ` - ${options.monthText}` : ""}`
      : `تقرير درجات نصفي - ${className}`;

  const titleRow = worksheet.addRow([titleText]);
  titleRow.font = { size: 18, bold: true, color: { argb: primaryArgb } };
  titleRow.alignment = { horizontal: "center", vertical: "middle" };
  worksheet.addRow([]);

  if (studentsData.length === 0) {
    worksheet.addRow(["لايوجد بيانات"]);
    return format === "csv" ? Buffer.from(await workbook.csv.writeBuffer()) : Buffer.from(await workbook.xlsx.writeBuffer());
  }

  // Extract columns and subjects
  const columnsRef = studentsData[0]?.reportData?.columns || [];
  const columnMaxScoresRef = (studentsData[0]?.reportData as any)?.columnMaxScores || {};
  const allSubjects = studentsData[0]?.reportData?.rows.map(r => r.subject) || [];

  const row1 = ["م", "رقم الطالب", "اسم الطالب"];
  const subColCount = columnsRef.length + 1; // assessments columns + المجموع 
  
  for (const subject of allSubjects) {
     row1.push(subject);
     for (let i = 1; i < subColCount; i++) row1.push(""); 
  }
  
  const header1 = worksheet.addRow(row1);
  header1.font = { bold: true, size: 12 };
  header1.alignment = { horizontal: "center", vertical: "middle" };
  // 2026: Using a very soft gray-green or subtle brand accent for subjects header
  header1.fill = { type: 'pattern', pattern: 'solid', fgColor: { argb: 'FFF5F5F5' } }; 

  let colIndex = 4;
  for (const subject of allSubjects) {
    if (subColCount > 1) {
      worksheet.mergeCells(header1.number, colIndex, header1.number, colIndex + subColCount - 1);
    }
    colIndex += subColCount;
  }

  const row2 = ["", "", ""];
  for (const subject of allSubjects) {
    for (let i = columnsRef.length - 1; i >= 0; i--) {
        const colTitle = columnsRef[i];
        if (!colTitle) continue;
        const maxScore = columnMaxScoresRef[colTitle] ? `${columnMaxScoresRef[colTitle]}` : "";
        row2.push(`${colTitle}${maxScore ? `\n(${maxScore})` : ""}`);
    }
    row2.push("المجموع");
  }

  const header2 = worksheet.addRow(row2);
  header2.font = { bold: true, color: { argb: "FFFFFFFF" } };
  header2.alignment = { horizontal: "center", vertical: "middle", wrapText: true };
  header2.fill = { type: 'pattern', pattern: 'solid', fgColor: { argb: primaryArgb } };
  header2.height = 35;

  worksheet.mergeCells(header1.number, 1, header2.number, 1); 
  worksheet.mergeCells(header1.number, 2, header2.number, 2); 
  worksheet.mergeCells(header1.number, 3, header2.number, 3); 

  worksheet.getColumn(1).width = 5;
  worksheet.getColumn(2).width = 15;
  worksheet.getColumn(3).width = 35;

  // Set widths for subject columns
  for (let i = 4; i <= row2.length; i++) {
    worksheet.getColumn(i).width = 12;
  }

  [header1, header2].forEach(row => {
    row.eachCell({ includeEmpty: true }, cell => {
      cell.border = {
        top: { style: 'thin', color: { argb: "FFE0E0E0" } }, 
        left: { style: 'thin', color: { argb: "FFE0E0E0" } },
        bottom: { style: 'thin', color: { argb: "FFE0E0E0" } }, 
        right: { style: 'thin', color: { argb: "FFE0E0E0" } }
      };
    });
  });

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
         for (let i = 0; i < columnsRef.length; i++) {
           const colName = columnsRef[i];
           if (!colName) continue;
           const g = subjectRow.grades[colName];
           rowData.push(g !== null && g !== undefined ? g : "-");
         }
         rowData.push(subjectRow.total);
      } else {
         for (let i = 0; i < subColCount; i++) rowData.push("-");
      }
    }

    const dRow = worksheet.addRow(rowData);
    dRow.alignment = { horizontal: "center", vertical: "middle" };
    dRow.height = 25;
    dRow.eachCell({ includeEmpty: true }, cell => {
       cell.border = {
         top: { style: 'thin', color: { argb: "FFF0F0F0" } }, 
         left: { style: 'thin', color: { argb: "FFF0F0F0" } },
         bottom: { style: 'thin', color: { argb: "FFF0F0F0" } }, 
         right: { style: 'thin', color: { argb: "FFF0F0F0" } }
       };
    });

    if (student.status === "auditor" || student.status === "homeschooled") {
       dRow.eachCell({ includeEmpty: true }, cell => {
          cell.fill = { 
            type: "pattern", 
            pattern: "solid", 
            fgColor: { argb: "FFFFEBEE" } 
          };
          cell.border = {
            top: { style: 'thin', color: { argb: 'FFFFCDD2' } },
            left: { style: 'thin', color: { argb: 'FFFFCDD2' } },
            bottom: { style: 'thin', color: { argb: 'FFFFCDD2' } },
            right: { style: 'thin', color: { argb: 'FFFFCDD2' } }
          };
       });
    }
  });

  if (format === "csv") {
    const csvData = await workbook.csv.writeBuffer();
    return Buffer.from(csvData);
  }

  const buffer = await workbook.xlsx.writeBuffer();
  return Buffer.from(buffer);
}

export async function generateAttendanceSheetExport(
  students: { name: string | null; phone: string | null; status?: string | null }[],
  options: { className: string; classColor?: string; format: "xlsx" | "csv" }
): Promise<Buffer> {
  const primaryArgb = hexToArgb(options.classColor);
  const workbook = new ExcelJS.Workbook();
  const worksheet = workbook.addWorksheet("الحضور والغياب", {
    views: [{ rightToLeft: true }],
    properties: { tabColor: { argb: primaryArgb } }
  });

  const titleText = `كشف الحضور والغياب - ${options.className}`;
  worksheet.addRow([titleText]);
  worksheet.mergeCells(1, 1, 1, 28);
  const titleCell = worksheet.getCell(1, 1);
  titleCell.font = { bold: true, size: 18, color: { argb: primaryArgb } };
  titleCell.alignment = { horizontal: "center", vertical: "middle" };

  const weeks = ["الأسبوع الأول", "الأسبوع الثاني", "الأسبوع الثالث", "الأسبوع الرابع", "الأسبوع الخامس"];
  const days = ["السبت", "الأحد", "الإثنين", "الثلاثاء", "الأربعاء"];

  const row2 = ["م", "اسم الطالب"];
  const row3 = ["", ""];

  for (const week of weeks) {
    row2.push(week, "", "", "", "");
    row3.push(...days);
  }
  
  row2.push("رقم التلفون");
  row3.push("");

  const header1 = worksheet.addRow(row2);
  const header2 = worksheet.addRow(row3);

  worksheet.mergeCells(2, 1, 3, 1);
  worksheet.mergeCells(2, 2, 3, 2);
  worksheet.mergeCells(2, 28, 3, 28);

  let weekStartCol = 3;
  for (let i = 0; i < 5; i++) {
    worksheet.mergeCells(2, weekStartCol, 2, weekStartCol + 4);
    weekStartCol += 5;
  }

  worksheet.getColumn(1).width = 5;
  worksheet.getColumn(2).width = 35;
  for (let i = 3; i <= 27; i++) {
    worksheet.getColumn(i).width = 6;
  }
  worksheet.getColumn(28).width = 20;

  [header1, header2].forEach((row) => {
    row.alignment = { horizontal: "center", vertical: "middle" };
    row.font = { bold: true, color: { argb: "FFFFFFFF" } };
    row.fill = { type: 'pattern', pattern: 'solid', fgColor: { argb: primaryArgb } };
    row.height = 25;
    row.eachCell({ includeEmpty: true }, (cell: any) => {
      cell.border = {
        top: { style: 'thin', color: { argb: 'FFE0E0E0' } }, 
        left: { style: 'thin', color: { argb: 'FFE0E0E0' } },
        bottom: { style: 'thin', color: { argb: 'FFE0E0E0' } }, 
        right: { style: 'thin', color: { argb: 'FFE0E0E0' } }
      };
    });
  });

  students.forEach((student, sIdx) => {
    const rowData: any[] = [sIdx + 1, student.name || ""];
    for (let i = 0; i < 25; i++) rowData.push("");
    rowData.push(student.phone || "");

    const dRow = worksheet.addRow(rowData);
    dRow.alignment = { horizontal: "center", vertical: "middle" };
    dRow.height = 22;
    dRow.eachCell({ includeEmpty: true }, (cell: any) => {
      cell.border = {
        top: { style: 'thin', color: { argb: 'FFF0F0F0' } }, 
        left: { style: 'thin', color: { argb: 'FFF0F0F0' } },
        bottom: { style: 'thin', color: { argb: 'FFF0F0F0' } }, 
        right: { style: 'thin', color: { argb: 'FFF0F0F0' } }
      };
    });
    
    if (student.status === "auditor") {
      dRow.eachCell({ includeEmpty: true }, (cell: any) => {
        cell.fill = { 
          type: 'pattern', 
          pattern: 'solid', 
          fgColor: { argb: 'FFFFEBEE' } 
        };
        cell.border = {
          top: { style: 'thin', color: { argb: 'FFFFCDD2' } },
          left: { style: 'thin', color: { argb: 'FFFFCDD2' } },
          bottom: { style: 'thin', color: { argb: 'FFFFCDD2' } },
          right: { style: 'thin', color: { argb: 'FFFFCDD2' } }
        };
      });
    }
  });

  if (options.format === "csv") {
    const csvData = await workbook.csv.writeBuffer();
    return Buffer.from(csvData);
  }

  const buffer = await workbook.xlsx.writeBuffer();
  return Buffer.from(buffer);
}

export async function generateStudentsParentsExport(
  students: {
    studentNumber: string;
    secretNumber?: string | null;
    studentName: string | null;
    studentEmail?: string | null;
    studentSex?: string | null;
    studentStatus: string | null;
    phone: string | null;
    fatherName: string | null;
    fatherPhone: string | null;
    motherName: string | null;
    motherPhone: string | null;
  }[],
  options: { className: string; classColor?: string; format: "xlsx" | "csv" }
): Promise<Buffer> {
  const workbook = new ExcelJS.Workbook();
  const primaryArgb = hexToArgb(options.classColor);
  
  const worksheet = workbook.addWorksheet("الطلاب وأولياء الأمور", {
    views: [{ rightToLeft: true }],
    properties: { tabColor: { argb: primaryArgb } }
  });

  // Modern Styling (2026 best practices: subtle shades, high contrast headers)
  const titleText = `كشف معلومات الطلاب مع أولياء الأمور - ${options.className}`;
  worksheet.addRow([titleText]);
  worksheet.mergeCells(1, 1, 1, 12);
  const titleCell = worksheet.getCell(1, 1);
  titleCell.font = { bold: true, size: 18, color: { argb: primaryArgb } }; // Use class color for title
  titleCell.alignment = { horizontal: "center", vertical: "middle" };
  worksheet.addRow([]); // Spacer

  const columns = [
    "م", 
    "رقم الطالب", 
    "الرقم السري", 
    "اسم الطالب", 
    "الجنس", 
    "البريد الإلكتروني", 
    "الجوال", 
    "اسم الأب", 
    "جوال الأب", 
    "اسم الأم", 
    "جوال الأم", 
    "الحالة"
  ];
  
  const headerRow = worksheet.addRow(columns);
  headerRow.font = { bold: true, color: { argb: "FFFFFFFF" } };
  headerRow.fill = { 
    type: "pattern", 
    pattern: "solid", 
    fgColor: { argb: primaryArgb } 
  };
  headerRow.alignment = { horizontal: "center", vertical: "middle" };
  headerRow.height = 30; // Increased height for better UX

  // Column settings
  const columnWidths = [5, 15, 12, 35, 10, 25, 15, 30, 15, 30, 15, 12];
  columnWidths.forEach((width, idx) => {
    worksheet.getColumn(idx + 1).width = width;
  });

  headerRow.eachCell((cell) => {
    cell.border = {
      top: { style: 'thin', color: { argb: 'FFE0E0E0' } },
      left: { style: 'thin', color: { argb: 'FFE0E0E0' } },
      bottom: { style: 'thin', color: { argb: 'FFE0E0E0' } },
      right: { style: 'thin', color: { argb: 'FFE0E0E0' } }
    };
  });

  // Student sex labels translation
  const sexLabels: Record<string, string> = {
    'male': 'ذكر',
    'female': 'أنثى',
    'unspecified': 'غير محدد'
  };

  students.forEach((student, sIdx) => {
    const rowStatus = student.studentStatus === 'active' ? 'نشط' : 
                      student.studentStatus === 'auditor' ? 'مستمع' :
                      student.studentStatus === 'graduated' ? 'متخرج' :
                      student.studentStatus === 'transferred' ? 'منقول' : 'غير محدد';
    
    const rowData = [
      sIdx + 1,
      student.studentNumber,
      student.secretNumber || "-",
      student.studentName || "-",
      sexLabels[student.studentSex || 'unspecified'] || "-",
      student.studentEmail || "-",
      student.phone || "-",
      student.fatherName || "-",
      student.fatherPhone || "-",
      student.motherName || "-",
      student.motherPhone || "-",
      rowStatus
    ];
    
    const dRow = worksheet.addRow(rowData);
    dRow.alignment = { horizontal: "center", vertical: "middle" };
    
    // Borders for data rows
    dRow.eachCell({ includeEmpty: true }, (cell) => {
      cell.border = {
        top: { style: 'thin', color: { argb: 'FFF0F0F0' } },
        left: { style: 'thin', color: { argb: 'FFF0F0F0' } },
        bottom: { style: 'thin', color: { argb: 'FFF0F0F0' } },
        right: { style: 'thin', color: { argb: 'FFF0F0F0' } }
      };
    });

    // Row Highlighting for auditors (Premium 2026 design: soft highlight)
    if (student.studentStatus === 'auditor') {
      dRow.eachCell({ includeEmpty: true }, (cell) => {
        cell.fill = { 
          type: 'pattern', 
          pattern: 'solid', 
          fgColor: { argb: 'FFFFEBEE' } // Light Soft Salmon/Red
        };
        // Add a slightly stronger border to auditor rows to make them distinct
        cell.border = {
          top: { style: 'thin', color: { argb: 'FFFFCDD2' } },
          left: { style: 'thin', color: { argb: 'FFFFCDD2' } },
          bottom: { style: 'thin', color: { argb: 'FFFFCDD2' } },
          right: { style: 'thin', color: { argb: 'FFFFCDD2' } }
        };
      });
    }
  });

  if (options.format === "csv") {
    return Buffer.from(await workbook.csv.writeBuffer());
  }

  return Buffer.from(await workbook.xlsx.writeBuffer());
}
