import readXlsxFile from "read-excel-file";

export async function parseExcel<T>(file: File): Promise<{ data: T[]; meta: { fields: string[] } }> {
  try {
    const rows = await readXlsxFile(file);
    if (!rows || rows.length === 0) {
      return { data: [], meta: { fields: [] } };
    }

    const firstRow = rows[0];
    if (!firstRow || firstRow.length === 0) {
      return { data: [], meta: { fields: [] } };
    }

    const headers = firstRow.map(String);

    const data: T[] = rows.slice(1).map((row) => {
      const obj: any = {};
      headers.forEach((header, index) => {
        // Convert all values to string to match CSV behavior
        obj[header] = 
          row[index] !== null && row[index] !== undefined 
            ? String(row[index]) 
            : "";
      });
      return obj as T;
    });

    return {
      data,
      meta: { fields: headers },
    };
  } catch (error) {
    console.error("Error parsing Excel file:", error);
    throw new Error("Failed to parse Excel file");
  }
}
