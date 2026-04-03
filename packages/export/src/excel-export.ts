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
 * Lightens a hex color by a given percentage
 * @param hex - The hex color (e.g., #RRGGBB)
 * @param percent - Percentage to lighten (0-100)
 * @returns Lightened ARGB string
 */
export function getLightenedArgb(hex?: string, percent: number = 80): string {
  if (!hex) hex = "#4A90E2";
  const cleanHex = hex.replace("#", "");
  
  const r = parseInt(cleanHex.substring(0, 2), 16);
  const g = parseInt(cleanHex.substring(2, 4), 16);
  const b = parseInt(cleanHex.substring(4, 6), 16);

  const lighten = (val: number) => {
    const newVal = Math.floor(val + (255 - val) * (percent / 100));
    return newVal.toString(16).padStart(2, '0').toUpperCase();
  };

  return `FF${lighten(r)}${lighten(g)}${lighten(b)}`;
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

  const worksheet = workbook.addWorksheet("قائمة الطلاب", {
    views: [{ rightToLeft: true }], // Adding RTL support for Arab eyes
    properties: { tabColor: { argb: primaryArgb } },
  });

  // Define columns (in Arabic)
  const columns = [
    { header: "رقم الطالب", key: "studentNumber", width: 20 },
    { header: "الاسم", key: "name", width: 30 },
    { header: "البريد الإلكتروني", key: "email", width: 30 },
    { header: "الجوال", key: "phone", width: 20 },
    { header: "الجنس", key: "sex", width: 15 },
    { header: "المتوسط", key: "average", width: 12 },
    { header: "الحالة", key: "status", width: 15 },
  ];

  const totalCols = columns.length;
  const titleRow = worksheet.addRow([`تقرير بيانات الفصل: ${className}${academicYear ? ` - ${academicYear}` : ""}`]);
  worksheet.mergeCells(titleRow.number, 1, titleRow.number, totalCols);
  titleRow.font = { size: 16, bold: true, color: { argb: primaryArgb } }; // Use primary color for header title
  titleRow.alignment = { horizontal: "center", vertical: "middle" };
  titleRow.height = 30;

  // Add statistics if available
  if (statistics) {
    worksheet.addRow([]);
    const statsRow = worksheet.addRow([
      "إجمالي الطلاب:",
      statistics.totalCount || 0,
      "الناجحين:",
      statistics.passedCount || 0,
      "الراسبين:",
      statistics.failedCount || 0,
      "المتوسط العام:",
      statistics.averageGrade ? `${statistics.averageGrade.toFixed(2)}%` : "N/A",
    ]);
    statsRow.font = { bold: true };
    worksheet.addRow([]);
  } else {
    worksheet.addRow([]);
  }


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
           fgColor: { argb: "#FA9C9C" } // Light red
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
          top: { style: "thin", color: { argb: "FF000000" } },
          left: { style: "thin", color: { argb: "FF000000" } },
          bottom: { style: "thin", color: { argb: "FF000000" } },
          right: { style: "thin", color: { argb: "FF000000" } },
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
  const safeName = className.replace(/[^a-zA-Z0-9-_\u0600-\u06FF]/g, "_"); // Support Arabic characters in filename
  return `${safeName}_طلاب_${timestamp}.${format}`;
}

export interface GradesExportOptions extends ExportOptions {
  subjectName?: string;
  assessmentItems: { id: number; title: string; month?: number; maxScore?: string | number }[];
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
  const worksheet = workbook.addWorksheet("كشف الدرجات", {
    views: [{ rightToLeft: true }],
  });

  const distinctMonths = [...new Set(assessmentItems.map((i) => i.month).filter((m): m is number => m != null))].sort((a, b) => a - b);

  const monthNames: Record<number, string> = {
    1: "محرم", 2: "صفر", 3: "ربيع أول",
    4: "ربيع ثاني", 5: "جمادى أول", 6: "جمادى ثاني",
    7: "رجب", 8: "شعبان", 9: "رمضان", 10: "شوال",
    11: "ذو القعدة", 12: "ذو الحجة"
  };

  const primaryArgb = hexToArgb(options.classColor);
  const softPrimaryArgb = getLightenedArgb(options.classColor, 95);

  const subjectPart = options.subjectName ? ` - مادة: ${options.subjectName}` : "";
  const titleText = `تقرير درجات الفصل: ${className}${academicYear ? ` - ${academicYear}` : ""}${subjectPart}`;
  
  const titleRow = worksheet.addRow([titleText]);
  titleRow.font = { size: 18, bold: true, color: { argb: primaryArgb } };
  titleRow.alignment = { horizontal: "center", vertical: "middle" };
  titleRow.height = 35;

  // Header 1 (Group Row) Setup
  const row1 = ["م", "رقم الطالب", "الاسم"];
  const headerGroups: { title: string; start: number; end: number }[] = [];
  let colIndex = 4;
  
  // Header 2 (Columns Row) Setup
  const row2 = ["", "", ""];
  const finalColumnsConfig: { key: string; width: number }[] = [];

  // Build columns structure
  if (distinctMonths.length === 0) {
     const groupStart = colIndex;
     let mMax = 0;
     assessmentItems.forEach((item) => {
        mMax += parseFloat(String(item.maxScore ?? "0")) || 0;
        row2.push(`${item.title}${item.maxScore ? `\n(${item.maxScore})` : ""}`);
        finalColumnsConfig.push({ key: `item_${item.id}`, width: 12 });
        colIndex++;
     });
     if (assessmentItems.length > 0) {
        row2.push(`المجموع\n(${mMax})`);
        finalColumnsConfig.push({ key: `month_total`, width: 12 });
        colIndex++;
     }
     row1.push(options.subjectName || "التقييمات");
     for(let i=1; i < colIndex - groupStart; i++) row1.push("");
     headerGroups.push({ title: options.subjectName || "التقييمات", start: groupStart, end: colIndex - 1 });
  } else {
     distinctMonths.forEach((m) => {
        const isExamMonth = m === 4 || m === 8;
        const mItems = assessmentItems.filter((i) => i.month === m);
        const groupStart = colIndex;

        if (isExamMonth) {
           row2.push("محصلة الشهور\n(20)");
           finalColumnsConfig.push({ key: `sumOfMonths_${m}`, width: 15 });
           colIndex++;
        }

        let mMax = 0;
        mItems.forEach((item) => {
           mMax += parseFloat(String(item.maxScore ?? "0")) || 0;
           row2.push(`${item.title}${item.maxScore ? `\n(${item.maxScore})` : ""}`);
           finalColumnsConfig.push({ key: `item_${item.id}`, width: 12 });
           colIndex++;
        });

        if (!isExamMonth) {
           row2.push(`المجموع\n(${mMax})`);
           finalColumnsConfig.push({ key: `month_total_${m}`, width: 12 });
        } else {
           row2.push("المجموع الكلي\n(50)");
           finalColumnsConfig.push({ key: `final_total_${m}`, width: 15 });
        }
        colIndex++;

        row1.push(monthNames[m] || `شهر ${m}`);
        for(let i=1; i < colIndex - groupStart; i++) row1.push("");
        headerGroups.push({ title: monthNames[m] || `شهر ${m}`, start: groupStart, end: colIndex - 1 });
     });
  }

  // Insert Header 1 (Group Row)
  const header1 = worksheet.addRow(row1);
  header1.font = { bold: true, size: 12, color: { argb: primaryArgb } }; 
  header1.alignment = { horizontal: "center", vertical: "middle" };
  header1.fill = { type: 'pattern', pattern: 'solid', fgColor: { argb: softPrimaryArgb } }; 

  // Format Title Span
  const totalCols = row1.length;
  worksheet.mergeCells(titleRow.number, 1, titleRow.number, totalCols);

  // Group Merging
  headerGroups.forEach(g => {
    if (g.end > g.start) {
      worksheet.mergeCells(header1.number, g.start, header1.number, g.end);
    }
  });

  // Insert Header 2 (Columns Row)
  const header2 = worksheet.addRow(row2);
  header2.font = { bold: true, color: { argb: "FFFFFFFF" } };
  header2.alignment = { horizontal: "center", vertical: "middle", wrapText: true };
  header2.fill = { type: 'pattern', pattern: 'solid', fgColor: { argb: primaryArgb } };
  header2.height = 35;

  // Merge base identifier columns vertically
  worksheet.mergeCells(header1.number, 1, header2.number, 1); 
  worksheet.mergeCells(header1.number, 2, header2.number, 2); 
  worksheet.mergeCells(header1.number, 3, header2.number, 3); 

  // Style base identifier merged columns like bulk report design
  [1, 2, 3].forEach(col => {
    const cell = header1.getCell(col);
    cell.fill = { type: 'pattern', pattern: 'solid', fgColor: { argb: primaryArgb } };
    cell.font = { bold: true, color: { argb: "FFFFFFFF" } };
  });

  // Set borders for headers
  [header1, header2].forEach(row => {
    row.eachCell({ includeEmpty: true }, (cell) => {
      cell.border = { top: { style: 'thin' }, left: { style: 'thin' }, bottom: { style: 'thin' }, right: { style: 'thin' } };
    });
  });

  // Apply widths
  worksheet.getColumn(1).width = 5;
  worksheet.getColumn(2).width = 20;
  worksheet.getColumn(3).width = 30;
  finalColumnsConfig.forEach((col, idx) => {
    worksheet.getColumn(4 + idx).width = col.width;
  });

  // Data rows
  for (const [i, student] of data.entries()) {
    const rowValues: any[] = [];
    rowValues[0] = i + 1; // "م"
    rowValues[1] = student.studentNumber;
    rowValues[2] = student.name;

    let colPointer = 3;

    if (distinctMonths.length === 0) {
      let mTotal = 0;
      assessmentItems.forEach((item) => {
        const val = student.grades[item.id];
        if (typeof val === "number" || typeof val === "string") {
          const num = parseFloat(val as string);
          if (!isNaN(num)) {
            const roundedVal = Math.round(num);
            rowValues[colPointer++] = roundedVal;
            mTotal += num;
          } else {
             rowValues[colPointer++] = "-";
          }
        } else {
          rowValues[colPointer++] = "-";
        }
      });
      if (assessmentItems.length > 0) {
         rowValues[colPointer++] = Math.round(mTotal);
      }
    } else {
      let allMonthsTotalRaw = 0;
      let allMonthsMaxTotal = 0;
      
      distinctMonths.forEach((m) => {
        const isExamMonth = m === 4 || m === 8;
        const mItems = assessmentItems.filter((i) => i.month === m);
        
        let scaledMonthly = 0;
        if (isExamMonth) {
          scaledMonthly = allMonthsMaxTotal > 0 ? Math.round((allMonthsTotalRaw / allMonthsMaxTotal) * 20) : 0;
          rowValues[colPointer++] = scaledMonthly;
        }

        let mTotal = 0;
        let mMax = 0;
        mItems.forEach((item) => {
          const ms = parseFloat(String(item.maxScore ?? "0"));
          if (!isNaN(ms)) mMax += ms;

          const val = student.grades[item.id];
          
          if (typeof val === "number" || typeof val === "string") {
            const num = parseFloat(val as string);
            if (!isNaN(num)) {
              rowValues[colPointer++] = Math.round(num);
              mTotal += num; // accumulate raw sum for logic scaling accurate decimals
            } else {
              rowValues[colPointer++] = "-";
            }
          } else {
            rowValues[colPointer++] = "-";
          }
        });

        if (!isExamMonth) {
          rowValues[colPointer++] = Math.round(mTotal);
          allMonthsTotalRaw += mTotal;
          allMonthsMaxTotal += mMax;
        } else {
          const scaledExam = mMax > 0 ? Math.round((mTotal / mMax) * 30) : 0;
          rowValues[colPointer++] = Math.round(scaledMonthly + scaledExam);
        }
      });
    }

    const row = worksheet.addRow(rowValues);
    row.eachCell({ includeEmpty: true }, (cell) => {
      cell.border = { top: { style: 'thin' }, left: { style: 'thin' }, bottom: { style: 'thin' }, right: { style: 'thin' } };
    });
    row.alignment = { vertical: "middle", horizontal: "center" };
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
  subjectName: string | undefined | null,
  format: "xlsx" | "csv",
): string {
  const timestamp = new Date()
    .toISOString()
    .replace(/[:.]/g, "-")
    .slice(0, -5);
  const safeClassName = className.replace(/[^a-zA-Z0-9-_\u0600-\u06FF]/g, "_");
  const safeSubjectName = subjectName ? subjectName.replace(/[^a-zA-Z0-9-_\u0600-\u06FF]/g, "_") : "عام";
  return `كشف_درجات_${safeClassName}_${safeSubjectName}_${timestamp}.${format}`;
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
  header1.font = { bold: true, size: 12, color: { argb: primaryArgb } }; // Same as primary color but bold 
  header1.alignment = { horizontal: "center", vertical: "middle" };
  const softPrimaryArgb = getLightenedArgb(options.classColor, 95); // Very soft primary shade
  const mediumSoftPrimaryArgb = getLightenedArgb(options.classColor, 85); // A bit more distinct
  
  header1.fill = { type: 'pattern', pattern: 'solid', fgColor: { argb: softPrimaryArgb } }; 

  const totalCols = row1.length;
  worksheet.mergeCells(titleRow.number, 1, titleRow.number, totalCols);
  titleRow.height = 35; // Taller header for cleaner look

  let colIndex = 4;
  for (const subject of allSubjects) {
    if (subColCount > 1) {
      worksheet.mergeCells(header1.number, colIndex, header1.number, colIndex + subColCount - 1);
    }
    colIndex += subColCount;
  }

  const row2 = ["", "", ""];
  for (const subject of allSubjects) {
    for (let i = 0; i < columnsRef.length; i++) {
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

  // Style the merged name/ID header cells separately to ensure they looks premium
  [1, 2, 3].forEach(colIndex => {
    const cell = header1.getCell(colIndex);
    cell.fill = { type: 'pattern', pattern: 'solid', fgColor: { argb: primaryArgb } };
    cell.font = { bold: true, color: { argb: "FFFFFFFF" } };
  });

  worksheet.getColumn(1).width = 5;
  worksheet.getColumn(2).width = 15;
  worksheet.getColumn(3).width = 35;

  // Set widths for subject columns
  for (let i = 4; i <= row2.length; i++) {
    worksheet.getColumn(i).width = 12;
  }

  [header1, header2].forEach(row => {
    row.eachCell({ includeEmpty: true }, (cell, colIndex) => {
      // Border color should be a subtle version of the primary or a clean gray
      cell.border = {
        top: { style: 'thin', color: { argb: "FF000000" } }, 
        left: { style: 'thin', color: { argb: "FF000000" } },
        bottom: { style: 'thin', color: { argb: "FF000000" } }, 
        right: { style: 'thin', color: { argb: "FF000000" } }
      };
      
      // If subject header and not the name/ID part, use soft shade
      if (row.number === header1.number && colIndex > 3) {
         cell.fill = { type: 'pattern', pattern: 'solid', fgColor: { argb: mediumSoftPrimaryArgb } };
      }
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
    dRow.eachCell({ includeEmpty: true }, (cell, colIndex) => {
       cell.border = {
         top: { style: 'thin', color: { argb: "FF000000" } }, 
         left: { style: 'thin', color: { argb: "FF000000" } },
         bottom: { style: 'thin', color: { argb: "FF000000" } }, 
         right: { style: 'thin', color: { argb: "FF000000" } }
       };

       // Alternate row shading for better readability (Subtle brand touch)
       if (sIdx % 2 !== 0 && colIndex > 3) {
          cell.fill = { type: 'pattern', pattern: 'solid', fgColor: { argb: 'FFF9F9F9' } };
       }

       // Coloring for "Total" (المجموع) column if needed
       // Total columns are every 'subColCount' after col 3
       const isTotalCol = (colIndex - 3) % subColCount === 0 && colIndex > 3;
       if (isTotalCol) {
          cell.font = { bold: true, color: { argb: primaryArgb } }; // Highlight total in Primary color
          cell.fill = { type: 'pattern', pattern: 'solid', fgColor: { argb: softPrimaryArgb } }; // Soft background
       }
    });

    if (student.status === "auditor" || student.status === "homeschooled") {
       dRow.eachCell({ includeEmpty: true }, cell => {
          cell.fill = { 
            type: "pattern", 
            pattern: "solid", 
            fgColor: { argb: "FFFFEBEE" } 
          };
          cell.border = {
            top: { style: 'thin', color: { argb: 'FF000000' } },
            left: { style: 'thin', color: { argb: 'FF000000' } },
            bottom: { style: 'thin', color: { argb: 'FF000000' } },
            right: { style: 'thin', color: { argb: 'FF000000' } }
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
        top: { style: 'thin', color: { argb: 'FF000000' } }, 
        left: { style: 'thin', color: { argb: 'FF000000' } },
        bottom: { style: 'thin', color: { argb: 'FF000000' } }, 
        right: { style: 'thin', color: { argb: 'FF000000' } }
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
        top: { style: 'thin', color: { argb: 'FF000000' } }, 
        left: { style: 'thin', color: { argb: 'FF000000' } },
        bottom: { style: 'thin', color: { argb: 'FF000000' } }, 
        right: { style: 'thin', color: { argb: 'FF000000' } }
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
          top: { style: 'thin', color: { argb: 'FF000000' } },
          left: { style: 'thin', color: { argb: 'FF000000' } },
          bottom: { style: 'thin', color: { argb: 'FF000000' } },
          right: { style: 'thin', color: { argb: 'FF000000' } }
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
      top: { style: 'thin', color: { argb: 'FF000000' } },
      left: { style: 'thin', color: { argb: 'FF000000' } },
      bottom: { style: 'thin', color: { argb: 'FF000000' } },
      right: { style: 'thin', color: { argb: 'FF000000' } }
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
        top: { style: 'thin', color: { argb: 'FF000000' } },
        left: { style: 'thin', color: { argb: 'FF000000' } },
        bottom: { style: 'thin', color: { argb: 'FF000000' } },
        right: { style: 'thin', color: { argb: 'FF000000' } }
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
          top: { style: 'thin', color: { argb: 'FF000000' } },
          left: { style: 'thin', color: { argb: 'FF000000' } },
          bottom: { style: 'thin', color: { argb: 'FF000000' } },
          right: { style: 'thin', color: { argb: 'FF000000' } }
        };
      });
    }
  });

  if (options.format === "csv") {
    return Buffer.from(await workbook.csv.writeBuffer());
  }

  return Buffer.from(await workbook.xlsx.writeBuffer());
}

export function generateAttendanceFilename(
  className: string,
  format: "xlsx" | "csv",
): string {
  const timestamp = new Date().toISOString().replace(/[:.]/g, "-").slice(0, -5);
  const safeName = className.replace(/[^a-zA-Z0-9-_\u0600-\u06FF]/g, "_");
  return `${safeName}_كشف_الحضور_${timestamp}.${format}`;
}

export function generateParentsReportFilename(
  className: string,
  format: "xlsx" | "csv",
): string {
  const timestamp = new Date().toISOString().replace(/[:.]/g, "-").slice(0, -5);
  const safeName = className.replace(/[^a-zA-Z0-9-_\u0600-\u06FF]/g, "_");
  return `${safeName}_بيانات_الأهالي_${timestamp}.${format}`;
}

export function generateBulkReportFilename(
  className: string,
  type: string,
  format: "xlsx" | "csv",
  monthText?: string
): string {
  const timestamp = new Date().toISOString().replace(/[:.]/g, "-").slice(0, -5);
  const safeName = className.replace(/[^a-zA-Z0-9-_\u0600-\u06FF]/g, "_");
  const typeText = type === "monthly" ? "شهري" : type === "half-yearly" ? "نصفي" : type === "final" ? "نهائي" : "درجات";
  const monthPart = monthText ? `_${monthText.replace(/[^a-zA-Z0-9-_\u0600-\u06FF]/g, "_")}` : "";
  return `${safeName}_تقرير_${typeText}${monthPart}_${timestamp}.${format}`;
}

