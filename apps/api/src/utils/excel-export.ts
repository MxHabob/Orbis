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
